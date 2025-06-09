use crate::utils::{
    browser,
    download::Download,
    page::{Name, Tag},
    save::save,
};

#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    #[error("Download({0})")]
    Download(fantoccini::error::CmdError),
    #[error("SaveHTML({0})")]
    SaveHTML(std::io::Error),
}

pub async fn save_html<Page>(
    page: Page,
    html: &Tag<Page, String>,
) -> Result<(), std::io::Error>
where
    Page: Name,
{
    let file = format!("downloads/{}.html", page.name());
    save(html.inner().as_bytes(), file).await
}

pub async fn run<Page>(
    client: &mut fantoccini::Client,
    page: Page,
) -> Result<Tag<Page, String>, Error>
where
    Page: Name + Clone,
    Tag<Page, String>: browser::Download<Page>,
{
    let download = Tag::download(client, page.clone());
    let html = download.await.map_err(Error::Download)?;
    save_html(page, &html).await.map_err(Error::SaveHTML)?;
    Ok(html)
}
