use chrono::Utc;
use dotenv::dotenv;
use eat::*;
use fantoccini::{error::CmdError, Client, ClientBuilder};
use odds::{
    bmbets::{self, football, menu},
    shared::db,
    utils,
};
use serde_json::{json, Map};
use std::{fmt::Debug, time::Instant};
use tokio::time::{sleep, Duration};

fn eat_and_find<Id, T>(
    label: &str,
    x_to_find: Id,
    xs: Vec<(String, T)>,
) -> Option<(usize, Id, T)>
where
    Id: for<'a> Eat<&'a str, (), ()> + PartialEq + Debug,
{
    xs.into_iter().enumerate().find_map(|(idx, (i, value))| {
        let (remains, x) = match Id::eat(&i, ()).ok() {
            Some(x) => x,
            _ => panic!("{}: {:?}", label, i),
        };
        if !remains.is_empty() {
            println!("label: {}, out: {:?}, remains: {:?}", label, x, remains);
            return None;
        }
        if x != x_to_find {
            return None;
        }
        Some((idx, x, value))
    })
}

async fn goto(client: &Client, e: db::Event) -> Result<(), CmdError> {
    let event_tabs = football::tab(e.clone());
    for event_tab in event_tabs {
        let (_, tab, button) = {
            menu::dropdown(client).await?;
            let links = menu::tab_links(client).await?;
            match eat_and_find("tab", event_tab, links) {
                Some(x) => x,
                _ => {
                    println!("no tab");
                    continue;
                }
            }
        };
        button.click().await?;
        let (_, toolbar, button) = {
            let event_toolbar = match football::toolbar(e.clone()) {
                Some(x) => x,
                _ => {
                    println!("event with no toolbar");
                    continue;
                }
            };
            let links = menu::toolbar_links(client, tab.tbar()).await?;
            match eat_and_find("toolbar", event_toolbar, links) {
                Some(x) => x,
                _ => {
                    println!("no toolbar");
                    continue;
                }
            }
        };
        println!("{:?} {:?}", tab, toolbar);
        button.click().await?;
        sleep(Duration::from_millis(1000)).await;
        let odds_content = menu::odds_content(client).await?;
        let odds_div = match football::variant(e.clone(), tab.clone())
            .into_iter()
            .next()
        {
            Some(event_variant) => {
                let links = menu::variants(odds_content).await?;
                match eat_and_find("variant", event_variant, links) {
                    Some((_, variant, button)) => {
                        println!("{:?}", variant);
                        button.click().await?;
                        button
                    }
                    _ => {
                        println!("no variant");
                        continue;
                    }
                }
            }
            _ => odds_content,
        };
        let odds = menu::odds_table(odds_div).await?;
        let n = odds.len();
        let iter = odds.into_iter().map(|(_book, odds)| odds);
        let r = utils::sum_columns(iter).into_iter().map(|x| x / n as f32);
        let r: Vec<_> = r.collect();
        println!("bmbets: {:?}", r);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let db = db::connect().await;
    let match_urls = {
        let match_ids: Vec<_> = {
            let now = Utc::now();
            let later = now + chrono::Duration::hours(db::prematch_hours());
            let bmbets =
                db::select_in_match(&db, [now, later], db::Source::Bmbets);
            let ids = bmbets.await.unwrap_or_else(|error| {
                panic!("{:#?}", error);
            });
            ids.into_iter().map(|x| x.id).collect()
        };
        let urls = db::fetch_match_urls(&db, match_ids, db::Source::Bmbets);
        urls.await.unwrap_or_else(|error| {
            panic!("{:#?}", error);
        })
    };
    println!("matches: {}", match_urls.len());
    println!("Elapsed time: {:.2?}", start.elapsed());
    let client = {
        let caps = json!({
            "moz:firefoxOptions": {},
            "pageLoadStrategy": "eager"
        });
        let caps: Map<_, _> = caps.as_object().unwrap().clone();
        ClientBuilder::native()
            .capabilities(caps)
            .connect(&utils::localhost(4444))
            .await
            .unwrap()
    };
    let odds_range = [db::prematch_odds_min(), db::prematch_odds_max()];
    for match_url in match_urls {
        let m = match_url.m;
        let url = format!("{}{}", bmbets::URL, match_url.url);
        println!("{} - {}", m.player1, m.player2);
        println!("{}", m.id);
        client.goto(&url).await.unwrap();
        let events = {
            let events =
                db::events_match_odd(&db, m.id, db::Book::Fortuna, odds_range);
            events.await.unwrap_or_else(|error| {
                panic!("{:#?}", error);
            })
        };
        let latest_download_date =
            events.iter().map(|e| e.download.date.clone()).max();
        for event in events {
            let date = event.download.date.clone();
            if Some(date) != latest_download_date {
                continue;
            }
            let odd = event.odd;
            let event = event.without_odd_and_download();
            println!("{:?}", event);
            println!("fortuna: {}", odd);
            match goto(&client, event).await {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("{:#?}", error);
                    continue;
                }
            }
        }
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
