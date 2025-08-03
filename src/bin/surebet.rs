use fantoccini::ClientBuilder;
use odds::{
    surebet,
    utils::{self, download::Download, page::Tag},
};
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut client = ClientBuilder::native()
        .connect(&utils::localhost(4444))
        .await
        .unwrap();
    let page = surebet::value::Page;
    let html = Tag::download(&mut client, page).await.unwrap();
    client.close().await.unwrap();
    println!("{}", html.inner());
    println!("Elapsed time: {:.2?}", start.elapsed());
}
