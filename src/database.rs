use std::path::PathBuf;
use std::str::FromStr;
use rusqlite::{Connection, named_params};
use anyhow::{Result, anyhow, Context};

use crate::{entry::Entry, config::Config, utils::str_to_datetime};

pub fn connect_to_db(config: &Config) -> Result<Connection> {
    if let Ok(conn) = Connection::open(&config.database_file) {
        return Ok(conn);
    }

    Err(anyhow!("Cannot create connection to database. This is the expected path to the db: {:?}", &config.database_file))
}

pub fn ensure_db_exists(config: &Config) -> Result<()> {
    let mut db_path = PathBuf::from_str(&config.database_file).unwrap();
    db_path.pop();

    if !db_path.exists() {
        std::fs::create_dir_all(&db_path)
            .context(
                format!("Cannot create the data folder for timetrack. The expected path was: {:?}", &db_path)
            )?;
    }

    Ok(())
}

pub fn create_tables(db: &Connection) -> Result<()> {
    let query = "
    CREATE TABLE IF NOT EXISTS entries (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        note VARCHAR(255) NOT NULL,
        start TIMESTAMP NOT NULL,
        end TIMESTAMP,
        sheet VARCHAR(255) NOT NULL
    )
    ";

    db.execute(query, ()).context("Tables could not be created in the database")?;

    Ok(())
}

pub fn write_entry(entry: &Entry, db: &Connection) -> Result<()> {
    match entry.id {
        Some(_) => update_entry(entry, db)?,
        None => create_entry(entry, db)?
    };

    Ok(())
}

pub fn update_entry(entry: &Entry, db: &Connection) -> Result<()> {
    let query = "
    UPDATE entries SET
        note = :note,
        start = :start,
        end = :end,
        sheet = :sheet
    WHERE
        id = :id
    ";

    let mut stmt = db.prepare(query)?;
    stmt.execute(named_params! {
        ":note": entry.name,
        ":start": entry.start,
        ":end": entry.end,
        ":sheet": entry.sheet,
        ":id": entry.id
    })?;

    Ok(())
}

pub fn create_entry(entry: &Entry, db: &Connection) -> Result<()> {
    let query = "
    INSERT INTO entries (note, start, end, sheet) VALUES (
        :note, :start, :end, :sheet
    )
    ";

    let mut stmt = db.prepare(query)?;
    stmt.execute(named_params! {
        ":note": entry.name,
        ":start": entry.start,
        ":end": entry.end,
        ":sheet": entry.sheet
    })?;

    Ok(())
}

pub fn current_entry(db: &Connection) -> Result<Option<Entry>> {
    let query = "
    SELECT id, note, start, end, sheet FROM entries WHERE end IS NULL;
    ";

    let mut stmt = db.prepare(query)?;
    let mut entries = stmt.query_map([], |row| {
        let end = row.get::<usize, Option<String>>(3)?
            .map(|t| str_to_datetime(&t).unwrap());

        let start = str_to_datetime(&row.get::<usize, String>(2)?)
            .unwrap();

        Ok(Entry {
            id: row.get(0)?,
            name: row.get(1)?,
            start,
            end,
            sheet: row.get(4)?
        })
    })?;

    let running_entry = entries.next();

    Option::transpose(running_entry).context("Error while parsing the current entry")
}

pub fn current_sheet(db: &Connection) -> Result<String> {
    Ok("default".to_string())
}
