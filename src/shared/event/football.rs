use crate::shared::db::ToDBRecord;
use std::cmp::Ordering;

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
    GoalBothHalves,
    BothGoalBothHalves,
    BothToScore(Part),
    BothToScoreGoals,
    BothToScoreSameHalf,
    BothToScoreOrGoalsOver,
    BothToScoreAtLeast(u32),
    WinnerBothToScore(Part, Part),
    Handicap(Part),
    Corners(Part),
    CornersHandicap(Part),
    CornersPlayer(Player, Part),
    MultiGoals(Part),
    MultiGoalsPlayer(Player),
    DrawNoBet(Part),
    MatchBothToScore,
    Offsides(Overtime),
    OffsidesPlayer(Player, Overtime),
    MoreOffsides(Overtime),
    MoreOffsidesHandicap(Overtime),
    MatchMultiScore,
    Penalty,
    PenaltySeries,
    DoubleChance(Part),
    DoubleChanceH1OrMatch,
    MatchCornerRange,
    HalfWithMoreGoals,
    HalfWithMoreYellowCards,
    WillGetCard,
    WillGetRedCard,
    DoubleChanceGoalRange,
    FirstGoal(Part),
    FirstGoalMatch,
    FirstGoalMinute,
    FirstGoalMinutePlayer(Player),
    CornerRange(Part),
    CornerRangePlayer(Player, Part),
    MatchScorePlayers,
    MatchCorners(Part),
    MatchCornersHandicap(Part),
    RestProduct,
    WinToNil(Player),
    WinBothHalves(Player),
    WinAtLeastOneHalf(Player),
    ExactScore(Part),
    ScoreBothHalves(Player),
    GoalBeforeMinute,
    NoGoalBeforeMinute,
    DoubleChanceBothToScore,
    WinDiff,
    FirstCorner(Part),
    MatchShotsOnTarget,
    ShotsOnTarget(Overtime),
    ShotsOnTargetPlayer(Player, Overtime),
    PlayerShot,
    PlayerShotOnTarget,
    MoreCorners(Part),
    MoreShotsOnTarget(Overtime),
    MoreShotsOnTargetHandicap(Overtime),
    MoreYellowCards,
    MoreFouls(Overtime),
    MoreFoulsHandicap(Overtime),
    MatchMoreCorners,
    Minute15,
    Minute30,
    Minute60,
    Minute75,
    PlayerToScore,
    YellowCards(Part),
    YellowCardsPlayer(Player, Part),
    RedCard(Part),
    RedCardPlayer(Player, Part),
    ResultDuringMatch,
    NotResultDuringMatch,
    MatchGoals(Part),
    MatchGoalsPlayer(Player),
    GoalRange,
    SubstituteWillScore(Overtime),
    WillBeLosingBut,
    Meeting(Part),
    Match(Part),
    ToAdvance,
    AdvanceBy,
    FinaleWinner,
    Shift(Part),
    Fouls(Overtime),
    FoulsPlayer(Player, Overtime),
    SuicideGoal,
    SuicideGoalPlayer(Player),
    Superoffer,
    MatchGoalSum,
    PlayerAssists,
    WinnerOr2GoalAdvantage,
    Hattrick,
    FoulsParticipant,
    GoalAfterMinute85,
}

#[derive(Debug, Clone)]
pub enum FootballOption {
    Win(u32),
    Win2(u32, u32),
    WinBool(u32, bool),
    WinLine(u32, Line),
    NotWin(u32),
    Line(Line),
    Score(u32, u32),
    Range(u32, u32),
    Player(Player),
    PlayerPlayer(Player, Player),
    PlayerBool(Player, bool),
    PlayerLine(Player, Line),
    NoPlayer,
    Draw,
    Eq,
    First,
    Second,
    Other,
    Bool(bool),
    BoolBool(bool, bool),
    BoolLine(bool, Line),
    Goals0,
}

#[derive(Debug, Clone)]
pub struct Line(pub Ordering, pub f64);

impl ToDBRecord for Football {
    fn to_db_record(&self) -> Option<String> {
        use Football::*;
        let x = match self {
            Winner(Part::FullTime) => "winner",
            Winner(Part::FirstHalf) => "winner_h1",
            Winner(Part::SecondHalf) => "winner_h2",
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

#[derive(Debug, Clone)]
pub enum Overtime {
    Include,
    Exclude,
}

impl std::fmt::Display for Football {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
