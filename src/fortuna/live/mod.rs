// pub mod sport_bets;
pub mod subpages;
pub mod tenis;

use crate::utils::{browser, download, page, page::Tag};

const URL: &str = "https://live.efortuna.pl";

#[derive(Debug)]
pub struct Page;

impl download::Download<fantoccini::Client, String> for Tag<Page, String> {
    type Error = fantoccini::error::CmdError;

    async fn download(
        client: &mut fantoccini::Client,
        data: String,
    ) -> Result<Self, Self::Error> {
        let url = format!("{URL}/{data}");
        browser::download_html(client, url.as_str(), super::COOKIE_ACCEPT)
            .await
            .map(Tag::new)
    }
}

impl page::Name for Page {
    const NAME: &'static str = "fortuna.live";
}
