pub mod tennis;

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
