use std::fs::File;
use std::io::Write;

use crate::bookmaker;

pub fn save<Book>(html: String) -> std::io::Result<()>
where
    Book: bookmaker::Name,
{
    let mut file = File::create(format!("downloads/{}.html", Book::NAME))?;
    file.write_all(html.as_bytes())
}
