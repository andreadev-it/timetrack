use chrono::{DateTime, Local, Duration};

use crate::config::Config;
use crate::utils::format_duration;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    pub id: Option<usize>,
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
    pub name: String,
    pub sheet: String
}

impl Entry {
    pub fn new(config: Config) -> Self {
        Entry {
            id: None,
            start: Local::now(),
            end: None,
            name: "".to_string(),
            sheet: config.default_sheet.clone()
        }
    }

    pub fn start(name: &str, sheet: &str, start: DateTime<Local>) -> Self {
        Entry {
            id: None,
            start,
            end: None,
            name: name.to_string(),
            sheet: sheet.to_string()
        }
    }

    pub fn stop(&mut self, end: DateTime<Local>) {
        self.end = Some(end);
    }

    pub fn get_duration(&self) -> Duration {
        let end = self.end.unwrap_or(Local::now());

        end - self.start
    }

    pub fn formatted(&self, show_id: &bool) -> String {
        let end = self.end.unwrap_or(Local::now());

        let id = if *show_id {
            // I can unwrap because all tasks are taken from the db and will have ids
            format!("{:<6}", self.id.unwrap())
        } else {
            "".to_string()
        };

        format!(
            "{}{}  {} - {}  {:<10}  {}",
            id,
            self.start.format("%a %b %d, %Y"),
            self.start.format("%H:%M:%S"),
            end.format("%H:%M:%S"),
            format_duration(&self.get_duration()),
            self.name
        )
    }
}
