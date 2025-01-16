pub mod en;
pub mod es;
pub mod it;
pub mod pl;

pub use en::*;
pub use es::*;
pub use it::*;
pub use pl::*;

pub enum Team {
    EN(EN),
    ES(ES),
    PL(PL),
    IT(IT),
}

pub enum FR {
    Auxerre,
    PSG,
}

pub enum GR {
    Leverkusen,
    SanktPauli,
}
