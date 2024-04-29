use crate::bookmaker;
use scraper::{Html, Selector};

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "superbet";
}

impl bookmaker::Site for Book {
    const SITE: &'static str = "https://superbet.pl/zaklady-bukmacherskie/live";

    const COOKIE_ACCEPT_CSS: &'static str =
        r#"button[id="onetrust-accept-btn-handler"]"#;
}

impl bookmaker::GetOdds for Book {
    fn get_odds(
        site: &String,
    ) -> Result<Vec<(bookmaker::Teams, bookmaker::Odds)>, ()> {
        let document = Html::parse_document(site);
        let match_selector = Selector::parse("div.event-card").unwrap();
        let odds_selector =
            Selector::parse("span.odd-button__odd-value-new").unwrap();
        let team1_selector =
            Selector::parse("div.e2e-event-team1-name").unwrap();
        let team2_selector =
            Selector::parse("div.e2e-event-team2-name").unwrap();
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
