use crate::utils::{self, browser::Browser, download::Download};

#[derive(Debug)]
pub enum Error<'a> {
    Browser(utils::browser::Error),
    Download(fantoccini::error::CmdError),
    Extract(&'a str, super::book::Error),
    SaveHTML(std::io::Error),
    SaveSportBets(std::io::Error),
}

pub async fn run<Book, Page>(port: u16) -> Result<(), Error<'static>>
where
    Book: super::book::Name,
    Browser<Book>: Download<
        // Output = Result<Page, utils::browser::Error>,
        // Error = fantoccini::error::CmdError,
        Output = Result<Page, fantoccini::error::CmdError>,
        Error = utils::browser::Error,
    >,
    Page: super::book::SportBets + super::book::Subpages + ToString,
{
    let result = Browser::new(port).download().await;
    let html = result.map_err(Error::Browser)?.map_err(Error::Download)?;
    utils::save::save(
        html.to_string().as_bytes(),
        format!("downloads/{}.html", Book::NAME),
    )
    .map_err(Error::SaveHTML)?;
    // let sport_bets = html
    //     .sport_bets()
    //     .map_err(|error| Error::Extract(Book::NAME, error))?;
    // let lines = sport_bets
    //     .into_iter()
    //     .map(|(teams, odds)| format!("{:?}\n{:?}\n", teams, odds))
    //     .collect::<Vec<_>>();
    utils::save::save(
        html.subpages().join("\n").as_bytes(),
        format!("downloads/{}.txt", Book::NAME),
    )
    .map_err(Error::SaveSportBets)?;
    Ok(())
}
