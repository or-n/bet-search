// pub mod sport_bets;
pub mod subpages;
pub mod tenis;

use crate::utils::{browser, download::Download, page, page::Tag};

const URL: &str = "https://live.efortuna.pl";

#[derive(Debug, Clone, Copy)]
pub struct Page;

impl Download<fantoccini::Client, Page> for Tag<Page, String> {
    type Error = fantoccini::error::CmdError;

    async fn download(
        client: &fantoccini::Client,
        _data: Page,
    ) -> Result<Self, Self::Error> {
        client.goto(URL).await?;
        browser::try_accepting_cookie(client, super::COOKIE_ACCEPT).await?;
        client.source().await.map(Tag::new)
    }
}

impl page::Name for Page {
    fn name(&self) -> String {
        "fortuna.live".to_string()
    }
}
