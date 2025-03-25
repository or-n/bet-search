use eat::*;
use fantoccini::ClientBuilder;
use fortuna::prematch::football;
use odds::fortuna;
use odds::shared;
use odds::utils::{
    browser, date,
    download::Download,
    page::{Name, Tag, Url},
    save::save,
};
use shared::book::Subpages;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut client = ClientBuilder::native()
        .connect(&browser::localhost(4444))
        .await
        .unwrap();
    let page = fortuna::prematch::football::Page;
    let html = Tag::download(&mut client, page).await.unwrap();
    let subpages = html.document().subpages();
    let total_count = subpages.len();
    let queue = Arc::new(Mutex::new(subpages));
    println!("Elapsed time: {:.2?}", start.elapsed());
    let start = Instant::now();
    let mut download_count = 0;
    let mut save_count = 0;
    let mut maybe_safe_save_count = 0;
    while let Some((subpage, date)) = queue.lock().await.pop() {
        if !date::in_hours(date, 12) {
            continue;
        }
        let html = Tag::download(&mut client, subpage.clone()).await.unwrap();
        download_count += 1;
        let Some(m) =
            football::subpage::to_match(subpage.url(), html.document())
        else {
            continue;
        };
        let r = fortuna::Url::eat(m.url.as_str(), ());
        let Ok((_i, url)) = r else {
            println!("{:?}", r);
            continue;
        };
        println!("{} - {}", m.players[0], m.players[1]);
        let Some(contents) = shared::event::match_contents(&m) else {
            continue;
        };
        let file = format!("downloads/{}", url.name());
        let _ = save(contents.as_bytes(), file).await;
        save_count += 1;
        let Some(m) = fortuna::safe::match_filter(m) else {
            continue;
        };
        let Some(contents) = shared::event::match_contents(&m) else {
            continue;
        };
        let file = format!("maybe_safe/{}", url.name());
        let _ = save(contents.as_bytes(), file).await;
        maybe_safe_save_count += 1;
    }
    client.close().await.unwrap();
    let elapsed = start.elapsed().as_secs_f32();
    println!("Elapsed time: {:.2?}", elapsed);
    println!("Total count: {}", total_count);
    println!("Download count: {}", download_count);
    println!("Save count: {}", save_count);
    println!("Save count (maybe safe): {}", maybe_safe_save_count);
    println!("{:.2?} / download", elapsed / download_count as f32);
}
