pub trait Name {
    const NAME: &'static str;
}

pub trait Site {
    const SITE: &'static str;

    const COOKIE_ACCEPT_CSS: &'static str;
}

pub type Odds = Vec<f32>;

pub struct Teams {
    pub team1: String,
    pub team2: String,
}

impl std::fmt::Debug for Teams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} vs {}", self.team1, self.team2)
    }
}

pub trait GetOdds {
    fn get_odds(site: &String) -> Result<Vec<(Teams, Odds)>, ()>;
}
