use chrono::{DateTime, Duration, Local};
use serde::Serialize;

use crate::config::Config;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, PartialOrd, Ord)]
pub struct Entry {
    pub id: Option<usize>,
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
    pub name: String,
    pub sheet: String,
}

impl Entry {
    pub fn new(config: Config) -> Self {
        Entry {
            id: None,
            start: Local::now(),
            end: None,
            name: "".to_string(),
            sheet: config.default_sheet.clone(),
        }
    }

    pub fn start(name: &str, sheet: &str, start: DateTime<Local>) -> Self {
        Entry {
            id: None,
            start,
            end: None,
            name: name.to_string(),
            sheet: sheet.to_string(),
        }
    }

    pub fn stop(&mut self, end: DateTime<Local>) {
        self.end = Some(end);
    }

    pub fn get_duration(&self) -> Duration {
        let end = self.end.unwrap_or(Local::now());

        end - self.start
    }
}
