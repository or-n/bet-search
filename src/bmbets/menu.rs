use fantoccini::{elements::Element, error::CmdError, Client, Locator};
use futures::future::join_all;

pub async fn list(
    client: &mut Client,
) -> Result<Vec<(String, Element)>, CmdError> {
    client.wait().for_element(Locator::Css("#elmTabUl")).await?;
    let menu_dropdown = client.find(Locator::Css("#elmTabUl")).await?;
    menu_dropdown.click().await.unwrap();
    let menu = client.find(Locator::Css(".list")).await?;
    let links = menu.find_all(Locator::Css("a")).await?;
    Ok(join_all(links.into_iter().map(|link| async move {
        let name = link.html(true).await.unwrap_or_else(|_| "".to_string());
        // let url = link.attr("href").await.unwrap().unwrap_or_default();
        (name, link)
    }))
    .await)
}
