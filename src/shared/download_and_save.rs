use crate::utils::{
    browser, download,
    page::{Name, Tag},
    save::save,
};
use download::Download;
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
) -> Result<Tag<Page, String>, Error>
where
    Page: Name,
    Tag<Page, String>: browser::Download<String>,
{
    let download = Tag::<Page, String>::download(client, "".to_string());
    let html = download.await.map_err(Error::Download)?;
    let file = format!("downloads/{}.html", Page::NAME);
    save(html.inner().as_bytes(), file).map_err(Error::SaveHTML)?;
    Ok(html)
}

// pub async fn run_subpages<Page>(
//     client: &mut fantoccini::Client,
//     subpages: Vec<String>,
// ) -> Result<(), Error>
// where
//     Page: Name,
//     Tag<Page, String>: browser::Download<String>,
// {
//     for subpage in subpages {
//         println!("{}", subpage);
//         let download = Tag::<Page, String>::download(client, subpage.clone());
//         let html = download.await.map_err(Error::Download)?;
//         if let Some((_, rest)) = subpage.split_once("/mecz/") {
//             let file = format!("downloads/{}_{}.html", Page::NAME, rest);
//             save(html.inner().as_bytes(), file).map_err(Error::SaveHTML)?;
//         }
//     }
//     Ok(())
// }
