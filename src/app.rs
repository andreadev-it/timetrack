use chrono::{DateTime, Local};
use anyhow::Result;
use rusqlite::Connection;
use crate::database::{current_sheet, write_entry, current_entry};
use crate::entry::Entry;
use crate::state::State;
use crate::utils::{time_from_now, format_duration};

pub fn start_task(task: &str, at: Option<DateTime<Local>>, state: &State) -> Result<()> {
    let current_sheet = current_sheet(&state.database)?;

    let start = match at {
        Some(t) => t,
        None => Local::now()
    };

    let entry = Entry::start(task, &current_sheet, start);

    write_entry(&entry, &state.database)?;

    Ok(())
}

pub fn stop_task(state: &State) -> Result<()> {
    let cur = current_entry(&state.database)?;

    match cur {
        None => println!("There is no active task."),
        Some(mut e) => {
            e.stop(Local::now());
            write_entry(&e, &state.database)?;
            println!("Checked out of sheet {}", e.sheet);
        }
    };

    Ok(())
}

pub fn display_tasks(print_json: &bool, current_month_only: &bool) {
    println!("Timetrack display");
    todo!();
}

pub fn checkout_sheet(name: &str) {
    println!("Timetrack sheet {name}");
    todo!();
}

pub fn display_sheets() {
    println!("Timetrack sheet");
    todo!();
}

pub fn current_task(state: &State) -> Result<()> {
    let entry = current_entry(&state.database)?;

    match entry {
        None => println!("There is no active task."),
        Some(e) => {
            let elapsed = time_from_now(&e.start);
            println!("Sheet: {}", e.sheet);
            println!("{} ({})", e.name, format_duration(&elapsed));
        }
    }

    Ok(())
}
