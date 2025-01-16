mod shared;
mod utils;

mod fortuna;
mod sts;
mod superbet;

use shared::book::Name;

#[tokio::main]
async fn main() -> Result<(), shared::download_and_save::Error<'static>> {
    tokio::try_join!(
        shared::download_and_save::run::<fortuna::Book, fortuna::live::Page>(
            4444
        ),
        // download_and_save::<sts::Book, sts::LivePage>(4445),
        // download_and_save::<superbet::Book, superbet::LivePage>(4446),
    )
    .map(|(fortuna)| {
        println!("{}: {:?}", fortuna::Book::NAME, fortuna);
        // println!("{}: {:?}", sts::Book::NAME, sts);
        // println!("{}: {:?}", superbet::Book::NAME, superbet);
    })
}
