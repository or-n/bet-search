pub mod team;

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

impl Into<String> for &LivePage {
    fn into(self) -> String {
        self.0.clone()
    }
}

use bookmaker::Error;

impl bookmaker::SportBets for LivePage {
    fn sport_bets(
        &self,
    ) -> Result<Vec<(bookmaker::Teams, bookmaker::Odds)>, Error> {
        use scraper::Selector;
        let team1 = Selector::parse("div.live-match-info__team--1").unwrap();
        let team2 = Selector::parse("div.live-match-info__team--2").unwrap();
        utils::sport_bets::extract(
            &self.0,
            Selector::parse("div.live-match").unwrap(),
            Selector::parse("span.odds_button__value").unwrap(),
            |x| {
                Ok([
                    x.select(&team1).next().ok_or(Error::MissingTeam1)?,
                    x.select(&team2).next().ok_or(Error::MissingTeam2)?,
                ])
            },
        )
    }
}
