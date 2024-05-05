mod bookmaker;
mod download;
mod utils;

mod efortuna;
mod sts;
mod superbet;

use bookmaker::Name;

#[derive(Debug)]
pub enum Error {
    Download(fantoccini::error::CmdError),
    Extract(()),
    Save(std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tokio::try_join!(
        download_and_save::<efortuna::Book>(4444),
        download_and_save::<sts::Book>(4445),
        download_and_save::<superbet::Book>(4446),
    )
    .map(|(efortuna, sts, superbet)| {
        println!("{}: {:?}", efortuna::Book::NAME, efortuna);
        println!("{}: {:?}", sts::Book::NAME, sts);
        println!("{}: {:?}", superbet::Book::NAME, superbet);
    })
}

async fn download_and_save<Book>(
    port: u16,
) -> Result<Result<(), utils::browser::Error>, Error>
where
    Book: bookmaker::Name + bookmaker::Site + bookmaker::GetOdds,
{
    match utils::browser::client(port).await {
        Ok(client) => {
            let html = download::download::<Book>(client)
                .await
                .map_err(Error::Download)?;
            let events = Book::get_odds(&html).map_err(Error::Extract)?;
            let content = events
                .into_iter()
                .map(|(teams, odds)| format!("{:?}\n{:?}\n", teams, odds))
                .collect::<Vec<_>>()
                .join("\n");
            utils::save::save(
                content.as_bytes(),
                format!("downloads/{}.txt", Book::NAME),
            )
            .map_err(Error::Save)?;
            Ok::<_, Error>(Ok(()))
        }
        Err(connect_error) => Ok(Err(connect_error)),
    }
}
