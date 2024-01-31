use std::{fs::File, io::Read};
use directories::ProjectDirs;
use rusqlite::Connection;
use anyhow::Result;

use crate::database::connect_to_db;
use crate::config::Config;

#[derive(Debug)]
pub struct State {
    pub current_sheet: String,
    pub last_task: Option<usize>,
    pub database: Connection
}

impl State {
    pub fn build(config: &Config) -> Result<State> {
        let db = connect_to_db(config)?;

        if let Some(proj_dirs) = ProjectDirs::from("com", "andreadev-it", "timetrack") {
            let data_dir = proj_dirs.data_local_dir();

            let mut data_file = data_dir.to_path_buf();
            data_file.push("data.txt");

            let mut sheet = "default".to_string();
            let mut last_task: Option<usize> = None;

            if let Ok(mut file) = File::open(data_file) {
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                let mut lines = content.lines();

                if let Some(s) = lines.next() {
                    sheet = s.to_string();
                }

                last_task = lines
                    .next()
                    .and_then(|id| id.parse().ok());

            }

            return Ok(State {
                current_sheet: sheet,
                last_task,
                database: db
            });
        }

        panic!("Cannot get project directories for this OS.");
    }
}
