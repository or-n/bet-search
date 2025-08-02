use crate::shared::db;
use eat::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Football {
    Win,
    NotWin,
}

#[derive(Debug, Clone)]
pub enum FootballOption {
    Draw,
    Player1,
    Player2,
}

impl Eat<&str, (), [String; 2]> for Football {
    fn eat(i: &str, _players: [String; 2]) -> Result<(&str, Self), ()> {
        use Football::*;
        if let Ok(i) = "Wynik meczu".drop(i) {
            eat!(i, " - dwójtyp", NotWin);
            return Ok((i, Win));
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
        }
    }
}

pub fn translate_db(x: Football, o: FootballOption) -> Result<db::Event, ()> {
    use Football::*;
    use FootballOption::*;
    match x {
        Win => Ok(db::Event {
            tag: db::Football::GoalD,
            a: match o {
                Draw => Some(-0.5),
                Player1 => None,
                Player2 => Some(0.5),
            },
            b: match o {
                Draw => Some(0.5),
                Player1 => Some(-0.5),
                Player2 => None,
            },
            ..db::Event::default()
        }),
        NotWin => Ok(db::Event {
            tag: db::Football::GoalD,
            a: match o {
                Draw => Some(0.5),
                Player1 => Some(-0.5),
                Player2 => None,
            },
            b: match o {
                Draw => Some(-0.5),
                Player1 => None,
                Player2 => Some(0.5),
            },
            ..db::Event::default()
        }),
    }
}
