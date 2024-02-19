use anyhow::Result;
use chrono::{DateTime, Local};

use crate::database::{current_entry, write_entry};
use crate::State;
use crate::style::{style_string, Styles};

pub fn stop_task(at: Option<DateTime<Local>>, state: &mut State) -> Result<()> {
    let end = at.unwrap_or(Local::now());

    let cur = current_entry(&state.database)?;

    match cur {
        None => println!(
            "{}",
            style_string("There is no active task.", Styles::Message)
        ),
        Some(mut e) => {
            // Check if the end is before the start
            if e.start > end {
                println!(
                    "{}",
                    style_string("Cannot stop a task before it started.", Styles::Message)
                );
                return Ok(());
            }

            e.stop(end);
            write_entry(&e, &state.database)?;

            // This is safe to unwrap because the entry is
            // taken from the db, which means it has an id.
            state.set_last_task(e.id.unwrap())?;

            println!(
                "{} {}",
                style_string("Checked out of sheet:", Styles::Message),
                e.sheet
            );
        }
    };

    Ok(())
}
