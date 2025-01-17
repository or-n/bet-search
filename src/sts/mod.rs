pub mod live;

use crate::shared::book;

pub struct Book;

impl book::Name for Book {
    const NAME: &'static str = "sts";
}
