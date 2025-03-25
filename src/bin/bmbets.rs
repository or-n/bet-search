use eat::*;
use fantoccini::{Client, ClientBuilder};
use futures::stream::StreamExt;
use odds::bmbets::{
    football::goto,
    search::{find_match, hits, Hit},
    URL,
};
use odds::fortuna;
use odds::shared::event;
use odds::utils::{browser, page::Name, read};
use scraper::Html;
use serde_json::{json, Map};
use std::io;
use std::io::Write;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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

async fn process_match(
    m: &event::Match<event::Football, String>,
    client: &mut Client,
) {
    let start = Instant::now();
    println!("{}", m.date.format("%Y-%m-%d %H:%M"));
    println!("{} - {}", m.players[0], m.players[1]);
    let hit = get_hit(client).await;
    println!("{} - {}", hit.players[0], hit.players[1]);
    println!("{}{}", URL, hit.relative_url);
    println!("Elapsed time: {:.2?}", start.elapsed());
    let start = Instant::now();
    client.goto(&hit.relative_url).await.unwrap();
    let new_events = futures::stream::iter(m.events.iter()).filter_map(|e| {
        let mut client = client.clone();
        async move {
            match goto(&mut client, e).await {
                Ok(new_e) => {
                    if new_e.odds.is_empty() {
                        return None;
                    }
                    Some(new_e)
                }
                Err(error) => {
                    println!("{:?}", e);
                    println!("{:?}", error);
                    None
                }
            }
        }
    });
    let new_events: Vec<_> = new_events.collect().await;
    let new_m = event::Match {
        url: m.url.clone(),
        date: m.date.clone(),
        players: m.players.clone(),
        events: new_events,
    };
    let Some(contents) = event::match_contents(&new_m) else {
        return;
    };
    let r = fortuna::Url::eat(new_m.url.as_str(), ());
    let (_i, url) = r.unwrap();
    let path = format!("safe/{}", url.name());
    let mut file = File::create(&path).await.unwrap();
    file.write_all(contents.as_bytes()).await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}

#[tokio::main]
async fn main() {
    let files = read::files("maybe_safe").unwrap();
    let matches = files
        .filter_map(|file| event::eat_match(file.as_str()).ok())
        .filter_map(fortuna::safe::football_match);
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
        process_match(&m, &mut client).await;
    }
    client.close().await.unwrap();
}
