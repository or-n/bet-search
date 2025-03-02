use fantoccini::{error::CmdError, Client, Locator};
use odds::utils::{
    browser,
    scrape::{clean_text, split2},
};
use scraper::{Element, Html, Selector};
use std::time::Instant;
use tokio::time::{sleep, Duration};

const URL: &str = "https://www.bmbets.com/value-bets";
const ENTER: &str = "\u{E007}";

async fn find_match(client: &mut Client) -> Result<String, CmdError> {
    client.goto(URL).await?;
    let search_input = client.find(Locator::Id("search")).await?;
    search_input.send_keys("Real").await?;
    search_input.send_keys(ENTER).await?;
    sleep(Duration::from_secs(2)).await;
    client.source().await
}

fn hits(document: Html) -> Vec<[String; 2]> {
    let hit = Selector::parse("span.hit").unwrap();
    document
        .select(&hit)
        .filter_map(|x| x.parent_element())
        .map(|x| clean_text(x.text()))
        .filter_map(|x| split2(x, " - "))
        .collect()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut client = browser::connect(4444).await;
    let html = find_match(&mut client).await.unwrap();
    let document = Html::parse_document(&html);
    let hits = hits(document);
    for players in hits {
        println!("{} - {}", players[0], players[1]);
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
