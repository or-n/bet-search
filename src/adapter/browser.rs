use fantoccini::{error::CmdError, Client, Locator};
use std::time::Duration;
use tokio::time::sleep;

pub async fn try_accepting_cookie(
    client: &Client,
    cookie_accept: &str,
) -> Result<bool, CmdError> {
    tokio::select! {
        accept = client.wait().for_element(Locator::Css(cookie_accept)) => {
            accept?.click().await?;
            Ok(true)
        }
        _ = sleep(Duration::from_millis(4000)) => {
            Ok(false)
        }
    }
}
