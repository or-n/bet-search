pub mod event;
pub mod live;
pub mod prematch;
pub mod safe;
use crate::utils::page::Name;
use eat::*;
// pub mod team;

pub const COOKIE_ACCEPT: &str = r#"button[id="cookie-consent-button-accept"]"#;

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
