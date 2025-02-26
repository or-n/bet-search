use crate::shared::book::Subpages;
use crate::utils::{
    browser, download,
    page::{Name, Tag},
};

impl Subpages<Page> for Tag<super::Page, String> {
    fn subpages(&self) -> Vec<Page> {
        use scraper::Selector;
        let subpage = Selector::parse("div.live-match a[href]").unwrap();
        let document = scraper::Html::parse_document(&self.inner());
        document
            .select(&subpage)
            .filter_map(|element| element.value().attr("href"))
            .map(|href| Page(href.to_string()))
            .collect()
    }
}

#[derive(Debug)]
pub struct Page(String);

// impl download::Download<fantoccini::Client, String> for Tag<Page, String> {
//     type Error = fantoccini::error::CmdError;

//     async fn download(
//         client: &mut fantoccini::Client,
//         data: Page,
//     ) -> Result<Self, Self::Error> {
//         let url = format!("{}/{}", super::URL, data.0);
//         browser::download_html(
//             client,
//             url.as_str(),
//             super::super::COOKIE_ACCEPT,
//         )
//         .await
//         .map(Tag::new)
//     }
// }

// impl Name for Page {
//     const NAME: &'static str = "fortuna.live.subpage";
// }
