use chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone};

pub fn eat_assume_year(i: &str) -> Option<NaiveDateTime> {
    let now = Local::now();
    let current_year = now.year();
    let full_date1 = format!("{}.{}", i, current_year);
    let format = "%d.%m. %H:%M.%Y";
    let date1 = NaiveDateTime::parse_from_str(&full_date1, format).ok()?;
    if date1 >= now.naive_local() {
        return Some(date1);
    }
    let full_date2 = format!("{}.{}", i, current_year + 1);
    let date2 = NaiveDateTime::parse_from_str(&full_date2, format).ok()?;
    Some(date2)
}

pub fn eat_fortuna(i: &str) -> Option<NaiveDateTime> {
    let now = Local::now().naive_local();
    if let Some(i) = i.strip_prefix("dzisiaj ") {
        return eat(&format!("{} {}", now.date(), i));
    }
    if let Some(i) = i.strip_prefix("jutro ") {
        let tomorrow = now.date().succ_opt()?;
        return eat(&format!("{} {}", tomorrow, i));
    }
    let i = i.trim_start_matches(|c: char| {
        c.is_alphabetic() || c == '.' || c == ',' || c.is_whitespace()
    });
    let format = "%e.%m.%Y, %H:%M";
    NaiveDateTime::parse_from_str(i, format).ok()
}

pub fn eat(i: &str) -> Option<NaiveDateTime> {
    let format = "%Y-%m-%d %H:%M";
    NaiveDateTime::parse_from_str(i, format).ok()
}

pub fn to_local(x: NaiveDateTime) -> DateTime<Local> {
    Local.from_local_datetime(&x).single().unwrap()
}
