use anyhow::Result;
use chrono::{DateTime, Local};

use crate::database::write_entry;
use crate::Entry;
use crate::State;

pub fn start_task(task: &str, at: Option<DateTime<Local>>, state: &State) -> Result<()> {
    let start = at.unwrap_or(Local::now());

    let entry = Entry::start(task, &state.current_sheet, start);

    write_entry(&entry, &state.database)?;

    println!("Checked into sheet: {}", entry.sheet);

    Ok(())
}
