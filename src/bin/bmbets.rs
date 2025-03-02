use fantoccini::{error::CmdError, Client, Locator};
use odds::utils::browser;
use std::time::Instant;

const URL: &str = "https://www.bmbets.com/value-bets";

async fn find_match(client: &mut Client) -> Result<(), CmdError> {
    client.goto(URL).await?;
    let search_input = client.find(Locator::Css("input.search")).await?;
    search_input.send_keys("Hello\n").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut client = browser::connect(4444).await;
    let _ = find_match(&mut client).await;
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
