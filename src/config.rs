use anyhow::{anyhow, Result};

use directories::ProjectDirs;

#[derive(Debug)]
pub struct Config {
    pub database_file: String,
    pub default_sheet: String,
}

impl Config {
    pub fn build() -> Result<Config> {
        let proj_dirs = ProjectDirs::from("com", "andreadev-it", "timetrack")
            .ok_or(anyhow!("Cannot get project directories for this OS."))?;

        // Get the data file path
        let data_dir = proj_dirs.data_local_dir();
        let mut db_file = data_dir.to_path_buf();
        db_file.push("database.db");

        if let Some(db_file_str) = db_file.to_str() {
            return Ok(Config {
                database_file: db_file_str.to_string(),
                default_sheet: "default".to_string(),
            });
        }

        Err(anyhow!("Seems like the path contains invalid unicode. Please forward this to the developer. The path was: {:?}", db_file))
    }
}
