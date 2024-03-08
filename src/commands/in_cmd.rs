use anyhow::Result;
use chrono::{DateTime, Local};

use crate::database::running_entry;
use crate::database::write_entry;
use crate::style::style_string;
use crate::style::Styles;
use crate::Entry;
use crate::State;

pub fn start_task(task: &str, at: Option<DateTime<Local>>, state: &State) -> Result<()> {
    let start = at.unwrap_or(Local::now());

    let cur_task = running_entry(&state.database, &state.current_sheet)?;

    // Guard for task already active
    if cur_task.is_some() {
        println!(
            "{} {}",
            style_string("Already checked into sheet:", Styles::Message),
            cur_task.unwrap().sheet
        );
        return Ok(());
    }

    let entry = Entry::start(task, &state.current_sheet, start);

    write_entry(&entry, &state.database)?;

    println!(
        "{} {}",
        style_string("Checked into sheet:", Styles::Message),
        entry.sheet
    );

    Ok(())
}
