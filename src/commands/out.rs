use anyhow::Result;
use chrono::{DateTime, Local};

use crate::State;
use crate::database::{current_entry, write_entry};

pub fn stop_task(at: Option<DateTime<Local>>, state: &mut State) -> Result<()> {
    let end = at.unwrap_or(Local::now());

    let cur = current_entry(&state.database)?;

    match cur {
        None => println!("There is no active task on sheet {}.", &state.current_sheet),
        Some(mut e) => {
            // Check if the end is before the start
            if e.start > end {
                println!("Cannot stop a task before it started.");
                return Ok(());
            }

            e.stop(end);
            write_entry(&e, &state.database)?;

            // This is safe to unwrap because the entry is
            // taken from the db, which means it has an id.
            state.set_last_task(e.id.unwrap())?;

            println!("Checked out of sheet {}", e.sheet);
        }
    };

    Ok(())
}
