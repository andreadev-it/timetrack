use std::fs;
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

            let content_res = fs::read_to_string(&data_file);

            match content_res {
                Ok(content) => {
                    // read the data file
                    let mut lines = content.lines();

                    if let Some(s) = lines.next() {
                        sheet = s.to_string();
                    }

                    last_task = lines
                        .next()
                        .and_then(|id| id.parse().ok());
                },
                Err(_) => {
                    // write the default file
                    fs::write(&data_file, format!("{}\n", &sheet))?;
                }

            }

            return Ok(State {
                current_sheet: sheet,
                last_task,
                database: db
            });
        }

        panic!("Cannot get project directories for this OS.");
    }

    pub fn change_sheet(&mut self, sheet: &str) -> Result<()> {
        self.current_sheet = sheet.to_string();
        
        self.update_file()?;

        Ok(())
    }

    pub fn set_last_task(&mut self, task_id: usize) -> Result<()> {
        self.last_task = Some(task_id);

        self.update_file()?;

        Ok(())
    }

    fn update_file(&mut self) -> Result<()> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "andreadev-it", "timetrack") {
            let data_dir = proj_dirs.data_local_dir();

            let mut data_file = data_dir.to_path_buf();
            data_file.push("data.txt");

            let last_task = self.last_task  
                .and_then(|t| Some(t.to_string()))
                .unwrap_or("".to_string());

            fs::write(
                &data_file,
                format!(
                    "{}\n{}",
                    self.current_sheet,
                    last_task
                ),
            )?;
        }

        Ok(())
    }
}
