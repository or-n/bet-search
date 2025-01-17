use fantoccini::ClientBuilder;
use std::marker::PhantomData;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
pub enum Error {
    NewSessionError {
        port: u16,
        error: fantoccini::error::NewSessionError,
    },
    Cmd(fantoccini::error::CmdError),
}

pub async fn client(port: u16) -> Result<fantoccini::Client, Error> {
    ClientBuilder::native()
        .connect(format!("http://localhost:{}", port).as_str())
        .await
        .map_err(|error| Error::NewSessionError { port, error })
}

pub struct Browser<T> {
    pub port: u16,
    _marker: PhantomData<T>,
}

impl<T> Browser<T> {
    pub fn new(port: u16) -> Self {
        Browser {
            port,
            _marker: PhantomData,
        }
    }

    pub async fn run(
        &self,
        url: &str,
        cookie_accept: &str,
    ) -> Result<String, Error> {
        let client = client(self.port).await?;
        cmd(client, url, cookie_accept).await.map_err(Error::Cmd)
    }
}

pub async fn cmd(
    client: fantoccini::Client,
    url: &str,
    cookie_accept: &str,
) -> Result<String, fantoccini::error::CmdError> {
    client.goto(url).await?;
    let mut cookie_accepted = false;
    let cookie_accept = fantoccini::Locator::Css(cookie_accept);
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
