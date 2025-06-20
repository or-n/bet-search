use chrono::{Duration, Utc};
use dotenv::dotenv;
use fantoccini::ClientBuilder;
use odds::{
    fortuna::{
        self,
        event::football::{translate_db, Football, FootballOption},
    },
    shared::{db, event::translate_event},
    utils::{browser, download::Download, page::Tag},
};
use std::{sync::Arc, time::Instant};
use surrealdb::Error;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let db = db::connect().await;
    let match_urls = {
        let now = Utc::now();
        let later = now + Duration::hours(db::prematch_hours());
        let ids = db::matches_date(&db, [now, later], db::Source::Fortuna);
        let ids = ids.await.unwrap_or_else(|error| {
            println!("{:?}", error);
            panic!()
        });
        let ids = ids.into_iter().map(|x| x.id).collect();
        let urls = db::fetch_match_urls(&db, ids, db::Source::Fortuna);
        urls.await.unwrap_or_else(|error| {
            println!("{:?}", error);
            panic!()
        })
    };
    println!("Elapsed time: {:.2?}", start.elapsed());
    let mut client = ClientBuilder::native()
        .connect(&browser::localhost(4444))
        .await
        .unwrap();
    let total_count = match_urls.len();
    println!("Total count: {}", total_count);
    let queue = Arc::new(Mutex::new(match_urls));
    let start = Instant::now();
    let mut download_count = 0;
    let save_count = 0;
    while let Some(match_url) = queue.lock().await.pop() {
        let m = match_url.m;
        let url = match_url.url;
        println!("{} - {}", m.player1, m.player2);
        let events = {
            let subpage =
                fortuna::prematch::football::subpage::Page(url.clone());
            let html =
                Tag::download(&mut client, subpage.clone()).await.unwrap();
            download_count += 1;
            html.document().events()
        };
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
                Err(error) => {
                    println!("{:?}", error);
                    continue;
                }
            }
        };
        let players = [m.player1, m.player2];
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
            println!("{:?}", football_event);
            for (option, odd) in football_event.odds {
                let db_event = {
                    let translate = translate_db(football_event.id, option);
                    match translate {
                        Ok(x) => x,
                        Err(()) => continue,
                    }
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
                        println!("Ok(EXISTS)");
                        id
                    } else {
                        let create_event: Result<Option<db::Record>, Error> =
                            db.create("real_event").content(match_event).await;
                        let option = create_event.unwrap_or_else(|error| {
                            println!("{:?}", error);
                            panic!()
                        });
                        println!("Ok(CREATE)");
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
                    Ok(_) => println!("Ok(RELATE)"),
                    Err(error) => println!("{:?}", error),
                }
            }
        }
    }
    client.close().await.unwrap();
    let elapsed = start.elapsed().as_secs_f32();
    println!("Elapsed time: {:.2?}", elapsed);
    println!("Total count: {}", total_count);
    println!("Download count: {}", download_count);
    println!("Save count: {}", save_count);
    println!("{:.2?} / download", elapsed / download_count as f32);
}
