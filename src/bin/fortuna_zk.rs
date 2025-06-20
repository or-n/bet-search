use dotenv::dotenv;
use odds::shared::db;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let _db = db::connect().await;
    println!("Elapsed time: {:.2?}", start.elapsed());
}
