use crate::utils::{
    browser,
    scrape::{clean_text, split2},
};
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};
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
    sleep(Duration::from_secs(4)).await;
    client.source().await
}

#[derive(Clone)]
pub struct Hit {
    pub players: [String; 2],
    pub date: NaiveDateTime,
    pub relative_url: String,
}

pub fn hits(document: Html) -> Vec<Hit> {
    let hit = Selector::parse("span.hit").unwrap();
    let date_col = Selector::parse("td.date-col div").unwrap();
    document
        .select(&hit)
        .filter_map(|x| x.parent_element())
        .filter_map(|a| {
            let text = clean_text(a.text());
            let players = split2(text, " - ")?;
            let relative_url = a.value().attr("href")?.to_string();
            let tr = a.parent_element()?.parent_element()?;
            let date_parts = tr.select(&date_col).map(|x| clean_text(x.text()));
            let date_parts: Vec<_> = date_parts.collect();
            if date_parts.len() < 2 {
                return None;
            }
            let year = Local::now().year();
            let format = "%b-%d";
            let parsed_date = NaiveDate::parse_from_str(
                &format!("{}-{}", date_parts[0], year),
                &format!("{}-%Y", format),
            );
            let format = "%H:%M";
            let parsed_time = NaiveTime::parse_from_str(&date_parts[1], format);
            let date = NaiveDateTime::new(parsed_date.ok()?, parsed_time.ok()?);
            Some(Hit {
                players,
                date,
                relative_url,
            })
        })
        .collect()
}
