use eat::*;
use event::{football::Football, Event, Match};
use fantoccini::{Client, ClientBuilder};
use futures::stream::StreamExt;
use io::Write;
use odds::{
    bmbets::{
        football::goto,
        search::{find_match, hits, Hit},
        URL,
    },
    fortuna,
    shared::event,
    utils::{browser, page::Name, read, save::save},
};
use scraper::Html;
use serde_json::{json, Map};
use std::io;

fn get_id() -> Option<Option<usize>> {
    print!("choose: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trim = input.trim();
    if trim == "-" {
        return Some(None);
    }
    trim.parse().ok().map(Some)
}

async fn get_hit(client: &mut Client, m: &Match<Football, String>) -> Hit {
    loop {
        println!("{}", m.date.format("%Y-%m-%d %H:%M"));
        println!("{} - {}", m.players[0], m.players[1]);
        print!("search: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let Some(result) = get_match(client, &input).await else {
            println!("no hits");
            continue;
        };
        if let Some(hit) = result {
            return hit;
        }
    }
}

async fn get_match(client: &mut Client, prompt: &str) -> Option<Option<Hit>> {
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
    let Some(mut id) = get_id()? else {
        return Some(None);
    };
    while id >= hits.len() {
        match get_id()? {
            Some(new_id) => id = new_id,
            _ => return Some(None),
        }
    }
    Some(Some(hits[id].clone()))
}

async fn match_filter(
    m: Match<Football, String>,
    client: &mut Client,
) -> Match<Football, String> {
    let hit = get_hit(client, &m).await;
    println!("{}{}", URL, hit.relative_url);
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
    Match {
        url: m.url,
        date: m.date,
        players: m.players,
        events,
    }
}

fn filter_event(
    event: Event<Football, String>,
) -> Option<Event<Football, String>> {
    use Football::*;
    if !matches!(event.id, Goals(_)) {
        return None;
    }
    Some(event)
}

#[tokio::main]
async fn main() {
    let files = read::files("maybe_safe").unwrap();
    let matches = files
        .filter_map(|file| event::eat_match(file.as_str()).ok())
        .filter_map(|m| {
            fortuna::event::football::translate_match(m, filter_event)
        });
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
        let Ok((_i, url)) = fortuna::Url::eat(m.url.as_str(), ()) else {
            continue;
        };
        let Some(contents) = event::match_contents(&m) else {
            continue;
        };
        let file = format!("safe/{}", url.name());
        let _ = save(contents.as_bytes(), file).await;
    }
    client.close().await.unwrap();
}
