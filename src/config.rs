use core::panic;

use directories::ProjectDirs;

#[derive(Debug)]
pub struct Config {
    pub database_file: String,
    pub default_sheet: String,
}

impl Config {
    pub fn build() -> Config {
        if let Some(proj_dirs) = ProjectDirs::from("com", "andreadev-it", "timetrack") {
            let data_dir = proj_dirs.data_local_dir();

            let mut db_file = data_dir.to_path_buf();
            db_file.push("database.db");

            if let Some(db_file_str) = db_file.to_str() {
                return Config {
                    database_file: db_file_str.to_string(),
                    default_sheet: "default".to_string(),
                };
            }

            panic!("Seems like the path contains invalid unicode. Please forward this to the developer. The path was: {:?}", db_file);
        }

        panic!("Cannot get project directories for this OS.");
    }
}
