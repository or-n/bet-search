use fantoccini::{
    elements::Element,
    error::CmdError,
    Client,
    Locator::{Css, XPath},
};
use futures::future::join_all;
use tokio::time::{timeout, Duration};

pub async fn dropdown(client: &Client) -> Result<(), CmdError> {
    let dropdown = timeout(
        Duration::from_secs(4),
        client.wait().for_element(Css("#elmTabUl")),
    )
    .await
    .map_err(|_| CmdError::WaitTimeout)??;
    let expanded = dropdown.attr("expanded").await?;
    if expanded != Some("true".to_string()) {
        dropdown.click().await?;
    }
    Ok(())
}

async fn links(list: Element) -> Result<Vec<(String, Element)>, CmdError> {
    let links = list.find_all(Css("a")).await?;
    Ok(join_all(links.into_iter().map(|link| async move {
        let name = link.html(true).await.unwrap();
        (name, link)
    }))
    .await)
}

pub async fn tab_links(
    client: &Client,
) -> Result<Vec<(String, Element)>, CmdError> {
    let list = timeout(
        Duration::from_secs(4),
        client.wait().for_element(Css(".list")),
    )
    .await
    .map_err(|_| CmdError::WaitTimeout)??;
    links(list).await
}

pub async fn toolbar_links(
    client: &Client,
    tbar: usize,
) -> Result<Vec<(String, Element)>, CmdError> {
    let list_id = format!(".ui-toolbar ul#tbar_{}", tbar);
    let list = timeout(
        Duration::from_secs(4),
        client.wait().for_element(Css(&list_id)),
    )
    .await
    .map_err(|_| CmdError::WaitTimeout)??;
    links(list).await
}

pub async fn odds_content(client: &Client) -> Result<Element, CmdError> {
    timeout(
        Duration::from_secs(4),
        client.wait().for_element(Css("#oddsContent")),
    )
    .await
    .map_err(|_| CmdError::WaitTimeout)?
}

pub async fn variants(
    content: Element,
) -> Result<Vec<(String, Element)>, CmdError> {
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
    let bmtrs = {
        let bmtbody = bmdiv.find(Css("tbody")).await?;
        bmtbody.find_all(Css("tr")).await?
    };
    let oddtrs = {
        let odddiv = parent.find(Css(".odddiv")).await?;
        let oddtbody = odddiv.find(Css("tbody")).await?;
        oddtbody.find_all(Css("tr")).await?
    };
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
