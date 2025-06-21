use fantoccini::{
    elements::Element,
    error::CmdError,
    Client,
    Locator::{Css, XPath},
};
use futures::future::join_all;
use tokio::time::{sleep, Duration};

pub const TAB: &str = ".list";
pub const TOOLBAR: &str = ".ui-toolbar";

pub async fn dropdown(client: &mut Client) -> Result<(), CmdError> {
    let dropdown = client.wait().for_element(Css("#elmTabUl")).await?;
    let expanded = dropdown.attr("expanded").await?;
    if expanded != Some("true".to_string()) {
        dropdown.click().await?;
    }
    Ok(())
}

async fn links(list: Element) -> Result<Vec<(String, Element)>, CmdError> {
    let links = list.find_all(Css("a")).await?;
    Ok(join_all(links.into_iter().map(|link| async move {
        let name = link.html(true).await.unwrap_or_else(|_| "".to_string());
        (name, link)
    }))
    .await)
}

pub async fn tab_links(
    client: &mut Client,
) -> Result<Vec<(String, Element)>, CmdError> {
    let list = client.wait().for_element(Css(TAB)).await?;
    links(list).await
}

pub async fn toolbar_links(
    client: &mut Client,
) -> Result<Vec<(String, Element)>, CmdError> {
    let toolbar = client.wait().for_element(Css(TOOLBAR)).await?;
    let list = toolbar.find(Css("ul")).await?;
    links(list).await
}

pub async fn variants(
    client: &mut Client,
) -> Result<Vec<(String, Element)>, CmdError> {
    let content = client.find(Css("#oddsContent")).await?;
    sleep(Duration::from_millis(2000)).await;
    let divs = content.find_all(Css("div.caption")).await?;
    let divs = join_all(divs.into_iter().map(|div| async move {
        let span = div.find(Css("span.caption-txt")).await?;
        let name = span.text().await?;
        Ok::<_, CmdError>((name, div))
    }));
    Ok(divs.await.into_iter().flatten().collect())
}

pub async fn odds_table(
    div: Element,
) -> Result<Vec<(String, Vec<f32>)>, CmdError> {
    let parent = div.find(XPath("..")).await?;
    let bmdiv = parent.find(Css(".bmdiv")).await?;
    let parent = bmdiv.find(XPath("..")).await?;
    if !parent.is_displayed().await? {
        div.click().await?;
    }
    let bmtbody = bmdiv.find(Css("tbody")).await?;
    let bmtrs = bmtbody.find_all(Css("tr")).await?;
    let odddiv = parent.find(Css(".odddiv")).await?;
    let oddtbody = odddiv.find(Css("tbody")).await?;
    let oddtrs = oddtbody.find_all(Css("tr")).await?;
    let trs: Vec<_> = bmtrs.into_iter().zip(oddtrs.into_iter()).collect();
    let r = join_all(trs.into_iter().map(|(bmtr, oddtr)| async move {
        let bm = bmtr.find(Css("td span.hidden-480")).await?;
        let bm = bm.text().await?;
        let oddtds = oddtr.find_all(Css(".odd-v")).await?;
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
