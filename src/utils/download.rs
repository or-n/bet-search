use std::time::Duration;
use tokio::time::sleep;

pub async fn download(
    client: fantoccini::Client,
    url: &str,
    cookie_accept: fantoccini::Locator<'_>,
) -> Result<String, fantoccini::error::CmdError> {
    client.goto(url).await?;
    let mut cookie_accepted = false;
    loop {
        let exit = tokio::select! {
            accept = client.wait().for_element(cookie_accept),
            if !cookie_accepted => {
                accept?.click().await?;
                cookie_accepted = true;
                false
            }
            _ = sleep(Duration::from_millis(1000)) => {
                true
            }
        };
        if exit {
            let html = client.source().await?;
            client.close().await?;
            return Ok(html);
        }
    }
}

pub trait Download {
    type Output;
    type Error;

    async fn download(&self) -> Result<Self::Output, Self::Error>;
}
