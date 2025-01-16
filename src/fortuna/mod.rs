pub mod live;
pub mod team;

use crate::shared::book;

pub struct Book;

impl book::Name for Book {
    const NAME: &'static str = "efortuna";
}
