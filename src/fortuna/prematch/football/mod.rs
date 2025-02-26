use crate::utils::{browser::download_html, download, page::Tag};

const URL: &str = "/zaklady-bukmacherskie/pilka-nozna";

pub struct Page;

impl download::Download<fantoccini::Client, ()> for Tag<Page, String> {
    type Error = fantoccini::error::CmdError;

    async fn download(
        client: &mut fantoccini::Client,
        _data: (),
    ) -> Result<Self, Self::Error> {
        let url = format!("{}{}", super::URL, URL);
        download_html(client, url.as_str(), super::super::COOKIE_ACCEPT)
            .await
            .map(Tag::new)
    }
}
