pub struct Teams {
    pub team1: String,
    pub team2: String,
}

impl std::fmt::Debug for Teams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} vs {}", self.team1, self.team2)
    }
}

pub trait Subpages<T> {
    fn subpages(&self) -> Vec<T>;
}
