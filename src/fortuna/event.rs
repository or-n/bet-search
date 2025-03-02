use crate::shared::event;
use eat::*;

impl Eat<&str, (), [String; 2]> for event::Football {
    fn eat(i: &str, [p1, p2]: [String; 2]) -> Result<(&str, Self), ()> {
        use event::Football::*;
        if let Ok(i) = p1.as_str().drop(i) {
            if let Ok(result) = eat_p1(i) {
                return Ok(result);
            }
        }
        if let Ok(i) = "1.druzyna".drop(i) {
            if let Ok(result) = eat_p1(i) {
                return Ok(result);
            }
        }
        if let Ok(i) = "1.drużyna".drop(i) {
            if let Ok(result) = eat_p1(i) {
                return Ok(result);
            }
        }
        if let Ok(i) = p2.as_str().drop(i) {
            if let Ok(result) = eat_p2(i) {
                return Ok(result);
            }
        }
        if let Ok(i) = "2.druzyna".drop(i) {
            if let Ok(result) = eat_p2(i) {
                return Ok(result);
            }
        }
        if let Ok(i) = "2.drużyna".drop(i) {
            if let Ok(result) = eat_p2(i) {
                return Ok(result);
            }
        }
        if let Ok(i) = "Liczba rzutów rożnych".drop(i) {
            return Ok((i, Corners));
        }
        if let Ok(i) = "1.połowa liczba rzutów rożnych".drop(i) {
            return Ok((i, CornersH1));
        }
        if let Ok(i) = "2.połowa liczba rzutów rożnych".drop(i) {
            return Ok((i, CornersH2));
        }
        if let Ok(i) = "Liczba goli".drop(i) {
            return Ok((i, Goals));
        }
        if let Ok(i) = "1.połowa liczba goli".drop(i) {
            return Ok((i, GoalsH1));
        }
        if let Ok(i) = "2.połowa liczba goli".drop(i) {
            return Ok((i, GoalsH2));
        }
        if let Ok(i) = "Dokładna liczba goli".drop(i) {
            return Ok((i, ExactGoals));
        }
        if let Ok(i) = "1.połowa: dokładna liczba goli".drop(i) {
            return Ok((i, ExactGoalsH1));
        }
        if let Ok(i) = "2.połowa: dokładna liczba goli".drop(i) {
            return Ok((i, ExactGoalsH2));
        }
        if let Ok(i) = "Obie drużyny strzelą gola".drop(i) {
            return Ok((i, BothToScore));
        }
        if let Ok(i) = "Obie drużyny strzelą gola w 1.połowie".drop(i) {
            return Ok((i, BothToScoreH1));
        }
        if let Ok(i) = "Obie drużyny strzelą gola w 2.połowie".drop(i) {
            return Ok((i, BothToScoreH2));
        }
        if let Ok(i) = "Handicap".drop(i) {
            return Ok((i, Handicap));
        }
        if let Ok(i) = "1.połowa: handicap".drop(i) {
            return Ok((i, HandicapH1));
        }
        if let Ok(i) = "2.połowa: handicap".drop(i) {
            return Ok((i, HandicapH2));
        }
        if let Ok(i) = "1.połowa".drop(i) {
            return Ok((i, H1));
        }
        if let Ok(i) = "2.połowa".drop(i) {
            return Ok((i, H2));
        }
        Ok(("", Unknown(i.to_string())))
    }
}

fn eat_p1(i: &str) -> Result<(&str, event::Football), ()> {
    use event::Football::*;
    if let Ok(i) = " liczba goli".drop(i) {
        return Ok((i, GoalsP1));
    }
    if let Ok(i) = " 1.połowa liczba goli".drop(i) {
        return Ok((i, GoalsP1H1));
    }
    if let Ok(i) = " 2.połowa liczba goli".drop(i) {
        return Ok((i, GoalsP1H2));
    }
    if let Ok(i) = " liczba rzutów rożnych".drop(i) {
        return Ok((i, CornersP1));
    }
    if let Ok(i) = " 1.połowa liczba rzutów rożnych".drop(i) {
        return Ok((i, CornersP1H1));
    }
    if let Ok(i) = " 2.połowa liczba rzutów rożnych".drop(i) {
        return Ok((i, CornersP1H2));
    }
    Err(())
}

fn eat_p2(i: &str) -> Result<(&str, event::Football), ()> {
    use event::Football::*;
    if let Ok(i) = " liczba goli".drop(i) {
        return Ok((i, GoalsP2));
    }
    if let Ok(i) = " 1.połowa liczba goli".drop(i) {
        return Ok((i, GoalsP2H1));
    }
    if let Ok(i) = " 2.połowa liczba goli".drop(i) {
        return Ok((i, GoalsP2H2));
    }
    if let Ok(i) = " liczba rzutów rożnych".drop(i) {
        return Ok((i, CornersP2));
    }
    if let Ok(i) = " 1.połowa liczba rzutów rożnych".drop(i) {
        return Ok((i, CornersP2H1));
    }
    if let Ok(i) = " 2.połowa liczba rzutów rożnych".drop(i) {
        return Ok((i, CornersP2H2));
    }
    Err(())
}
