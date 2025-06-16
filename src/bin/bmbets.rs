use chrono::{Duration, Utc};
use dotenv::dotenv;
use fantoccini::ClientBuilder;
use odds::{shared::db, utils::browser};
use serde_json::{json, Map};
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let db = db::connect().await;
    let now = Utc::now();
    let later = now + Duration::hours(db::prematch_hours());
    let bmbets = db::matches_date(&db, [now, later], db::Source::Bmbets);
    let bmbets = match bmbets.await {
        Ok(ids) => ids,
        Err(error) => {
            println!("{:?}", error);
            return;
        }
    };
    let match_ids: Vec<_> = bmbets.into_iter().map(|x| x.id).collect();
    let match_urls =
        match db::fetch_match_urls(&db, match_ids, db::Source::Bmbets).await {
            Ok(xs) => xs,
            Err(error) => {
                println!("{:?}", error);
                return;
            }
        };
    println!("matches: {}", match_urls.len());
    println!("Elapsed time: {:.2?}", start.elapsed());
    let caps = json!({
        "moz:firefoxOptions": {},
        "pageLoadStrategy": "eager"
    });
    let caps: Map<_, _> = caps.as_object().unwrap().clone();
    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect(&browser::localhost(4444))
        .await
        .unwrap();
    for match_url in match_urls {
        let m = match_url.m;
        let url = match_url.url;
        println!("{} - {}", m.player1, m.player2);
        println!("{}", url);
        let events =
            db::events_match_odd(&db, m.id, db::Book::Fortuna, [3., 3.5]);
        let events = match events.await {
            Ok(x) => x,
            Err(error) => {
                println!("{:?}", error);
                continue;
            }
        };
        for event in events {
            println!("{:?}", event);
        }
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
