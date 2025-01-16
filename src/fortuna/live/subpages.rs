use crate::shared::book::Subpages;

impl Subpages for super::Page {
    fn subpages(&self) -> Vec<String> {
        run(self)
    }
}

pub fn run(page: &super::Page) -> Vec<String> {
    use scraper::Selector;
    let subpage = Selector::parse("div.live-match a[href]").unwrap();
    let document = scraper::Html::parse_document(&page.0);
    document
        .select(&subpage)
        .filter_map(|element| element.value().attr("href"))
        .map(|href| href.to_string())
        .collect()
}
