use chrono::{DateTime, Duration, Local, Utc};
use dotenv::dotenv;
use fantoccini::{Client, ClientBuilder};
use odds::{
    bmbets::search::{find_match, hits, Hit},
    shared::db,
    utils::{browser, date},
};
use scraper::Html;
use serde_json::{json, Map};
use std::{
    collections::HashSet,
    io::{self, Write},
    time::Instant,
};

fn get_id() -> Option<Option<usize>> {
    print!("choose: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trim = input.trim();
    if trim == "-" {
        return Some(None);
    }
    trim.parse().ok().map(Some)
}

async fn get_hit(client: &mut Client, m: &db::Match) -> Option<Hit> {
    loop {
        let local = m.date.0.with_timezone(&chrono::Local);
        println!("{}", local.format("%Y-%m-%d %H:%M"));
        println!("{} - {}", m.player1, m.player2);
        print!("search: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "skip" {
            return None;
        }
        let Some(result) = get_match(client, &input, local).await else {
            println!("no hits");
            continue;
        };
        if let Some(hit) = result {
            return Some(hit);
        }
    }
}

async fn get_match(
    client: &mut Client,
    prompt: &str,
    local: DateTime<Local>,
) -> Option<Option<Hit>> {
    let html = find_match(client, prompt).await.unwrap();
    let document = Html::parse_document(&html);
    let hits = hits(document);
    let hits = hits.into_iter().filter(|hit| {
        let diff = local.signed_duration_since(date::to_local(hit.date));
        diff.num_minutes().abs() < Duration::hours(12).num_minutes()
    });
    let hits: Vec<_> = hits.collect();
    if hits.is_empty() {
        return None;
    }
    for (id, hit) in hits.iter().enumerate() {
        println!(
            "{id}: {} | {} - {}",
            hit.date.format("%Y-%m-%d %H:%M"),
            hit.players[0],
            hit.players[1]
        );
    }
    let Some(mut id) = get_id()? else {
        return Some(None);
    };
    while id >= hits.len() {
        match get_id()? {
            Some(new_id) => id = new_id,
            _ => return Some(None),
        }
    }
    Some(Some(hits[id].clone()))
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    let db = db::connect().await;
    let now = Utc::now();
    let later = now + Duration::hours(12);
    let fortuna =
        match db::immediate_matches(&db, now, later, db::Source::Fortuna).await
        {
            Ok(match_urls) => match_urls,
            Err(error) => {
                println!("{:?}", error);
                return;
            }
        };
    let bmbets = match db::immediate_matches(
        &db,
        now,
        later,
        db::Source::Bmbets,
    )
    .await
    {
        Ok(match_urls) => match_urls,
        Err(error) => {
            println!("{:?}", error);
            return;
        }
    };
    let set_a: HashSet<_> = fortuna.into_iter().collect();
    let set_b: HashSet<_> = bmbets.into_iter().collect();
    let match_ids: Vec<_> =
        set_a.difference(&set_b).map(|x| x.id.clone()).collect();
    let match_urls =
        match db::fetch_match_urls(&db, match_ids, db::Source::Fortuna).await {
            Ok(xs) => xs,
            Err(error) => {
                println!("{:?}", error);
                return;
            }
        };
    println!("matches: {}", match_urls.len());
    println!("Elapsed time: {:.2?}", start.elapsed());
    let caps = json!({
        "moz:firefoxOptions": {},
        "pageLoadStrategy": "eager"
    });
    let caps: Map<_, _> = caps.as_object().unwrap().clone();
    let mut client = ClientBuilder::native()
        .capabilities(caps)
        .connect(&browser::localhost(4444))
        .await
        .unwrap();
    for match_url in match_urls {
        let match_id = match_url.m.id.clone();
        let m = match_url.m.without_id();
        if let Some(hit) = get_hit(&mut client, &m).await {
            let relate = db
                .query(format!(
                    "RELATE {match_id}->on->source:bmbets SET url=$url;"
                ))
                .bind(("url", hit.relative_url.clone()));
            let r = relate.await;
            match r {
                Ok(_) => println!("Ok(RELATE)"),
                Err(error) => println!("{:?}", error),
            }
        }
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
