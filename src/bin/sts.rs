use fantoccini::ClientBuilder;
use odds::{
    shared, sts,
    utils::{self, download::Download, page::Tag},
};
use shared::book::Subpages;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut client = ClientBuilder::native()
        .connect(&utils::localhost(4444))
        .await
        .unwrap();
    let page = sts::live::tennis::Page;
    let html = Tag::download(&mut client, page).await.unwrap();
    client.close().await.unwrap();
    let subpages = html.document().subpages();
    for subpage in subpages {
        for m in subpage.matches {
            println!("{} {:?} {}", m.url, m.sets, subpage.tournament);
        }
    }
    println!("Elapsed time: {:.2?}", start.elapsed());
}
