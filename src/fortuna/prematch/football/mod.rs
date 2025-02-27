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

#[derive(Clone, Copy)]
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
        let mut new_count = 0;
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

pub enum EventType {
    Goals,
    GoalsH1,
    GoalsH2,
    ExactGoals,
    ExactGoalsH1,
    ExactGoalsH2,
    BothToScore,
    BothToScoreH1,
    BothToScoreH2,
    Handicap,
    HandicapH1,
    HandicapH2,
    H1,
    H2,
    Unknown(String),
}

impl Eat<&str, (), ()> for EventType {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use EventType::*;
        if let Ok(i) = "Liczba goli".drop(i) {
            return Ok((i, Goals));
        }
        if let Ok(i) = "1.połowa liczba goli".drop(i) {
            return Ok((i, GoalsH1));
        }
        if let Ok(i) = "2.połowa liczba goli".drop(i) {
            return Ok((i, GoalsH2));
        }
        if let Ok(i) = "Dokładna liczba goli".drop(i) {
            return Ok((i, ExactGoals));
        }
        if let Ok(i) = "1.połowa: dokładna liczba goli".drop(i) {
            return Ok((i, ExactGoalsH1));
        }
        if let Ok(i) = "2.połowa: dokładna liczba goli".drop(i) {
            return Ok((i, ExactGoalsH2));
        }
        if let Ok(i) = "Obie drużyny strzelą gola".drop(i) {
            return Ok((i, BothToScore));
        }
        if let Ok(i) = "Obie drużyny strzelą gola w 1.połowie".drop(i) {
            return Ok((i, BothToScoreH1));
        }
        if let Ok(i) = "Obie drużyny strzelą gola w 2.połowie".drop(i) {
            return Ok((i, BothToScoreH2));
        }
        if let Ok(i) = "Handicap".drop(i) {
            return Ok((i, Handicap));
        }
        if let Ok(i) = "1.połowa: handicap".drop(i) {
            return Ok((i, HandicapH1));
        }
        if let Ok(i) = "2.połowa: handicap".drop(i) {
            return Ok((i, HandicapH2));
        }
        if let Ok(i) = "1.połowa".drop(i) {
            return Ok((i, H1));
        }
        if let Ok(i) = "2.połowa".drop(i) {
            return Ok((i, H2));
        }
        Ok(("", Unknown(i.to_string())))
    }
}
