use crate::shared::db;
use eat::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tab {
    Winner,
    AsianHandicap,
    EuropeanHandicap,
    Corners,
    CornersTotalRange,
    TotalsGoals,
    TotalGoalsByIntervals,
    TotalGoalsNumberByRange,
    TotalGoalsBothTeamsToScore,
    DoubleChance,
    Cards,
    IndividualTotalGoals,
    IndividualOddEven,
    IndividualCorners,
    BothTeamsToScore,
    DrawNoBet,
    ExactGoalsNumber,
    Penalty,
    OddEven,
}

impl Tab {
    pub fn tbar(&self) -> usize {
        use Tab::*;
        match self {
            Winner => 3,
            AsianHandicap => 1,
            TotalsGoals => 4,
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Toolbar {
    FT,
    H1,
    H2,
}

#[derive(Debug, PartialEq)]
pub enum Variant {
    Handicap(f64),
    Total(f64),
}

pub fn tab(event: db::Event) -> Vec<Tab> {
    use db::Football::*;
    use Tab::*;
    match event.tag {
        GoalD => match (event.a, event.b) {
            (Some(0.5), None) => vec![Winner, AsianHandicap],
            (Some(-0.5), None) => vec![DoubleChance, AsianHandicap],
            (None, Some(-0.5)) => vec![Winner, AsianHandicap],
            (None, Some(0.5)) => vec![DoubleChance, AsianHandicap],
            (Some(-0.5), Some(0.5)) => vec![Winner],
            (Some(0.5), Some(-0.5)) => vec![DoubleChance],
            _ => vec![],
        },
        Goals => vec![TotalsGoals],
    }
}

pub fn toolbar(event: db::Event) -> Option<Toolbar> {
    use db::Football::*;
    use Toolbar::*;
    match event.tag {
        GoalD => match (event.ta, event.tb) {
            (None, None) => Some(FT),
            _ => None,
        },
        Goals => match (event.ta, event.tb) {
            (None, None) => Some(FT),
            _ => None,
        },
    }
}

pub fn variant(event: db::Event, tab: Tab) -> Vec<Variant> {
    use db::Football::*;
    use Tab::*;
    use Variant::*;
    match event.tag {
        GoalD => match (event.a, event.b, tab) {
            (Some(0.5), None, AsianHandicap) => vec![Handicap(0.5)],
            (Some(-0.5), None, AsianHandicap) => vec![Handicap(-0.5)],
            (None, Some(-0.5), AsianHandicap) => vec![Handicap(-0.5)],
            (None, Some(0.5), AsianHandicap) => vec![Handicap(0.5)],
            _ => vec![],
        },
        Goals => match (event.a, event.b, tab) {
            (Some(x), None, TotalsGoals) => vec![Total(x)],
            (None, Some(x), TotalsGoals) => vec![Total(x)],
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
        if let Ok(i) = "Corners".drop(i) {
            eat!(i, ". Total (Range)", CornersTotalRange);
            return Ok((i, Corners));
        }
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
        eat!(i, "Individual Odd/Even", IndividualOddEven);
        eat!(i, "Individual Corners", IndividualCorners);
        eat!(i, "Both Teams To Score", BothTeamsToScore);
        eat!(i, "Draw No Bet", DrawNoBet);
        eat!(i, "Exact Goals Number", ExactGoalsNumber);
        eat!(i, "Penalty", Penalty);
        eat!(i, "Odd/Even", OddEven);
        Err(())
    }
}

impl Eat<&str, (), ()> for Toolbar {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use Toolbar::*;
        eat!(i, "Full Time", FT);
        eat!(i, "1st Half", H1);
        eat!(i, "2nd Half", H2);
        Err(())
    }
}

impl Eat<&str, (), ()> for Variant {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use Variant::*;
        if let Ok(i) = "Handicap ".drop(i) {
            let (i, value) = f64::eat(i, ())?;
            return Ok((i, Handicap(value)));
        }
        if let Ok(i) = "Total ".drop(i) {
            let (i, value) = f64::eat(i, ())?;
            return Ok((i, Total(value)));
        }
        Err(())
    }
}
