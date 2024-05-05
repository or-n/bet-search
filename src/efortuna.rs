use crate::bookmaker;
use crate::utils::{self, browser::Browser};

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "efortuna";
}

pub struct LivePage(String);

impl utils::download::Download for Browser<Book> {
    type Output = Result<LivePage, utils::browser::Error>;
    type Error = fantoccini::error::CmdError;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        Ok(match utils::browser::client(self.port).await {
            Ok(client) => Ok(LivePage(
                utils::download::download(
                    client,
                    "https://live.efortuna.pl/",
                    fantoccini::Locator::Css(
                        r#"button[id="cookie-consent-button-accept"]"#,
                    ),
                )
                .await?,
            )),
            Err(connect_error) => Err(connect_error),
        })
    }
}

impl bookmaker::SportBets for LivePage {
    fn sport_bets(
        &self,
    ) -> Result<Vec<(bookmaker::Teams, bookmaker::Odds)>, ()> {
        use scraper::{Html, Selector};
        let document = Html::parse_document(&self.0);
        let match_selector = Selector::parse("div.live-match").unwrap();
        let odds_selector = Selector::parse("span.odds_button__value").unwrap();
        let team1_selector =
            Selector::parse("div.live-match-info__team--1").unwrap();
        let team2_selector =
            Selector::parse("div.live-match-info__team--2").unwrap();
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
                let team1 = x.select(&team1_selector).next().unwrap();
                let team2 = x.select(&team2_selector).next().unwrap();
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
