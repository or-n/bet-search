use crate::shared::db;
use eat::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Football {
    Win,
    NotWin,
    Goals,
}

#[derive(Debug, Clone)]
pub enum FootballOption {
    Draw,
    Player1,
    Player2,
    LT(f64),
    GT(f64),
}

impl Eat<&str, (), [String; 2]> for Football {
    fn eat(i: &str, _players: [String; 2]) -> Result<(&str, Self), ()> {
        use Football::*;
        if let Ok(i) = "Wynik meczu".drop(i) {
            eat!(i, " - dwójtyp", NotWin);
            return Ok((i, Win));
        }
        if let Ok(i) = "Liczba goli".drop(i) {
            if i.is_empty() {
                return Ok((i, Goals));
            }
        }
        Err(())
    }
}

impl Eat<&str, (), (Football, [String; 2])> for FootballOption {
    fn eat(
        i: &str,
        (event, players): (Football, [String; 2]),
    ) -> Result<(&str, Self), ()> {
        use Football::*;
        use FootballOption::*;
        match event {
            Win => {
                if let Ok(i) = players[0].as_str().drop(i) {
                    return Ok((i, Player1));
                }
                if let Ok(i) = players[1].as_str().drop(i) {
                    return Ok((i, Player2));
                }
                eat!(i, "Remis", Draw);
                Err(())
            }
            NotWin => {
                eat!(i, "10", Player2);
                eat!(i, "12", Draw);
                eat!(i, "02", Player1);
                Err(())
            }
            Goals => {
                println!("parsing goals option: {:?}", i);
                let lt = || {
                    eat!(i, "mniej niż ", true);
                    eat!(i, "więcej niż ", false);
                    Err(())
                };
                let (i, lt) = lt()?;
                // let i = '\u{a0}'.drop(i)?;
                let (i, x) = f64::eat(i, ())?;
                Ok((i, if lt { LT(x) } else { GT(x) }))
            }
        }
    }
}

pub fn translate_db(x: Football, o: FootballOption) -> Result<db::Event, ()> {
    use Football::*;
    use FootballOption::*;
    match x {
        Win => {
            let (a, b) = match o {
                Draw => (Some(-0.5), Some(0.5)),
                Player1 => (None, Some(-0.5)),
                Player2 => (Some(0.5), None),
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
                Player1 => (Some(-0.5), None),
                Player2 => (None, Some(0.5)),
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
                LT(x) => (None, Some(x)),
                GT(x) => (Some(x), None),
                _ => return Err(()),
            };
            Ok(db::Event {
                tag: db::Football::Goals,
                a,
                b,
                ..db::Event::default()
            })
        }
    }
}
