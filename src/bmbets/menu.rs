use fantoccini::{
    elements::Element,
    error::{CmdError, ErrorStatus, WebDriver},
    Client, Locator,
};
use futures::future::join_all;
use tokio::time::{sleep, Duration};

pub const TAB: &str = ".list";
pub const TOOLBAR: &str = ".ui-toolbar";

pub async fn dropdown(client: &mut Client) -> Result<(), CmdError> {
    let dropdown = client.wait().for_element(Locator::Css("#elmTabUl")).await?;
    let expanded = dropdown.attr("expanded").await?;
    if expanded != Some("true".to_string()) {
        dropdown.click().await?;
    }
    Ok(())
}

pub async fn tab(client: &mut Client) -> Result<Element, CmdError> {
    client.wait().for_element(Locator::Css(TAB)).await
}

pub async fn toolbar(client: &mut Client) -> Result<Element, CmdError> {
    let toolbar = client.wait().for_element(Locator::Css(TOOLBAR)).await?;
    let divs = toolbar.find_all(Locator::Css("div")).await?;
    for div in divs {
        if div.is_displayed().await? {
            return Ok(div);
        }
    }
    let webdriver = WebDriver::new(ErrorStatus::NoSuchElement, "toolbar div");
    Err(CmdError::Standard(webdriver))
}

pub async fn links(list: Element) -> Result<Vec<(String, Element)>, CmdError> {
    let links = list.find_all(Locator::Css("a")).await?;
    Ok(join_all(links.into_iter().map(|link| async move {
        let name = link.html(true).await.unwrap_or_else(|_| "".to_string());
        (name, link)
    }))
    .await)
}

pub async fn odds_divs(client: &mut Client) -> Result<Vec<Element>, CmdError> {
    sleep(Duration::from_millis(500)).await;
    let content = client.find(Locator::Css("#oddsContent")).await?;
    let divs = content.find_all(Locator::Css("div.caption")).await?;
    Ok(divs.into_iter().collect())
}
