use dotenv::dotenv;
use eat::*;
use fantoccini::{Client, ClientBuilder};
use odds::{
    fortuna,
    shared::event::Event,
    utils::{
        self,
        download::Download,
        page::{Tag, Url},
    },
};
use std::collections::HashSet;

async fn print_match_odds(client: &Client, url: String) {
    use fortuna::prematch::football::subpage;
    let page = subpage::Page(url.clone());
    let players = {
        let document = subpage::init_download(client, page.url().as_str())
            .await
            .unwrap()
            .document();
        document.players()
    };
    println!("{} - {}", players[0], players[1]);
    let document = {
        let interest = {
            let mut xs = HashSet::new();
            use football::Football::*;
            use fortuna::event::football;
            xs.insert(Win);
            xs.insert(WinH1);
            xs.insert(WinH2);
            xs.insert(NotWin);
            xs.insert(Goals);
            xs.insert(GoalsH1);
            xs.insert(GoalsH2);
            xs.insert(Handicap);
            xs.insert(Corners);
            xs.insert(Penalty);
            xs.insert(BTS);
            {
                use football::FootballPlayer::*;
                use football::Player::*;
                xs.insert(Individual(P1, Goals));
                xs.insert(Individual(P2, Goals));
                xs.insert(Individual(P1, Corners));
                xs.insert(Individual(P2, Corners));
            }
            xs
        };
        let data = (page.clone(), interest, players.clone());
        let result = Tag::download(client, data).await;
        match result {
            Ok(html) => html.document(),
            Err(error) => {
                println!("download: {:#?}", error);
                return;
            }
        }
    };
    let events = document.events();
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

pub fn env(name: &str) -> String {
    std::env::var(name).expect(name)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = ClientBuilder::native()
        .connect(&utils::localhost(4444))
        .await
        .unwrap();
    let url = env("URL");
    let url = match "https://www.efortuna.pl".drop(url.as_str()) {
        Ok(i) => i.into(),
        _ => url,
    };
    print_match_odds(&client, url).await;
    client.close().await.unwrap();
}
