use crate::fortuna;
use crate::utils::{self, browser::Browser, download::Download};

#[derive(Debug)]
pub enum Error<'a> {
    Download(fantoccini::error::CmdError),
    Extract(&'a str, super::book::Error),
    SaveHTML(std::io::Error),
    SaveSportBets(std::io::Error),
}

pub async fn run<Book, Page>(
    port: u16,
) -> Result<Result<(), utils::browser::Error>, Error<'static>>
where
    Book: super::book::Name,
    Browser<Book>: Download<
        Output = Result<Page, utils::browser::Error>,
        Error = fantoccini::error::CmdError,
    >,
    Page: super::book::SportBets + super::book::Subpages,
    for<'a> &'a Page: Into<String>,
{
    let result = match Browser::new(port)
        .download()
        .await
        .map_err(Error::Download)?
    {
        Ok(html) => {
            let html_string: String = (&html).into();
            utils::save::save(
                html_string.as_bytes(),
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
        Err(connect_error) => Err(connect_error),
    };
    Ok(result)
}
