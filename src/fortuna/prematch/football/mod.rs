use crate::fortuna::COOKIE_ACCEPT;
use crate::utils::{browser, download::Download, page::Tag};

const URL: &str = "/zaklady-bukmacherskie/pilka-nozna";

pub struct Page;

impl Download<fantoccini::Client, ()> for Tag<Page, String> {
    type Error = fantoccini::error::CmdError;

    async fn download(
        client: &mut fantoccini::Client,
        _data: (),
    ) -> Result<Self, Self::Error> {
        let url = format!("{}{}", super::URL, URL);
        client.goto(url.as_str()).await?;
        browser::try_accepting_cookie(client, COOKIE_ACCEPT).await?;
        client.source().await.map(Tag::new)
    }
}
