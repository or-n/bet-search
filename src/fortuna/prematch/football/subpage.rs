use crate::fortuna::prematch::URL;
use crate::shared::book::Subpages;
use crate::utils::{
    download::Download,
    page::{Name, Tag},
};

impl Subpages<Page> for Tag<super::Page, String> {
    fn subpages(&self) -> Vec<Page> {
        use scraper::Selector;
        let subpage = Selector::parse("a.event-link").unwrap();
        let document = scraper::Html::parse_document(&self.inner());
        document
            .select(&subpage)
            .filter_map(|element| element.value().attr("href"))
            .map(|href| Page(href.to_string()))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Page(String);

impl Download<fantoccini::Client, Page> for Tag<Page, String> {
    type Error = fantoccini::error::CmdError;

    async fn download(
        client: &mut fantoccini::Client,
        data: Page,
    ) -> Result<Self, Self::Error> {
        let url = format!("{}{}", URL, data.0);
        client.goto(url.as_str()).await?;
        client.source().await.map(Tag::new)
    }
}

impl Name for Page {
    fn name(&self) -> String {
        format!("fortuna{}", self.0.replace("/", "."))
    }
}
