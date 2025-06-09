use fantoccini::ClientBuilder;
use odds::shared;
use odds::sts;
use odds::utils::{browser, download::Download, page::Tag};
use shared::book::Subpages;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut client = ClientBuilder::native()
        .connect(&browser::localhost(4444))
        .await
        .unwrap();
    let page = sts::live::tennis::Page;
    let html = Tag::download(&mut client, page).await.unwrap();
    // client.close().await.unwrap();
    let subpages = html.document().subpages();
    for subpage in subpages {
        println!("{} {}", subpage.url, subpage.tournament);
    }
    println!("Elapsed time: {:.2?}", start.elapsed());
}
