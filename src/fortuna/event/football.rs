use crate::shared::db;
use eat::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Football {
    Win,
    WinH1,
    WinH2,
    NotWin,
    Goals,
    GoalsH1,
    GoalsH2,
    Handicap,
    Individual(Player, FootballPlayer),
    BTS,
    Penalty,
    Corners,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FootballPlayer {
    Goals,
    Corners,
}

#[derive(Debug, Clone)]
pub enum FootballOption {
    Draw,
    P(Player),
    L(Line),
    PL(Player, Line),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    P1,
    P2,
}

#[derive(Debug, Clone)]
pub enum Line {
    LT(f64),
    GT(f64),
}

impl Eat<&str, (), [String; 2]> for Football {
    fn eat(i: &str, players: [String; 2]) -> Result<(&str, Self), ()> {
        use Football::*;
        use Player::*;
        if let Ok(i) = "Wynik meczu".drop(i) {
            eat!(i, " - dwójtyp", NotWin);
            return Ok((i, Win));
        }
        if let Ok(i) = "Liczba goli".drop(i) {
            if i.is_empty() {
                return Ok((i, Goals));
            }
        }
        eat!(i, "Handicap", Handicap);
        if let Ok(i) = players[0].as_str().drop(i) {
            let (i, x) = individual(i)?;
            return Ok((i, Individual(P1, x)));
        }
        if let Ok(i) = players[1].as_str().drop(i) {
            let (i, x) = individual(i)?;
            return Ok((i, Individual(P2, x)));
        }
        eat!(i, "Obie drużyny strzelą gola", BTS);
        eat!(i, "Rzut karny", Penalty);
        if let Ok(i) = "1.połowa".drop(i) {
            eat!(i, " liczba goli", GoalsH1);
            return Ok((i, WinH1));
        }
        if let Ok(i) = "2.połowa".drop(i) {
            eat!(i, " liczba goli", GoalsH2);
            return Ok((i, WinH2));
        }
        eat!(
            i,
            "liczba rzutów rożnych (razem z ewentualną dogrywką)",
            Corners
        );
        Err(())
    }
}

fn individual(i: &str) -> Result<(&str, FootballPlayer), ()> {
    use FootballPlayer::*;
    let i = " ".drop(i)?;
    eat!(i, "liczba goli", Goals);
    eat!(i, "liczba rzutów rożnych", Corners);
    Err(())
}

impl Eat<&str, (), (Football, [String; 2])> for FootballOption {
    fn eat(
        i: &str,
        (event, players): (Football, [String; 2]),
    ) -> Result<(&str, Self), ()> {
        use Football::*;
        use FootballOption::*;
        use Line::*;
        use Player::*;
        match event {
            Win => {
                if let Ok(i) = players[0].as_str().drop(i) {
                    return Ok((i, P(P1)));
                }
                if let Ok(i) = players[1].as_str().drop(i) {
                    return Ok((i, P(P2)));
                }
                eat!(i, "Remis", Draw);
                Err(())
            }
            NotWin => {
                eat!(i, "10", P(P2));
                eat!(i, "12", Draw);
                eat!(i, "02", P(P1));
                Err(())
            }
            Goals => {
                let lt = || {
                    eat!(i, "mniej niż ", true);
                    eat!(i, "więcej niż ", false);
                    Err(())
                };
                let (i, lt) = lt()?;
                let (i, x) = f64::eat(i, ())?;
                let line = if lt { LT(x) } else { GT(x) };
                Ok((i, L(line)))
            }
            Handicap => {
                let p = || {
                    eat!(i, "1 ", P1);
                    eat!(i, "2 ", P2);
                    Err(())
                };
                let (i, p) = p()?;
                let lt = || {
                    eat!(i, "-", true);
                    eat!(i, "+", false);
                    Err(())
                };
                let (i, lt) = lt()?;
                let (i, x) = f64::eat(i, ())?;
                let line = if lt { LT(x) } else { GT(x) };
                Ok((i, PL(p, line)))
            }
            Individual(_, _) => todo!(),
            BTS => todo!(),
            Penalty => todo!(),
            WinH1 => todo!(),
            WinH2 => todo!(),
            GoalsH1 => todo!(),
            GoalsH2 => todo!(),
            Corners => todo!(),
        }
    }
}

pub fn translate_db(x: Football, o: FootballOption) -> Result<db::Event, ()> {
    use Football::*;
    use FootballOption::*;
    use Line::*;
    use Player::*;
    match x {
        Win => {
            let (a, b) = match o {
                Draw => (Some(-0.5), Some(0.5)),
                P(P1) => (None, Some(-0.5)),
                P(P2) => (Some(0.5), None),
                _ => return Err(()),
            };
            Ok(db::Event {
                tag: db::Football::GoalD,
                a,
                b,
                ..db::Event::default()
            })
        }
        NotWin => {
            let (a, b) = match o {
                Draw => (Some(0.5), Some(-0.5)),
                P(P1) => (Some(-0.5), None),
                P(P2) => (None, Some(0.5)),
                _ => return Err(()),
            };
            Ok(db::Event {
                tag: db::Football::GoalD,
                a,
                b,
                ..db::Event::default()
            })
        }
        Goals => {
            let (a, b) = match o {
                L(LT(x)) => (None, Some(x)),
                L(GT(x)) => (Some(x), None),
                _ => return Err(()),
            };
            Ok(db::Event {
                tag: db::Football::Goals,
                a,
                b,
                ..db::Event::default()
            })
        }
        _ => todo!(),
    }
}
