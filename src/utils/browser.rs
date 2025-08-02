// use std::process::{Child, Command};
use fantoccini::{error::CmdError, Client, Locator};
use std::time::Duration;
use tokio::time::sleep;

pub const ENTER: &str = "\u{E007}";
pub const _TAB: &str = "\u{E004}";
pub const _ESC: &str = "\u{E00C}";

// pub fn spawn(port: u16) -> std::io::Result<Child> {
//     Command::new("geckodriver")
//         .arg("--port")
//         .arg(port.to_string())
//         .spawn()
// }

pub fn localhost(port: u16) -> String {
    format!("http://localhost:{}", port)
}

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
