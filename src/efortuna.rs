use crate::bookmaker;

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "efortuna";
}

impl bookmaker::Site for Book {
    const SITE: &'static str = "https://live.efortuna.pl/";

    const COOKIE_ACCEPT_CSS: &'static str =
        r#"button[id="cookie-consent-button-accept"]"#;
}
