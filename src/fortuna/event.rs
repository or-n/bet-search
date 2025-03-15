use crate::shared::event;
use eat::*;

impl Eat<&str, (), [String; 2]> for event::Football {
    fn eat(i: &str, players: [String; 2]) -> Result<(&str, Self), ()> {
        use event::Football::*;
        use event::Half::*;
        use event::Player::*;
        if let Ok((i, p)) = eat_player(i, players.clone()) {
            if let Ok(result) = eat_event_player(i, p) {
                return Ok(result);
            }
        }
        if let Ok((i, half)) = eat_half(i) {
            if let Ok(i) = ' '.drop(i) {
                if let Ok(i) = "liczba rzutów rożnych".drop(i) {
                    return Ok((i, CornersHalf(half)));
                }
                if let Ok(i) = "liczba goli".drop(i) {
                    return Ok((i, GoalsHalf(half)));
                }
                if let Ok(i) = "dokładna liczba goli".drop(i) {
                    return Ok((i, ExactGoalsHalf(half)));
                }
                if let Ok(i) = "handicap".drop(i) {
                    return Ok((i, HandicapHalf(half)));
                }
            }
            return Ok((i, Half(half)));
        }
        if let Ok(i) = "Dokładna liczba goli".drop(i) {
            return Ok((i, ExactGoals));
        }
        if let Ok(i) = "Obie drużyny ".drop(i) {
            let i = "strzelą ".drop(i)?;
            if let Ok(i) = "gola".drop(i) {
                if let Ok(i) = "w 1.połowie".drop(i) {
                    return Ok((i, BothToScoreHalf(H1)));
                }
                if let Ok(i) = "w 2.połowie".drop(i) {
                    return Ok((i, BothToScoreHalf(H2)));
                }
                return Ok((i, BothToScore));
            }
            if let Ok(i) = "po 2 lub więcej goli".drop(i) {
                return Ok((i, BothToScore2));
            }
        }
        if let Ok(i) = "Handicap".drop(i) {
            return Ok((i, Handicap));
        }
        if let Ok(i) = "Multigole".drop(i) {
            return Ok((i, MultiGoals));
        }
        if let Ok(i) = "Spotkanie bez remisu".drop(i) {
            return Ok((i, DrawNoBet));
        }
        if let Ok(i) = "Liczba ".drop(i) {
            if let Ok(i) = "goli".drop(i) {
                return Ok((i, Goals));
            }
            if let Ok(i) = "rzutów rożnych".drop(i) {
                return Ok((i, Corners));
            }
            if let Ok(i) = "spalonych".drop(i) {
                return Ok((i, Offsides));
            }
            if let Ok(i) = "strzałów w światło bramki".drop(i) {
                return Ok((i, ShotsOnTarget));
            }
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, YellowCards));
            }
        }
        if let Ok(i) = "Rzut karny".drop(i) {
            return Ok((i, Penalty));
        }
        if let Ok(i) = "Wynik meczu - dwójtyp".drop(i) {
            return Ok((i, DoubleChance));
        }
        if let Ok(i) = "Podwójna szansa (1.poł. lub mecz)".drop(i) {
            return Ok((i, DoubleChanceH1OrMatch));
        }
        if let Ok(i) = "Połowa z wiekszą liczbą goli".drop(i) {
            return Ok((i, HalfWithMoreGoals));
        }
        if let Ok(i) = "Otrzyma kartkę".drop(i) {
            return Ok((i, WillGetCard));
        }
        if let Ok(i) = "Dwójtyp".drop(i) {
            if let Ok(i) = "/liczba goli".drop(i) {
                return Ok((i, DoubleChanceGoalRange));
            }
            if let Ok(i) = "/obie drużyny strzelą gola".drop(i) {
                return Ok((i, DoubleChanceBothToScore));
            }
        }
        if let Ok(i) = "1.gol".drop(i) {
            if let Ok(i) = "/spotkanie".drop(i) {
                return Ok((i, FirstGoalMatch));
            }
            if let Ok(i) = "-minuta".drop(i) {
                return Ok((i, FirstGoalMinute));
            }
            return Ok((i, FirstGoal));
        }
        if let Ok(i) = "Pozostałe zakłady łączone".drop(i) {
            return Ok((i, RestProduct));
        }
        if let Ok(i) = "Padnie gol-minuta".drop(i) {
            return Ok((i, GoalBeforeMinute));
        }
        if let Ok(i) = "Różnica zwycięstwa".drop(i) {
            return Ok((i, WinDiff));
        }
        if let Ok(i) = "1.rzut rożny w spotkaniu".drop(i) {
            return Ok((i, FirstCorner));
        }
        if let Ok(i) = "Zawodnicy - strzały".drop(i) {
            return Ok((i, PlayerShot));
        }
        if let Ok(i) = eat_more(i) {
            let i = ' '.drop(i)?;
            if let Ok(i) = "rzutów rożnych".drop(i) {
                return Ok((i, MoreCorners));
            }
            if let Ok(i) = "strzałów w światło bramki".drop(i) {
                return Ok((i, MoreCorners));
            }
            if let Ok(i) = "fauli".drop(i) {
                return Ok((i, MoreFouls));
            }
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, MoreYellowCards));
            }
        }
        if let Ok(i) = "1-30 minuta spotkania".drop(i) {
            return Ok((i, Minute30));
        }
        if let Ok(i) = "1-60 minuta spotkania".drop(i) {
            return Ok((i, Minute60));
        }
        if let Ok(i) = "1-75 minuta spotkania".drop(i) {
            return Ok((i, Minute75));
        }
        if let Ok(i) = "Będzie wynik w trakcie spotkania".drop(i) {
            return Ok((i, ResultDuringMatch));
        }
        if let Ok(i) = "Spotkanie/".drop(i) {
            if let Ok((i, p)) = eat_player(i, players.clone()) {
                return Ok((i, MatchGoalsPlayer(p)));
            }
            if let Ok((i, half)) = eat_half(i) {
                if let Ok(i) = " liczba goli".drop(i) {
                    return Ok((i, MatchGoalsHalf(half)));
                }
            }
        }
        if let Ok(i) = "Przedział goli".drop(i) {
            return Ok((i, GoalRange));
        }
        if let Ok(i) = "Zawodnik rezerwowy strzeli gola (nie wystąpi od początku spotkania)".drop(i) {
            return Ok((i, SubstituteWillScore));
        }
        if let Ok(i) = "Będzie przegrywać, ale..".drop(i) {
            return Ok((i, WillBeLosingBut));
        }
        if let Ok(i) = "Mecz".drop(i) {
            if let Ok(i) = " + strzelcy goli".drop(i) {
                return Ok((i, MatchScorePlayers));
            }
            if let Ok(i) = " + strzały na bramkę".drop(i) {
                return Ok((i, MatchShotsOnTarget));
            }
            if let Ok(i) = ": liczba rzutów rożnych".drop(i) {
                return Ok((i, Corners));
            }
            if let Ok(i) = ": która drużyna strzeli gola".drop(i) {
                return Ok((i, PlayerToScore));
            }
            if let Ok(i) = ": więcej rzutów rożnych".drop(i) {
                return Ok((i, MatchMoreCorners));
            }
            if let Ok(i) = ": Przedział rzutów rożnych".drop(i) {
                return Ok((i, MatchCornerRange));
            }
            if let Ok(i) = ": Multiwynik".drop(i) {
                return Ok((i, MatchMultiScore));
            }
            if let Ok(i) = "/liczba goli".drop(i) {
                return Ok((i, MatchGoals));
            }
            if let Ok(i) = "/obie drużyny strzelą gola".drop(i) {
                return Ok((i, MatchBothToScore));
            }
        }
        Ok(("", Unknown(i.to_string())))
    }
}

fn eat_more(i: &str) -> Result<&str, ()> {
    if let Ok(i) = "Wiecej".drop(i) {
        return Ok(i);
    }
    if let Ok(i) = "Więcej".drop(i) {
        return Ok(i);
    }
    Err(())
}

fn eat_player(
    i: &str,
    [p1, p2]: [String; 2],
) -> Result<(&str, event::Player), ()> {
    use event::Player::*;
    if let Ok(i) = "1.druzyna".drop(i) {
        return Ok((i, P1));
    }
    if let Ok(i) = "1.drużyna".drop(i) {
        return Ok((i, P1));
    }
    if let Ok(i) = p1.as_str().drop(i) {
        return Ok((i, P1));
    }
    if let Ok(i) = "2.druzyna".drop(i) {
        return Ok((i, P2));
    }
    if let Ok(i) = "2.drużyna".drop(i) {
        return Ok((i, P2));
    }
    if let Ok(i) = p2.as_str().drop(i) {
        return Ok((i, P2));
    }
    Err(())
}

fn eat_event_player(
    i: &str,
    p: event::Player,
) -> Result<(&str, event::Football), ()> {
    use event::Football::*;
    let i = ' '.drop(i)?;
    if let Ok(i) = "przedział rzutów rożnych".drop(i) {
        return Ok((i, CornerRange(p)));
    }
    if let Ok(i) = "1.gol-minuta".drop(i) {
        return Ok((i, FirstGoalMinutePlayer(p)));
    }
    if let Ok(i) = "wygra do zera".drop(i) {
        return Ok((i, WinToNil(p)));
    }
    if let Ok(i) = "wygra obie połowy".drop(i) {
        return Ok((i, WinBothHalves(p)));
    }
    if let Ok(i) = "wygra przynajmniej jedna połowę".drop(i) {
        return Ok((i, WinAtLeastOneHalf(p)));
    }
    if let Ok(i) = "Multigole".drop(i) {
        return Ok((i, MultiGoalsPlayer(p)));
    }
    if let Ok(i) = "strzeli gola w obu połowach".drop(i) {
        return Ok((i, ScoreBothHalves(p)));
    }
    if let Ok(i) = "dokładna liczba goli".drop(i) {
        return Ok((i, ExactGoalsPlayer(p)));
    }
    if let Ok(i) = "liczba ".drop(i) {
        if let Ok(i) = "goli".drop(i) {
            return Ok((i, GoalsPlayer(p)));
        }
        if let Ok(i) = "rzutów rożnych".drop(i) {
            return Ok((i, CornersPlayer(p)));
        }
        if let Ok(i) = "strzałów w światło bramki".drop(i) {
            return Ok((i, ShotsOnTargetPlayer(p)));
        }
        if let Ok(i) = eat_yellow(i) {
            return Ok((i, YellowCardsPlayer(p)));
        }
    }
    if let Ok((i, half)) = eat_half(i) {
        let i = ' '.drop(i)?;
        if let Ok(i) = "liczba ".drop(i) {
            if let Ok(i) = "goli".drop(i) {
                return Ok((i, GoalsPlayerHalf(p, half)));
            }
            if let Ok(i) = "rzutów rożnych".drop(i) {
                return Ok((i, CornersPlayerHalf(p, half)));
            }
            if let Ok(i) = eat_yellow(i) {
                return Ok((i, YellowCardsHalf(p, half)));
            }
        }
    }
    Err(())
}

fn eat_half(i: &str) -> Result<(&str, event::Half), ()> {
    use event::Half::*;
    if let Ok(i) = "1.połowa".drop(i) {
        return Ok((i, H1));
    }
    if let Ok(i) = "2.połowa".drop(i) {
        return Ok((i, H2));
    }
    Err(())
}

fn eat_yellow(i: &str) -> Result<&str, ()> {
    let i = {
        if let Ok(i) = "zółtych".drop(i) {
            return Ok(i);
        }
        if let Ok(i) = "żółtych".drop(i) {
            return Ok(i);
        }
        Err(())
    }?;
    " kartek (bez żółtych kartek dla trenera i sztabu)".drop(i)
}
