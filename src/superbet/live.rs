use crate::shared::{book, sport_bets};
use crate::utils;

pub const URL: &str = "https://superbet.pl/zaklady-bukmacherskie/live";

pub struct Page(String);

impl utils::download::Download for utils::browser::Browser<super::Book> {
    type Output = Result<Page, fantoccini::error::CmdError>;
    type Error = utils::browser::Error;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        let cookie_accept = fantoccini::Locator::Css(
            r#"button[id="onetrust-accept-btn-handler"]"#,
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
