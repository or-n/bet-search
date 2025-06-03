pub trait ToDBRecord {
    fn to_db_record(&self) -> Option<String>;
}

pub fn sanitize(x: &str) -> String {
    x.chars()
        .map(|c| match c {
            'ą' => 'a',
            'ć' => 'c',
            'ę' => 'e',
            'ł' => 'l',
            'ń' => 'n',
            'ó' => 'o',
            'ś' => 's',
            'ź' => 'z',
            'ż' => 'z',
            'Ą' => 'A',
            'Ć' => 'C',
            'Ę' => 'E',
            'Ł' => 'L',
            'Ń' => 'N',
            'Ó' => 'O',
            'Ś' => 'S',
            'Ź' => 'Z',
            'Ż' => 'Z',
            ' ' => '_',
            '-' => '_',
            'á' => 'a',
            'ř' => 'r',
            _ => c,
        })
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

pub enum Football {
    Win,
    WinDiff,
    WinEitherH1H2,
    NotWin,
    NotWinEitherH1FT,
    Goals,
    Not0GoalsP1P2,
    Not0GoalsP1P2AndGoals,
    Not0GoalsP1P2EitherH1H2,
    Not0GoalsH1H2,
    Targets,
    YellowCards,
    RedCards,
    Corners,
    Offsides,
    Fouls,
    Penalty,
}

pub enum Player {
    P1,
    P2,
}

pub struct Event {
    pub tag: Football,
    pub params: Params,
}

#[derive(Default)]
pub struct Params {
    pub player: Option<Player>,
    pub time_min: Option<f64>,
    pub time_max: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub handicap: Option<f64>,
}
