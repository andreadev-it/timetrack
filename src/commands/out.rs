use anyhow::Result;
use chrono::{DateTime, Local};

use crate::database::{running_entry, write_entry};
use crate::style::{style_string, Styles};
use crate::State;

pub fn stop_task(at: Option<DateTime<Local>>, state: &mut State) -> Result<()> {
    let end = at.unwrap_or(Local::now());

    let cur = running_entry(&state.database, &state.current_sheet)?;

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

            println!(
                "{} {}",
                style_string("Checked out of sheet:", Styles::Message),
                e.sheet
            );
        }
    };

    Ok(())
}
