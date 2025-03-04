use fantoccini::ClientBuilder;
use odds::bmbets::search::{find_match, hits};
use odds::shared::event;
use odds::utils::browser;
use scraper::Html;
use serde_json::{json, Map};
use std::fs;
use std::io;
use std::io::Write;
use std::time::Instant;
use tokio::time::{sleep, Duration};

const URL: &str = "https://www.bmbets.com";

fn get_id() -> usize {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(number) = input.trim().parse() {
            return number;
        }
    }
}

#[tokio::main]
async fn main() {
    let entries = fs::read_dir("downloads").unwrap();
    let matches = entries.filter_map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path().to_string_lossy().into_owned();
        let contents = fs::read_to_string(&path).unwrap();
        event::eat_match(&contents).ok()
    });
    let matches: Vec<_> = matches.collect();
    if matches.is_empty() {
        println!("no matches");
        return;
    }
    let id = 0;
    let m = &matches[id];
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
    let html = find_match(&mut client, &m.players[0]).await.unwrap();
    let document = Html::parse_document(&html);
    let hits = hits(document);
    if hits.is_empty() {
        println!("no hits");
        return;
    }
    for (id, (players, _url)) in hits.iter().enumerate() {
        println!("{id}: {} - {}", players[0], players[1]);
    }
    print!("choose: ");
    io::stdout().flush().unwrap();
    let id = get_id();
    let (players, relative_url) = &hits[id];
    let url = format!("{}{}", URL, relative_url);
    println!("{} - {}", players[0], players[1]);
    println!("{}", url);
    client.goto(&url).await.unwrap();
    sleep(Duration::from_secs(2)).await;
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
