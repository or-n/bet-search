use dotenv::dotenv;
use fantoccini::ClientBuilder;
use odds::{
    fortuna::{
        self,
        event::football::{Football, FootballOption},
        prematch::football,
    },
    shared::{self, book::Subpages, db, event::translate_match_events},
    utils::{
        browser, date,
        download::Download,
        page::{Tag, Url},
    },
};
use std::{sync::Arc, time::Instant};
use surrealdb::{engine::remote::ws::Client, error::Api, Error, Surreal};
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
        let Some(_fortuna_m) =
            football::subpage::to_match_events(subpage.url(), html.document())
        else {
            continue;
        };
        let r: Result<Option<db::Record>, Error> = db
            .create("download")
            .content(db::Download {
                date: chrono::Utc::now().into(),
                m: m.id,
                source: db::Source::Fortuna,
            })
            .await;
        println!("{:?}", r);
        // let Some(m) = translate_match_events::<Football, FootballOption>(m)
        // else {
        //     continue;
        // };
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
