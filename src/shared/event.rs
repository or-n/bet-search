use crate::utils::scrape::split2;
use eat::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Football {
    Goals,
    GoalsH1,
    GoalsH2,
    GoalsP1,
    GoalsP1H1,
    GoalsP1H2,
    GoalsP2,
    GoalsP2H1,
    GoalsP2H2,
    ExactGoals,
    ExactGoalsH1,
    ExactGoalsH2,
    BothToScore,
    BothToScoreH1,
    BothToScoreH2,
    Handicap,
    HandicapH1,
    HandicapH2,
    H1,
    H2,
    Corners,
    CornersH1,
    CornersH2,
    CornersP1,
    CornersP1H1,
    CornersP1H2,
    CornersP2,
    CornersP2H1,
    CornersP2H2,
    Unknown(String),
}

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

#[derive(Debug)]
pub struct Event<T1, T2> {
    pub id: T1,
    pub odds: Vec<(T2, f32)>,
}

#[derive(Debug)]
pub struct Match<T1, T2> {
    pub url: String,
    pub players: [String; 2],
    pub events: Vec<Event<T1, T2>>,
}

pub fn eat_match(i: &str) -> Result<Match<String, String>, ()> {
    let parts: Vec<_> = i.split("\n\n").collect();
    let url = parts[0].to_string();
    let players = split2(parts[1].to_string(), "\n").ok_or(())?;
    let events = parts[2..].into_iter().filter_map(|part| {
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
    Ok(Match {
        url,
        players,
        events: events.collect(),
    })
}

pub fn event_contents<T1, T2>(event: &Event<T1, T2>) -> String
where
    T1: Debug,
    T2: Debug,
{
    let odds: Vec<_> = event
        .odds
        .iter()
        .map(|pair| format!("{:?}", pair))
        .collect();
    format!("{:?}\n{}", event.id, odds.join("\n"))
}

pub fn match_contents<T1, T2>(m: &Match<T1, T2>) -> Option<String>
where
    T1: Debug,
    T2: Debug,
{
    let events = m.events.iter().map(event_contents);
    let events: Vec<_> = events.collect();
    if events.is_empty() {
        return None;
    }
    Some(format!(
        "{}\n\n{}\n{}\n\n{}",
        m.url,
        m.players[0],
        m.players[1],
        events.join("\n\n")
    ))
}
