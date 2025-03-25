use crate::shared::event;
use eat::*;
use event::Football::*;
use event::Part;

macro_rules! eat {
    ($i:ident -> $pattern:pat, $text:expr, $r:expr) => {
        if let Ok($pattern) = $text.drop($i) {
            return Ok(($i, $r));
        }
    };
}

macro_rules! eat2 {
    ($i:ident, $text:expr) => {
        if let Ok($i) = $text.drop($i) {
            return Ok($i);
        }
    };
}

impl Eat<&str, (), [String; 2]> for event::Football {
    fn eat(i: &str, players: [String; 2]) -> Result<(&str, Self), ()> {
        if let Ok((i, p)) = eat_player(i, players.clone()) {
            return eat_event_player(i, p);
        }
        if let Ok((i, part)) = eat_part(i) {
            return eat_event_part(i, part, players);
        }
        eat!(i -> i, "Dokładna liczba goli", ExactGoals(Part::FullTime));
        if let Ok(i) = eat_both(i) {
            let i = " strzelą ".drop(i)?;
            if let Ok(i) = "gola".drop(i) {
                if let Ok(i) = ' '.drop(i) {
                    eat!(i -> i, "w 1.połowie", BothToScore(Part::FirstHalf));
                    eat!(i -> i, "w 2.połowie", BothToScore(Part::FirstHalf));
                    eat!(i -> i, "w tej samej połowie", BothToScoreSameHalf);
                }
                eat!(i -> i, "/liczba goli", BothToScoreGoals);
                return Ok((i, BothToScore(Part::FullTime)));
            }
            eat!(i -> i, "po 2 lub więcej goli", BothToScore2);
        }
        eat!(i -> i, "Handicap", Handicap(Part::FullTime));
        eat!(i -> i, "Multigole", MultiGoals(Part::FullTime));
        if let Ok(i) = "Liczba ".drop(i) {
            eat!(i -> i, "goli", Goals(Part::FullTime));
            eat!(i -> i, "rzutów rożnych", Corners(Part::FullTime));
            eat!(i -> i, "spalonych", Offsides);
            eat!(i -> i, "strzałów w światło bramki", ShotsOnTarget);
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, YellowCards(Part::FullTime)));
            }
        }
        eat!(i -> i, "Rzut karny", Penalty);
        eat!(i -> i, "Wynik meczu - dwójtyp", DoubleChance);
        eat!(i -> i, "Podwójna szansa (1.poł. lub mecz)", DoubleChanceH1OrMatch);
        eat!(i -> i, "Połowa z wiekszą liczbą goli", HalfWithMoreGoals);
        eat!(i -> i, "Otrzyma kartkę", WillGetCard);
        if let Ok(i) = "Dwójtyp".drop(i) {
            eat!(i -> i, "/liczba goli", DoubleChanceGoalRange);
            eat!(i -> i, "/obie drużyny strzelą gola", DoubleChanceBothToScore);
        }
        if let Ok(i) = "1.gol".drop(i) {
            eat!(i -> i, "/spotkanie", FirstGoalMatch);
            eat!(i -> i, "-minuta", FirstGoalMinute);
            return Ok((i, FirstGoal(Part::FullTime)));
        }
        eat!(i -> i, "Pozostałe zakłady łączone", RestProduct);
        eat!(i -> i, "Padnie gol-minuta", GoalBeforeMinute);
        eat!(i -> i, "Różnica zwycięstwa", WinDiff);
        eat!(i -> i, "1.rzut rożny w spotkaniu", FirstCorner(Part::FullTime));
        if let Ok(i) = "Zawodnicy - strzały".drop(i) {
            eat!(i -> i, " celne", PlayerShotOnTarget);
            return Ok((i, PlayerShot));
        }
        if let Ok(i) = eat_more(i) {
            let i = ' '.drop(i)?;
            eat!(i -> i, "rzutów rożnych", MoreCorners(Part::FullTime));
            eat!(i -> i, "strzałów w światło bramki", MoreShotsOnTarget);
            eat!(i -> i, "fauli", MoreFouls);
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, MoreYellowCards));
            }
        }
        eat!(i -> i, "1-30 minuta spotkania", Minute30);
        eat!(i -> i, "1-60 minuta spotkania", Minute60);
        eat!(i -> i, "1-75 minuta spotkania", Minute75);
        eat!(i -> i, "Będzie wynik w trakcie spotkania", ResultDuringMatch);
        eat!(i -> i, "Nie będzie wyniku w trakcie spotkania", NotResultDuringMatch);
        if let Ok(i) = "Spotkanie".drop(i) {
            eat!(i -> i, " bez remisu", DrawNoBet(Part::FullTime));
            if let Ok(i) = '/'.drop(i) {
                if let Ok((i, p)) = eat_player(i, players.clone()) {
                    eat!(i -> i, " liczba goli", MatchGoalsPlayer(p));
                }
                if let Ok((i, part)) = eat_part(i) {
                    eat!(i -> i, " liczba goli", MatchGoals(part));
                }
            }
        }
        eat!(i -> i, "Przedział goli", GoalRange);
        eat!(i -> i, "Zawodnik rezerwowy strzeli gola (nie wystąpi od początku spotkania)", SubstituteWillScore);
        eat!(i -> i, "Będzie przegrywać, ale..", WillBeLosingBut);
        if let Ok(i) = "Mecz".drop(i) {
            eat!(i -> i, " + strzelcy goli", MatchScorePlayers);
            eat!(i -> i, " + strzały na bramkę", MatchShotsOnTarget);
            if let Ok(i) = ": ".drop(i) {
                eat!(i -> i, "liczba rzutów rożnych", MatchCorners(Part::FullTime));
                eat!(i -> i, "która drużyna strzeli gola", PlayerToScore);
                eat!(i -> i, "więcej rzutów rożnych", MatchMoreCorners);
                eat!(i -> i, "Przedział rzutów rożnych", MatchCornerRange);
                eat!(i -> i, "Multiwynik", MatchMultiScore);
            }
            eat!(i -> i, "/liczba goli", MatchGoals(Part::FullTime));
            eat!(i -> i, "/obie drużyny strzelą gola", MatchBothToScore);
        }
        Err(())
    }
}

fn eat_more(i: &str) -> Result<&str, ()> {
    eat2!(i, "Wiecej");
    eat2!(i, "Więcej");
    eat2!(i, "wiecej");
    eat2!(i, "więcej");
    Err(())
}

fn eat_player(
    i: &str,
    [p1, p2]: [String; 2],
) -> Result<(&str, event::Player), ()> {
    use event::Player::*;
    eat!(i -> i, p1.as_str(), P1);
    eat!(i -> i, p2.as_str(), P2);
    let player = |i| {
        eat!(i -> i, '1', P1);
        eat!(i -> i, '2', P2);
        Err(())
    };
    let (i, p) = player(i)?;
    let i = '.'.drop(i)?;
    eat!(i -> i, "druzyna", p);
    eat!(i -> i, "drużyna", p);
    Err(())
}

fn eat_event_part(
    i: &str,
    part: event::Part,
    players: [String; 2],
) -> Result<(&str, event::Football), ()> {
    if let Ok(i) = ' '.drop(i) {
        if let Ok(i) = "liczba ".drop(i) {
            if let Ok(i) = "goli".drop(i) {
                if let Ok(i) = '/'.drop(i) {
                    if let Ok((i, part2)) = eat_part(i) {
                        eat!(i -> i, " liczba goli", Goals2(part, part2));
                    }
                }
                return Ok((i, Goals(part)));
            }
            if let Ok(i) = "rzutów rożnych".drop(i) {
                eat!(i -> i, " handicap", CornersHandicap(part));
                return Ok((i, Corners(part)));
            }
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, YellowCards(part)));
            }
        }
        eat!(i -> i, "bez remisu", DrawNoBet(part));
    }
    if let Ok(i) = ": ".drop(i) {
        if let Ok(i) = eat_more(i) {
            if let Ok(i) = ' '.drop(i) {
                eat!(i -> i, "rzutów rożnych", MoreCorners(part));
            }
        }
        if let Ok(i) = "liczba ".drop(i) {
            if let Ok(i) = "rzutów rożnych".drop(i) {
                eat!(i -> i, " handicap", CornersHandicap(part));
                return Ok((i, Corners(part)));
            }
        }
        eat!(i -> i, "przedział rzutów rożnych", CornerRange(part));
        eat!(i -> i, "Multigole", MultiGoals(part));
        eat!(i -> i, "dokładny wynik", ExactScore(part));
        eat!(i -> i, "dokładna liczba goli", ExactGoals(part));
        eat!(i -> i, "1.gol", FirstGoal(part));
        eat!(i -> i, "1.rzut rożny", FirstCorner(part));
        eat!(i -> i, "handicap", Handicap(part));
        if let Ok((i, p)) = eat_player(i, players) {
            eat!(i -> i, " przedział rzutów rożnych", CornerRangePlayer(p, part));
        }
    }
    if let Ok(i) = '/'.drop(i) {
        eat!(i -> i, "spotkanie", Meeting(part));
        if let Ok(i) = "mecz".drop(i) {
            eat!(i -> i, " i liczba goli", WinnerMatchAndGoals(part));
            return Ok((i, Match(part)));
        }
        if let Ok(i) = eat_both(i) {
            let i = " strzelą ".drop(i)?;
            if let Ok(i) = "gola".drop(i) {
                if let Ok(i) = ' '.drop(i) {
                    eat!(i -> i, "w 1.połowie",
                        WinnerBothToScore(part, Part::FirstHalf));
                    eat!(i -> i, "w 2.połowie",
                        WinnerBothToScore(part, Part::SecondHalf));
                }
            }
        }
        if let Ok((i, part2)) = eat_part(i) {
            if let Ok(i) = ' '.drop(i) {
                if let Ok(i) = eat_both(i) {
                    let i = " strzelą ".drop(i)?;
                    eat!(i -> i, "gola", Winner2BothToScore(part, part2));
                }
            }
            return Ok((i, Winner2(part, part2)));
        }
    }
    Ok((i, Winner(part)))
}

fn eat_both(i: &str) -> Result<&str, ()> {
    eat2!(i, "Obie drużyny");
    eat2!(i, "obie drużyny");
    Err(())
}

fn eat_event_player(
    i: &str,
    p: event::Player,
) -> Result<(&str, event::Football), ()> {
    use event::Football::*;
    let i = ' '.drop(i)?;
    eat!(i -> i, "przedział rzutów rożnych", CornerRangePlayer(p, Part::FullTime));
    eat!(i -> i, "1.gol-minuta", FirstGoalMinutePlayer(p));
    if let Ok(i) = "wygra ".drop(i) {
        eat!(i -> i, "do zera", WinToNil(p));
        eat!(i -> i, "obie połowy", WinBothHalves(p));
        eat!(i -> i, "przynajmniej jedna połowę", WinAtLeastOneHalf(p));
    }
    eat!(i -> i, "Multigole", MultiGoalsPlayer(p));
    eat!(i -> i, "strzeli gola w obu połowach", ScoreBothHalves(p));
    eat!(i -> i, "dokładna liczba goli", ExactGoalsPlayer(p));
    if let Ok(i) = "liczba ".drop(i) {
        eat!(i -> i, "goli", GoalsPlayer(p, Part::FullTime));
        eat!(i -> i, "rzutów rożnych", CornersPlayer(p, Part::FullTime));
        eat!(i -> i, "strzałów w światło bramki", ShotsOnTargetPlayer(p));
        eat!(i -> i, "spalonych", OffsidesPlayer(p));
        if let Ok(i) = eat_yellow(i) {
            return Ok((i, YellowCardsPlayer(p, Part::FullTime)));
        }
    }
    if let Ok((i, part)) = eat_part(i) {
        if let Ok(i) = ' '.drop(i) {
            if let Ok(i) = "liczba ".drop(i) {
                eat!(i -> i, "goli", GoalsPlayer(p, part));
                eat!(i -> i, "rzutów rożnych", CornersPlayer(p, part));
                if let Ok(i) = eat_yellow(i) {
                    return Ok((i, YellowCardsPlayer(p, part)));
                }
            }
        }
    }
    Err(())
}

fn eat_part(i: &str) -> Result<(&str, Part), ()> {
    eat!(i -> i, "1.połowa", Part::FirstHalf);
    eat!(i -> i, "2.połowa", Part::SecondHalf);
    Err(())
}

fn eat_yellow(i: &str) -> Result<&str, ()> {
    let yellow = |i| {
        eat2!(i, "zółtych");
        eat2!(i, "żółtych");
        Err(())
    };
    let i = yellow(i)?;
    " kartek (bez żółtych kartek dla trenera i sztabu)".drop(i)
}
