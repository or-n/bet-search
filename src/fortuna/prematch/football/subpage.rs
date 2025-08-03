use crate::adapter::scrape::clean_text;
use crate::fortuna;
use crate::fortuna::prematch::URL;
use crate::shared::event::Event;
use crate::utils::{
    download::Download,
    page::{Name, Tag, Url},
};
use eat::*;
use fantoccini::{error::CmdError, Client, Locator};
use scraper::{Html, Selector};
use serde_json::json;
use std::collections::HashSet;
use tokio::time::{timeout, Duration};

impl Eat<&str, (), ()> for Page {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        if i.is_empty() {
            return Err(());
        }
        Ok(("", Page(i.to_string())))
    }
}

impl Tag<Page, Html> {
    pub fn events(&self) -> Vec<Event<String, String>> {
        let id = Selector::parse(".match-detail-card").unwrap();
        let id_name = Selector::parse("h2").unwrap();
        let odds = Selector::parse(".outcomes-container").unwrap();
        let odd = Selector::parse("button").unwrap();
        let odd_name = Selector::parse(".odds-button__name").unwrap();
        let odd_value = Selector::parse(".odds-button__value-current").unwrap();
        self.inner()
            .select(&id)
            .map(|id_item| {
                let id = {
                    let id_name_item = id_item.select(&id_name).next().unwrap();
                    clean_text(id_name_item.text())
                };
                let odds = id_item
                    .select(&odds)
                    .flat_map(|odds_item| {
                        odds_item.select(&odd).filter_map(|odd_item| {
                            let name = {
                                let odd_name_item =
                                    odd_item.select(&odd_name).next()?;
                                clean_text(odd_name_item.text())
                            };
                            let value = {
                                let odd_value_item =
                                    odd_item.select(&odd_value).next()?;
                                clean_text(odd_value_item.text())
                                    .parse::<f64>()
                                    .ok()?
                            };
                            Some((name, value))
                        })
                    })
                    .collect();
                Event { id, odds }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Page(pub String);

async fn get_bet_titles(group: &fantoccini::elements::Element) -> Vec<String> {
    match group.find_all(Locator::Css("h2")).await {
        Ok(headings) => {
            let mut titles = Vec::new();
            for heading in headings {
                match heading.text().await {
                    Ok(text) => titles.push(text),
                    Err(e) => {
                        eprintln!("Text error: {:?}", e);
                    }
                }
            }
            titles
        }
        Err(e) => {
            eprintln!("Find error: {:?}", e);
            vec![]
        }
    }
}

type Interest = HashSet<fortuna::event::football::Football>;
type Players = [String; 2];

impl Download<Client, (Page, Interest, Players)> for Tag<Page, String> {
    type Error = CmdError;

    async fn download(
        client: &Client,
        data: (Page, Interest, Players),
    ) -> Result<Self, Self::Error> {
        let mut htmls = vec![];
        client.goto(data.0.url().as_str()).await?;
        let interest = data.1;
        let players = data.2;
        let result = timeout(
            Duration::from_secs(4),
            client
                .wait()
                .for_element(Locator::Css(".match-detail-card")),
        )
        .await;
        if let Err(_elapsed) = result {
            return Err(CmdError::WaitTimeout);
        }
        let groups =
            client.find_all(Locator::Css(".match-detail-card")).await?;
        let scroll = r#"
            const elem = arguments[0];
            elem.scrollIntoView({behavior: 'auto', block: 'center'});
            const rect = elem.getBoundingClientRect();
            const bottomOverlayHeight = 150;
            if (rect.bottom > (window.innerHeight - bottomOverlayHeight)) {
                window.scrollBy(0, rect.bottom - (window.innerHeight - bottomOverlayHeight));
            }
        "#;
        for group in groups {
            let titles = get_bet_titles(&group).await;
            let titles: Vec<_> = titles
                .into_iter()
                .filter(|t| !t.trim().is_empty())
                .collect();
            if titles.is_empty() {
                continue;
            }
            use fortuna::event::football::Football;
            let event = match Football::eat(titles[0].as_str(), players.clone())
            {
                Ok(("", event)) => event,
                _ => continue,
            };
            println!("{:?}", titles[0]);
            if !interest.contains(&event) {
                continue;
            }
            let has_odds = group
                .find(Locator::Css(".outcomes-container"))
                .await
                .is_ok();
            if !has_odds {
                client
                    .execute(scroll, vec![json!(group)])
                    .await
                    .unwrap_or_else(|err| panic!("scroll: {:#?}", err));
                group
                    .click()
                    .await
                    .unwrap_or_else(|err| panic!("click: {:#?}", err));
            }
            let html = group.html(false).await;
            htmls.push(html.unwrap_or_default());
        }
        Ok(htmls.join("")).map(Tag::new)
    }
}

impl Name for Page {
    fn name(&self) -> String {
        self.0.replace("/", ".")
    }
}

impl Url for Page {
    fn url(&self) -> String {
        format!("{}{}", URL, self.0)
    }
}
