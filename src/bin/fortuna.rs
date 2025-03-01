use eat::*;
use football::EventType;
use fortuna::prematch::football;
use odds::fortuna;
use odds::shared;
use odds::utils::{
    browser, date,
    download::Download,
    page::{Name, Tag, Url},
    save::save,
};
use scraper::Html;
use shared::book::Subpages;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

fn contents(document: Tag<football::subpage::Page, Html>) -> Option<String> {
    let Some(players) = document.players() else {
        return None;
    };
    println!("{} - {}", players[0], players[1]);
    let events = document.events().into_iter().filter_map(|event| {
        let safe_odds = event
            .odds
            .into_iter()
            .filter(|(_, x)| *x >= 3.1 && *x <= 3.3);
        if safe_odds.clone().peekable().peek().is_none() {
            return None;
        }
        let (rest, event_type) =
            EventType::eat(event.name.as_str(), ()).unwrap();
        if rest != "" || matches!(event_type, EventType::Unknown(_)) {
            return None;
        }
        let safe_odds: Vec<_> =
            safe_odds.map(|pair| format!("{:?}", pair)).collect();
        Some(format!("{}\n{}", event.name, safe_odds.join("\n")))
    });
    let events: Vec<_> = events.collect();
    if events.is_empty() {
        return None;
    }
    Some(events.join("\n"))
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut client = browser::connect(4444).await;
    let page = fortuna::prematch::football::Page;
    let html = shared::download_and_save::run(&mut client, page)
        .await
        .unwrap();
    let subpages = html.document().subpages();
    let queue = Arc::new(Mutex::new(subpages));
    while let Some((subpage, date)) = queue.lock().await.pop() {
        if !date::in_days(date, 1) {
            continue;
        }
        let html = Tag::download(&mut client, subpage.clone()).await.unwrap();
        let Some(contents) = contents(html.document()) else {
            continue;
        };
        let file = format!("downloads/{}", subpage.name());
        let f = format!("{}\n\n{}", subpage.url(), contents);
        let _ = save(f.as_bytes(), file).await;
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
