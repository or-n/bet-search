pub mod live;

use crate::shared::{self, book};
use crate::utils::{self, browser::Browser};

pub struct Book;

impl book::Name for Book {
    const NAME: &'static str = "superbet";
}
