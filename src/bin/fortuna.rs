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
    let now = chrono::Utc::now();
    let later = now + chrono::Duration::hours(12);
    let match_ids =
        match db::immediate_matches(&db, now, later, db::Source::Fortuna).await
        {
            Ok(xs) => xs,
            Err(error) => {
                println!("{:?}", error);
                return;
            }
        };
    let match_ids = match_ids.into_iter().map(|x| x.id).collect();
    let match_urls =
        match db::fetch_match_urls(&db, match_ids, db::Source::Fortuna).await {
            Ok(xs) => xs,
            Err(error) => {
                println!("{:?}", error);
                return;
            }
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
        let subpage = fortuna::prematch::football::subpage::Page(url.clone());
        let html = Tag::download(&mut client, subpage.clone()).await.unwrap();
        download_count += 1;
        let r: Result<Option<db::Record>, Error> = db
            .create("download")
            .content(db::Download {
                date: chrono::Utc::now().into(),
                m: m.id.clone(),
                source: db::Source::Fortuna,
            })
            .await;
        if let Err(error) = r {
            println!("{:?}", error);
            continue;
        }
        let players = [m.player1, m.player2];
        let events = html.document().events();
        for event in events {
            let r = translate_event::<Football, FootballOption>(
                event.clone(),
                players.clone(),
            );
            if let Some(football_event) = r {
                println!("{:?}", football_event);
                for (option, _odd) in football_event.odds {
                    let r = translate_db(football_event.id, option);
                    if let Ok(db_event) = r {
                        println!("{:?}", db_event);
                        let r: Result<Option<db::Record>, Error> = db
                            .create("real_event")
                            .content(db::MatchEvent {
                                m: m.id.clone(),
                                event: db_event.clone(),
                            })
                            .await;
                        println!("{:?}", r);
                    }
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
