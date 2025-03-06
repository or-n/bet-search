use crate::utils::{
    browser,
    scrape::{clean_text, split2},
};
use fantoccini::{error::CmdError, Client, Locator};
use scraper::{Element, Html, Selector};
use tokio::time::{sleep, Duration};

pub async fn find_match(
    client: &mut Client,
    match_name: &str,
) -> Result<String, CmdError> {
    client.goto(super::URL).await?;
    let search = client.find(Locator::Id("search")).await?;
    search.send_keys(match_name).await?;
    search.send_keys(browser::ENTER).await?;
    sleep(Duration::from_secs(2)).await;
    client.source().await
}

#[derive(Clone)]
pub struct Hit {
    pub players: [String; 2],
    pub relative_url: String,
}

pub fn hits(document: Html) -> Vec<Hit> {
    let hit = Selector::parse("span.hit").unwrap();
    document
        .select(&hit)
        .filter_map(|x| x.parent_element())
        .filter_map(|a| {
            let text = clean_text(a.text());
            let players = split2(text, " - ")?;
            let relative_url = a.value().attr("href")?.to_string();
            Some(Hit {
                players,
                relative_url,
            })
        })
        .collect()
}
