mod bookmaker;
mod utils;

mod efortuna;
mod sts;
mod superbet;

use bookmaker::Name;
use utils::{browser::Browser, download::Download};

#[derive(Debug)]
pub enum Error {
    Download(fantoccini::error::CmdError),
    Extract(()),
    Save(std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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

async fn download_and_save<Book, HTML>(
    port: u16,
) -> Result<Result<(), utils::browser::Error>, Error>
where
    Book: bookmaker::Name,
    Browser<Book>: Download<
        Output = Result<HTML, utils::browser::Error>,
        Error = fantoccini::error::CmdError,
    >,
    HTML: bookmaker::SportBets,
{
    let result = match Browser::new(port)
        .download()
        .await
        .map_err(Error::Download)?
    {
        Ok(html) => {
            let sport_bets = html.sport_bets().map_err(Error::Extract)?;
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
