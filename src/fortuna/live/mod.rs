pub mod sport_bets;
pub mod subpages;
pub mod tenis;

use crate::utils;

pub struct Page(String);

const URL: &str = "https://live.efortuna.pl/";

impl utils::download::Download for utils::browser::Browser<super::Book> {
    type Output = Result<Page, fantoccini::error::CmdError>;
    type Error = utils::browser::Error;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        let cookie_accept = fantoccini::Locator::Css(
            r#"button[id="cookie-consent-button-accept"]"#,
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
