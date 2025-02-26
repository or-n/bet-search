// use std::process::{Child, Command};
use fantoccini::{error::CmdError, Client, ClientBuilder, Locator};
use std::time::Duration;
use tokio::time::sleep;

pub trait Download<T>:
    super::download::Download<Client, T, Error = CmdError>
{
}

impl<T, Data> Download<Data> for T where
    T: super::download::Download<Client, Data, Error = CmdError>
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

pub async fn connect(port: u16) -> Client {
    ClientBuilder::native()
        .connect(format!("http://localhost:{}", port).as_str())
        .await
        .unwrap()
}

pub async fn try_accepting_cookie(
    client: &mut Client,
    cookie_accept: &str,
) -> Result<bool, CmdError> {
    tokio::select! {
        accept = client.wait().for_element(Locator::Css(cookie_accept)) => {
            accept?.click().await?;
            Ok(true)
        }
        _ = sleep(Duration::from_millis(2000)) => {
            Ok(false)
        }
    }
}
