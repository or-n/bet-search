pub mod subpage;

use crate::fortuna::COOKIE_ACCEPT;
use crate::shared::event::Match;
use crate::utils::{
    browser, date,
    download::Download,
    page::{Name, Tag},
    scrape::clean_text,
};
use eat::*;
use fantoccini::{error::CmdError, Client, Locator};
use scraper::{Html, Selector};
use serde_json::json;

pub const URL: &str = "/zaklady-bukmacherskie/pika-nozna";

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
        let _ = client
            .wait()
            .for_element(Locator::Css(".card-group"))
            .await?;
        let groups = client.find_all(Locator::Css(".card-group")).await?;
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
            let has_matches = group.find(Locator::Css("article")).await.is_ok();
            if !has_matches {
                client.execute(scroll, vec![json!(group)]).await?;
                group.click().await?;
            }
        }
        client.source().await.map(Tag::new)
    }
}

impl Tag<Page, Html> {
    pub fn matches(&self) -> Vec<Match> {
        let group = Selector::parse(".card-group").unwrap();
        let group_header = Selector::parse(".offer-card-group-header").unwrap();
        let group_name = Selector::parse("a").unwrap();
        let matches = Selector::parse("article").unwrap();
        let football_match = Selector::parse("a").unwrap();
        let time = Selector::parse("time").unwrap();
        let player =
            Selector::parse(".fixture-card__participant-name").unwrap();
        self.inner()
            .select(&group)
            .filter_map(|group_item| {
                let group_name = {
                    let group_header_item =
                        group_item.select(&group_header).next()?;
                    let group_name_item =
                        group_header_item.select(&group_name).next()?;
                    clean_text(group_name_item.text())
                };
                let matches_item = group_item.select(&matches).next()?;
                let player = player.clone();
                let time = time.clone();
                let matches = matches_item.select(&football_match).filter_map(
                    move |football_match_item| {
                        let date = {
                            let time =
                                football_match_item.select(&time).next()?;
                            let time = clean_text(time.text());
                            date::eat_fortuna(&time)?
                        };
                        let url = football_match_item
                            .value()
                            .attr("href")?
                            .to_string();
                        let players = {
                            let players = football_match_item
                                .select(&player)
                                .map(|player_item| {
                                    clean_text(player_item.text())
                                });
                            let players: Vec<_> = players.collect();
                            players.try_into().unwrap()
                        };
                        Some(Match {
                            url,
                            date,
                            players,
                            tournament: group_name.clone(),
                        })
                    },
                );
                Some(matches)
            })
            .flatten()
            .collect()
    }
}

impl Name for Page {
    fn name(&self) -> String {
        "fortuna.football".to_string()
    }
}
