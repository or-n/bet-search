use eat::*;
use event::{Event, Match};
use fantoccini::{error::CmdError, Client, ClientBuilder};
use odds::bmbets::{
    football::{tab, toolbar, Tab, Toolbar},
    menu,
    search::{find_match, hits, Hit},
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

fn get_id() -> Option<usize> {
    print!("choose: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().ok()
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
    let mut id = get_id()?;
    while id >= hits.len() {
        id = get_id()?;
    }
    Some(hits[id].clone())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("TabList")]
    TabList(CmdError),
    #[error("TabTranslate")]
    TabTranslate,
    #[error("TabFind")]
    TabFind,
    #[error("TabClick")]
    TabClick(CmdError),
    #[error("ToolbarList")]
    ToolbarList(CmdError),
    #[error("ToolbarTranslate")]
    ToolbarTranslate,
    #[error("ToolbarFind")]
    ToolbarFind,
    #[error("ToolbarClick")]
    ToolbarClick(Toolbar, CmdError),
    #[error("Divs")]
    Divs(CmdError),
}

async fn goto_event(
    client: &mut Client,
    e: &Event<event::Football>,
) -> Result<(), Error> {
    use Error::*;
    menu::dropdown(client).await.map_err(TabList)?;
    let tab_element = menu::tab(client).await.map_err(TabList)?;
    let tab_list = menu::links(tab_element).await.map_err(TabList)?;
    let mut tab_list = tab_list.into_iter().filter_map(|(name, button)| {
        let (_, x) = Tab::eat(name.as_str(), ()).ok()?;
        Some((x, (name, button)))
    });
    let event_tab = tab(&e.id).ok_or(TabTranslate)?;
    let event_toolbar = toolbar(&e.id).ok_or(ToolbarTranslate)?;
    let (_tab, (tab_name, tab_button)) =
        tab_list.find(|(x, _)| *x == event_tab).ok_or(TabFind)?;
    tab_button.click().await.map_err(TabClick)?;
    let toolbar = menu::toolbar(client).await.map_err(ToolbarList)?;
    let toolbar_list = menu::links(toolbar).await.map_err(ToolbarList)?;
    let mut toolbar_list =
        toolbar_list.into_iter().filter_map(|(name, button)| {
            let (_, x) = Toolbar::eat(name.as_str(), ()).ok()?;
            Some((x, (name, button)))
        });
    let (toolbar, (toolbar_name, toolbar_button)) = toolbar_list
        .find(|(x, _)| *x == event_toolbar)
        .ok_or(ToolbarFind)?;
    toolbar_button
        .click()
        .await
        .map_err(|x| ToolbarClick(toolbar.clone(), x))?;
    let content = menu::odds_content(client).await.map_err(Divs)?;
    let divs = menu::odds_divs(content).await.map_err(Divs)?;
    println!("{:?} {:?} {:?} {}", e, tab_name, toolbar_name, divs.len());
    for (name, div) in divs {
        println!("{}", name);
        let table = menu::odds_table(div).await.map_err(Divs)?;
        for odds in table {
            println!("{:?}", odds);
        }
    }
    Ok(())
}

async fn get_hit(client: &mut Client) -> Hit {
    loop {
        print!("search: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let Some(hit) = get_match(client, &input).await else {
            println!("no hits");
            continue;
        };
        return hit;
    }
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
    let hit = get_hit(&mut client).await;
    println!("{} - {}", hit.players[0], hit.players[1]);
    println!("{}{}", URL, hit.relative_url);
    println!("Elapsed time: {:.2?}", start.elapsed());
    let start = Instant::now();
    client.goto(&hit.relative_url).await.unwrap();
    for e in &m.events {
        if let event::Football::Unknown(_) = e.id {
            continue;
        }
        if let Err(error) = goto_event(&mut client, e).await {
            println!("{:?}", e);
            println!("{:?}", error);
            return;
        }
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
