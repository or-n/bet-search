use crate::fortuna::prematch::URL;
use crate::shared::book::Subpages;
use crate::utils::{
    download::Download,
    page::{Name, Tag, Url},
};
use scraper::{ElementRef, Html, Node, Selector};

impl Subpages<Page> for Tag<super::Page, Html> {
    fn subpages(&self) -> Vec<Page> {
        let subpage = Selector::parse("a.event-link").unwrap();
        self.inner()
            .select(&subpage)
            .filter_map(|element| element.value().attr("href"))
            .map(|href| Page(href.to_string()))
            .collect()
    }
}

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub odds: Vec<(String, f32)>,
}

fn clean_text(texts: scraper::element_ref::Text) -> String {
    texts
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main_text(element: ElementRef) -> String {
    element
        .first_child()
        .and_then(|node| {
            if let Node::Text(text) = node.value() {
                Some(text.trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_default()
}

impl Tag<Page, Html> {
    pub fn events(&self) -> Vec<Event> {
        let market = Selector::parse("div.market").unwrap();
        let name = Selector::parse("h3 > a").unwrap();
        let odds = Selector::parse("div.odds a").unwrap();
        let odds_name = Selector::parse("span.odds-name").unwrap();
        let odds_value = Selector::parse("span.odds-value").unwrap();
        self.inner()
            .select(&market)
            .map(|element| {
                let name = element
                    .select(&name)
                    .next()
                    .map(|a| clean_text(a.text()))
                    .unwrap();
                let odds = element
                    .select(&odds)
                    .filter_map(|a| {
                        let name =
                            a.select(&odds_name).next().map(|n| main_text(n));
                        let value = a
                            .select(&odds_value)
                            .next()
                            .map(|v| clean_text(v.text()))
                            .and_then(|v| v.parse::<f32>().ok());
                        match (name, value) {
                            (Some(name), Some(value)) => Some((name, value)),
                            _ => None,
                        }
                    })
                    .collect();
                Event { name, odds }
            })
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
        client.goto(data.url().as_str()).await?;
        client.source().await.map(Tag::new)
    }
}

impl Name for Page {
    fn name(&self) -> String {
        format!("fortuna{}", self.0.replace("/", "."))
    }
}

impl Url for Page {
    fn url(&self) -> String {
        format!("{}{}", URL, self.0)
    }
}
