use odds::bmbets::search::{find_match, hits};
use odds::shared::event;
use odds::utils::browser;
use scraper::Html;
use std::fs;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let entries = fs::read_dir("downloads").unwrap();
    let matches = entries.filter_map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path().to_string_lossy().into_owned();
        let contents = fs::read_to_string(&path).unwrap();
        event::eat_match(&contents).ok()
    });
    for m in matches {
        println!("{:?}", m);
    }
    let start = Instant::now();
    let mut client = browser::connect(4444).await;
    let html = find_match(&mut client).await.unwrap();
    let document = Html::parse_document(&html);
    let hits = hits(document);
    for (players, link) in hits {
        println!("{} - {} | {}", players[0], players[1], link);
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
