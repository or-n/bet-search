use eat::*;
use event::football::Football;
use fantoccini::{Client, ClientBuilder};
use futures::stream::StreamExt;
use odds::bmbets::{
    football::goto,
    search::{find_match, hits, Hit},
    URL,
};
use odds::fortuna;
use odds::shared::event;
use odds::utils::{browser, page::Name, read, save::save};
use scraper::Html;
use serde_json::{json, Map};
use std::io;
use std::io::Write;
use std::time::Instant;

fn get_id() -> Option<usize> {
    print!("choose: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().ok()
}

async fn get_hit(client: &mut Client) -> Hit {
    loop {
        print!("search: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let Some(hit) = get_match(client, &input).await else {
            println!("no hits");
            continue;
        };
        return hit;
    }
}

async fn get_match(client: &mut Client, prompt: &str) -> Option<Hit> {
    let html = find_match(client, prompt).await.unwrap();
    let document = Html::parse_document(&html);
    let hits = hits(document);
    if hits.is_empty() {
        return None;
    }
    for (id, hit) in hits.iter().enumerate() {
        println!(
            "{id}: {} | {} - {}",
            hit.date.format("%Y-%m-%d %H:%M"),
            hit.players[0],
            hit.players[1]
        );
    }
    let mut id = get_id()?;
    while id >= hits.len() {
        id = get_id()?;
    }
    Some(hits[id].clone())
}

async fn match_filter(
    m: event::Match<Football, String>,
    client: &mut Client,
) -> event::Match<Football, String> {
    let start = Instant::now();
    println!("{}", m.date.format("%Y-%m-%d %H:%M"));
    println!("{} - {}", m.players[0], m.players[1]);
    let hit = get_hit(client).await;
    println!("{} - {}", hit.players[0], hit.players[1]);
    println!("{}{}", URL, hit.relative_url);
    println!("Elapsed time: {:.2?}", start.elapsed());
    let start = Instant::now();
    client.goto(&hit.relative_url).await.unwrap();
    let events = futures::stream::iter(m.events.iter()).filter_map(|e| {
        let mut client = client.clone();
        async move {
            match goto(&mut client, e).await {
                Ok(e) => {
                    if e.odds.is_empty() {
                        return None;
                    }
                    Some(e)
                }
                Err(error) => {
                    println!("{:?}", e);
                    println!("{:?}", error);
                    None
                }
            }
        }
    });
    let events: Vec<_> = events.collect().await;
    println!("Elapsed time: {:.2?}", start.elapsed());
    event::Match {
        url: m.url,
        date: m.date,
        players: m.players,
        events,
    }
}

#[tokio::main]
async fn main() {
    let files = read::files("maybe_safe").unwrap();
    let matches = files
        .filter_map(|file| event::eat_match(file.as_str()).ok())
        .filter_map(fortuna::event::football::translate_match);
    let matches: Vec<_> = matches.collect();
    if matches.is_empty() {
        println!("no matches");
        return;
    }
    let caps = json!({
        "moz:firefoxOptions": {},
        "pageLoadStrategy": "eager"
    });
    let caps: Map<_, _> = caps.as_object().unwrap().clone();
    let mut client = ClientBuilder::native()
        .capabilities(caps)
        .connect(&browser::localhost(4444))
        .await
        .unwrap();
    for m in matches {
        let m = match_filter(m, &mut client).await;
        if m.events.is_empty() {
            continue;
        }
        let Some(contents) = event::match_contents(&m) else {
            continue;
        };
        let Ok((_i, url)) = fortuna::Url::eat(m.url.as_str(), ()) else {
            continue;
        };
        let file = format!("safe/{}", url.name());
        let _ = save(contents.as_bytes(), file).await;
    }
    client.close().await.unwrap();
}
