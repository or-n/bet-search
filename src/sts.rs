use crate::bookmaker;
use crate::utils::{self, browser::Browser};

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "sts";
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
                    "https://www.sts.pl/live",
                    fantoccini::Locator::Css(
                        r#"button[id="CybotCookiebotDialogBodyLevelButtonLevelOptinAllowAll"]"#,
                    ),
                )
                .await?,
            )),
            Err(connect_error) => Err(connect_error),
        })
    }
}

use bookmaker::Error;

impl bookmaker::SportBets for LivePage {
    fn sport_bets(
        &self,
    ) -> Result<Vec<(bookmaker::Teams, bookmaker::Odds)>, Error> {
        use scraper::Selector;
        let team = Selector::parse("div.match-tile-scoreboard-team__name span")
            .unwrap();
        utils::sport_bets::extract(
            &self.0,
            Selector::parse("div.match-tile-container").unwrap(),
            Selector::parse("span.odds-button__odd-value").unwrap(),
            |x| {
                let mut teams = x.select(&team);
                Ok([
                    teams.next().ok_or(Error::MissingTeam1)?,
                    teams.next().ok_or(Error::MissingTeam2)?,
                ])
            },
        )
    }
}
