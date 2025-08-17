use dotenv::dotenv;
use fantoccini::{Client, ClientBuilder};
use odds::{
    fortuna,
    shared::event::Event,
    utils::{self, download::Download, page::Tag},
};
use std::collections::HashSet;

async fn save_match_odds(client: &Client, players: [String; 2], url: String) {
    println!("{} - {}", players[0], players[1]);
    let events = {
        let subpage = fortuna::prematch::football::subpage::Page(url.clone());
        let interest = {
            let mut xs = HashSet::new();
            use fortuna::event::football::Football::*;
            xs.insert(Win);
            xs.insert(NotWin);
            xs.insert(Goals);
            xs.insert(Handicap);
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
    let events = events.into_iter().filter_map(|e| {
        let odds = e
            .odds
            .into_iter()
            .filter(|(_, odd)| *odd >= 3. && *odd <= 3.5);
        let odds: Vec<_> = odds.collect();
        if odds.is_empty() {
            return None;
        }
        Some(Event { id: e.id, odds })
    });
    let events: Vec<_> = events.collect();
    println!("{:#?}", events);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = ClientBuilder::native()
        .connect(&utils::localhost(4444))
        .await
        .unwrap();
    let pre = "/zaklady-bukmacherskie/pika-nozna/";
    let sub = "polska-3/ekstraklasa-polska/pogon-sz-gornik-z?tab=offer";
    let url = format!("{}{}", pre, sub);
    let player1 = "Pogoń Sz.";
    let player2 = "Górnik Z.";
    let players = [player1.into(), player2.into()];
    save_match_odds(&client, players, url).await;
    client.close().await.unwrap();
}
