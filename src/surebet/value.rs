use crate::utils::{download::Download, page::Tag};
use fantoccini::{error::CmdError, Client};

pub struct Page;

const URL: &str = "https://en.surebet.com/valuebets";

impl Download<Client, Page> for Tag<Page, String> {
    type Error = CmdError;

    async fn download(
        client: &mut Client,
        _data: Page,
    ) -> Result<Self, Self::Error> {
        client.goto(URL).await?;
        client.source().await.map(Tag::new)
    }
}

/*
#valuebets-table
.valuebet_record
data-start-at
data-value
data-overvalue
tr td.event span.minor = players
tr td.coeff abbr:data-bs-original-title = event
tr td.text-center a:href
*/
