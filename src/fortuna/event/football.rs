use crate::shared::db::{self, ToDBRecord};
use crate::shared::event::{Event, Match};
use eat::*;
use std::cmp::Ordering;
use Football::*;

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
    WinOverParticipant(u32, f64, String),
    WinParticipant(u32, String),
    NotWin(u32),
    L(Line),
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
    Both,
    BothParticipant(String),
    Neither,
    Only(Player),
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

pub fn translate_event(
    event: Event<String, String>,
    players: [String; 2],
    url: String,
) -> Option<Event<Football, FootballOption>> {
    let i = event.id.as_str();
    let r = Football::eat(i, players.clone());
    if let Err(error) = r {
        println!("{} {:?}", i, error);
        println!("{}", url);
    }
    let (rest, id) = r.ok()?;
    if !rest.is_empty() {
        println!("{} {:?}", id, rest);
        println!("{}", i);
        println!("");
        return None;
    }
    let odds = event.odds.into_iter().filter_map(|(x, value)| {
        let i = x.as_str();
        let r = FootballOption::eat(i, players.clone());
        if let Err(error) = r {
            println!("{} {:?}", i, error);
            println!("{}", url);
        }
        let (i, y) = r.ok()?;
        if !i.is_empty() {
            println!("{:?} {:?}", y, i);
            println!("{}", x);
            println!("");
            return None;
        }
        Some((y, value))
    });
    let odds: Vec<_> = odds.collect();
    Some(Event { id, odds: odds })
}

pub fn translate_match(
    m: &Match<String, String>,
    filter_event: impl Fn(
        Event<Football, FootballOption>,
    ) -> Option<Event<Football, FootballOption>>,
) -> Option<Match<Football, FootballOption>> {
    let events = m.events.clone().into_iter();
    let events = events.filter_map(|event| {
        translate_event(event, m.players.clone(), m.url.clone())
    });
    let events = events.filter_map(filter_event);
    let events: Vec<_> = events.collect();
    if events.is_empty() {
        return None;
    }
    Some(Match {
        url: m.url.clone(),
        date: m.date,
        players: m.players.clone(),
        events,
    })
}

fn player_nr(player: Player) -> u32 {
    match player {
        Player::P1 => 1,
        Player::P2 => 2,
    }
}

impl Eat<&str, (), [String; 2]> for FootballOption {
    fn eat(i: &str, players: [String; 2]) -> Result<(&str, Self), ()> {
        use FootballOption::*;
        if let Ok(i) = "02".drop(i) {
            return Ok((i, NotWin(1)));
        }
        if let Ok(i) = "10".drop(i) {
            return Ok((i, NotWin(2)));
        }
        if let Ok(i) = "12".drop(i) {
            return Ok((i, NotWin(0)));
        }
        if let Ok((i, r1)) = u32::eat(i, ()) {
            if let Ok(i) = '/'.drop(i) {
                if r1 > 2 {
                    return Err(());
                }
                if let Ok((i, r2)) = u32::eat(i, ()) {
                    if r2 > 2 {
                        return Err(());
                    }
                    return Ok((i, Win2(r1, r2)));
                }
                if let Ok((i, b)) = eat_bool(i) {
                    return Ok((i, WinBool(r1, b)));
                }
                if let Ok((i, line)) = eat_line(i) {
                    return Ok((i, WinLine(r1, line)));
                }
            }
            if let Ok(i) = ':'.drop(i) {
                if let Ok((i, r2)) = u32::eat(i, ()) {
                    return Ok((i, Score(r1, r2)));
                }
            }
            if let Ok(i) = '-'.drop(i) {
                if let Ok((i, r2)) = u32::eat(i, ()) {
                    return Ok((i, Range(r1, r2)));
                }
            }
            if r1 <= 2 {
                return Ok((i, Win(r1)));
            }
        }
        if let Ok((i, p)) = eat_player(i, players.clone()) {
            if let Ok(i) = ' '.drop(i) {
                if let Ok((i, line)) = eat_line(i) {
                    return Ok((i, PlayerLine(p, line)));
                }
            }
            if let Ok(i) = '/'.drop(i) {
                if let Ok((i, b)) = eat_bool(i) {
                    return Ok((i, PlayerBool(p, b)));
                }
                if let Ok((i, line)) = eat_line(i) {
                    return Ok((i, PlayerLine(p, line)));
                }
            }
            if let Ok(i) = " - ".drop(i) {
                if let Ok((i, p2)) = eat_player(i, players.clone()) {
                    return Ok((i, PlayerPlayer(p, p2)));
                }
            }
            if let Ok(i) = " wygra, ".drop(i) {
                if let Ok(i) = "Powyżej ".drop(i) {
                    if let Ok((i, x)) = eat_till(i, " goli") {
                        let (_, n) = f64::eat(x, ())?;
                        if let Ok(i) = ", ".drop(i) {
                            if let Ok((i, x)) = eat_till(i, " strzeli gola") {
                                return Ok((
                                    i,
                                    WinOverParticipant(
                                        player_nr(p),
                                        n,
                                        x.to_string(),
                                    ),
                                ));
                            }
                        }
                    }
                }
                if let Ok((i, x)) = eat_till(i, " strzeli gola") {
                    return Ok((
                        i,
                        WinParticipant(player_nr(p), x.to_string()),
                    ));
                }
            }
            return Ok((i, Player(p)));
        }
        if let Ok((i, line)) = eat_line(i) {
            return Ok((i, L(line)));
        }
        if let Ok((i, b1)) = eat_bool(i) {
            if let Ok(i) = '/'.drop(i) {
                if let Ok((i, b2)) = eat_bool(i) {
                    return Ok((i, BoolBool(b1, b2)));
                }
                if let Ok((i, line)) = eat_line(i) {
                    return Ok((i, BoolLine(b1, line)));
                }
            }
            return Ok((i, Bool(b1)));
        }
        if let Ok(i) = "Tylko ".drop(i) {
            if let Ok((i, p)) = eat_player(i, players.clone()) {
                return Ok((i, Only(p)));
            }
        }
        if let Ok(i) = "Remis".drop(i) {
            if let Ok(i) = ", ".drop(i) {
                if let Ok((i, x)) = eat_till(i, " strzeli gola") {
                    return Ok((i, WinParticipant(0, x.to_string())));
                }
            }
            return Ok((i, Draw));
        }
        if let Ok(i) = "Obie".drop(i) {
            if let Ok(i) = " drużyny strzelą gola, ".drop(i) {
                if let Ok((i, x)) = eat_till(i, " strzeli gola") {
                    return Ok((i, BothParticipant(x.to_string())));
                }
            }
            return Ok((i, Both));
        }
        eat!(i, "Nikt", NoPlayer);
        eat!(i, "Równo", Eq);
        eat!(i, "Pierwszy", First);
        eat!(i, "Drugi", Second);
        eat!(i, "inny", Other);
        eat!(i, "Brak goli", Goals0);
        eat!(i, "Żadna", Neither);
        Err(())
    }
}

fn eat_till<'a>(
    i: &'a str,
    pattern: &'a str,
) -> Result<(&'a str, &'a str), ()> {
    if let Some(index) = i.find(pattern) {
        let x = &i[..index];
        let i = &i[index + pattern.len()..];
        return Ok((i, x));
    }
    Err(())
}

fn eat_line(i: &str) -> Result<(&str, Line), ()> {
    let over = |i| {
        eat!(i, "Wiecej ");
        eat!(i, "wiecej ");
        eat!(i, "+ ");
        eat!(i, "+");
        Err(())
    };
    if let Ok(i) = over(i) {
        let (i, x) = eat_f64(i)?;
        let line = Line(Ordering::Greater, x);
        return Ok((i, line));
    }
    let under = |i| {
        eat!(i, "Mniej ");
        eat!(i, "mniej ");
        eat!(i, "- ");
        eat!(i, "-");
        Err(())
    };
    if let Ok(i) = under(i) {
        let (i, x) = eat_f64(i)?;
        let line = Line(Ordering::Less, x);
        return Ok((i, line));
    }
    if let Ok((i, x)) = eat_f64(i) {
        if let Ok(i) = '+'.drop(i) {
            let line = Line(Ordering::Greater, x);
            return Ok((i, line));
        }
    }
    if let Ok((i, x)) = eat_f64(i) {
        if let Ok(i) = '-'.drop(i) {
            let line = Line(Ordering::Less, x);
            return Ok((i, line));
        }
    }
    Err(())
}

fn eat_f64(i: &str) -> Result<(&str, f64), ()> {
    if let Ok((i, x)) = f64::eat(i, ()) {
        return Ok((i, x));
    }
    if let Ok((i, x)) = u64::eat(i, ()) {
        return Ok((i, x as f64));
    }
    Err(())
}

fn eat_bool(i: &str) -> Result<(&str, bool), ()> {
    eat!(i, "Tak", true);
    eat!(i, "Nie", false);
    eat!(i, "tak", true);
    eat!(i, "nie", false);
    Err(())
}

impl Eat<&str, (), [String; 2]> for Football {
    fn eat(i: &str, players: [String; 2]) -> Result<(&str, Self), ()> {
        if let Ok((i, p)) = eat_player(i, players.clone()) {
            return eat_event_player(i, p);
        }
        if let Ok((i, part)) = eat_part(i) {
            return eat_event_part(i, part, players);
        }
        eat!(i, "Dokładna liczba goli", ExactGoals(Part::FullTime));
        if let Ok(i) = eat_both(i) {
            let i = " strzelą ".drop(i)?;
            if let Ok(i) = "gola".drop(i) {
                if let Ok(i) = ' '.drop(i) {
                    eat!(i, "w 1.połowie", BothToScore(Part::FirstHalf));
                    eat!(i, "w 2.połowie", BothToScore(Part::FirstHalf));
                    eat!(i, "w tej samej połowie", BothToScoreSameHalf);
                    eat!(
                        i,
                        "lub liczba goli wyższa niż",
                        BothToScoreOrGoalsOver
                    );
                }
                eat!(i, "/liczba goli", BothToScoreGoals);
                return Ok((i, BothToScore(Part::FullTime)));
            }
            eat!(i, "po 2 lub więcej goli", BothToScoreAtLeast(2));
            eat!(i, "po 3 lub więcej goli", BothToScoreAtLeast(3));
        }
        eat!(i, "Handicap", Handicap(Part::FullTime));
        eat!(i, "Multigole", MultiGoals(Part::FullTime));
        if let Ok(i) = "Liczba ".drop(i) {
            eat!(i, "goli", Goals(Part::FullTime));
            eat!(i, "rzutów rożnych", Corners(Part::FullTime));
            if let Ok(i) = "spalonych".drop(i) {
                if let Ok(i) = eat_overtime(i) {
                    return Ok((i, Offsides(Overtime::Include)));
                }
                return Ok((i, Offsides(Overtime::Exclude)));
            }
            if let Ok(i) = "strzałów w światło bramki".drop(i) {
                if let Ok(i) = eat_overtime(i) {
                    return Ok((i, ShotsOnTarget(Overtime::Include)));
                }
                return Ok((i, ShotsOnTarget(Overtime::Exclude)));
            }
            if let Ok(i) = "fauli".drop(i) {
                if let Ok(i) = eat_overtime(i) {
                    return Ok((i, Fouls(Overtime::Include)));
                }
                return Ok((i, Fouls(Overtime::Exclude)));
            }
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, YellowCards(Part::FullTime)));
            }
        }
        eat!(i, "Rzut karny", Penalty);
        eat!(i, "Będą serie rzutów karnych", PenaltySeries);
        eat!(i, "Wynik meczu - dwójtyp", DoubleChance(Part::FullTime));
        eat!(
            i,
            "Podwójna szansa (1.poł. lub mecz)",
            DoubleChanceH1OrMatch
        );
        if let Ok(i) = eat_half_with_more(i) {
            eat!(i, "goli", HalfWithMoreGoals);
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, HalfWithMoreYellowCards));
            }
        }
        eat!(i, "Otrzyma kartkę", WillGetCard);
        eat!(i, "Otrzyma czerwoną kartkę", WillGetRedCard);
        if let Ok(i) = "Dwójtyp".drop(i) {
            eat!(i, "/liczba goli", DoubleChanceGoalRange);
            eat!(i, "/obie drużyny strzelą gola", DoubleChanceBothToScore);
        }
        if let Ok(i) = "1.gol".drop(i) {
            eat!(i, "/spotkanie", FirstGoalMatch);
            eat!(i, "-minuta", FirstGoalMinute);
            return Ok((i, FirstGoal(Part::FullTime)));
        }
        eat!(i, "Pozostałe zakłady łączone", RestProduct);
        eat!(i, "Padnie gol-minuta", GoalBeforeMinute);
        eat!(i, "Nie padnie gol-minuta", NoGoalBeforeMinute);
        eat!(i, "Różnica zwycięstwa", WinDiff);
        eat!(i, "1.rzut rożny w spotkaniu", FirstCorner(Part::FullTime));
        if let Ok(i) = "Zawodnicy - ".drop(i) {
            if let Ok(i) = "strzały".drop(i) {
                eat!(i, " celne", PlayerShotOnTarget);
                return Ok((i, PlayerShot));
            }
            eat!(i, "asysty", PlayerAssists);
        }
        if let Ok(i) = eat_more(i) {
            let i = ' '.drop(i)?;
            eat!(i, "rzutów rożnych", MoreCorners(Part::FullTime));
            if let Ok(i) = "strzałów w światło bramki".drop(i) {
                if let Ok(i) = " - handicap".drop(i) {
                    if let Ok(i) = eat_overtime(i) {
                        return Ok((
                            i,
                            MoreShotsOnTargetHandicap(Overtime::Include),
                        ));
                    }
                    return Ok((
                        i,
                        MoreShotsOnTargetHandicap(Overtime::Exclude),
                    ));
                }
                if let Ok(i) = eat_overtime(i) {
                    return Ok((i, MoreShotsOnTarget(Overtime::Include)));
                }
                return Ok((i, MoreShotsOnTarget(Overtime::Exclude)));
            }
            if let Ok(i) = "fauli".drop(i) {
                if let Ok(i) = " - handicap".drop(i) {
                    if let Ok(i) = eat_overtime(i) {
                        return Ok((i, MoreFoulsHandicap(Overtime::Include)));
                    }
                    return Ok((i, MoreFoulsHandicap(Overtime::Exclude)));
                }
                if let Ok(i) = eat_overtime(i) {
                    return Ok((i, MoreFouls(Overtime::Include)));
                }
                return Ok((i, MoreFouls(Overtime::Exclude)));
            }
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, MoreYellowCards));
            }
            if let Ok(i) = "spalonych".drop(i) {
                if let Ok(i) = " - handicap".drop(i) {
                    if let Ok(i) = eat_overtime(i) {
                        return Ok((
                            i,
                            MoreOffsidesHandicap(Overtime::Include),
                        ));
                    }
                    return Ok((i, MoreOffsidesHandicap(Overtime::Exclude)));
                }
                if let Ok(i) = eat_overtime(i) {
                    return Ok((i, MoreOffsides(Overtime::Include)));
                }
                return Ok((i, MoreOffsides(Overtime::Exclude)));
            }
        }
        eat!(i, "1-15 minuta spotkania", Minute15);
        eat!(i, "1-30 minuta spotkania", Minute30);
        eat!(i, "1-60 minuta spotkania", Minute60);
        eat!(i, "1-75 minuta spotkania", Minute75);
        eat!(i, "Będzie wynik w trakcie spotkania", ResultDuringMatch);
        eat!(
            i,
            "Nie będzie wyniku w trakcie spotkania",
            NotResultDuringMatch
        );
        if let Ok(i) = "Spotkanie".drop(i) {
            eat!(i, " bez remisu", DrawNoBet(Part::FullTime));
            if let Ok(i) = '/'.drop(i) {
                if let Ok((i, p)) = eat_player(i, players.clone()) {
                    eat!(i, " liczba goli", MatchGoalsPlayer(p));
                }
                if let Ok((i, part)) = eat_part(i) {
                    eat!(i, " liczba goli", MatchGoals(part));
                }
            }
            return Ok((i, Winner(Part::FullTime)));
        }
        eat!(i, "Przedział goli", GoalRange);
        if let Ok(i) = "Zawodnik rezerwowy strzeli gola ".drop(i) {
            eat!(
                i,
                "(razem z dogrywką. nie wystąpi od początku spotkania)",
                SubstituteWillScore(Overtime::Include)
            );
            eat!(
                i,
                "(nie wystąpi od początku spotkania)",
                SubstituteWillScore(Overtime::Exclude)
            );
        }
        eat!(i, "Będzie przegrywać, ale..", WillBeLosingBut);
        if let Ok(i) = "Mecz".drop(i) {
            eat!(i, " + strzelcy goli", MatchScorePlayers);
            eat!(i, " + strzały na bramkę", MatchShotsOnTarget);
            if let Ok(i) = ": ".drop(i) {
                if let Ok(i) = "liczba rzutów rożnych".drop(i) {
                    eat!(i, " handicap", MatchCornersHandicap(Part::FullTime));
                    return Ok((i, MatchCorners(Part::FullTime)));
                }
                eat!(i, "która drużyna strzeli gola", PlayerToScore);
                eat!(i, "więcej rzutów rożnych", MatchMoreCorners);
                eat!(i, "Przedział rzutów rożnych", MatchCornerRange);
                eat!(i, "Multiwynik", MatchMultiScore);
                eat!(i, "suma goli", MatchGoalSum);
            }
            eat!(i, "/liczba goli", MatchGoals(Part::FullTime));
            eat!(i, "/obie drużyny strzelą gola", MatchBothToScore);
            return Ok((i, Winner(Part::FullTime)));
        }
        eat!(i, "Awans", ToAdvance);
        eat!(i, "awans", ToAdvance);
        eat!(i, "Sposób awansu", AdvanceBy);
        eat!(i, "Dokładny wynik", ExactScore(Part::FullTime));
        eat!(i, "Zwycięzca finału", FinaleWinner);
        eat!(i, "Gol w obu połowach", GoalBothHalves);
        eat!(
            i,
            "Gol w minucie meczu (85:00 lub później)",
            GoalAfterMinute85
        );
        eat!(
            i,
            "1i2 drużyna strzeli gola w obu połowach",
            BothGoalBothHalves
        );
        if let Ok(i) = eat_red(i) {
            return Ok((i, RedCard(Part::FullTime)));
        }
        eat!(i, "padnie gol samobójczy", SuicideGoal);
        if let Ok(i) = "SUPEROFERTA+: mecz".drop(i) {
            let i = "\n                                    : ".drop(i)?;
            let i = players[0].as_str().drop(i)?;
            let sep = |i| {
                eat!(i, "- ");
                eat!(i, " -");
                Err(())
            };
            let i = sep(i)?;
            let i = players[1].as_str().drop(i)?;
            return Ok((i, Superoffer));
        }
        eat!(
            i,
            "Drużyna wygra lub będzie prowadziła różnicą dwóch goli w meczu",
            WinnerOr2GoalAdvantage
        );
        eat!(i, "Zostanie zdobyty hattrick", Hattrick);
        eat!(i, "Faule zawodnika", FoulsParticipant);
        Err(())
    }
}

fn eat_half_with_more(i: &str) -> Result<&str, ()> {
    eat!(i, "Połowa z wiekszą liczbą ");
    eat!(i, "Połowa z większą liczbą ");
    Err(())
}

fn eat_more(i: &str) -> Result<&str, ()> {
    eat!(i, "Wiecej");
    eat!(i, "Więcej");
    eat!(i, "wiecej");
    eat!(i, "więcej");
    Err(())
}

fn eat_player(i: &str, [p1, p2]: [String; 2]) -> Result<(&str, Player), ()> {
    use Player::*;
    eat!(i, p1.as_str(), P1);
    eat!(i, p2.as_str(), P2);
    let player = |i| {
        eat!(i, '1', P1);
        eat!(i, '2', P2);
        Err(())
    };
    let (i, p) = player(i)?;
    let i = '.'.drop(i)?;
    eat!(i, "druzyna", p);
    eat!(i, "drużyna", p);
    Err(())
}

fn eat_overtime(i: &str) -> Result<&str, ()> {
    " (razem z dogrywką)".drop(i)
}

fn eat_event_part(
    i: &str,
    part: Part,
    players: [String; 2],
) -> Result<(&str, Football), ()> {
    if let Ok(i) = ' '.drop(i) {
        if let Ok(i) = "liczba ".drop(i) {
            if let Ok(i) = "goli".drop(i) {
                if let Ok(i) = '/'.drop(i) {
                    if let Ok((i, part2)) = eat_part(i) {
                        eat!(i, " liczba goli", Goals2(part, part2));
                    }
                }
                return Ok((i, Goals(part)));
            }
            if let Ok(i) = "rzutów rożnych".drop(i) {
                eat!(i, " handicap", CornersHandicap(part));
                return Ok((i, Corners(part)));
            }
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, YellowCards(part)));
            }
        }
        eat!(i, "bez remisu", DrawNoBet(part));
        eat!(i, "- dwójtyp", DoubleChance(part));
    }
    if let Ok(i) = ": ".drop(i) {
        if let Ok(i) = eat_more(i) {
            if let Ok(i) = ' '.drop(i) {
                eat!(i, "rzutów rożnych", MoreCorners(part));
            }
        }
        if let Ok(i) = "liczba ".drop(i) {
            if let Ok(i) = "rzutów rożnych".drop(i) {
                eat!(i, " handicap", CornersHandicap(part));
                return Ok((i, Corners(part)));
            }
        }
        eat!(i, "przedział rzutów rożnych", CornerRange(part));
        eat!(i, "Multigole", MultiGoals(part));
        eat!(i, "dokładny wynik", ExactScore(part));
        eat!(i, "dokładna liczba goli", ExactGoals(part));
        eat!(i, "1.gol", FirstGoal(part));
        eat!(i, "1.rzut rożny", FirstCorner(part));
        eat!(i, "handicap", Handicap(part));
        if let Ok((i, p)) = eat_player(i, players) {
            let i = ' '.drop(i)?;
            eat!(i, "przedział rzutów rożnych", CornerRangePlayer(p, part));
            if let Ok(i) = eat_red(i) {
                return Ok((i, RedCardPlayer(p, part)));
            }
        }
        if let Ok(i) = eat_red(i) {
            return Ok((i, RedCard(part)));
        }
    }
    if let Ok(i) = '/'.drop(i) {
        eat!(i, "spotkanie", Meeting(part));
        if let Ok(i) = "mecz".drop(i) {
            eat!(i, " i liczba goli", WinnerMatchAndGoals(part));
            return Ok((i, Match(part)));
        }
        if let Ok(i) = eat_both(i) {
            let i = " strzelą ".drop(i)?;
            if let Ok(i) = "gola".drop(i) {
                if let Ok(i) = ' '.drop(i) {
                    eat!(
                        i,
                        "w 1.połowie",
                        WinnerBothToScore(part, Part::FirstHalf)
                    );
                    eat!(
                        i,
                        "w 2.połowie",
                        WinnerBothToScore(part, Part::SecondHalf)
                    );
                }
            }
        }
        if let Ok((i, part2)) = eat_part(i) {
            if let Ok(i) = ' '.drop(i) {
                if let Ok(i) = eat_both(i) {
                    let i = " strzelą ".drop(i)?;
                    eat!(i, "gola", Winner2BothToScore(part, part2));
                }
            }
            return Ok((i, Winner2(part, part2)));
        }
    }
    eat!(i, "-zmiana", Shift(part));
    Ok((i, Winner(part)))
}

fn eat_both(i: &str) -> Result<&str, ()> {
    eat!(i, "Obie drużyny");
    eat!(i, "obie drużyny");
    Err(())
}

fn eat_event_player(i: &str, p: Player) -> Result<(&str, Football), ()> {
    let i = ' '.drop(i)?;
    eat!(
        i,
        "przedział rzutów rożnych",
        CornerRangePlayer(p, Part::FullTime)
    );
    eat!(i, "1.gol-minuta", FirstGoalMinutePlayer(p));
    if let Ok(i) = "wygra ".drop(i) {
        eat!(i, "do zera", WinToNil(p));
        eat!(i, "obie połowy", WinBothHalves(p));
        eat!(i, "przynajmniej jedna połowę", WinAtLeastOneHalf(p));
    }
    eat!(i, "Multigole", MultiGoalsPlayer(p));
    if let Ok(i) = "strzeli ".drop(i) {
        eat!(i, "gola w obu połowach", ScoreBothHalves(p));
        eat!(i, "bramkę samobójczą", SuicideGoalPlayer(p));
    }
    eat!(i, "dokładna liczba goli", ExactGoalsPlayer(p));
    if let Ok(i) = "liczba ".drop(i) {
        eat!(i, "goli", GoalsPlayer(p, Part::FullTime));
        eat!(i, "rzutów rożnych", CornersPlayer(p, Part::FullTime));
        if let Ok(i) = "strzałów w światło bramki".drop(i) {
            if let Ok(i) = eat_overtime(i) {
                return Ok((i, ShotsOnTargetPlayer(p, Overtime::Include)));
            }
            return Ok((i, ShotsOnTargetPlayer(p, Overtime::Exclude)));
        }
        if let Ok(i) = "spalonych".drop(i) {
            if let Ok(i) = eat_overtime(i) {
                return Ok((i, OffsidesPlayer(p, Overtime::Include)));
            }
            return Ok((i, OffsidesPlayer(p, Overtime::Exclude)));
        }
        if let Ok(i) = "fauli".drop(i) {
            if let Ok(i) = eat_overtime(i) {
                return Ok((i, FoulsPlayer(p, Overtime::Include)));
            }
            return Ok((i, FoulsPlayer(p, Overtime::Exclude)));
        }
        if let Ok(i) = eat_yellow(i) {
            return Ok((i, YellowCardsPlayer(p, Part::FullTime)));
        }
    }
    if let Ok((i, part)) = eat_part(i) {
        if let Ok(i) = ' '.drop(i) {
            if let Ok(i) = "liczba ".drop(i) {
                eat!(i, "goli", GoalsPlayer(p, part));
                eat!(i, "rzutów rożnych", CornersPlayer(p, part));
                if let Ok(i) = eat_yellow(i) {
                    return Ok((i, YellowCardsPlayer(p, part)));
                }
            }
        }
    }
    if let Ok(i) = eat_red(i) {
        return Ok((i, RedCardPlayer(p, Part::FullTime)));
    }
    Err(())
}

fn eat_part(i: &str) -> Result<(&str, Part), ()> {
    eat!(i, "1.połowa", Part::FirstHalf);
    eat!(i, "2.połowa", Part::SecondHalf);
    Err(())
}

fn eat_yellow(i: &str) -> Result<&str, ()> {
    let yellow = |i| {
        eat!(i, "zółtych");
        eat!(i, "żółtych");
        Err(())
    };
    let i = yellow(i)?;
    " kartek (bez żółtych kartek dla trenera i sztabu)".drop(i)
}

fn eat_red(i: &str) -> Result<&str, ()> {
    let red = |i| {
        eat!(i, "Czerwona");
        eat!(i, "czerwona");
        Err(())
    };
    let i = red(i)?;
    " kartka (bez czerwonych kartek dla trenera i sztabu)".drop(i)
}

fn time_min(part: Part) -> Option<f64> {
    match part {
        Part::SecondHalf => Some(45.),
        _ => None,
    }
}

fn time_max(part: Part) -> Option<f64> {
    match part {
        Part::FirstHalf => Some(45.),
        _ => None,
    }
}

fn db_player(x: Player) -> db::Player {
    match x {
        Player::P1 => db::Player::P1,
        Player::P2 => db::Player::P2,
    }
}

pub fn translate(x: Football, o: FootballOption) -> Result<db::Event, ()> {
    use Football::*;
    use FootballOption::*;
    match x {
        Winner(part) => Ok(db::Event {
            tag: db::Football::Win,
            params: db::Params {
                player: match o {
                    Win(0) => None,
                    Win(1) => Some(db::Player::P1),
                    Win(2) => Some(db::Player::P2),
                    _ => panic!(),
                },
                time_min: time_min(part),
                time_max: time_max(part),
                ..db::Params::default()
            },
        }),
        Handicap(part) => match o {
            PlayerLine(p, line) => Ok(db::Event {
                tag: db::Football::Win,
                params: db::Params {
                    player: Some(db_player(p)),
                    time_min: time_min(part),
                    time_max: time_max(part),
                    handicap: match line.0 {
                        Ordering::Greater => Some(line.1),
                        Ordering::Less => Some(-line.1),
                        _ => panic!(),
                    },
                    ..db::Params::default()
                },
            }),
            _ => panic!(),
        },
        Goals(part) => Ok(db::Event {
            tag: db::Football::Goals,
            params: db::Params {
                time_min: time_min(part),
                time_max: time_max(part),
                min: match o {
                    L(Line(Ordering::Greater, value)) => Some(value),
                    L(Line(Ordering::Equal, value)) => Some(value),
                    _ => panic!(),
                },
                max: match o {
                    L(Line(Ordering::Less, value)) => Some(value),
                    L(Line(Ordering::Equal, value)) => Some(value),
                    _ => panic!(),
                },
                ..db::Params::default()
            },
        }),
        _ => todo!(),
    }
}
