use fantoccini::{error::CmdError, Client, Locator};
use std::time::Duration;
use tokio::time::sleep;

pub async fn old_try_accepting_cookie(
    client: &Client,
    cookie_accept: &str,
) -> Result<bool, CmdError> {
    // println!("trying to accept cookie");
    tokio::select! {
        accept = client.wait().for_element(Locator::Css(cookie_accept)) => {
            // println!("trying to click");
            accept?.click().await?;
            // println!("clicked");
            Ok(true)
        }
        _ = sleep(Duration::from_millis(1000)) => {
            Ok(false)
        }
    }
}

pub async fn try_ready(client: &Client) -> Result<(), CmdError> {
    loop {
        let ready = client.execute(
            "return document.readyState;",
            vec![],
        ).await?;
        if ready.as_str() == Some("interactive") {
            break Ok(());
        }
        sleep(Duration::from_millis(100)).await;
    }
}

pub async fn try_accepting_cookie(client: &Client, _cookie_accept: &str) -> Result<bool, CmdError> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(1);
    loop {
        if tokio::time::Instant::now() > deadline {
            return Ok(false);
        }
        let clicked = client.execute(
            r#"
            const host = document.querySelector('#usercentrics-cmp-ui');
            const root = host?.shadowRoot;
            const btn =
                root?.querySelector('button[data-action-type="accept"]')
                || root?.querySelector('#accept');
            if (btn) {
                btn.click();
                return true;
            }
            return false;
            "#,
            vec![],
        ).await?;
        let overlay_gone = client.execute(
            r#"
            const host = document.querySelector('#usercentrics-cmp-ui');
            return !host || host.offsetParent === null;
            "#,
            vec![],
        ).await?;
        if clicked.as_bool() == Some(true) && overlay_gone.as_bool() == Some(true) {
            return Ok(true);
        }
        sleep(Duration::from_millis(300)).await;
    }
}

// use fantoccini::{Client, error::CmdError, Locator};
// use tokio::time::{sleep, Duration};

// pub async fn try_accepting_cookie(
//     client: &Client,
//     cookie_accept: &str,
// ) -> Result<bool, CmdError> {
//     let deadline = tokio::time::Instant::now() + Duration::from_secs(6);

//     loop {
//         if tokio::time::Instant::now() > deadline {
//             return Ok(false);
//         }

//         if let Ok(el) = client.find(Locator::Css(cookie_accept)).await {
//             let displayed = el.is_displayed().await.unwrap_or(false);
//             let enabled = el.is_enabled().await.unwrap_or(false);

//             if displayed && enabled {
//                 el.click().await?;
//                 return Ok(true);
//             }
//         }

//         sleep(Duration::from_millis(200)).await;
//     }
// }
