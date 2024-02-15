use anyhow::Result;
use langtime::parse;
use colored::Colorize;

use crate::State;
use crate::commands::display::{print_tasks_heading, ReadableOptions, print_task_readable};
use crate::database::{get_entry_by_id, current_entry, update_entry};

pub fn edit_task(
    id: &Option<usize>,
    start: &Option<String>,
    end: &Option<String>,
    move_to: &Option<String>,
    notes: &Option<String>,
    state: &mut State
) -> Result<()> {

    let running_entry = current_entry(&state.database)?;

    let entry = if let Some(id) = id {
        get_entry_by_id(id, &state.database)?
    } else { None };

    if running_entry.is_none() && entry.is_none() {
        println!("There is no active task, and no id was given.");
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

    let options = ReadableOptions {
        show_ids: true,
        padding: 0
    };

    print_tasks_heading(&options);
    print_task_readable(&entry, &options);

    Ok(())
}
