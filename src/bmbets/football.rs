use crate::bmbets::menu;
use crate::shared::event;
use eat::*;
use event::football::{Football, Part, Player};
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

pub fn tab(event: &Football) -> Option<Tab> {
    use Football::*;
    use Tab::*;
    match event {
        Football::Winner(_) => Some(Tab::Winner),
        Goals(_) => Some(TotalsGoals),
        GoalsPlayer(_, _) => Some(IndividualTotalGoals),
        ExactGoals(_) => Some(ExactGoalsNumber),
        BothToScore(_) => Some(BothTeamsToScore),
        Handicap(_) => Some(AsianHandicap),
        Football::Corners(_) => Some(Tab::Corners),
        CornersPlayer(_, _) => Some(IndividualCorners),
        _ => None,
    }
}

pub fn toolbar(event: &Football) -> Option<Toolbar> {
    use Football::*;
    use Player::*;
    use Toolbar::*;
    match event {
        Football::Winner(part) => Some(Toolbar::Part_(*part)),
        Goals(part) => Some(Toolbar::Part_(*part)),
        GoalsPlayer(P1, part) => Some(Home(*part)),
        GoalsPlayer(P2, part) => Some(Away(*part)),
        ExactGoals(part) => Some(Toolbar::Part_(*part)),
        // BothToScore => Some(Toolbar::FullTime),
        // BothToScoreH1 => Some(Toolbar::FirstHalf),
        // BothToScoreH2 => Some(Toolbar::SecondHalf),
        // Handicap => Some(Toolbar::FullTime),
        // HandicapH1 => Some(Toolbar::FirstHalf),
        // HandicapH2 => Some(Toolbar::SecondHalf),
        Corners(part) => Some(Total(*part)),
        CornersPlayer(P1, part) => Some(HomeTotal(*part)),
        CornersPlayer(P2, part) => Some(AwayTotal(*part)),
        _ => None,
    }
}

impl Eat<&str, (), ()> for Tab {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use Tab::*;
        eat!(i, "1x2", Winner);
        eat!(i, "Asian Handicap", AsianHandicap);
        eat!(i, "European Handicap", EuropeanHandicap);
        eat!(i, "Corners", Corners);
        eat!(i, "Total Goals By Intervals", TotalGoalsByIntervals);
        eat!(i, "Total Goals Number By Range", TotalGoalsNumberByRange);
        eat!(
            i,
            "Total Goals/Both Teams To Score",
            TotalGoalsBothTeamsToScore
        );
        eat!(i, "Totals Goals", TotalsGoals);
        eat!(i, "Double Chance", DoubleChance);
        eat!(i, "Cards", Cards);
        eat!(i, "Individual Total Goals", IndividualTotalGoals);
        eat!(i, "Individual Corners", IndividualCorners);
        eat!(i, "Both Teams To Score", BothTeamsToScore);
        eat!(i, "Draw No Bet", DrawNoBet);
        eat!(i, "Exact Goals Number", ExactGoalsNumber);
        eat!(i, "Penalty", Penalty);
        Err(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Toolbar {
    Part_(Part),
    Winner(Part),
    AsianHandicap(Part),
    Total(Part),
    DoubleChance,
    HomeTotal(Part),
    AwayTotal(Part),
    Home(Part),
    Away(Part),
}

impl Eat<&str, (), ()> for Toolbar {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use Toolbar::*;
        eat!(i, "Full Time", Part_(Part::FullTime));
        eat!(i, "1st Half", Part_(Part::FirstHalf));
        eat!(i, "2nd Half", Part_(Part::SecondHalf));
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
    fn eat(i: &str, parens: bool) -> Result<(&str, Self), ()> {
        use Part::*;
        if parens {
            eat!(i, " (H1)", FirstHalf);
            eat!(i, " (H2)", SecondHalf);
            Ok((i, FullTime))
        } else {
            eat!(i, " FT", FullTime);
            eat!(i, " H1", FirstHalf);
            eat!(i, " H2", SecondHalf);
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

fn chances(x: &Vec<f32>) -> Vec<f32> {
    let y = x.into_iter().map(|x| 1. / x);
    let s = 1. / y.clone().sum::<f32>();
    y.map(|x| x * s).collect()
}

pub async fn goto(
    client: &mut Client,
    e: &Event<Football, String>,
) -> Result<Event<Football, String>, Error> {
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
    println!("{:?} {:?} {:?}", e, tab_name, toolbar_name);
    let odds = futures::stream::iter(e.odds.iter()).filter_map(
        |(variant_text, odd)| {
            let variant = eat_variant(&e.id, variant_text.as_str());
            let divs = divs.clone();
            async move {
                if let Variant::Unknown(_) = variant {
                    return None;
                }
                let name = variant.table_name();
                println!("{:?} {:?}", variant, name);
                if let Some((_, div)) = divs.iter().find(|(n, _)| *n == name) {
                    let table = menu::odds_table(div.clone())
                        .await
                        .map_err(Divs)
                        .ok()?;
                    let mut sum = Vec::new();
                    for (_book, odds) in &table {
                        if sum.is_empty() {
                            sum = odds.clone();
                        } else if sum.len() == odds.len() {
                            sum.iter_mut()
                                .zip(odds.iter())
                                .for_each(|(sum, odd)| *sum += odd);
                        } else {
                            panic!() //happens
                        }
                    }
                    let books =
                        table.iter().map(|(book, _)| format!("{}", book));
                    let books: Vec<_> = books.collect();
                    sum.iter_mut().for_each(|sum| *sum /= table.len() as f32);
                    let mean = sum;
                    let chances = chances(&mean);
                    let mean_chance = variant.choose(&chances);
                    let value = odd * mean_chance;
                    if value > 0.97 {
                        println!("{}", books.join(" "));
                        println!("{:?} {:?} {}", mean, chances, value);
                        return Some((variant_text.clone(), *odd));
                    }
                }
                None
            }
        },
    );
    let odds = odds.collect().await;
    Ok(Event {
        id: e.id.clone(),
        odds,
    })
}

#[allow(dead_code)]
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

fn overunder(i: &str) -> Result<(&str, OverUnder), ()> {
    use OverUnder::*;
    eat!(i, "mniej", Under);
    eat!(i, "Mniej", Under);
    eat!(i, "wiecej", Over);
    eat!(i, "Wiecej", Over);
    Err(())
}

fn eat_variant(e: &Football, i: &str) -> Variant {
    if let Ok((i, x)) = overunder(i) {
        return overunder_variant(e, i, x);
    }
    Variant::Unknown(i.to_string())
}

fn overunder_variant(e: &Football, i: &str, x: OverUnder) -> Variant {
    use Football::*;
    let s = i.to_string();
    match e {
        Goals(_) => Variant::Total(s, x),
        GoalsPlayer(_, _) => Variant::Handicap(s, x),
        _ => todo!(),
    }
}

pub fn pos_line(x: &String) -> String {
    let trim = x.trim();
    if trim.chars().next() == Some('-') {
        trim.to_string()
    } else {
        format!("+{}", trim)
    }
}

impl Variant {
    pub fn table_name(&self) -> String {
        use Variant::*;
        match self {
            Total(x, _) => format!("Total {}", pos_line(x)),
            Handicap(x, _) => format!("Handicap {}", pos_line(x)),
            Unknown(_) => panic!(),
        }
    }

    pub fn choose(&self, values: &Vec<f32>) -> f32 {
        use OverUnder::*;
        use Variant::*;
        match self {
            Total(_, Over) => values[0],
            Total(_, Under) => values[1],
            _ => panic!(),
        }
    }
}
