use crate::shared::{book, sport_bets};
use crate::utils;

const URL: &str = "https://www.sts.pl/live";

pub struct Page(String);

impl utils::download::Download for utils::browser::Browser<super::Book> {
    type Output = Result<Page, fantoccini::error::CmdError>;
    type Error = utils::browser::Error;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        let cookie_accept = fantoccini::Locator::Css(
            r#"button[id="CybotCookiebotDialogBodyLevelButtonLevelOptinAllowAll"]"#,
        );
        let browser = utils::browser::client(self.port).await?;
        let page = utils::download::download(browser, URL, cookie_accept).await;
        Ok(page.map(Page))
    }
}

impl ToString for Page {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

use book::Error;

impl book::SportBets for Page {
    fn sport_bets(&self) -> Result<Vec<(book::Teams, book::Odds)>, Error> {
        use scraper::Selector;
        let team = Selector::parse("div.match-tile-scoreboard-team__name span")
            .unwrap();
        sport_bets::extract(
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
