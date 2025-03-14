pub mod subpage;

use crate::fortuna::COOKIE_ACCEPT;
use crate::utils::{
    browser,
    download::Download,
    page::{Name, Tag},
};
use eat::*;
use fantoccini::{error::CmdError, Client, Locator};
use tokio::time::{sleep, Duration};

const URL: &str = "/zaklady-bukmacherskie/pilka-nozna";

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
        let mut previous_count = 0;
        let mut new_count;
        let scroll = "window.scrollTo(0, document.body.scrollHeight);";
        loop {
            let elements = client.find_all(Locator::Css(".event-link")).await?;
            new_count = elements.len();
            if new_count == previous_count {
                println!("nothing new");
                break;
            }
            println!("found: {}", new_count);
            previous_count = new_count;
            client.execute(scroll, vec![]).await?;
            sleep(Duration::from_secs(2)).await;
        }
        client.source().await.map(Tag::new)
    }
}

impl Name for Page {
    fn name(&self) -> String {
        "fortuna.football".to_string()
    }
}
