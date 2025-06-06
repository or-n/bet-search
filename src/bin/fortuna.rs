use chrono::{Duration, Local};
use dotenv::dotenv;
use fantoccini::ClientBuilder;
use odds::{
    fortuna::{
        self,
        event::football::{Football, FootballOption},
        prematch::football,
    },
    shared::{self, book::Subpages, event::translate_match_events},
    utils::{
        browser,
        download::Download,
        page::{Tag, Url},
    },
};
use std::{env, sync::Arc, time::Instant};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let url = env::var("DB_URL").expect("DB_URL");
    let user = env::var("DB_USERNAME").expect("DB_USERNAME");
    let pass = env::var("DB_PASSWORD").expect("DB_PASSWORD");
    println!("{} {}", url, user);
    let db = Surreal::new::<Ws>(&url).await.expect("DB connect");
    println!("connected");
    db.signin(Root {
        username: &user,
        password: &pass,
    })
    .await
    .expect("DB auth");
    db.use_ns("bet").use_db("bet").await.expect("DB namespace");
    let mut client = ClientBuilder::native()
        .connect(&browser::localhost(4444))
        .await
        .unwrap();
    let page = fortuna::prematch::football::Page;
    let html = Tag::download(&mut client, page).await.unwrap();
    let matches = html.document().matches();
    let total_count = matches.len();
    let queue = Arc::new(Mutex::new(matches));
    println!("Elapsed time: {:.2?}", start.elapsed());
    let start = Instant::now();
    let mut download_count = 0;
    let mut save_count = 0;
    while let Some(m) = queue.lock().await.pop() {
        if m.date <= Local::now().naive_local() + Duration::hours(12) {
            continue;
        }
        let subpage = fortuna::prematch::football::subpage::Page(m.url.clone());
        let html = Tag::download(&mut client, subpage.clone()).await.unwrap();
        download_count += 1;
        let Some(m) =
            football::subpage::to_match_events(subpage.url(), html.document())
        else {
            continue;
        };
        println!("{} - {}", m.players[0], m.players[1]);
        let Some(m) = translate_match_events::<Football, FootballOption>(m)
        else {
            continue;
        };
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
