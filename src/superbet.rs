use crate::bookmaker;

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "superbet";
}

impl bookmaker::Site for Book {
    const SITE: &'static str = "https://superbet.pl/zaklady-bukmacherskie/live";
}
