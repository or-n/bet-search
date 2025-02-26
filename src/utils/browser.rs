// use std::process::{Child, Command};
use std::time::Duration;
use tokio::time::sleep;

pub trait Download<T>:
    super::download::Download<
    fantoccini::Client,
    T,
    Error = fantoccini::error::CmdError,
>
{
}

impl<T, Data> Download<Data> for T where
    T: super::download::Download<
        fantoccini::Client,
        Data,
        Error = fantoccini::error::CmdError,
    >
{
}

// #[derive(Debug)]
// pub enum Error {
//     Spawn(std::io::Error),
//     NewSession {
//         port: u16,
//         error: fantoccini::error::NewSessionError,
//     },
//     Cmd(fantoccini::error::CmdError),
// }

// pub fn spawn(port: u16) -> std::io::Result<Child> {
//     Command::new("geckodriver")
//         .arg("--port")
//         .arg(port.to_string())
//         .spawn()
// }

pub async fn connect(
    port: u16,
) -> Result<fantoccini::Client, fantoccini::error::NewSessionError> {
    // spawn(port).map_err(Error::Spawn)?;
    // use serde_json::{json, Map, Value};
    // let mut caps = Map::new();
    // caps.insert(
    //     "moz:firefoxOptions".to_string(),
    //     json!({ "args": ["-headless"] }),
    // );
    fantoccini::ClientBuilder::native()
        // .capabilities(caps)
        .connect(format!("http://localhost:{}", port).as_str())
        .await
}

pub async fn download_html(
    client: &mut fantoccini::Client,
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
            _ = sleep(Duration::from_millis(600)) => {
                true
            }
        };
        if exit {
            return client.source().await;
        }
    }
}
