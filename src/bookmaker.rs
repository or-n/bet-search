pub trait Name {
    const NAME: &'static str;
}

pub trait Site {
    const SITE: &'static str;

    const COOKIE_ACCEPT_CSS: &'static str;
}

pub type Odds = Vec<f32>;

#[derive(Debug)]
pub struct Teams {
    pub team1: String,
    pub team2: String,
}

pub trait GetOdds {
    fn get_odds(site: &String) -> Result<Vec<(Teams, Odds)>, ()>;
}
