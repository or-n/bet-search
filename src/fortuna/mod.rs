pub mod event;
pub mod live;
pub mod prematch;
use crate::utils::page::Name;
use eat::*;

pub const COOKIE_ACCEPT: &str = r#"button[id="accept"]"#;
// pub const COOKIE_ACCEPT: &str = r#"button[data-action-type="accept"]"#;
// pub const COOKIE_ACCEPT: &str = r#"button[class="uc-accept-button"]"#;
pub const LOGIN_CLOSE: &str = r#"button[class="close"]"#;

#[derive(Debug, Clone)]
pub enum Url {
    Prematch(prematch::Url),
}

impl Eat<&str, (), ()> for Url {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        if let Ok((i, prematch)) = prematch::Url::eat(i, ()) {
            return Ok((i, Url::Prematch(prematch)));
        }
        Err(())
    }
}

impl Name for Url {
    fn name(&self) -> String {
        match self {
            Url::Prematch(prematch) => {
                format!("fortuna.{}", prematch.name())
            }
        }
    }
}
