use anyhow::Result;
use colored::Colorize;
use langtime::parse;

use crate::Entry;
use crate::State;
use crate::database::get_sheet_entries;

pub fn display_tasks(
    print_json: &bool,
    sheet: Option<&String>,
    start: Option<&String>,
    end: Option<&String>,
    ids: &bool,
    state: &State
) -> Result<()> {
    // Getting the data
    let sheet = sheet.unwrap_or(&state.current_sheet);
    let mut entries = get_sheet_entries(sheet, &state.database)?;

    // Sorting
    entries.sort_by(|a, b| a.start.cmp(&b.start));

    // Filtering
    let start = start.and_then(|s| Some(parse(s)))
        .transpose()?;
    let end = end.and_then(|e| Some(parse(e)))
        .transpose()?;

    let entries: Vec<Entry> = entries.into_iter().filter(|e| {
        if start.is_some() {
            if e.start < start.unwrap() {
                return false;
            }
        }

        if end.is_some() {
            if e.start > end.unwrap() {
                return false;
            }
        }

        true
    }).collect();

    // Displaying
    display_tasks_stdout(sheet, &entries, ids);

    Ok(())
}

fn display_tasks_stdout(sheet: &str, entries: &Vec<Entry>, show_ids: &bool) {
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
