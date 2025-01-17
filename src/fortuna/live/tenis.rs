use crate::fortuna::live::{COOKIE_ACCEPT, URL};
use crate::utils::{browser, download};

pub struct Page(String);

impl download::Download for browser::Browser<Page> {
    type Output = Result<Page, fantoccini::error::CmdError>;
    type Error = browser::Error;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        let cookie_accept = fantoccini::Locator::Css(COOKIE_ACCEPT);
        let browser = browser::client(self.port).await?;
        let url = format!("{}/sport/LPLTENNIS", URL);
        let page = download::run(browser, url.as_str(), cookie_accept).await;
        Ok(page.map(Page))
    }
}

impl ToString for Page {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
