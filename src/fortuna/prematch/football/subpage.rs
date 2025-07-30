use crate::fortuna::prematch::URL;
use crate::shared::event::Event;
use crate::utils::{
    download::Download,
    page::{Name, Tag, Url},
    scrape::clean_text,
};
use eat::*;
use fantoccini::{error::CmdError, Client, Locator};
use scraper::{Html, Selector};
use serde_json::json;
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
                let odds = {
                    let odds_item = match id_item.select(&odds).next() {
                        Some(x) => x,
                        _ => return Event { id, odds: vec![] },
                    };
                    odds_item
                        .select(&odd)
                        .filter_map(|odd_item| {
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
                        .collect()
                };
                Event { id, odds }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Page(pub String);

impl Download<Client, Page> for Tag<Page, String> {
    type Error = CmdError;

    async fn download(
        client: &mut Client,
        data: Page,
    ) -> Result<Self, Self::Error> {
        client.goto(data.url().as_str()).await?;
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
        let scroll =
            "arguments[0].scrollIntoView({behavior: 'auto', block: 'center'});";
        for group in groups {
            let has_odds = group
                .find(Locator::Css(".outcomes-container"))
                .await
                .is_ok();
            if !has_odds {
                client.execute(scroll, vec![json!(group)]).await?;
                group.click().await?;
            }
        }
        client.source().await.map(Tag::new)
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
