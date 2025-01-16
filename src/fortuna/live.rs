use crate::shared::{self, book};
use crate::utils::{self, browser::Browser};

pub struct Page(String);

impl utils::download::Download for Browser<super::Book> {
    type Output = Result<Page, utils::browser::Error>;
    type Error = fantoccini::error::CmdError;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        Ok(match utils::browser::client(self.port).await {
            Ok(client) => Ok(Page(
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

impl Into<String> for &Page {
    fn into(self) -> String {
        self.0.clone()
    }
}

use book::Error;

impl book::SportBets for Page {
    fn sport_bets(&self) -> Result<Vec<(book::Teams, book::Odds)>, Error> {
        use scraper::Selector;
        let team1 = Selector::parse("div.live-match-info__team--1").unwrap();
        let team2 = Selector::parse("div.live-match-info__team--2").unwrap();
        shared::sport_bets::extract(
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
