mod bookmaker;
mod efortuna;
mod sts;
mod superbet;

use reqwest;
use std::fs::File;
use std::io::Write;

async fn get_site(site: &str) -> Result<String, reqwest::Error> {
    reqwest::get(site).await?.text().await
}

async fn download_bookmaker_site<Book>() -> std::io::Result<()>
where
    Book: bookmaker::Name + bookmaker::Site,
{
    let text = get_site(Book::SITE).await.unwrap();
    let mut file = File::create(format!("downloads/{}.html", Book::NAME))?;
    file.write_all(text.as_bytes())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tokio::try_join!(
        download_bookmaker_site::<efortuna::Book>(),
        download_bookmaker_site::<sts::Book>(),
        download_bookmaker_site::<superbet::Book>(),
    )
    .map(|_| ())
}
