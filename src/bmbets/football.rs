use crate::bmbets::menu;
use crate::shared::event;
use eat::*;
use event::Event;
use fantoccini::{error::CmdError, Client};
use futures::StreamExt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tab {
    Winner,
    AsianHandicap,
    EuropeanHandicap,
    Corners,
    TotalsGoals,
    TotalGoalsByIntervals,
    TotalGoalsNumberByRange,
    TotalGoalsBothTeamsToScore,
    DoubleChance,
    Cards,
    IndividualTotalGoals,
    IndividualCorners,
    BothTeamsToScore,
    DrawNoBet,
    ExactGoalsNumber,
    Penalty,
}

pub fn tab(event: &event::Football) -> Option<Tab> {
    use event::Football::*;
    use Tab::*;
    match event {
        Goals | GoalsH1 | GoalsH2 => Some(TotalsGoals),
        GoalsP1 | GoalsP1H1 | GoalsP1H2 | GoalsP2 | GoalsP2H1 | GoalsP2H2 => {
            Some(IndividualTotalGoals)
        }
        ExactGoals | ExactGoalsH1 | ExactGoalsH2 => Some(ExactGoalsNumber),
        BothToScore | BothToScoreH1 | BothToScoreH2 => Some(BothTeamsToScore),
        Handicap | HandicapH1 | HandicapH2 => Some(AsianHandicap),
        H1 | H2 => Some(Winner),
        event::Football::Corners | CornersH1 | CornersH2 => Some(Tab::Corners),
        CornersP1 | CornersP1H1 | CornersP1H2 | CornersP2 | CornersP2H1
        | CornersP2H2 => Some(IndividualCorners),
        Unknown(_) => None,
    }
}

pub fn toolbar(event: &event::Football) -> Option<Toolbar> {
    use event::Football::*;
    use Toolbar::*;
    match event {
        Goals => Some(Toolbar::FullTime),
        GoalsH1 => Some(Toolbar::FirstHalf),
        GoalsH2 => Some(Toolbar::SecondHalf),
        GoalsP1 => Some(Home(Part::FullTime)),
        GoalsP1H1 => Some(Home(Part::FirstHalf)),
        GoalsP1H2 => Some(Home(Part::SecondHalf)),
        GoalsP2 => Some(Away(Part::FullTime)),
        GoalsP2H1 => Some(Away(Part::FirstHalf)),
        GoalsP2H2 => Some(Away(Part::SecondHalf)),
        ExactGoals => Some(Toolbar::FullTime),
        ExactGoalsH1 => Some(Toolbar::FirstHalf),
        ExactGoalsH2 => Some(Toolbar::SecondHalf),
        BothToScore => Some(Toolbar::FullTime),
        BothToScoreH1 => Some(Toolbar::FirstHalf),
        BothToScoreH2 => Some(Toolbar::SecondHalf),
        Handicap => Some(Toolbar::FullTime),
        HandicapH1 => Some(Toolbar::FirstHalf),
        HandicapH2 => Some(Toolbar::SecondHalf),
        H1 => Some(Toolbar::FirstHalf),
        H2 => Some(Toolbar::SecondHalf),
        Corners => Some(Total(Part::FullTime)),
        CornersH1 => Some(Total(Part::FirstHalf)),
        CornersH2 => Some(Total(Part::SecondHalf)),
        CornersP1 => Some(HomeTotal(Part::FullTime)),
        CornersP1H1 => Some(HomeTotal(Part::FirstHalf)),
        CornersP1H2 => Some(HomeTotal(Part::SecondHalf)),
        CornersP2 => Some(AwayTotal(Part::FullTime)),
        CornersP2H1 => Some(AwayTotal(Part::FirstHalf)),
        CornersP2H2 => Some(AwayTotal(Part::SecondHalf)),
        Unknown(_) => None,
    }
}

impl Eat<&str, (), ()> for Tab {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use Tab::*;
        if let Ok(i) = "1x2".drop(i) {
            return Ok((i, Winner));
        }
        if let Ok(i) = "Asian Handicap".drop(i) {
            return Ok((i, AsianHandicap));
        }
        if let Ok(i) = "European Handicap".drop(i) {
            return Ok((i, EuropeanHandicap));
        }
        if let Ok(i) = "Corners".drop(i) {
            return Ok((i, Corners));
        }
        if let Ok(i) = "Total Goals By Intervals".drop(i) {
            return Ok((i, TotalGoalsByIntervals));
        }
        if let Ok(i) = "Total Goals Number By Range".drop(i) {
            return Ok((i, TotalGoalsNumberByRange));
        }
        if let Ok(i) = "Total Goals/Both Teams To Score".drop(i) {
            return Ok((i, TotalGoalsBothTeamsToScore));
        }
        if let Ok(i) = "Totals Goals".drop(i) {
            return Ok((i, TotalsGoals));
        }
        if let Ok(i) = "Double Chance".drop(i) {
            return Ok((i, DoubleChance));
        }
        if let Ok(i) = "Cards".drop(i) {
            return Ok((i, Cards));
        }
        if let Ok(i) = "Individual Total Goals".drop(i) {
            return Ok((i, IndividualTotalGoals));
        }
        if let Ok(i) = "Individual Corners".drop(i) {
            return Ok((i, IndividualCorners));
        }
        if let Ok(i) = "Both Teams To Score".drop(i) {
            return Ok((i, BothTeamsToScore));
        }
        if let Ok(i) = "Draw No Bet".drop(i) {
            return Ok((i, DrawNoBet));
        }
        if let Ok(i) = "Exact Goals Number".drop(i) {
            return Ok((i, ExactGoalsNumber));
        }
        if let Ok(i) = "Penalty".drop(i) {
            return Ok((i, Penalty));
        }
        Err(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Toolbar {
    FullTime,
    FirstHalf,
    SecondHalf,
    Winner(Part),
    AsianHandicap(Part),
    Total(Part),
    DoubleChance,
    HomeTotal(Part),
    AwayTotal(Part),
    Home(Part),
    Away(Part),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Part {
    FullTime,
    FirstHalf,
    SecondHalf,
}

impl Eat<&str, (), ()> for Toolbar {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use Toolbar::*;
        if let Ok(i) = "Full Time".drop(i) {
            return Ok((i, FullTime));
        }
        if let Ok(i) = "1st Half".drop(i) {
            return Ok((i, FirstHalf));
        }
        if let Ok(i) = "2nd Half".drop(i) {
            return Ok((i, SecondHalf));
        }
        if let Ok(i) = "1x2".drop(i) {
            let (i, part) = Part::eat(i, true)?;
            return Ok((i, Winner(part)));
        }
        if let Ok(i) = "Asian Handicap".drop(i) {
            let (i, part) = Part::eat(i, true)?;
            return Ok((i, AsianHandicap(part)));
        }
        if let Ok(i) = "Total".drop(i) {
            let (i, part) = Part::eat(i, true)?;
            return Ok((i, Total(part)));
        }
        if let Ok(i) = "Double Chance".drop(i) {
            return Ok((i, DoubleChance));
        }
        if let Ok(i) = "Home Total".drop(i) {
            let (i, part) = Part::eat(i, true)?;
            return Ok((i, HomeTotal(part)));
        }
        if let Ok(i) = "Away Total".drop(i) {
            let (i, part) = Part::eat(i, true)?;
            return Ok((i, AwayTotal(part)));
        }
        if let Ok(i) = "Home".drop(i) {
            let (i, part) = Part::eat(i, false)?;
            return Ok((i, Home(part)));
        }
        if let Ok(i) = "Away".drop(i) {
            let (i, part) = Part::eat(i, false)?;
            return Ok((i, Away(part)));
        }
        Err(())
    }
}

impl Eat<&str, (), bool> for Part {
    fn eat(i: &str, data: bool) -> Result<(&str, Self), ()> {
        use Part::*;
        if data {
            if let Ok(i) = " (H1)".drop(i) {
                return Ok((i, FirstHalf));
            }
            if let Ok(i) = " (H2)".drop(i) {
                return Ok((i, SecondHalf));
            }
            Ok((i, FullTime))
        } else {
            if let Ok(i) = " FT".drop(i) {
                return Ok((i, FullTime));
            }
            if let Ok(i) = " H1".drop(i) {
                return Ok((i, FirstHalf));
            }
            if let Ok(i) = " H2".drop(i) {
                return Ok((i, SecondHalf));
            }
            Err(())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
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

pub async fn goto(
    client: &mut Client,
    e: &Event<event::Football, String>,
) -> Result<Event<event::Football, String>, Error> {
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
    let new_odds =
        futures::stream::iter(e.odds.iter()).filter_map(|(variant, odd)| {
            let x = eat_variant(&e.id, &variant);
            let divs = divs.clone();
            async move {
                if let Variant::Unknown(_) = x {
                    return None;
                }
                let name = x.table_name();
                println!("{:?} {} {}", x, name, odd);
                if let Some((_, div)) = divs.iter().find(|(n, _)| *n == name) {
                    let table = menu::odds_table(div.clone())
                        .await
                        .map_err(Divs)
                        .ok()?;
                    let mut sum = Vec::new();
                    for (book, odds) in &table {
                        if sum.is_empty() {
                            sum = odds.clone();
                        } else if sum.len() == odds.len() {
                            sum.iter_mut()
                                .zip(odds.iter())
                                .for_each(|(sum, odd)| *sum += odd);
                        } else {
                            panic!()
                        }
                    }
                    sum.iter_mut().for_each(|sum| *sum /= table.len() as f32);
                    let mean = sum;
                    let mean_odd = x.choose_odd(&mean);
                    println!("{:?} {}", mean, mean_odd);
                }
                None::<(String, f32)>
            }
        });
    let new_odds = new_odds.collect().await;
    let new_e = Event {
        id: e.id.clone(),
        odds: new_odds,
    };
    Ok(new_e)
}

#[derive(Debug)]
enum Variant {
    Handicap(String, OverUnder),
    Total(String, OverUnder),
    Unknown(String),
}

#[derive(Debug)]
enum OverUnder {
    Over,
    Under,
}

pub fn eat_variant(e: &event::Football, i: &str) -> Variant {
    if let Ok(i) = "mniej ".drop(i) {
        return under(e, i);
    }
    if let Ok(i) = "Mniej ".drop(i) {
        return under(e, i);
    }
    if let Ok(i) = "wiecej ".drop(i) {
        return over(e, i);
    }
    if let Ok(i) = "Wiecej ".drop(i) {
        return over(e, i);
    }
    Variant::Unknown(i.to_string())
}

pub fn under(e: &event::Football, i: &str) -> Variant {
    use event::Football::*;
    use OverUnder::*;
    let s = i.to_string();
    match e {
        Goals => Variant::Total(s, Under),
        GoalsH1 => Variant::Total(s, Under),
        GoalsH2 => Variant::Total(s, Under),
        GoalsP1 => Variant::Handicap(s, Under),
        GoalsP1H1 => Variant::Handicap(s, Under),
        GoalsP1H2 => Variant::Handicap(s, Under),
        GoalsP2 => Variant::Handicap(s, Under),
        GoalsP2H1 => Variant::Handicap(s, Under),
        GoalsP2H2 => Variant::Handicap(s, Under),
        _ => todo!(),
    }
}

pub fn over(e: &event::Football, i: &str) -> Variant {
    use event::Football::*;
    use OverUnder::*;
    let s = i.to_string();
    match e {
        Goals => Variant::Total(s, Over),
        GoalsH1 => Variant::Total(s, Over),
        GoalsH2 => Variant::Total(s, Over),
        GoalsP1 => Variant::Handicap(s, Over),
        GoalsP1H1 => Variant::Handicap(s, Over),
        GoalsP1H2 => Variant::Handicap(s, Over),
        GoalsP2 => Variant::Handicap(s, Over),
        GoalsP2H1 => Variant::Handicap(s, Over),
        GoalsP2H2 => Variant::Handicap(s, Over),
        _ => todo!(),
    }
}

pub fn pos_line(x: &String) -> String {
    if x.chars().next() == Some('-') {
        x.clone()
    } else {
        format!("+{}", x)
    }
}

impl Variant {
    pub fn table_name(&self) -> String {
        use OverUnder::*;
        use Variant::*;
        match self {
            Total(x, _) => format!("Total {}", pos_line(x)),
            Handicap(x, _) => format!("Handicap {}", pos_line(x)),
            Unknown(_) => panic!(),
        }
    }

    pub fn choose_odd(&self, odds: &Vec<f32>) -> f32 {
        use OverUnder::*;
        use Variant::*;
        match self {
            Total(_, Over) => odds[0],
            Total(_, Under) => odds[1],
            _ => panic!(),
        }
    }
}
