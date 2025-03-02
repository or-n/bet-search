use crate::utils::{
    browser,
    scrape::{clean_text, split2},
};
use fantoccini::{error::CmdError, Client, Locator};
use scraper::{Element, Html, Selector};
use tokio::time::{sleep, Duration};

pub async fn find_match(client: &mut Client) -> Result<String, CmdError> {
    client.goto(super::URL).await?;
    let search_input = client.find(Locator::Id("search")).await?;
    search_input.send_keys("Real").await?;
    search_input.send_keys(browser::ENTER).await?;
    sleep(Duration::from_secs(2)).await;
    client.source().await
}

pub fn hits(document: Html) -> Vec<([String; 2], String)> {
    let hit = Selector::parse("span.hit").unwrap();
    document
        .select(&hit)
        .filter_map(|x| x.parent_element())
        .filter_map(|a| {
            let text = clean_text(a.text());
            let players = split2(text, " - ")?;
            let link = a.value().attr("href")?.to_string();
            Some((players, link))
        })
        .collect()
}
