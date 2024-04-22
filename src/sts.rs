use crate::bookmaker;

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "sts";
}

impl bookmaker::Site for Book {
    const SITE: &'static str = "https://www.sts.pl/live";
}
