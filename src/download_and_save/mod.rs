mod download;
mod save;

use crate::bookmaker;

#[derive(Debug)]
pub enum Error {
    Web(fantoccini::error::CmdError),
    IO(std::io::Error),
}

pub async fn download_and_save<Book>(port: u16) -> Result<(), Error>
where
    Book: bookmaker::Name + bookmaker::Site + bookmaker::GetOdds,
{
    let html = download::download::<Book>(port).await.map_err(Error::Web)?;
    let matches = Book::get_odds(&html).unwrap();
    for (teams, odds) in matches {
        println!("{:?}", teams);
        println!("{:?}", odds);
    }
    save::save::<Book>(html).map_err(Error::IO)
}
