use crate::shared::db;
use eat::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum Football {
    Win,
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
        eat!(i, "Mecz", Win);
        Err(())
    }
}

impl Eat<&str, (), (Football, [String; 2])> for FootballOption {
    fn eat(
        i: &str,
        (event, _players): (Football, [String; 2]),
    ) -> Result<(&str, Self), ()> {
        use Football::*;
        use FootballOption::*;
        match event {
            Win => {
                eat!(i, "0", Draw);
                eat!(i, "1", Player1);
                eat!(i, "2", Player2);
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
    }
}
