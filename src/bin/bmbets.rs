use eat::*;
use event::{Event, Match};
use fantoccini::{Client, ClientBuilder};
use odds::bmbets::{
    football::{tab, toolbar, Tab, Toolbar},
    menu,
    search::{find_match, hits, Hit},
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

async fn get_safe_matches() -> Vec<Match<event::Football>> {
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
    matches.collect()
}

async fn get_match(client: &mut Client, prompt: &str) -> Option<Hit> {
    let html = find_match(client, prompt).await.unwrap();
    let document = Html::parse_document(&html);
    let hits = hits(document);
    if hits.is_empty() {
        return None;
    }
    for (id, hit) in hits.iter().enumerate() {
        println!("{id}: {} - {}", hit.players[0], hit.players[1]);
    }
    print!("choose: ");
    io::stdout().flush().unwrap();
    let id = if hits.len() == 1 { 0 } else { get_id() };
    Some(hits[id].clone())
}

async fn goto_event(
    client: &mut Client,
    e: &Event<event::Football>,
) -> Option<()> {
    let menu_list = menu::list(client).await.ok()?;
    let mut menu_list = menu_list.into_iter().filter_map(|(name, button)| {
        let (_, x) = Tab::eat(&name, ()).ok()?;
        Some((x, button))
    });
    let event_tab = tab(&e.id)?;
    println!("tab {:?}", event_tab);
    let (_tab, menu_button) = menu_list.find(|(tab, _)| *tab == event_tab)?;
    println!("found");
    menu_button.click().await.ok()?;
    println!("clicked");
    let toolbar_list = menu::list_toolbar(client).await.ok()?;
    let mut toolbar_list =
        toolbar_list.into_iter().filter_map(|(name, button)| {
            let (_, toolbar) = Toolbar::eat(&name, ()).ok()?;
            Some((toolbar, button))
        });
    let event_toolbar = toolbar(&e.id)?;
    println!("toolbar {:?}", event_toolbar);
    let (_toolbar, toolbar_button) =
        toolbar_list.find(|(x, _)| *x == event_toolbar)?;
    println!("found");
    toolbar_button.click().await.ok()?;
    println!("clicked");
    Some(())
}

#[tokio::main]
async fn main() {
    let matches = get_safe_matches().await;
    if matches.is_empty() {
        println!("no matches");
        return;
    }
    let match_id = 0;
    let m = &matches[match_id];
    println!("{} - {}", m.players[0], m.players[1]);
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
    println!("search: {}", m.players[0]);
    let Some(hit) = get_match(&mut client, &m.players[0]).await else {
        println!("no hits");
        return;
    };
    println!("{} - {}", hit.players[0], hit.players[1]);
    println!("{}", hit.relative_url);
    println!("Elapsed time: {:.2?}", start.elapsed());
    let start = Instant::now();
    client.goto(&hit.relative_url).await.unwrap();
    for e in &m.events {
        println!("{:?}", e);
        let _ = goto_event(&mut client, e).await;
        // sleep(Duration::from_secs(1)).await;
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
