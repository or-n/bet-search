pub mod subpage;

use crate::fortuna::COOKIE_ACCEPT;
use crate::shared::event::Match;
use crate::utils::{
    browser, date,
    download::Download,
    page::{Name, Tag},
    scrape::clean_text,
};
use chrono::NaiveDateTime;
use eat::*;
use fantoccini::{error::CmdError, Client, Locator};
use scraper::{Html, Selector};
use tokio::time::{sleep, Duration};

const URL: &str = "/zaklady-bukmacherskie/pilka-nozna";

#[derive(Debug, Clone)]
pub enum Url {
    Root(Page),
    Subpage(subpage::Page),
}

impl Eat<&str, (), ()> for Url {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        let i = URL.drop(i)?;
        if let Ok((i, subpage)) = subpage::Page::eat(i, ()) {
            return Ok((i, Url::Subpage(subpage)));
        }
        Ok((i, Url::Root(Page)))
    }
}

impl Name for Url {
    fn name(&self) -> String {
        match self {
            Url::Root(_) => "".to_string(),
            Url::Subpage(subpage) => subpage.name(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Page;

impl Download<Client, Page> for Tag<Page, String> {
    type Error = CmdError;

    async fn download(
        client: &mut Client,
        _data: Page,
    ) -> Result<Self, Self::Error> {
        let url = format!("{}{}", super::URL, URL);
        client.goto(url.as_str()).await?;
        browser::try_accepting_cookie(client, COOKIE_ACCEPT).await?;
        let mut previous_count = 0;
        let mut new_count;
        let scroll = "window.scrollTo(0, document.body.scrollHeight);";
        loop {
            let elements = client.find_all(Locator::Css(".event-link")).await?;
            new_count = elements.len();
            if new_count == previous_count {
                println!("nothing new");
                break;
            }
            println!("found: {}", new_count);
            previous_count = new_count;
            client.execute(scroll, vec![]).await?;
            sleep(Duration::from_secs(2)).await;
        }
        client.source().await.map(Tag::new)
    }
}

impl Tag<Page, Html> {
    pub fn matches(&self) -> Vec<Match> {
        let event = Selector::parse("table.events-table tr").unwrap();
        let title = Selector::parse("td.col-title").unwrap();
        let subpage = Selector::parse("a.event-link").unwrap();
        let date = Selector::parse("span.event-datetime").unwrap();
        self.inner()
            .select(&event)
            .filter_map(|element| {
                let title = element.select(&title).next()?;
                let url = title
                    .select(&subpage)
                    .filter_map(|element| element.value().attr("href"))
                    .next()
                    .map(|href| href.to_string())?;
                let players = title.value().attr("data-value").unwrap();
                let (before, after) = players.split_once(" - ")?;
                let before = before.trim().to_string();
                let after = after.trim().to_string();
                let players = [before, after];
                let datetime = element
                    .select(&date)
                    .next()
                    .map(|a| clean_text(a.text()))?;
                let date = date::eat_assume_year(&datetime).unwrap();
                let m = Match { players, date, url };
                Some(m)
            })
            .collect()
    }
}

impl Name for Page {
    fn name(&self) -> String {
        "fortuna.football".to_string()
    }
}
