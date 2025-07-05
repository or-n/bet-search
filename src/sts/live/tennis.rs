use crate::shared::book;
use crate::utils::{browser, download::Download, page, scrape};
use book::Subpages;
use fantoccini::{error::CmdError, Client};
use page::{Name, Tag, Url};
use scrape::clean_text;
use scraper::{Html, Selector};
use tokio::time::{sleep, Duration};

const URL: &str = "/tenis";
const COOKIE_ACCEPT: &str =
    r#"button[id="CybotCookiebotDialogBodyLevelButtonLevelOptinAllowAll"]"#;

#[derive(Debug, Clone, Copy)]
pub struct Page;

impl Download<Client, Page> for Tag<Page, String> {
    type Error = CmdError;

    async fn download(
        client: &mut Client,
        page: Page,
    ) -> Result<Self, Self::Error> {
        let url = page.url();
        client.goto(url.as_str()).await?;
        sleep(Duration::from_secs(1)).await;
        browser::try_accepting_cookie(client, COOKIE_ACCEPT).await?;
        client.source().await.map(Tag::new)
    }
}

impl Url for Page {
    fn url(&self) -> String {
        format!("{}{}", super::URL, URL)
    }
}

impl Name for Page {
    fn name(&self) -> String {
        "sts.live.tennis".to_string()
    }
}

pub struct Subpage {
    pub urls: Vec<String>,
    pub tournament: String,
}

impl Subpages<Subpage> for Tag<Page, Html> {
    fn subpages(&self) -> Vec<Subpage> {
        let event = Selector::parse("bb-live-league").unwrap();
        let tournament =
            Selector::parse(".match-tile-region-info__text").unwrap();
        let subpage = Selector::parse("bb-live-match-tile a").unwrap();
        self.inner()
            .select(&event)
            .filter_map(|element| {
                let urls: Vec<String> = element
                    .select(&subpage)
                    .filter_map(|e| {
                        e.value().attr("href").map(|s| s.to_string())
                    })
                    .collect();
                let tournament = element
                    .select(&tournament)
                    .next()
                    .map(|x| clean_text(x.text()))?;
                let page = Subpage { urls, tournament };
                Some(page)
            })
            .collect()
    }
}
