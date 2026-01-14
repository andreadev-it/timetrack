use anyhow::Result;
use chrono::{DateTime, Local};

use crate::database::running_entry;
use crate::database::write_entry;
use crate::style::style_string;
use crate::style::Styles;
use crate::Entry;
use crate::State;

pub fn start_task(task: &str, at: Option<DateTime<Local>>, switch: &bool, state: &State) -> Result<()> {
    let start = at.unwrap_or(Local::now());

    let cur_task = running_entry(&state.database, &state.current_sheet)?;

    let mut has_stopped_task = false;

    // Check if task is already active and eventually stop it
    if let Some(mut cur_task) = cur_task {
        // If you shouldn't switch, notify the user and quit
        if !switch {
            println!(
                "{} {}",
                style_string("Already checked into sheet:", Styles::Message),
                cur_task.sheet
            );
            return Ok(());
        }

        // If you can't switch, notify the user
        if cur_task.start > start {
            println!(
                "{}",
                style_string("Cannot switch to the new task, because it would stop the previous task before it was started.", Styles::Message)
            );
            return Ok(())
        }

        // Stop the previous task
        cur_task.stop(start);
        write_entry(&cur_task, &state.database)?;
        has_stopped_task = true;
    }

    let entry = Entry::start(task, &state.current_sheet, start);

    write_entry(&entry, &state.database)?;

    let message = match has_stopped_task {
        true => "Previous task stopped and checked into sheet:",
        false => "Checked into sheet:"
    };

    println!(
        "{} {}",
        style_string(message, Styles::Message),
        entry.sheet
    );

    Ok(())
}
