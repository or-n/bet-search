pub trait Name {
    const NAME: &'static str;
}

pub type Odds = Vec<Result<f32, String>>;

pub struct Teams {
    pub team1: String,
    pub team2: String,
}

impl std::fmt::Debug for Teams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} vs {}", self.team1, self.team2)
    }
}

pub trait SportBets {
    fn sport_bets(&self) -> Result<Vec<(Teams, Odds)>, ()>;
}
