use crate::shared::{self, book};
use book::Error;

impl book::SportBets for super::Page {
    fn sport_bets(&self) -> Result<Vec<(book::Teams, book::Odds)>, Error> {
        use scraper::Selector;
        let team1 = Selector::parse("div.live-match-info__team--1").unwrap();
        let team2 = Selector::parse("div.live-match-info__team--2").unwrap();
        shared::sport_bets::extract(
            &self.0,
            Selector::parse("div.live-match").unwrap(),
            Selector::parse("span.odds_button__value").unwrap(),
            |x| {
                Ok([
                    x.select(&team1).next().ok_or(Error::MissingTeam1)?,
                    x.select(&team2).next().ok_or(Error::MissingTeam2)?,
                ])
            },
        )
    }
}
