mod bookmaker;
mod download_and_save;

mod efortuna;
mod sts;
mod superbet;

use download_and_save::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tokio::try_join!(
        //download_and_save::<efortuna::Book>(4444),
        //download_and_save::<sts::Book>(4445),
        download_and_save::<superbet::Book>(4446),
    )
    .map(|_| ())
}
