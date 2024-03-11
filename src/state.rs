use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use rusqlite::Connection;
use std::fs;

use crate::config::Config;
use crate::database::connect_to_db;

#[derive(Debug)]
pub struct State {
    pub current_sheet: String,
    pub last_sheet: String,
    pub database: Connection,
}

impl State {
    pub fn build(config: &Config) -> Result<State> {
        let db = connect_to_db(config)?;

        let proj_dirs = ProjectDirs::from("com", "andreadev-it", "timetrack")
            .ok_or(anyhow!("Cannot get project directories for this OS."))?;

        // Prepare the default state
        let mut state = State {
            current_sheet: "default".to_string(),
            last_sheet: "default".to_string(),
            database: db,
        };

        // Get the data file path
        let data_dir = proj_dirs.data_local_dir();
        let mut data_file = data_dir.to_path_buf();
        data_file.push("data.txt");

        // Extract the data from the file
        let content_res = fs::read_to_string(&data_file);

        match content_res {
            Ok(content) => {
                let mut lines = content.lines();

                if let Some(s) = lines.next() {
                    state.current_sheet = s.to_string();
                }

                if let Some(s) = lines.next() {
                    state.last_sheet = s.to_string();
                }
            }
            Err(_) => {
                // write the default file
                state
                    .update_file()
                    .context("Cannot write the default state file.")?;
            }
        }

        Ok(state)
    }

    pub fn change_sheet(&mut self, sheet: &str) -> Result<()> {
        self.last_sheet = self.current_sheet.clone();
        self.current_sheet = sheet.to_string();

        self.update_file()?;

        Ok(())
    }

    fn update_file(&mut self) -> Result<()> {
        let proj_dirs = ProjectDirs::from("com", "andreadev-it", "timetrack")
            .ok_or(anyhow!("Cannot get project directories for this OS."))?;

        let data_dir = proj_dirs.data_local_dir();

        let mut data_file = data_dir.to_path_buf();
        data_file.push("data.txt");

        fs::write(
            &data_file,
            format!("{}\n{}", self.current_sheet, self.last_sheet),
        )?;

        Ok(())
    }
}
