use crate::shared::event::{Event, Match};
use eat::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Football {}

#[derive(Debug, Clone)]
pub enum FootballOption {}

impl Eat<&str, (), [String; 2]> for Football {
    fn eat(_i: &str, _players: [String; 2]) -> Result<(&str, Self), ()> {
        Err(())
    }
}

impl Eat<&str, (), [String; 2]> for FootballOption {
    fn eat(_i: &str, _players: [String; 2]) -> Result<(&str, Self), ()> {
        Err(())
    }
}
