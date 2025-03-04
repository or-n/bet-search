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
use scraper::Html;
use shared::book::Subpages;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

fn contents(document: Tag<football::subpage::Page, Html>) -> Option<String> {
    let Some(players) = document.players() else {
        return None;
    };
    println!("{} - {}", players[0], players[1]);
    let events = document.events().into_iter().filter_map(|event| {
        let odds: Vec<_> = event
            .odds
            .into_iter()
            .map(|pair| format!("{:?}", pair))
            .collect();
        Some(format!("{}\n{}", event.name, odds.join("\n")))
    });
    let events: Vec<_> = events.collect();
    if events.is_empty() {
        return None;
    }
    Some(format!(
        "{}\n{}\n\n{}",
        players[0],
        players[1],
        events.join("\n\n")
    ))
}

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
    while let Some((subpage, date)) = queue.lock().await.pop() {
        if !date::in_hours(date, 12) {
            continue;
        }
        let html = Tag::download(&mut client, subpage.clone()).await.unwrap();
        download_count += 1;
        let Some(contents) = contents(html.document()) else {
            continue;
        };
        let file = format!("downloads/{}", subpage.name());
        let f = format!("{}\n\n{}", subpage.url(), contents);
        let _ = save(f.as_bytes(), file).await;
        save_count += 1;
    }
    client.close().await.unwrap();
    let elapsed = start.elapsed().as_secs_f32();
    println!("Elapsed time: {:.2?}", elapsed);
    println!("Total count: {}", total_count);
    println!("Download count: {}", download_count);
    println!("Save count: {}", save_count);
    println!("{:.2?} / download", elapsed / download_count as f32);
}
