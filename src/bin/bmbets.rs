use odds::utils::browser;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let client = browser::connect(4444).await;
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
