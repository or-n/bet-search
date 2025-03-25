use crate::shared::event;
use eat::*;
use event::Football::*;
use event::Part;

macro_rules! eat {
    ($i:ident, $text:expr, $r:expr) => {
        if let Ok($i) = $text.drop($i) {
            return Ok(($i, $r));
        }
    };
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
        eat!(i, "Dokładna liczba goli", ExactGoals(Part::FullTime));
        if let Ok(i) = eat_both(i) {
            let i = " strzelą ".drop(i)?;
            if let Ok(i) = "gola".drop(i) {
                if let Ok(i) = ' '.drop(i) {
                    eat!(i, "w 1.połowie", BothToScore(Part::FirstHalf));
                    eat!(i, "w 2.połowie", BothToScore(Part::FirstHalf));
                    eat!(i, "w tej samej połowie", BothToScoreSameHalf);
                }
                eat!(i, "/liczba goli", BothToScoreGoals);
                return Ok((i, BothToScore(Part::FullTime)));
            }
            eat!(i, "po 2 lub więcej goli", BothToScore2);
        }
        eat!(i, "Handicap", Handicap(Part::FullTime));
        eat!(i, "Multigole", MultiGoals(Part::FullTime));
        if let Ok(i) = "Liczba ".drop(i) {
            eat!(i, "goli", Goals(Part::FullTime));
            eat!(i, "rzutów rożnych", Corners(Part::FullTime));
            eat!(i, "spalonych", Offsides);
            eat!(i, "strzałów w światło bramki", ShotsOnTarget);
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, YellowCards(Part::FullTime)));
            }
        }
        eat!(i, "Rzut karny", Penalty);
        eat!(i, "Wynik meczu - dwójtyp", DoubleChance);
        eat!(
            i,
            "Podwójna szansa (1.poł. lub mecz)",
            DoubleChanceH1OrMatch
        );
        eat!(i, "Połowa z wiekszą liczbą goli", HalfWithMoreGoals);
        eat!(i, "Otrzyma kartkę", WillGetCard);
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
        eat!(i, "Różnica zwycięstwa", WinDiff);
        eat!(i, "1.rzut rożny w spotkaniu", FirstCorner(Part::FullTime));
        if let Ok(i) = "Zawodnicy - strzały".drop(i) {
            eat!(i, " celne", PlayerShotOnTarget);
            return Ok((i, PlayerShot));
        }
        if let Ok(i) = eat_more(i) {
            let i = ' '.drop(i)?;
            eat!(i, "rzutów rożnych", MoreCorners(Part::FullTime));
            eat!(i, "strzałów w światło bramki", MoreShotsOnTarget);
            eat!(i, "fauli", MoreFouls);
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, MoreYellowCards));
            }
        }
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
        }
        eat!(i, "Przedział goli", GoalRange);
        eat!(i, "Zawodnik rezerwowy strzeli gola (nie wystąpi od początku spotkania)",
            SubstituteWillScore);
        eat!(i, "Będzie przegrywać, ale..", WillBeLosingBut);
        if let Ok(i) = "Mecz".drop(i) {
            eat!(i, " + strzelcy goli", MatchScorePlayers);
            eat!(i, " + strzały na bramkę", MatchShotsOnTarget);
            if let Ok(i) = ": ".drop(i) {
                eat!(i, "liczba rzutów rożnych", MatchCorners(Part::FullTime));
                eat!(i, "która drużyna strzeli gola", PlayerToScore);
                eat!(i, "więcej rzutów rożnych", MatchMoreCorners);
                eat!(i, "Przedział rzutów rożnych", MatchCornerRange);
                eat!(i, "Multiwynik", MatchMultiScore);
            }
            eat!(i, "/liczba goli", MatchGoals(Part::FullTime));
            eat!(i, "/obie drużyny strzelą gola", MatchBothToScore);
        }
        Err(())
    }
}

fn eat_more(i: &str) -> Result<&str, ()> {
    eat!(i, "Wiecej");
    eat!(i, "Więcej");
    eat!(i, "wiecej");
    eat!(i, "więcej");
    Err(())
}

fn eat_player(
    i: &str,
    [p1, p2]: [String; 2],
) -> Result<(&str, event::Player), ()> {
    use event::Player::*;
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
            eat!(i, " przedział rzutów rożnych", CornerRangePlayer(p, part));
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
    Ok((i, Winner(part)))
}

fn eat_both(i: &str) -> Result<&str, ()> {
    eat!(i, "Obie drużyny");
    eat!(i, "obie drużyny");
    Err(())
}

fn eat_event_player(
    i: &str,
    p: event::Player,
) -> Result<(&str, event::Football), ()> {
    use event::Football::*;
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
    eat!(i, "strzeli gola w obu połowach", ScoreBothHalves(p));
    eat!(i, "dokładna liczba goli", ExactGoalsPlayer(p));
    if let Ok(i) = "liczba ".drop(i) {
        eat!(i, "goli", GoalsPlayer(p, Part::FullTime));
        eat!(i, "rzutów rożnych", CornersPlayer(p, Part::FullTime));
        eat!(i, "strzałów w światło bramki", ShotsOnTargetPlayer(p));
        eat!(i, "spalonych", OffsidesPlayer(p));
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
