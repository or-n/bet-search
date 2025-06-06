pub mod tennis;

// use crate::shared::book;
use crate::utils::{browser, download::Download, page};
use fantoccini::{error::CmdError, Client};
use page::Tag;

const URL: &str = "https://www.sts.pl/live";
const COOKIE_ACCEPT: &str =
    r#"button[id="CybotCookiebotDialogBodyLevelButtonLevelOptinAllowAll"]"#;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Page(String);

impl Download<Client, Page> for Tag<Page, String> {
    type Error = CmdError;

    async fn download(
        client: &mut Client,
        _data: Page,
    ) -> Result<Self, Self::Error> {
        client.goto(URL).await?;
        browser::try_accepting_cookie(client, COOKIE_ACCEPT).await?;
        client.source().await.map(Tag::new)
    }
}

impl page::Name for Page {
    fn name(&self) -> String {
        "sts.live".to_string()
    }
}

// use book::Error;

// impl book::SportBets for Page {
//     fn sport_bets(&self) -> Result<Vec<(book::Teams, book::Odds)>, Error> {
//         use scraper::Selector;
//         let team = Selector::parse("div.match-tile-scoreboard-team__name span")
//             .unwrap();
//         sport_bets::extract(
//             &self.0,
//             Selector::parse("div.match-tile-container").unwrap(),
//             Selector::parse("span.odds-button__odd-value").unwrap(),
//             |x| {
//                 let mut teams = x.select(&team);
//                 Ok([
//                     teams.next().ok_or(Error::MissingTeam1)?,
//                     teams.next().ok_or(Error::MissingTeam2)?,
//                 ])
//             },
//         )
//     }
// }
