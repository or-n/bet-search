use crate::shared::book;
use crate::utils::{browser, download::Download, page, scrape};
use book::Subpages;
use fantoccini::{error::CmdError, Client, Locator};
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
        let mut previous_count = 0;
        let mut new_count;
        let scroll = r#"
            let el = document.querySelector('.live-matchtiles-wrapper');
            if (!el) return "no .live-matchtiles-wrapper";
            let event = new MouseEvent('wheel', {
                deltaY: 1000,
                bubbles: true,
                cancelable: true,
                view: window,
            });

            el.dispatchEvent(event);
            return "scrolled";
        "#;
        loop {
            let elements =
                client.find_all(Locator::Css("bb-live-league")).await?;
            new_count = elements.len();
            if new_count == previous_count {
                println!("nothing new");
                break;
            }
            println!("found: {}", new_count);
            previous_count = new_count;
            let r = client.execute(scroll, vec![]).await?;
            println!("scroll result: {:?}", r);
            sleep(Duration::from_secs(2)).await;
        }
        sleep(Duration::from_secs(4)).await;
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
    pub url: String,
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
                let url = element
                    .select(&subpage)
                    .filter_map(|element| element.value().attr("href"))
                    .next()
                    .map(|href| href.to_string())?;
                let tournament = element
                    .select(&tournament)
                    .next()
                    .map(|x| clean_text(x.text()))?;
                let page = Subpage { url, tournament };
                Some(page)
            })
            .collect()
    }
}
