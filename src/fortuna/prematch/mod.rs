pub mod football;
use crate::utils::page::Name;
use eat::*;

pub const URL: &str = "https://www.efortuna.pl";

#[derive(Debug, Clone)]
pub enum Url {
    Football(football::Url),
    Root,
}

impl Eat<&str, (), ()> for Url {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        let i = URL.drop(i)?;
        if let Ok((i, football)) = football::Url::eat(i, ()) {
            return Ok((i, Url::Football(football)));
        }
        Ok((i, Url::Root))
    }
}

impl Name for Url {
    fn name(&self) -> String {
        match self {
            Url::Football(football) => format!("football{}", football.name()),
            Url::Root => "".to_string(),
        }
    }
}
