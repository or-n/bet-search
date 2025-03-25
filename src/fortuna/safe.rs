use crate::shared::event;
use eat::*;
use event::{Event, Match};

fn fortuna_football(
    event: Event<String, String>,
    players: [String; 2],
) -> Option<Event<event::Football, String>> {
    let i = event.id.as_str();
    let r = event::Football::eat(i, players);
    if let Err(error) = r {
        println!("{} {:?}", i, error);
        println!("");
        return None;
    }
    let (rest, id) = r.ok()?;
    if !rest.is_empty() {
        println!("{} {:?}", id, rest);
        println!("{}", i);
        println!("");
        return None;
    }
    Some(Event {
        id,
        odds: event.odds,
    })
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

pub fn safe_match<T1, T2>(m: Match<T1, T2>) -> Option<Match<T1, T2>> {
    let events: Vec<_> = m.events.into_iter().filter_map(safe_event).collect();
    if events.is_empty() {
        return None;
    }
    Some(event::Match::<T1, T2> { events, ..m })
}

pub fn football_match(
    m: Match<String, String>,
) -> Option<Match<event::Football, String>> {
    let events: Vec<_> = m
        .events
        .into_iter()
        .filter_map(|event| fortuna_football(event, m.players.clone()))
        .collect();
    if events.is_empty() {
        return None;
    }
    Some(event::Match {
        url: m.url,
        date: m.date,
        players: m.players,
        events,
    })
}
