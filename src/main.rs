mod bookmaker;
mod utils;

mod efortuna;
mod sts;
mod superbet;

use bookmaker::Name;
use utils::{browser::Browser, download::Download};

#[tokio::main]
async fn main() -> Result<(), Error<'static>> {
    tokio::try_join!(
        download_and_save::<efortuna::Book, efortuna::LivePage>(4444),
        download_and_save::<sts::Book, sts::LivePage>(4445),
        download_and_save::<superbet::Book, superbet::LivePage>(4446),
    )
    .map(|(efortuna, sts, superbet)| {
        println!("{}: {:?}", efortuna::Book::NAME, efortuna);
        println!("{}: {:?}", sts::Book::NAME, sts);
        println!("{}: {:?}", superbet::Book::NAME, superbet);
    })
}

#[derive(Debug)]
pub enum Error<'a> {
    Download(fantoccini::error::CmdError),
    Extract(&'a str, bookmaker::Error),
    Save(std::io::Error),
}

async fn download_and_save<Book, Page>(
    port: u16,
) -> Result<Result<(), utils::browser::Error>, Error<'static>>
where
    Book: bookmaker::Name,
    Browser<Book>: Download<
        Output = Result<Page, utils::browser::Error>,
        Error = fantoccini::error::CmdError,
    >,
    Page: bookmaker::SportBets,
{
    let result = match Browser::new(port)
        .download()
        .await
        .map_err(Error::Download)?
    {
        Ok(html) => {
            let sport_bets = html
                .sport_bets()
                .map_err(|error| Error::Extract(Book::NAME, error))?;
            let content = sport_bets
                .into_iter()
                .map(|(teams, odds)| format!("{:?}\n{:?}\n", teams, odds))
                .collect::<Vec<_>>()
                .join("\n");
            utils::save::save(
                content.as_bytes(),
                format!("downloads/{}.txt", Book::NAME),
            )
            .map_err(Error::Save)?;
            Ok(())
        }
        Err(connect_error) => Err(connect_error),
    };
    Ok(result)
}
