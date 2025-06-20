use crate::shared::db;
use eat::*;

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

pub fn tab(event: db::Event) -> Vec<Tab> {
    use Tab::*;
    match event.tag {
        db::Football::GoalD => match (event.a, event.b) {
            (Some(0.5), None) => vec![Winner, AsianHandicap],
            (Some(-0.5), None) => vec![DoubleChance, AsianHandicap],
            (None, Some(-0.5)) => vec![Winner, AsianHandicap],
            (None, Some(0.5)) => vec![DoubleChance, AsianHandicap],
            (Some(-0.5), Some(0.5)) => vec![Winner],
            (Some(0.5), Some(-0.5)) => vec![DoubleChance],
            _ => vec![],
        },
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

#[allow(dead_code)]
#[derive(Debug)]
enum Variant {
    Handicap(String, OverUnder),
    Total(String, OverUnder),
    Unknown(String),
}

#[allow(dead_code)]
#[derive(Debug)]
enum OverUnder {
    Over,
    Under,
}

pub fn pos_line(x: &String) -> String {
    let trim = x.trim();
    if trim.chars().next() == Some('-') {
        trim.to_string()
    } else {
        format!("+{}", trim)
    }
}
