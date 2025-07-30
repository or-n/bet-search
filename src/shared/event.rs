use crate::shared::db;
use eat::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Event<T1, T2> {
    pub id: T1,
    pub odds: Vec<(T2, f64)>,
}

#[derive(Debug)]
pub struct Match {
    pub url: String,
    pub date: chrono::NaiveDateTime,
    pub players: [String; 2],
}

impl Match {
    pub fn db_id(&self) -> String {
        let p1 = db::sanitize(self.players[0].as_str());
        let p2 = db::sanitize(self.players[1].as_str());
        let date = self.date.format("%Y_%m%d_%H%M");
        format!("{date}_{p1}_vs_{p2}")
    }
}

pub fn translate_event<T1, T2>(
    event: Event<String, String>,
    players: [String; 2],
) -> Option<Event<T1, T2>>
where
    T1: for<'a> Eat<&'a str, (), [String; 2]> + Debug + Copy,
    T2: for<'a> Eat<&'a str, (), (T1, [String; 2])> + Debug,
{
    let i = event.id.as_str();
    let r = T1::eat(i, players.clone());
    let (rest, id) = r.ok()?;
    if !rest.is_empty() {
        return None;
    }
    let odds = event.odds.into_iter().filter_map(|(x, value)| {
        let i = x.as_str();
        let r = T2::eat(i, (id, players.clone()));
        let (i, y) = r.ok()?;
        if !i.is_empty() {
            return None;
        }
        Some((y, value))
    });
    let odds: Vec<_> = odds.collect();
    Some(Event { id, odds: odds })
}
