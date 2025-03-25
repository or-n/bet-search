use crate::shared::event;
use event::{Event, Match};

fn event_filter<T1, T2>(event: Event<T1, T2>) -> Option<Event<T1, T2>> {
    let odds = event.odds.into_iter();
    let odds = odds.filter(|(_, x)| *x >= 3.1 && *x <= 3.3);
    let odds: Vec<_> = odds.collect();
    if odds.is_empty() {
        return None;
    }
    Some(event::Event { odds, ..event })
}

pub fn match_filter<T1, T2>(m: Match<T1, T2>) -> Option<Match<T1, T2>> {
    let events = m.events.into_iter().filter_map(event_filter);
    let events: Vec<_> = events.collect();
    if events.is_empty() {
        return None;
    }
    Some(event::Match::<T1, T2> { events, ..m })
}
