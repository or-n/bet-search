mod shared;
mod utils;

mod fortuna;
mod sts;
mod superbet;

#[tokio::main]
async fn main() -> Result<(), shared::download_and_save::Error<'static>> {
    use utils::page::Name;
    tokio::try_join!(shared::download_and_save::run::<fortuna::live::Page>(
        4444
    ),)
    .map(|fortuna| {
        println!("{}: {:?}", fortuna::live::Page::NAME, fortuna);
    })
}
