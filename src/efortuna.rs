use crate::bookmaker;
use scraper::{Html, Selector};

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "efortuna";
}

impl bookmaker::Site for Book {
    const SITE: &'static str = "https://live.efortuna.pl/";

    const COOKIE_ACCEPT_CSS: &'static str =
        r#"button[id="cookie-consent-button-accept"]"#;
}

impl bookmaker::GetOdds for Book {
    fn get_odds(
        site: &String,
    ) -> Result<Vec<(bookmaker::Teams, bookmaker::Odds)>, ()> {
        let document = Html::parse_document(site);
        let match_selector = Selector::parse("div.live-match").unwrap();
        let odds_selector = Selector::parse("span.odds_button__value").unwrap();
        let team1_selector =
            Selector::parse("div.live-match-info__team--1").unwrap();
        let team2_selector =
            Selector::parse("div.live-match-info__team--2").unwrap();
        let team = |x: scraper::ElementRef| x.inner_html().trim().to_string();
        let matches: Vec<_> = document
            .select(&match_selector)
            .map(|x| {
                let team1 = x.select(&team1_selector).next().unwrap();
                let team2 = x.select(&team2_selector).next().unwrap();
                let teams = bookmaker::Teams {
                    team1: team(team1),
                    team2: team(team2),
                };
                let odds = x
                    .select(&odds_selector)
                    .map(|x| x.inner_html().trim().parse::<f32>().unwrap())
                    .collect();
                (teams, odds)
            })
            .collect();
        Ok(matches)
    }
}
