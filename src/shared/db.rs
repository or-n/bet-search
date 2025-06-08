use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::env;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Datetime, RecordId, Surreal,
};

pub async fn connect() -> Surreal<Client> {
    let url = env::var("DB_URL").expect("DB_URL");
    let user = env::var("DB_USERNAME").expect("DB_USERNAME");
    let pass = env::var("DB_PASSWORD").expect("DB_PASSWORD");
    println!("{} {}", url, user);
    let db = Surreal::new::<Ws>(&url).await.expect("DB connect");
    println!("connected");
    db.signin(Root {
        username: &user,
        password: &pass,
    })
    .await
    .expect("DB auth");
    db.use_ns("bet").use_db("bet").await.expect("DB namespace");
    db
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub date: Datetime,
    pub player1: String,
    pub player2: String,
    pub sport: Sport,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchWithId {
    pub id: RecordId,
    pub date: Datetime,
    pub player1: String,
    pub player2: String,
    pub sport: Sport,
}

#[derive(Debug, Deserialize)]
pub struct MatchUrl {
    #[serde(rename = "in")]
    pub m: MatchWithId,
    pub url: String,
}

#[derive(Debug)]
pub enum Sport {
    Football,
    Basketball,
    Tennis,
    Volleyball,
}

#[derive(Debug)]
pub enum Source {
    Fortuna,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Download {
    pub date: Datetime,
    #[serde(rename = "match")]
    pub m: RecordId,
    pub source: Source,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: RecordId,
}

impl Serialize for Sport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let id_str = match self {
            Sport::Football => "sport:football",
            Sport::Basketball => "sport:basketball",
            Sport::Tennis => "sport:tennis",
            Sport::Volleyball => "sport:volleyball",
        };
        let thing: Thing = id_str.parse().unwrap();
        thing.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Sport {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let thing = Thing::deserialize(deserializer)?;
        match thing.to_string().as_str() {
            "sport:football" => Ok(Sport::Football),
            "sport:basketball" => Ok(Sport::Basketball),
            "sport:tennis" => Ok(Sport::Tennis),
            "sport:volleyball" => Ok(Sport::Volleyball),
            other => {
                Err(de::Error::custom(format!("Unknown sport id: {}", other)))
            }
        }
    }
}

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let id_str = match self {
            Source::Fortuna => "source:fortuna",
        };
        let thing: Thing = id_str.parse().unwrap();
        thing.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let thing = Thing::deserialize(deserializer)?;
        match thing.to_string().as_str() {
            "source:fortuna" => Ok(Source::Fortuna),
            other => {
                Err(de::Error::custom(format!("Unknown source id: {}", other)))
            }
        }
    }
}
