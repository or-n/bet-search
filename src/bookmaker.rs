pub trait Name {
    const NAME: &'static str;
}

pub trait Site {
    const SITE: &'static str;

    const COOKIE_ACCEPT_CSS: &'static str;
}

#[allow(dead_code)]
pub type Odds = Vec<f32>;

pub trait GetOdds {
    fn get_odds(site: &String) -> Result<Vec<Odds>, ()>;
}
