use chrono::*;
use dotenv::dotenv;
use fantoccini::{Client, ClientBuilder};
use odds::{
    adapter::date,
    fortuna,
    shared::db,
    utils::{self, download::Download, page::Tag},
};
use std::time::Instant;
use surrealdb::{engine::remote::ws, error::Api, Error, Surreal};

async fn save_football_matches(client: &mut Client, db: Surreal<ws::Client>) {
    let matches: Vec<_> = {
        let page = fortuna::prematch::football::Page;
        let html = Tag::download(client, page).await.unwrap();
        html.document().matches()
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
                tournament: m.tournament.clone(),
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
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let db = db::connect().await;
    let mut client = ClientBuilder::native()
        .connect(&utils::localhost(4444))
        .await
        .unwrap();
    save_football_matches(&mut client, db).await;
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
