mod shared;
mod utils;

mod fortuna;
// mod sts;
// mod superbet;

use fantoccini::Client;
use shared::book::Subpages;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Instant;
use thiserror::Error;
use tokio::sync::Mutex;
use utils::{
    browser,
    download::Download,
    page::{Name, Tag},
    save::save,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Download({0})")]
    Download(fantoccini::error::CmdError),
    #[error("SaveHTML({0})")]
    SaveHTML(std::io::Error),
}

pub fn save_html<Page>(
    page: Page,
    html: &Tag<Page, String>,
) -> Result<(), std::io::Error>
where
    Page: Name,
{
    let file = format!("downloads/{}.html", page.name());
    save(html.inner().as_bytes(), file)
}

async fn download_and_save<Page>(
    client: &mut Client,
    page: Page,
) -> Result<Tag<Page, String>, Error>
where
    Page: Name + Clone,
    Tag<Page, String>: browser::Download<Page>,
{
    let download = Tag::download(client, page.clone());
    let html = download.await.map_err(Error::Download)?;
    save_html(page, &html).map_err(Error::SaveHTML)?;
    Ok(html)
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut client = browser::connect(4444).await;
    let page = fortuna::prematch::football::Page;
    let html = download_and_save(&mut client, page).await.unwrap();
    let queue = Arc::new(Mutex::new(html.subpages()));
    while let Some(subpage) = queue.lock().await.pop() {
        println!("{}", subpage.name());
        let _ = download_and_save(&mut client, subpage).await;
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
