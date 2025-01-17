pub mod sport_bets;
pub mod subpages;
pub mod tenis;

use crate::utils::{browser, download, page};

pub struct Page(String);

const URL: &str = "https://live.efortuna.pl/";
const COOKIE_ACCEPT: &str = r#"button[id="cookie-consent-button-accept"]"#;

impl download::Download for browser::Browser<Page> {
    type Output = Page;
    type Error = browser::Error;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        self.run(URL, COOKIE_ACCEPT).await.map(Page)
    }
}

impl ToString for Page {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl page::Name for Page {
    const NAME: &'static str = "fortuna.live";
}
