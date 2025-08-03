use chrono::{Duration, Utc};
use dotenv::dotenv;
use fantoccini::{Client, ClientBuilder};
use odds::{
    adapter::browser,
    fortuna::{
        self,
        event::football::{translate_db, Football, FootballOption},
    },
    shared::{db, event::translate_event},
    utils::{self, download::Download, page::Tag},
};
use std::collections::HashSet;
use std::{sync::Arc, time::Instant};
use surrealdb::{engine::remote::ws, Error, Surreal};
use tokio::sync::Mutex;

async fn save_match_odds(
    client: &Client,
    db: &Surreal<ws::Client>,
    m: db::MatchWithId,
    url: String,
) {
    println!("{} - {}", m.player1, m.player2);
    let players = [m.player1, m.player2];
    let events = {
        let subpage = fortuna::prematch::football::subpage::Page(url.clone());
        let interest = {
            let mut xs = HashSet::new();
            use fortuna::event::football::Football::*;
            xs.insert(Win);
            xs.insert(NotWin);
            xs.insert(Goals);
            xs
        };
        let data = (subpage.clone(), interest, players.clone());
        let result = Tag::download(client, data).await;
        match result {
            Ok(html) => html.document().events(),
            Err(error) => {
                println!("download: {:#?}", error);
                vec![]
            }
        }
    };
    println!("{:#?}", events);
    let download_record = {
        let download: Result<Option<db::Record>, Error> = db
            .create("download")
            .content(db::Download {
                date: Utc::now().into(),
                m: m.id.clone(),
                source: db::Source::Fortuna,
            })
            .await;
        match download {
            Ok(option) => option.unwrap(),
            Err(error) => panic!("download record: {:#?}", error),
        }
    };
    for event in events {
        let football_event = {
            let translate = translate_event::<Football, FootballOption>(
                event.clone(),
                players.clone(),
            );
            match translate {
                Some(x) => x,
                _ => continue,
            }
        };
        // println!("{:#?}", football_event);
        for (option, odd) in football_event.odds {
            let db_event = match translate_db(football_event.id, option) {
                Ok(x) => x,
                Err(()) => continue,
            };
            println!("{:?}", db_event);
            let event_record: db::Record = {
                let match_event = db::MatchEvent {
                    m: m.id.clone(),
                    event: db_event.clone(),
                };
                let exists = {
                    let ids = db::event_ids(&db, match_event.clone());
                    ids.await.unwrap_or(vec![]).into_iter().next()
                };
                if let Some(id) = exists {
                    id
                } else {
                    let create_event: Result<Option<db::Record>, Error> =
                        db.create("real_event").content(match_event).await;
                    let option = create_event.unwrap_or_else(|error| {
                        panic!("{:#?}", error);
                    });
                    option.unwrap()
                }
            };
            let relate = db
                .query(
                    "RELATE book:fortuna->offers->$event SET
                        odd=$odd,
                        download=$download;",
                )
                .bind(("event", event_record.id))
                .bind(("odd", odd))
                .bind(("download", download_record.id.clone()));
            match relate.await {
                Ok(_) => println!("saved odd {}", odd),
                Err(error) => println!("relate: {:#?}", error),
            }
        }
    }
}

async fn save_football_odds(client: &Client, db: &Surreal<ws::Client>) {
    let start = Instant::now();
    let match_urls = {
        let match_ids = {
            let now = Utc::now();
            let later = now + Duration::hours(db::prematch_hours());
            let ids = db::select_match(
                &db,
                [now, later],
                Some("Ekstraklasa Polska".into()),
            );
            let ids = ids.await.unwrap_or_else(|error| {
                panic!("ids: {:#?}", error);
            });
            ids.into_iter().map(|x| x.id).collect()
        };
        let urls = db::fetch_match_urls(&db, match_ids, db::Source::Fortuna);
        urls.await.unwrap_or_else(|error| {
            panic!("urls: {:#?}", error);
        })
    };
    println!("Elapsed time: {:.2?}", start.elapsed());
    let total_count = match_urls.len();
    println!("Total count: {}", total_count);
    let queue = Arc::new(Mutex::new(match_urls));
    let start = Instant::now();
    let url = {
        use fortuna::prematch::*;
        format!("{}{}", URL, football::URL)
    };
    client.goto(url.as_str()).await.unwrap();
    browser::try_accepting_cookie(client, fortuna::COOKIE_ACCEPT)
        .await
        .unwrap();
    while let Some(match_url) = queue.lock().await.pop() {
        save_match_odds(client, db, match_url.m, match_url.url).await;
    }
    let elapsed = start.elapsed().as_secs_f32();
    println!("Total count: {}", total_count);
    println!("Elapsed time: {:.2?}", elapsed);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = db::connect().await;
    let client = ClientBuilder::native()
        .connect(&utils::localhost(4444))
        .await
        .unwrap();
    save_football_odds(&client, &db).await;
    client.close().await.unwrap();
}
