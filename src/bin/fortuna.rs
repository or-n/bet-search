use dotenv::dotenv;
use fantoccini::ClientBuilder;
use odds::{
    fortuna::{
        self,
        event::football::{translate_db, Football, FootballOption},
    },
    shared::{db, event::translate_event2},
    utils::{browser, download::Download, page::Tag},
};
use std::{sync::Arc, time::Instant};
use surrealdb::{engine::remote::ws::Client, Error, Surreal};
use tokio::sync::Mutex;

async fn get_matches(db: &Surreal<Client>) -> Result<Vec<db::MatchUrl>, Error> {
    db.query(
        "SELECT in, url FROM on
        WHERE out = book:fortuna
        AND in.date > time::now()
        AND in.date < time::now() + 12h
        FETCH in;
    ",
    )
    .await?
    .take(0)
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let db = db::connect().await;
    let match_urls = match get_matches(&db).await {
        Ok(match_urls) => match_urls,
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
    let mut save_count = 0;
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
                m: m.id,
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
            let r = translate_event2::<Football, FootballOption>(
                event.clone(),
                players.clone(),
            );
            if let Some(football_event) = r {
                println!("{:?}", football_event);
                for (option, _odd) in football_event.odds {
                    let r = translate_db(football_event.id, option);
                    if let Ok(db_event) = r {
                        println!("{:?}", db_event);
                    }
                }
            }
        }
        // let events = shared::event::match_events_to_db(&m);
        // for event in events {
        //     let id = m.db_id();
        //     let _response = db
        //         .query(format!("CREATE real_event:{id} SET event={event};"))
        //         .await
        //         .unwrap();
        //     println!("saved {id}");
        //     save_count += 1;
        // }
    }
    client.close().await.unwrap();
    let elapsed = start.elapsed().as_secs_f32();
    println!("Elapsed time: {:.2?}", elapsed);
    println!("Total count: {}", total_count);
    println!("Download count: {}", download_count);
    println!("Save count: {}", save_count);
    println!("{:.2?} / download", elapsed / download_count as f32);
}
