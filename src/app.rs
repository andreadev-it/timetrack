use chrono::{DateTime, Local};
use anyhow::Result;
use colored::Colorize;
use langtime::parse;

use crate::database::{write_entry, current_entry, get_all_sheets, get_sheet_entries, remove_entry_by_id, remove_entries_by_sheet, get_entry_by_id, update_entry};
use crate::entry::Entry;
use crate::state::State;
use crate::utils::{time_from_now, format_duration};

pub fn start_task(task: &str, at: Option<DateTime<Local>>, state: &State) -> Result<()> {
    let start = at.unwrap_or(Local::now());

    let entry = Entry::start(task, &state.current_sheet, start);

    write_entry(&entry, &state.database)?;

    println!("Checked into sheet: {}", entry.sheet);

    Ok(())
}

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

pub fn display_tasks(print_json: &bool, month: &Option<String>, sheet: &Option<String>, state: &State, ids: &bool) -> Result<()> {
    let sheet = sheet.as_ref().unwrap_or(&state.current_sheet);

    let entries = get_sheet_entries(sheet, &state.database)?;

    display_tasks_stdout(sheet, &entries, ids);

    Ok(())
}

pub fn display_tasks_stdout(sheet: &str, entries: &Vec<Entry>, show_ids: &bool) {
    println!("{} {}", "Timesheet:".bold(), sheet);

    let id_label = if *show_ids { format!("{:<6}", "ID") } else { "".to_string() };
    let date_label = format!("{:<18}", "Date");
    let start_label = format!("{:<11}", "Start");
    let end_label = format!("{:<10}", "End");
    let duration_label = format!("{:<12}", "Duration");
    let notes_label = format!("Notes");

    let full_label = format!("{}{}{}{}{}{}",
        id_label,
        date_label,
        start_label,
        end_label,
        duration_label,
        notes_label
    );

    println!("    {}", full_label.bold());
    for entry in entries {
        println!("    {}", entry.formatted(show_ids));
    }
}

pub fn kill_task(id: &usize, state: &mut State) -> Result<()> {
    remove_entry_by_id(id, &state.database)?;

    println!("{} {}", "Removed entry:".bold(), id);

    Ok(())
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
            println!("{} {}", sheet, "(active)".green().bold());
        }
        else {
            println!("{}", sheet);
        }
    }

    Ok(())
}

pub fn kill_sheet(sheet: &str, state: &mut State) -> Result<()> {
    remove_entries_by_sheet(sheet, &state.database)?;

    println!("{} {}", "Removed sheet:".bold(), sheet);

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

pub fn edit_task(
    id: &Option<usize>,
    start: &Option<String>,
    end: &Option<String>,
    move_to: &Option<String>,
    notes: &Option<String>,
    state: &State
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

    println!("{}", "Entry updated:".bold());
    println!("{}", entry.formatted(&false));

    Ok(())
}
