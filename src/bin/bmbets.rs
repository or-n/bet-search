use fantoccini::{Client, ClientBuilder};
use odds::bmbets::{
    football::goto,
    search::{find_match, hits, Hit},
    URL,
};
use odds::fortuna::safe;
use odds::shared::event;
use odds::utils::browser;
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
        println!("{id}: {} - {}", hit.players[0], hit.players[1]);
    }
    let mut id = get_id()?;
    while id >= hits.len() {
        id = get_id()?;
    }
    Some(hits[id].clone())
}

#[tokio::main]
async fn main() {
    let matches = safe::get_safe_matches().await;
    if matches.is_empty() {
        println!("no matches");
        return;
    }
    let match_id = 0;
    let m = &matches[match_id];
    println!("{} - {}", m.players[0], m.players[1]);
    let start = Instant::now();
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
    let hit = get_hit(&mut client).await;
    println!("{} - {}", hit.players[0], hit.players[1]);
    println!("{}{}", URL, hit.relative_url);
    println!("Elapsed time: {:.2?}", start.elapsed());
    let start = Instant::now();
    client.goto(&hit.relative_url).await.unwrap();
    for e in &m.events {
        if let event::Football::Unknown(_) = e.id {
            continue;
        }
        if let Err(error) = goto(&mut client, e).await {
            println!("{:?}", e);
            println!("{:?}", error);
            return;
        }
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
