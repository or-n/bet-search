use crate::bookmaker;

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "efortuna";
}

impl bookmaker::Site for Book {
    const SITE: &'static str = "https://live.efortuna.pl/";
}
