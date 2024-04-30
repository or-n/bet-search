mod download;
mod save;

use crate::bookmaker;

#[derive(Debug)]
pub enum Error {
    Web(fantoccini::error::CmdError),
    Save(std::io::Error),
}

pub async fn download_and_save<Book>(port: u16) -> Result<(), Error>
where
    Book: bookmaker::Name + bookmaker::Site + bookmaker::GetOdds,
{
    let html = download::download::<Book>(port).await.map_err(Error::Web)?;
    let matches = Book::get_odds(&html).unwrap();
    let content = matches
        .into_iter()
        .map(|(teams, odds)| format!("{:?}\n{:?}\n", teams, odds))
        .collect::<Vec<_>>()
        .join("\n");
    save::save(content.as_bytes(), format!("downloads/{}.txt", Book::NAME))
        .map_err(Error::Save)
}
