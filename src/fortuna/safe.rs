use crate::shared::event;
use eat::*;
use event::{Event, Match};
use std::fs;

fn fortuna_football(
    event: Event<String, String>,
    players: [String; 2],
) -> Option<Event<event::Football, String>> {
    if let Ok(("", id)) = event::Football::eat(event.id.as_str(), players) {
        return Some(Event {
            id,
            odds: event.odds,
        });
    }
    None
}

fn safe_event<T1, T2>(event: Event<T1, T2>) -> Option<Event<T1, T2>> {
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

fn safe_match<T1, T2>(m: Match<T1, T2>) -> Option<Match<T1, T2>> {
    let events: Vec<_> = m.events.into_iter().filter_map(safe_event).collect();
    if events.is_empty() {
        return None;
    }
    Some(event::Match::<T1, T2> { events, ..m })
}

pub async fn get_safe_matches() -> Vec<Match<event::Football, String>> {
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
        Some(event::Match::<event::Football, String> {
            url: m.url,
            date: m.date,
            players: m.players,
            events,
        })
    });
    matches.collect()
}
