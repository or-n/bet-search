use crate::utils::{
    browser, download,
    page::{Name, Tag},
    save::save,
};
use download::Download;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Download({0})")]
    Download(fantoccini::error::CmdError),
    #[error("SaveHTML({0})")]
    SaveHTML(std::io::Error),
}

pub async fn run<Page>(
    client: &mut fantoccini::Client,
    page: Page,
) -> Result<Tag<Page, String>, Error>
where
    Page: Name,
    Tag<Page, String>: browser::Download<()>,
{
    let download = Tag::<Page, String>::download(client, ());
    let html = download.await.map_err(Error::Download)?;
    let file = format!("downloads/{}.html", page.name());
    save(html.inner().as_bytes(), file).map_err(Error::SaveHTML)?;
    Ok(html)
}

pub async fn run_subpages<Page>(
    client: &mut fantoccini::Client,
    subpages: Vec<Page>,
) -> Result<(), Error>
where
    Page: Name + Clone,
    Tag<Page, String>: browser::Download<Page>,
{
    for subpage in subpages.iter() {
        println!("{}", subpage.name());
        let download = Tag::<Page, String>::download(client, subpage.clone());
        let html = download.await.map_err(Error::Download)?;
        let file = format!("downloads/{}.html", subpage.name());
        save(html.inner().as_bytes(), file).map_err(Error::SaveHTML)?;
    }
    println!("done {}", subpages.len());
    Ok(())
}
