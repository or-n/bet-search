use eat::*;
use event::{Event, Match};
use fantoccini::ClientBuilder;
use odds::bmbets::{
    football, menu,
    search::{find_match, hits},
    URL,
};
use odds::shared::event;
use odds::utils::browser;
use scraper::Html;
use serde_json::{json, Map};
use std::fs;
use std::io;
use std::io::Write;
use std::time::Instant;
use tokio::time::{sleep, Duration};

fn get_id() -> usize {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(number) = input.trim().parse() {
            return number;
        }
    }
}

fn fortuna_football(
    event: Event<String>,
    players: [String; 2],
) -> Option<Event<event::Football>> {
    if let Ok(("", id)) = event::Football::eat(&event.id, players) {
        return Some(Event {
            id,
            odds: event.odds,
        });
    }
    None
}

fn safe_event<T>(event: Event<T>) -> Option<Event<T>> {
    let odds: Vec<_> = event
        .odds
        .into_iter()
        .filter(|(_, x)| *x >= 3.1 && *x <= 3.3)
        .collect();
    if odds.is_empty() {
        return None;
    }
    Some(event::Event { odds, ..event })
}

fn safe_match<T>(m: Match<T>) -> Option<Match<T>> {
    let events: Vec<_> = m.events.into_iter().filter_map(safe_event).collect();
    if events.is_empty() {
        return None;
    }
    Some(event::Match { events, ..m })
}

#[tokio::main]
async fn main() {
    let entries = fs::read_dir("downloads").unwrap();
    let matches = entries.filter_map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path().to_string_lossy().into_owned();
        let contents = fs::read_to_string(&path).unwrap();
        event::eat_match(&contents).ok()
    });
    let matches = matches.filter_map(safe_match);
    let matches = matches.filter_map(|m| {
        let events: Vec<_> = m
            .events
            .into_iter()
            .filter_map(|event| fortuna_football(event, m.players.clone()))
            .collect();
        if events.is_empty() {
            return None;
        }
        Some(event::Match {
            events,
            url: m.url,
            players: m.players,
        })
    });
    let matches: Vec<_> = matches.collect();
    if matches.is_empty() {
        println!("no matches");
        return;
    }
    let id = 0;
    let m = &matches[id];
    println!("{} - {}", m.players[0], m.players[1]);
    for event in &m.events {
        println!("{:?}", event);
    }
    let start = Instant::now();
    let caps = json!({
        "moz:firefoxOptions": {},
        "pageLoadStrategy": "eager"
    });
    let caps: Map<_, _> = caps.as_object().unwrap().clone();
    let mut client = ClientBuilder::native()
        .capabilities(caps)
        .connect(&browser::localhost(4444))
        .await
        .unwrap();
    let html = find_match(&mut client, &m.players[0]).await.unwrap();
    let document = Html::parse_document(&html);
    let hits = hits(document);
    if hits.is_empty() {
        println!("no hits");
        return;
    }
    for (id, (players, _url)) in hits.iter().enumerate() {
        println!("{id}: {} - {}", players[0], players[1]);
    }
    print!("choose: ");
    io::stdout().flush().unwrap();
    let id = get_id();
    let (players, relative_url) = &hits[id];
    let match_url = format!("{}{}", URL, relative_url);
    println!("{} - {}", players[0], players[1]);
    println!("{}", match_url);
    client.goto(&match_url).await.unwrap();
    let menu_list = menu::list(&mut client).await.unwrap();
    if menu_list.len() < 2 {
        println!("menu list len < 2");
        return;
    }
    let (name, menu_button) = &menu_list[1];
    println!("{}", name);
    menu_button.click().await.unwrap();
    let toolbar_list = menu::list_toolbar(&mut client).await.unwrap();
    let toolbar_list: Vec<_> = toolbar_list
        .into_iter()
        .filter_map(|(name, button)| {
            let (_, toolbar) = football::Toolbar::eat(&name, ()).ok()?;
            Some((toolbar, button))
        })
        .collect();
    for (toolbar, _) in toolbar_list {
        println!("{:?}", toolbar);
    }
    sleep(Duration::from_secs(5)).await;
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
