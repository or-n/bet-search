use crate::fortuna::live::{COOKIE_ACCEPT, URL};
use crate::utils::{browser, download};

pub struct Page(String);

impl download::Download for browser::Browser<Page> {
    type Output = Page;
    type Error = browser::Error;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        let url = format!("{}/sports/LPLTENNIS", URL);
        self.run(url.as_str(), COOKIE_ACCEPT).await.map(Page)
    }
}

impl ToString for Page {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
