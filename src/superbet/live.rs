use crate::shared::{book, sport_bets};
use crate::utils::{browser, download, page};

pub const URL: &str = "https://superbet.pl/zaklady-bukmacherskie/live";
pub const COOKIE_ACCEPT: &str = r#"button[id="onetrust-accept-btn-handler"]"#;

pub struct Page(String);

impl download::Download<Page, ()> for browser::Browser {
    type Error = browser::Error;

    async fn download(&self, _data: ()) -> Result<Page, Self::Error> {
        self.run(URL, COOKIE_ACCEPT).await.map(Page)
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
