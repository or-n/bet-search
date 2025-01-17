pub mod sport_bets;
pub mod subpages;
pub mod tenis;

use crate::utils::{browser, download};

pub struct Page(String);

const URL: &str = "https://live.efortuna.pl/";
const COOKIE_ACCEPT: &str = r#"button[id="cookie-consent-button-accept"]"#;

impl download::Download for browser::Browser<super::Book> {
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
