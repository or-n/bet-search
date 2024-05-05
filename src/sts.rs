use crate::bookmaker;
use scraper::{Html, Selector};

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "sts";
}

impl bookmaker::Site for Book {
    const SITE: &'static str = "https://www.sts.pl/live";

    const COOKIE_ACCEPT_CSS: &'static str =
        r#"button[id="CybotCookiebotDialogBodyLevelButtonLevelOptinAllowAll"]"#;
}

impl bookmaker::GetOdds for Book {
    fn get_odds(
        site: &String,
    ) -> Result<Vec<(bookmaker::Teams, bookmaker::Odds)>, ()> {
        let document = Html::parse_document(site);
        let match_selector =
            Selector::parse("div.match-tile-container").unwrap();
        let odds_selector =
            Selector::parse("span.odds-button__odd-value").unwrap();
        let team_selector =
            Selector::parse("div.match-tile-scoreboard-team__name span")
                .unwrap();
        let team = |x: scraper::ElementRef| x.inner_html().trim().to_string();
        let ratio_result = |x: scraper::ElementRef| {
            x.inner_html()
                .trim()
                .parse::<f32>()
                .map_err(|_| x.inner_html())
        };
        let matches: Vec<_> = document
            .select(&match_selector)
            .map(|x| {
                let mut teams = x.select(&team_selector);
                let team1 = teams.next().unwrap();
                let team2 = teams.next().unwrap();
                let teams = bookmaker::Teams {
                    team1: team(team1),
                    team2: team(team2),
                };
                let odds = x.select(&odds_selector).map(ratio_result).collect();
                (teams, odds)
            })
            .collect();
        Ok(matches)
    }
}
