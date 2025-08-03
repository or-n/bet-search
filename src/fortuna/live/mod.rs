// pub mod sport_bets;
pub mod subpages;
pub mod tenis;

use crate::adapter::browser;
use crate::utils::{download::Download, page, page::Tag};
use fantoccini::{error::CmdError, Client};

const URL: &str = "https://live.efortuna.pl";

#[derive(Debug, Clone, Copy)]
pub struct Page;

impl Download<Client, Page> for Tag<Page, String> {
    type Error = CmdError;

    async fn download(
        client: &Client,
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
