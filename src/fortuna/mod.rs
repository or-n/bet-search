pub mod live;
pub mod team;

use crate::bookmaker;

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "efortuna";
}
