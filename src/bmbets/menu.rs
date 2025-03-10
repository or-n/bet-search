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
pub async fn odds_content(client: &mut Client) -> Result<Element, CmdError> {
    client.find(Locator::Css("#oddsContent")).await
}

pub async fn odds_divs(
    content: Element,
) -> Result<Vec<(String, Element)>, CmdError> {
    sleep(Duration::from_millis(2000)).await;
    let divs = content.find_all(Locator::Css("div.caption")).await?;
    let divs = join_all(divs.into_iter().map(|div| async move {
        let span = div.find(Locator::Css("span.caption-txt")).await?;
        let name = span.text().await?;
        Ok::<_, CmdError>((name, div))
    }));
    Ok(divs.await.into_iter().flatten().collect())
}

pub async fn odds_table(
    div: Element,
) -> Result<Vec<(String, Vec<f32>)>, CmdError> {
    let parent = div.find(Locator::XPath("..")).await?;
    let bmdiv = parent.find(Locator::Css(".bmdiv")).await?;
    let parent = bmdiv.find(Locator::XPath("..")).await?;
    if !parent.is_displayed().await? {
        div.click().await?;
    }
    let bmtbody = bmdiv.find(Locator::Css("tbody")).await?;
    let bmtrs = bmtbody.find_all(Locator::Css("tr")).await?;
    let odddiv = parent.find(Locator::Css(".odddiv")).await?;
    let oddtbody = odddiv.find(Locator::Css("tbody")).await?;
    let oddtrs = oddtbody.find_all(Locator::Css("tr")).await?;
    let trs: Vec<_> = bmtrs.into_iter().zip(oddtrs.into_iter()).collect();
    let r = join_all(trs.into_iter().map(|(bmtr, oddtr)| async move {
        let bm = bmtr.find(Locator::Css("td span.hidden-480")).await?;
        let bm = bm.text().await?;
        let oddtds = oddtr.find_all(Locator::Css(".odd-v")).await?;
        let odds = join_all(oddtds.into_iter().map(|td| async move {
            let text = td.text().await.ok()?;
            if text.is_empty() {
                return None;
            }
            text.parse::<f32>().ok()
        }));
        let odds = odds.await.into_iter().flatten().collect();
        Ok::<_, CmdError>((bm, odds))
    }));
    Ok(r.await.into_iter().flatten().collect())
}
