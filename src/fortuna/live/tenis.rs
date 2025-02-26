use crate::fortuna::{live::URL, COOKIE_ACCEPT};
use crate::utils::{browser, download, page::Tag};

pub struct Page;

impl download::Download<fantoccini::Client, ()> for Tag<Page, String> {
    type Error = fantoccini::error::CmdError;

    async fn download(
        client: &mut fantoccini::Client,
        _data: (),
    ) -> Result<Self, Self::Error> {
        let url = format!("{}/sports/LPLTENNIS", URL);
        client.goto(url.as_str()).await?;
        browser::try_accepting_cookie(client, COOKIE_ACCEPT).await?;
        client.source().await.map(Tag::new)
    }
}
