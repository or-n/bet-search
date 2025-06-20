use chrono::{Duration, Utc};
use dotenv::dotenv;
use eat::*;
use fantoccini::{error::CmdError, Client, ClientBuilder};
use odds::{
    bmbets::{football, menu},
    shared::db,
    utils::browser,
};
use serde_json::{json, Map};
use std::time::Instant;
use tokio::time;

async fn goto(
    client: &mut Client,
    event: db::EventWithOdd,
) -> Result<(), CmdError> {
    let e = event.without_odd();
    let event_tabs = football::tab(e);
    menu::dropdown(client).await?;
    let tab_links = menu::tab_links(client).await?;
    for (name, button) in tab_links {
        if let Ok(("", tab)) = football::Tab::eat(name.as_str(), ()) {
            if event_tabs.contains(&tab) {
                println!("{:?}", tab);
                button.click().await?;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let odds_range = [db::prematch_odds_min(), db::prematch_odds_max()];
    let db = db::connect().await;
    let match_urls = {
        let match_ids: Vec<_> = {
            let now = Utc::now();
            let later = now + Duration::hours(db::prematch_hours());
            let bmbets =
                db::matches_date(&db, [now, later], db::Source::Bmbets);
            let ids = bmbets.await.unwrap_or_else(|error| {
                println!("{:?}", error);
                panic!()
            });
            ids.into_iter().map(|x| x.id).collect()
        };
        let urls = db::fetch_match_urls(&db, match_ids, db::Source::Bmbets);
        urls.await.unwrap_or_else(|error| {
            println!("{:?}", error);
            panic!()
        })
    };
    println!("matches: {}", match_urls.len());
    println!("Elapsed time: {:.2?}", start.elapsed());
    let mut client = {
        let caps = json!({
            "moz:firefoxOptions": {},
            "pageLoadStrategy": "eager"
        });
        let caps: Map<_, _> = caps.as_object().unwrap().clone();
        ClientBuilder::native()
            .capabilities(caps)
            .connect(&browser::localhost(4444))
            .await
            .unwrap()
    };
    let _ = client.goto("https://bmbets.com").await;
    time::sleep(time::Duration::from_secs(4)).await;
    for match_url in match_urls {
        let m = match_url.m;
        let url = match_url.url;
        println!("{} - {}", m.player1, m.player2);
        println!("{}", m.id);
        let _ = client.goto(&url).await;
        time::sleep(time::Duration::from_secs(1)).await;
        let events = {
            let events =
                db::events_match_odd(&db, m.id, db::Book::Fortuna, odds_range);
            events.await.unwrap_or_else(|error| {
                println!("{:?}", error);
                vec![]
            })
        };
        for event in events {
            println!("{:?}", event);
            let _ = goto(&mut client, event).await;
        }
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
