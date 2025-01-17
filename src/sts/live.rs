use crate::shared::{book, sport_bets};
use crate::utils::{browser, download, page};

const URL: &str = "https://www.sts.pl/live";
const COOKIE_ACCEPT: &str =
    r#"button[id="CybotCookiebotDialogBodyLevelButtonLevelOptinAllowAll"]"#;

pub struct Page(String);

impl download::Download for browser::Browser<Page> {
    type Output = Result<Page, fantoccini::error::CmdError>;
    type Error = browser::Error;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        let cookie_accept = fantoccini::Locator::Css(COOKIE_ACCEPT);
        let browser = browser::client(self.port).await?;
        let page = download::run(browser, URL, cookie_accept).await;
        Ok(page.map(Page))
    }
}

impl ToString for Page {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl page::Name for Page {
    const NAME: &'static str = "sts.live";
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
