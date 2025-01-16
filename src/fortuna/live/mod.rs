pub mod sport_bets;
pub mod subpages;

use crate::utils::{self, browser::Browser};

pub struct Page(String);

const URL: &str = "https://live.efortuna.pl/";

impl utils::download::Download for Browser<super::Book> {
    type Output = Result<Page, utils::browser::Error>;
    type Error = fantoccini::error::CmdError;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        Ok(match utils::browser::client(self.port).await {
            Ok(client) => Ok(Page(
                utils::download::download(
                    client,
                    URL,
                    fantoccini::Locator::Css(
                        r#"button[id="cookie-consent-button-accept"]"#,
                    ),
                )
                .await?,
            )),
            Err(connect_error) => Err(connect_error),
        })
    }
}

impl ToString for Page {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
