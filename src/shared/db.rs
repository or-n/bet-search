use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::env;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::{Datetime, Thing},
    Error, RecordId, Surreal,
};

pub fn prematch_hours() -> i64 {
    env::var("PREMATCH_HOURS")
        .expect("PREMATCH_HOURS")
        .parse()
        .expect("PREMATCH_HOURS")
}

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

pub async fn matches_date(
    db: &Surreal<Client>,
    date_range: [DateTime<Utc>; 2],
    source: Source,
) -> Result<Vec<Record>, Error> {
    let date_range: [Datetime; 2] = date_range.map(|x| x.into());
    db.query(
        "SELECT in AS id FROM on
        WHERE out = $source
        AND in.date IN $date_min>..$date_max;
    ",
    )
    .bind(("source", source))
    .bind(("date_min", date_range[0].clone()))
    .bind(("date_max", date_range[1].clone()))
    .await?
    .take(0)
}

pub async fn matches_date_odd(
    db: &Surreal<Client>,
    date_range: [DateTime<Utc>; 2],
    book: Book,
    range: [f64; 2],
) -> Result<Vec<Record>, Error> {
    let date_range: [Datetime; 2] = date_range.map(|x| x.into());
    db.query(
        "SELECT out.match as id FROM offers
        WHERE in = $book
        AND odd IN $min..=$max
        AND out.match.date IN $date_min>..$date_max
        FETCH out;",
    )
    .bind(("book", book))
    .bind(("date_min", date_range[0].clone()))
    .bind(("date_max", date_range[1].clone()))
    .bind(("min", range[0]))
    .bind(("max", range[1]))
    .await?
    .take(0)
}

pub async fn events_match_odd(
    db: &Surreal<Client>,
    m: RecordId,
    book: Book,
    range: [f64; 2],
) -> Result<Vec<EventWithOdd>, Error> {
    db.query(
        "SELECT
            out.tag as tag,
            out.time_min as time_min,
            out.time_max as time_max,
            out.min as min,
            out.max as max,
            odd
        FROM offers
        WHERE in = $book
        AND odd IN $min..=$max
        AND out.match = $match
        FETCH out;",
    )
    .bind(("book", book))
    .bind(("min", range[0]))
    .bind(("max", range[1]))
    .bind(("match", m))
    .await?
    .take(0)
}

pub async fn fetch_match_urls(
    db: &Surreal<Client>,
    ids: Vec<RecordId>,
    source: Source,
) -> Result<Vec<MatchUrl>, Error> {
    db.query(
        "SELECT in, url FROM on
        WHERE out = $source
        AND in IN $ids
        FETCH in;
    ",
    )
    .bind(("source", source))
    .bind(("ids", ids))
    .await?
    .take(0)
}

pub async fn event_ids(
    db: &Surreal<Client>,
    e: Event,
) -> Result<Vec<Record>, Error> {
    db.query(
        "SELECT id FROM real_event
        WHERE tag=$tag
        AND time_min=$ta
        AND time_max=$tb
        AND min=$a
        AND max=$b",
    )
    .bind(("tag", e.tag))
    .bind(("ta", e.ta))
    .bind(("tb", e.tb))
    .bind(("a", e.a))
    .bind(("b", e.b))
    .await?
    .take(0)
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

#[derive(Debug, Default, Clone)]
pub enum Football {
    #[default]
    GoalD,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Event {
    pub tag: Football,
    #[serde(rename = "time_min")]
    pub ta: Option<f64>,
    #[serde(rename = "time_max")]
    pub tb: Option<f64>,
    #[serde(rename = "min")]
    pub a: Option<f64>,
    #[serde(rename = "max")]
    pub b: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EventWithOdd {
    pub tag: Football,
    #[serde(rename = "time_min")]
    pub ta: Option<f64>,
    #[serde(rename = "time_max")]
    pub tb: Option<f64>,
    #[serde(rename = "min")]
    pub a: Option<f64>,
    #[serde(rename = "max")]
    pub b: Option<f64>,
    pub odd: f64,
}

#[derive(Debug, Serialize)]
pub struct MatchEvent {
    #[serde(rename = "match")]
    pub m: RecordId,
    #[serde(flatten)]
    pub event: Event,
}

#[derive(Debug, Clone)]
pub enum EventResult {
    Hit,
    Miss,
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

impl MatchWithId {
    pub fn without_id(self) -> Match {
        Match {
            date: self.date,
            player1: self.player1,
            player2: self.player2,
            sport: self.sport,
        }
    }
}

#[derive(Debug)]
pub enum Sport {
    Football,
    Basketball,
    Tennis,
    Volleyball,
}

#[derive(Debug)]
pub enum Book {
    Fortuna,
}

#[derive(Debug)]
pub enum Source {
    Fortuna,
    Bmbets,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Download {
    pub date: Datetime,
    #[serde(rename = "match")]
    pub m: RecordId,
    pub source: Source,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct Record {
    #[allow(dead_code)]
    pub id: RecordId,
}

impl Serialize for Football {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let id_str = match self {
            Football::GoalD => "football:goal_diff",
        };
        let thing: Thing = id_str.parse().unwrap();
        thing.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Football {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let thing = Thing::deserialize(deserializer)?;
        match thing.to_string().as_str() {
            "football:goal_diff" => Ok(Football::GoalD),
            other => Err(de::Error::custom(format!(
                "Unknown football id: {}",
                other
            ))),
        }
    }
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

impl Serialize for Book {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let id_str = match self {
            Book::Fortuna => "book:fortuna",
        };
        let thing: Thing = id_str.parse().unwrap();
        thing.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Book {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let thing = Thing::deserialize(deserializer)?;
        match thing.to_string().as_str() {
            "book:fortuna" => Ok(Book::Fortuna),
            other => {
                Err(de::Error::custom(format!("Unknown source id: {}", other)))
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
            Source::Bmbets => "source:bmbets",
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
            "source:bmbets" => Ok(Source::Bmbets),
            other => {
                Err(de::Error::custom(format!("Unknown source id: {}", other)))
            }
        }
    }
}

impl Serialize for EventResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let id_str = match self {
            EventResult::Hit => "result:hit",
            EventResult::Miss => "result:miss",
        };
        let thing: Thing = id_str.parse().unwrap();
        thing.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EventResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let thing = Thing::deserialize(deserializer)?;
        match thing.to_string().as_str() {
            "result:hit" => Ok(EventResult::Hit),
            "result:miss" => Ok(EventResult::Miss),
            other => {
                Err(de::Error::custom(format!("Unknown result id: {}", other)))
            }
        }
    }
}
