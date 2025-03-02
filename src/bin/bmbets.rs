use odds::bmbets::search::{find_match, hits};
use odds::utils::browser;
use scraper::Html;
use std::time::Instant;

#[tokio::main]
async fn main() {
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
