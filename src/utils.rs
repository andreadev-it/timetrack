use chrono::{NaiveDateTime, DateTime, Local, TimeZone, Duration};
use anyhow::Result;

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

    format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
}
