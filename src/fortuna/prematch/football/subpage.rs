use crate::fortuna::prematch::URL;
use crate::shared::{book::Subpages, event::Event};
use crate::utils::{
    date,
    download::Download,
    page::{Name, Tag, Url},
    scrape::{clean_text, main_text, split2},
};
use chrono::NaiveDateTime;
use scraper::{Html, Selector};

impl Subpages<(Page, NaiveDateTime)> for Tag<super::Page, Html> {
    fn subpages(&self) -> Vec<(Page, NaiveDateTime)> {
        let event = Selector::parse("table.events-table tr").unwrap();
        let subpage = Selector::parse("a.event-link").unwrap();
        let date = Selector::parse("span.event-datetime").unwrap();
        self.inner()
            .select(&event)
            .filter_map(|element| {
                let page: Page = element
                    .select(&subpage)
                    .filter_map(|element| element.value().attr("href"))
                    .next()
                    .map(|href| Page(href.to_string()))?;
                let datetime = element
                    .select(&date)
                    .next()
                    .map(|a| clean_text(a.text()))?;
                let d = date::eat(&datetime).unwrap();
                Some((page, d))
            })
            .collect()
    }
}

impl Tag<Page, Html> {
    pub fn players(&self) -> Option<[String; 2]> {
        let event_name = Selector::parse("span.event-name").unwrap();
        let name = self
            .inner()
            .select(&event_name)
            .next()
            .map(|x| clean_text(x.text()))
            .unwrap();
        split2(name, " - ")
    }

    pub fn events(&self) -> Vec<Event<String>> {
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
                Event { id: name, odds }
            })
            .collect()
    }

    pub fn _date(&self) -> String {
        let date = Selector::parse("span.event-datetime").unwrap();
        self.inner()
            .select(&date)
            .next()
            .map(|a| clean_text(a.text()))
            .unwrap()
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
