use chrono::*;
use dotenv::dotenv;
use fantoccini::ClientBuilder;
use odds::{
    fortuna,
    shared::db,
    utils::{browser, date, download::Download, page::Tag},
};
use std::time::Instant;
use surrealdb::{error::Api, Error};

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let db = db::connect().await;
    let mut client = ClientBuilder::native()
        .connect(&browser::localhost(4444))
        .await
        .unwrap();
    let matches: Vec<_> = {
        let page = fortuna::prematch::football::Page;
        let html = Tag::download(&mut client, page).await.unwrap();
        let groups = html.document().match_groups();
        let matches = groups.into_iter().flat_map(|(_a, vec_b)| vec_b);
        matches.collect()
    };
    for m in &matches {
        let id = m.db_id();
        let create: Result<Option<db::Record>, Error> = db
            .create(("match", id.clone()))
            .content(db::Match {
                date: date::to_local(m.date).with_timezone(&Utc).into(),
                player1: m.players[0].clone(),
                player2: m.players[1].clone(),
                sport: db::Sport::Football,
            })
            .await;
        let relate = db
            .query(format!(
                "RELATE match:{id}->on->source:fortuna SET url=$url;"
            ))
            .bind(("url", m.url.clone()));
        match create {
            Ok(created) => {
                println!("{id} {:?}", created);
                let r = relate.await;
                println!("{:?}", r);
            }
            Err(Error::Api(Api::Query(msg)))
                if msg.ends_with("already exists") =>
            {
                println!("{id} already exists");
                let r = relate.await;
                println!("{:?}", r);
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
