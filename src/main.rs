mod shared;
mod utils;

mod fortuna;
// mod sts;
// mod superbet;

use shared::book::Subpages;

#[tokio::main]
async fn main() {
    use shared::download_and_save::{run, run_subpages};
    let mut client = utils::browser::connect(4444).await.unwrap();
    let result = run(&mut client, fortuna::live::Page).await;
    let html = result.unwrap();
    let subpages = html.subpages();
    let _ =
        run_subpages::<fortuna::live::subpages::Page>(&mut client, subpages)
            .await;
    client.close().await.unwrap();
}
