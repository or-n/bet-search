use eat::*;

#[derive(Debug)]
pub enum Tabs {
    Winner,
    AsianHandicap,
    EuropeanHandicap,
    Corners,
    TotalGoals,
    DoubleChance,
    Cards,
    IndividualTotalGoals,
    IndividualCorners,
    BothTeamsToScore,
    DrawNoBet,
    ExactGoalsNumber,
    Penalty,
}

impl Eat<&str, (), ()> for Tabs {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use Tabs::*;
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
        if let Ok(i) = "Total Goals".drop(i) {
            return Ok((i, TotalGoals));
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

#[derive(Debug)]
pub enum Toolbar {
    FullTime,
    FirstHalf,
    SecondHalf,
    Winner,
    WinnerFirstHalf,
    WinnerSecondHalf,
    AsianHandicap,
    AsianHandicapFirstHalf,
    AsianHandicapSecondHalf,
    Total,
    TotalFirstHalf,
    TotalSecondHalf,
    DoubleChance,
    HomeTotal,
    HomeTotalFirstHalf,
    HomeTotalSecondHalf,
    AwayTotal,
    AwayTotalFirstHalf,
    AwayTotalSecondHalf,
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
            return Ok((i, Winner));
        }
        if let Ok(i) = "1x2 (H1)".drop(i) {
            return Ok((i, WinnerFirstHalf));
        }
        if let Ok(i) = "1x2 (H2)".drop(i) {
            return Ok((i, WinnerSecondHalf));
        }
        if let Ok(i) = "Asian Handicap".drop(i) {
            return Ok((i, AsianHandicap));
        }
        if let Ok(i) = "Asian Handicap (H1)".drop(i) {
            return Ok((i, AsianHandicapFirstHalf));
        }
        if let Ok(i) = "Asian Handicap (H2)".drop(i) {
            return Ok((i, AsianHandicapSecondHalf));
        }
        if let Ok(i) = "Total".drop(i) {
            return Ok((i, Total));
        }
        if let Ok(i) = "Total (H1)".drop(i) {
            return Ok((i, TotalFirstHalf));
        }
        if let Ok(i) = "Total (H2)".drop(i) {
            return Ok((i, TotalSecondHalf));
        }
        if let Ok(i) = "Double Chance".drop(i) {
            return Ok((i, DoubleChance));
        }
        if let Ok(i) = "Home Total".drop(i) {
            return Ok((i, HomeTotal));
        }
        if let Ok(i) = "Home Total (H1)".drop(i) {
            return Ok((i, HomeTotalFirstHalf));
        }
        if let Ok(i) = "Home Total (H2)".drop(i) {
            return Ok((i, HomeTotalSecondHalf));
        }
        if let Ok(i) = "Away Total".drop(i) {
            return Ok((i, AwayTotal));
        }
        if let Ok(i) = "Away Total (H1)".drop(i) {
            return Ok((i, AwayTotalFirstHalf));
        }
        if let Ok(i) = "Away Total (H2)".drop(i) {
            return Ok((i, AwayTotalSecondHalf));
        }
        Err(())
    }
}
