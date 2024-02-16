use anyhow::Result;
use chrono::{DateTime, Datelike, Duration, Local, LocalResult, NaiveDateTime, TimeZone};

pub fn str_to_datetime(s: &str) -> Result<DateTime<Local>> {
    let no_tz = s.replace("+00:00", "");
    let dt = NaiveDateTime::parse_from_str(&no_tz, "%Y-%m-%d %H:%M:%S%.f")?;

    Ok(Local.from_utc_datetime(&dt))
}

pub fn time_from_now(dt: &DateTime<Local>) -> Duration {
    let now = Local::now();

    now - dt
}

pub fn format_duration(d: &Duration) -> String {
    let duration_in_seconds = d.num_seconds();
    let hours = duration_in_seconds / 60 / 60;
    let minutes = (duration_in_seconds % 3600) / 60;
    let seconds = duration_in_seconds % 60;

    format!("{}:{:0>2}:{:0>2}", hours, minutes, seconds)
}

pub fn get_month_boundaries(month: &str) -> Result<(DateTime<Local>, DateTime<Local>)> {
    let start = get_month_from_string(month)?;
    let end = get_last_day_of_month(start)?;

    Ok((start, end))
}

// The month is written as 2024-01
pub fn get_month_from_string(month_str: &str) -> Result<DateTime<Local>> {
    let year = month_str.split('-').next().unwrap().parse::<i32>().unwrap();
    let month = month_str.split('-').nth(1).unwrap().parse::<u32>().unwrap();

    let res = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0);

    match res {
        chrono::LocalResult::None => Err(anyhow::anyhow!("Invalid month")),
        chrono::LocalResult::Single(dt) => Ok(dt),
        chrono::LocalResult::Ambiguous(_, _) => Err(anyhow::anyhow!("Ambiguous month")),
    }
}

pub fn get_first_day_of_month(dt: DateTime<Local>) -> Result<DateTime<Local>> {
    let res = Local.with_ymd_and_hms(dt.year(), dt.month(), 1, 0, 0, 0);

    match res {
        LocalResult::None => Err(anyhow::anyhow!("Invalid month")),
        LocalResult::Single(dt) => Ok(dt),
        LocalResult::Ambiguous(_, _) => Err(anyhow::anyhow!("Ambiguous month")),
    }
}

pub fn get_last_day_of_month(dt: DateTime<Local>) -> Result<DateTime<Local>> {
    let mut month = dt.month() + 1;
    let mut year = dt.year();

    if month > 12 {
        month = 1;
        year += 1;
    }

    let res = Local.with_ymd_and_hms(year, month, 1, 0, 0, 0);

    match res {
        LocalResult::None => Err(anyhow::anyhow!("Invalid month")),
        LocalResult::Single(dt) => Ok(dt - Duration::days(1)),
        LocalResult::Ambiguous(_, _) => Err(anyhow::anyhow!("Ambiguous month")),
    }
}

pub fn day_begin(dt: DateTime<Local>) -> DateTime<Local> {
    Local
        .with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 0, 0, 0)
        .unwrap()
}

pub fn day_end(dt: DateTime<Local>) -> DateTime<Local> {
    Local
        .with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 23, 59, 59)
        .unwrap()
}

pub fn is_same_day(dt1: &DateTime<Local>, dt2: &DateTime<Local>) -> bool {
    dt1.year() == dt2.year() && dt1.month() == dt2.month() && dt1.day() == dt2.day()
}

mod tests {
    use super::*;

    #[test]
    fn test_get_last_day_of_month() {
        // Normal month
        let dt = Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let res = get_last_day_of_month(dt).unwrap();

        assert!(res.year() == 2024 && res.month() == 1 && res.day() == 31);

        // Leap year
        let dt = Local.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap();
        let res = get_last_day_of_month(dt).unwrap();

        assert!(res.year() == 2024 && res.month() == 2 && res.day() == 29);

        // December (year boundary)
        let dt = Local.with_ymd_and_hms(2024, 12, 1, 0, 0, 0).unwrap();
        let res = get_last_day_of_month(dt).unwrap();

        assert!(res.year() == 2024 && res.month() == 12 && res.day() == 31);

        // Non-leap February
        let dt = Local.with_ymd_and_hms(2023, 2, 1, 0, 0, 0).unwrap();
        let res = get_last_day_of_month(dt).unwrap();

        assert!(res.year() == 2023 && res.month() == 2 && res.day() == 28);
    }
}
