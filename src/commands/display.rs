use anyhow::Result;
use colored::Colorize;
use langtime::parse;

use crate::Entry;
use crate::State;
use crate::database::get_sheet_entries;
use crate::utils::{day_begin, day_end, format_duration};

pub struct ReadableOptions {
    pub show_ids: bool,
    pub padding: usize
}


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
    let start = start
        .map(|s| parse(s))
        .transpose()?
        .map(day_begin);

    let end = end
        .map(|e| parse(e))
        .transpose()?
        .map(day_end);

    entries.retain(|e| {
        if start.is_some() && e.start < start.unwrap() {
            return false;
        }

        if end.is_some() && e.start > end.unwrap() {
            return false;
        }

        true
    });

    // Displaying
    let options = ReadableOptions {
        show_ids: *ids,
        padding: 4
    };
    print_all_tasks_readable(sheet, &entries, &options);

    Ok(())
}

pub fn print_all_tasks_readable(sheet: &str, entries: &Vec<Entry>, options: &ReadableOptions) {
    println!("{} {}", "Timesheet:".bold(), sheet);

    print_tasks_heading(options);

    for entry in entries {
        print_task_readable(entry, options);
    }
}

pub fn print_tasks_heading(options: &ReadableOptions) {
    let id_label = if options.show_ids { format!("{:<6}", "ID") } else { "".to_string() };
    let date_label = format!("{:<18}", "Date");
    let start_label = format!("{:<11}", "Start");
    let end_label = format!("{:<10}", "End");
    let duration_label = format!("{:<12}", "Duration");
    let notes_label = "Notes";

    let full_label = format!("{}{}{}{}{}{}",
        id_label,
        date_label,
        start_label,
        end_label,
        duration_label,
        notes_label
    );

    let pad = " ".repeat(options.padding);

    println!("{}{}", pad, full_label.bold());
}

pub fn print_task_readable(entry: &Entry, options: &ReadableOptions) {
    let end = entry.end
        .map(|d| d.format("%H:%M:%S").to_string())
        .unwrap_or("".to_string());

    let id = if options.show_ids {
        // I can unwrap because all tasks are taken from the db and will have ids
        format!("{:<6}", entry.id.unwrap())
    } else {
        "".to_string()
    };

    let pad = " ".repeat(options.padding);

    println!(
        "{}{}{}  {} - {}  {:<10}  {}",
        pad,
        id,
        entry.start.format("%a %b %d, %Y"),
        entry.start.format("%H:%M:%S"),
        end,
        format_duration(&entry.get_duration()),
        entry.name
    )
}
