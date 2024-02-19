use anyhow::Result;
use colored::Colorize;
use langtime::parse;

use crate::commands::display::{print_task_readable, print_tasks_heading, ReadableOptions};
use crate::database::{current_entry, get_entry_by_id, update_entry};
use crate::State;

pub fn edit_task(
    id: &Option<usize>,
    start: &Option<String>,
    end: &Option<String>,
    move_to: &Option<String>,
    notes: &Option<String>,
    state: &mut State,
) -> Result<()> {
    let running_entry = current_entry(&state.database)?;

    let entry = if let Some(id) = id {
        get_entry_by_id(id, &state.database)?
    } else {
        None
    };

    if running_entry.is_none() && entry.is_none() {
        println!("The task was not found. Either the given id is invalid or there is no task running.");
        return Ok(());
    }

    // This will be fine because of the preceding if statement
    let mut entry = entry.unwrap_or_else(|| running_entry.unwrap());

    if let Some(start) = start {
        entry.start = parse(start)?;
    }

    if let Some(end) = end {
        entry.end = Some(parse(end)?);
    }

    if let Some(move_to) = move_to {
        entry.sheet = move_to.to_string();
    }

    if let Some(notes) = notes {
        entry.name = notes.to_string();
    }

    update_entry(&entry, &state.database)?;

    // Display output
    println!("{}", "Entry updated:".bold());

    let options = ReadableOptions::new();

    print_tasks_heading(&options);
    print_task_readable(&entry, true, &options);

    Ok(())
}
