use crate::shared::db::{self, ToDBRecord};
use crate::utils::{date, scrape::split2};
use eat::*;
use std::fmt::{Debug, Display};

fn eat_pair(i: &str) -> Result<(&str, (String, String)), ()> {
    let i = '('.drop(i)?;
    let i = '\"'.drop(i)?;
    let Some((a, i)) = i.split_once('\"') else {
        return Err(());
    };
    let i = ','.drop(i)?;
    let i = ' '.drop(i)?;
    let Some((b, i)) = i.split_once(')') else {
        return Err(());
    };
    let pair = (a.to_string(), b.to_string());
    Ok((i, pair))
}

#[derive(Debug, Clone)]
pub struct Event<T1, T2> {
    pub id: T1,
    pub odds: Vec<(T2, f32)>,
}

#[derive(Debug)]
pub struct Match {
    pub url: String,
    pub date: chrono::NaiveDateTime,
    pub players: [String; 2],
}

#[derive(Debug)]
pub struct MatchEvents<T1, T2> {
    pub url: String,
    pub date: chrono::NaiveDateTime,
    pub players: [String; 2],
    pub events: Vec<Event<T1, T2>>,
}

impl Match {
    pub fn db_id(&self) -> String {
        let p1 = db::sanitize(self.players[0].as_str());
        let p2 = db::sanitize(self.players[1].as_str());
        let date = self.date.format("%Y_%m%d_%H%M");
        format!("{date}_{p1}_vs_{p2}")
    }
}

impl<T1, T2> MatchEvents<T1, T2> {
    pub fn db_id(&self) -> String {
        let p1 = db::sanitize(self.players[0].as_str());
        let p2 = db::sanitize(self.players[1].as_str());
        format!("{p1}_vs_{p2}")
    }
}

pub fn eat_match_events(i: &str) -> Result<MatchEvents<String, String>, ()> {
    let parts: Vec<_> = i.split("\n\n").collect();
    let url = parts[0].to_string();
    let date = date::eat(parts[1]).ok_or(())?;
    let players = split2(parts[2].to_string(), "\n").ok_or(())?;
    let events = parts[3..].into_iter().filter_map(|part| {
        let lines: Vec<_> = part.split('\n').collect();
        let odds: Vec<_> = lines[1..]
            .into_iter()
            .filter_map(|line| eat_pair(line).ok())
            .filter_map(|(_, (name, value))| {
                let value: f32 = value.parse().ok()?;
                Some((name, value))
            })
            .collect();
        let id = lines[0].to_string();
        Some(Event { id, odds })
    });
    Ok(MatchEvents {
        url,
        date,
        players,
        events: events.collect(),
    })
}

pub fn event_contents<T1, T2>(event: &Event<T1, T2>) -> String
where
    T1: Display,
    T2: Debug,
{
    let odds: Vec<_> = event
        .odds
        .iter()
        .map(|pair| format!("{:?}", pair))
        .collect();
    format!("{}\n{}", event.id, odds.join("\n"))
}

pub fn match_events_contents<T1, T2>(m: &MatchEvents<T1, T2>) -> Option<String>
where
    T1: Display,
    T2: Debug,
{
    let events = m.events.iter().map(event_contents);
    let events: Vec<_> = events.collect();
    if events.is_empty() {
        return None;
    }
    Some(format!(
        "{}\n\n{}\n\n{}\n{}\n\n{}",
        m.url,
        m.date.format("%Y-%m-%d %H:%M").to_string(),
        m.players[0],
        m.players[1],
        events.join("\n\n")
    ))
}

pub fn match_events_to_db<T1: ToDBRecord, T2>(
    m: &MatchEvents<T1, T2>,
) -> Vec<String> {
    let events = m.events.iter().map(|x| x.id.to_db_record());
    let events: Option<Vec<_>> = events.collect();
    events.unwrap_or(vec![])
}

pub fn translate_event2<T1, T2>(
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

pub fn translate_event<T1, T2>(
    event: Event<String, String>,
    players: [String; 2],
    url: String,
) -> Option<Event<T1, T2>>
where
    T1: for<'a> Eat<&'a str, (), [String; 2]> + Debug,
    T2: for<'a> Eat<&'a str, (), [String; 2]> + Debug,
{
    let i = event.id.as_str();
    let r = T1::eat(i, players.clone());
    if let Err(error) = r {
        println!("event: {} {:?}", i, error);
        println!("{}", url);
    }
    let (rest, id) = r.ok()?;
    if !rest.is_empty() {
        println!("event: {:?} {:?}", id, rest);
        println!("event: {}", i);
        println!("");
        return None;
    }
    let odds = event.odds.into_iter().filter_map(|(x, value)| {
        let i = x.as_str();
        let r = T2::eat(i, players.clone());
        if let Err(error) = r {
            println!("option: {} {:?}", i, error);
            println!("{}", url);
        }
        let (i, y) = r.ok()?;
        if !i.is_empty() {
            println!("option: {:?} {:?}", y, i);
            println!("option: {}", x);
            println!("");
            return None;
        }
        Some((y, value))
    });
    let odds: Vec<_> = odds.collect();
    Some(Event { id, odds: odds })
}

pub fn translate_match_events<T1, T2>(
    m: MatchEvents<String, String>,
) -> Option<MatchEvents<T1, T2>>
where
    T1: for<'a> Eat<&'a str, (), [String; 2]> + Debug,
    T2: for<'a> Eat<&'a str, (), [String; 2]> + Debug,
{
    let events = m.events.clone().into_iter();
    let events = events.filter_map(|event| {
        translate_event(event, m.players.clone(), m.url.clone())
    });
    let events: Vec<_> = events.collect();
    if events.is_empty() {
        return None;
    }
    Some(MatchEvents {
        players: m.players,
        date: m.date,
        url: m.url,
        events,
    })
}
