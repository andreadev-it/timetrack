use chrono::{DateTime, Local};

use crate::config::Config;

#[derive(Debug, PartialEq, Eq)]
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
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct StringEntry {
//     pub id: Option<usize>,
//     pub start: String,
//     pub end: Option<String>,
//     pub name: String,
//     pub sheet: String
// }
