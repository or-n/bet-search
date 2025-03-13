use crate::shared::event;
use eat::*;
use event::{Event, Match};
use std::fs;

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

pub async fn get_safe_matches() -> Vec<Match<event::Football>> {
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
