use crate::adapter::{browser, scrape};
use crate::shared::book;
use crate::utils::{download::Download, page};
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
        client: &Client,
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
    pub matches: Vec<Match>,
    pub tournament: String,
}

pub struct Match {
    pub url: String,
    pub sets: Vec<(usize, usize)>,
}

impl Subpages<Subpage> for Tag<Page, Html> {
    fn subpages(&self) -> Vec<Subpage> {
        let event = Selector::parse("bb-live-league").unwrap();
        let tournament =
            Selector::parse(".match-tile-region-info__text").unwrap();
        let subpage = Selector::parse("bb-live-match-tile a").unwrap();
        let sets =
            Selector::parse(".live-match-tile-scoreboard-score__partials")
                .unwrap();
        let set_scores = Selector::parse("div").unwrap();
        self.inner()
            .select(&event)
            .filter_map(|element| {
                let matches: Vec<Match> = element
                    .select(&subpage)
                    .filter_map(|e| {
                        let url = e.value().attr("href")?.to_string();
                        let sets: Vec<(usize, usize)> = e
                            .select(&sets)
                            .filter_map(|e| {
                                let mut iter = e.select(&set_scores);
                                let a = iter.next()?;
                                let b = iter.next()?;
                                let a_value = clean_text(a.text());
                                let b_value = clean_text(b.text());
                                let a_value = a_value.parse::<usize>().ok()?;
                                let b_value = b_value.parse::<usize>().ok()?;
                                Some((a_value, b_value))
                            })
                            .collect();
                        Some(Match { url, sets })
                    })
                    .collect();
                let tournament = element
                    .select(&tournament)
                    .next()
                    .map(|x| clean_text(x.text()))?;
                let page = Subpage {
                    matches,
                    tournament,
                };
                Some(page)
            })
            .collect()
    }
}
