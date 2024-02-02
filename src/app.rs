use chrono::{DateTime, Local};
use anyhow::Result;
use colored::Colorize;

use crate::database::{write_entry, current_entry, get_all_sheets, get_sheet_entries};
use crate::entry::Entry;
use crate::state::State;
use crate::utils::{time_from_now, format_duration};

pub fn start_task(task: &str, at: Option<DateTime<Local>>, state: &State) -> Result<()> {
    let start = match at {
        Some(t) => t,
        None => Local::now()
    };

    let entry = Entry::start(task, &state.current_sheet, start);

    write_entry(&entry, &state.database)?;

    println!("Checked into sheet: {}", entry.sheet);

    Ok(())
}

pub fn stop_task(at: Option<DateTime<Local>>, state: &mut State) -> Result<()> {
    let end = match at {
        Some(t) => t,
        None => Local::now()
    };

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

pub fn display_tasks(print_json: &bool, month: &Option<String>, sheet: &Option<String>, state: &State) -> Result<()> {
    let sheet = sheet.as_ref().unwrap_or(&state.current_sheet);

    let entries = get_sheet_entries(sheet, &state.database)?;

    display_tasks_stdout(sheet, &entries);

    Ok(())
}

pub fn display_tasks_stdout(sheet: &str, entries: &Vec<Entry>) {
    println!("Timesheet: {}", sheet);

    println!("Date              Start      End       Duration    Notes");
    for entry in entries {
        println!("{}", entry.formatted());
    }
}

pub fn checkout_sheet(name: &str, state: &mut State) -> Result<()> {
    state.change_sheet(name)?;

    println!("Switched to sheet {}", name);

    Ok(())
}

pub fn display_sheets(state: &State) -> Result<()> {
    println!("{}", "Timesheets:".bold());

    let sheets = get_all_sheets(&state.database)?;

    for sheet in sheets {
        if sheet == state.current_sheet {
            println!("{} (active)", sheet.green().bold());
        }
        else {
            println!("{}", sheet);
        }
    }

    Ok(())
}

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
