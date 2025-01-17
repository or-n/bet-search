use crate::shared::{book, sport_bets};
use crate::utils::{browser, download, page};

pub const URL: &str = "https://superbet.pl/zaklady-bukmacherskie/live";

pub struct Page(String);

impl download::Download for browser::Browser<Page> {
    type Output = Result<Page, fantoccini::error::CmdError>;
    type Error = browser::Error;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        let cookie_accept = fantoccini::Locator::Css(
            r#"button[id="onetrust-accept-btn-handler"]"#,
        );
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
        let team1 = Selector::parse("div.e2e-event-team1-name").unwrap();
        let team2 = Selector::parse("div.e2e-event-team2-name").unwrap();
        sport_bets::extract(
            &self.0,
            Selector::parse("div.event-card").unwrap(),
            Selector::parse("span.odd-button__odd-value-new").unwrap(),
            |x| {
                Ok([
                    x.select(&team1).next().ok_or(Error::MissingTeam1)?,
                    x.select(&team2).next().ok_or(Error::MissingTeam2)?,
                ])
            },
        )
    }
}
