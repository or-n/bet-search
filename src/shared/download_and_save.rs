use super::book;
use crate::utils::{browser, download, page, save};
use download::Download;

#[derive(Debug)]
pub enum Error<'a> {
    Browser(browser::Error),
    Download(fantoccini::error::CmdError),
    Extract(&'a str, super::book::Error),
    SaveHTML(std::io::Error),
    SaveSportBets(std::io::Error),
}

pub async fn run<Page>(port: u16) -> Result<(), Error<'static>>
where
    browser::Browser<Page>: download::Download<
        Output = Result<Page, fantoccini::error::CmdError>,
        Error = browser::Error,
    >,
    Page: page::Name + book::SportBets + book::Subpages + ToString,
{
    let result = browser::Browser::new(port).download().await;
    let html = result.map_err(Error::Browser)?.map_err(Error::Download)?;
    save::save(
        html.to_string().as_bytes(),
        format!("downloads/{}.html", Page::NAME),
    )
    .map_err(Error::SaveHTML)?;
    // let sport_bets = html
    //     .sport_bets()
    //     .map_err(|error| Error::Extract(Book::NAME, error))?;
    // let lines = sport_bets
    //     .into_iter()
    //     .map(|(teams, odds)| format!("{:?}\n{:?}\n", teams, odds))
    //     .collect::<Vec<_>>();
    save::save(
        html.subpages().join("\n").as_bytes(),
        format!("downloads/{}.txt", Page::NAME),
    )
    .map_err(Error::SaveSportBets)?;
    Ok(())
}
