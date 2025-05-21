use crate::shared::db::ToDBRecord;

#[derive(Debug, Clone)]
pub enum Football {
    Winner(Part),
    Winner2(Part, Part),
    Winner2BothToScore(Part, Part),
    WinnerMatchAndGoals(Part),
    Goals(Part),
    Goals2(Part, Part),
    GoalsPlayer(Player, Part),
    ExactGoals(Part),
    ExactGoalsPlayer(Player),
    BothToScore(Part),
    BothToScoreGoals,
    BothToScore2,
    BothToScoreSameHalf,
    WinnerBothToScore(Part, Part),
    Handicap(Part),
    Corners(Part),
    CornersHandicap(Part),
    CornersPlayer(Player, Part),
    MultiGoals(Part),
    MultiGoalsPlayer(Player),
    DrawNoBet(Part),
    MatchBothToScore,
    Offsides,
    OffsidesPlayer(Player),
    MatchMultiScore,
    Penalty,
    DoubleChance,
    DoubleChanceH1OrMatch,
    MatchCornerRange,
    HalfWithMoreGoals,
    WillGetCard,
    DoubleChanceGoalRange,
    FirstGoal(Part),
    FirstGoalMatch,
    FirstGoalMinute,
    FirstGoalMinutePlayer(Player),
    CornerRange(Part),
    CornerRangePlayer(Player, Part),
    MatchScorePlayers,
    MatchCorners(Part),
    RestProduct,
    WinToNil(Player),
    WinBothHalves(Player),
    WinAtLeastOneHalf(Player),
    ExactScore(Part),
    ScoreBothHalves(Player),
    GoalBeforeMinute,
    DoubleChanceBothToScore,
    WinDiff,
    FirstCorner(Part),
    MatchShotsOnTarget,
    ShotsOnTarget,
    ShotsOnTargetPlayer(Player),
    PlayerShot,
    PlayerShotOnTarget,
    MoreCorners(Part),
    MoreShotsOnTarget,
    MoreYellowCards,
    MoreFouls,
    MatchMoreCorners,
    Minute15,
    Minute30,
    Minute60,
    Minute75,
    PlayerToScore,
    YellowCards(Part),
    YellowCardsPlayer(Player, Part),
    ResultDuringMatch,
    NotResultDuringMatch,
    MatchGoals(Part),
    MatchGoalsPlayer(Player),
    GoalRange,
    SubstituteWillScore,
    WillBeLosingBut,
    Meeting(Part),
    Match(Part),
    ToAdvance,
    AdvanceBy,
}

impl ToDBRecord for Football {
    fn to_db_record(&self) -> Option<String> {
        use Football::*;
        let x = match self {
            Winner(Part::FullTime) => "winner".to_string(),
            Winner(Part::FirstHalf) => "winner_h1".to_string(),
            Winner(Part::SecondHalf) => "winner_h2".to_string(),
            _ => return None,
        };
        Some(format!("football_event:{x}"))
    }
}

#[derive(Debug, Clone)]
pub enum Player {
    P1,
    P2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Part {
    FullTime,
    FirstHalf,
    SecondHalf,
}

impl std::fmt::Display for Football {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
