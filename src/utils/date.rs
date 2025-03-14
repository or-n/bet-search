use chrono::{Datelike, Duration, Local, NaiveDateTime};

pub fn eat(i: &str) -> Option<NaiveDateTime> {
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

pub fn eat2(i: &str) -> Option<NaiveDateTime> {
    let format = "%Y-%m-%d %H:%M";
    NaiveDateTime::parse_from_str(i, format).ok()
}

pub fn in_hours(date: NaiveDateTime, n: i64) -> bool {
    let now = Local::now().naive_local();
    date <= now + Duration::hours(n)
}
