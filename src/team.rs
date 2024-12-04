pub mod en;
pub mod pl;

pub use en::*;
pub use pl::*;

pub enum Team {
    EN(EN),
    ES(ES),
    PL(PL),
    IT(IT),
}

pub enum ES {
    RealMadrid,
    Bilbao,
}

pub enum IT {
    Fiorentina,
    Empoli,
}
