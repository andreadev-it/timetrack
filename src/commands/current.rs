use anyhow::Result;
use colored::Colorize;

use crate::database::current_entry;
use crate::utils::{format_duration, time_from_now};
use crate::State;

pub fn current_task(state: &State) -> Result<()> {
    let entry = current_entry(&state.database)?;

    match entry {
        None => println!("There is no active task."),
        Some(e) => {
            let elapsed = time_from_now(&e.start);
            println!("Sheet: {}", e.sheet.green());
            println!("{} ({})", e.name.cyan(), format_duration(&elapsed));
        }
    }

    Ok(())
}
