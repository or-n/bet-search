mod shared;
mod utils;

mod fortuna;
// mod sts;
// mod superbet;

use shared::book::Subpages;

#[tokio::main]
async fn main() {
    let mut client = utils::browser::connect(4444).await.unwrap();
    use shared::download_and_save::run;
    let result = run::<fortuna::live::Page>(&mut client).await;
    let html = result.unwrap();
    println!("{:?}", html.subpages());
    // let _ = run_subpages(&mut client, html.subpages()).await;
    client.close().await.unwrap();
}
