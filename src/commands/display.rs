use anyhow::Result;
use chrono::Duration;
use colored::Colorize;
use langtime::parse;
use serde_json::to_string_pretty;

use crate::database::get_sheet_entries;
use crate::utils::{day_begin, day_end, format_duration, is_same_day};
use crate::Entry;
use crate::State;

pub struct ReadableOptions {
    pub show_ids: bool,
    pub show_timesheet: bool,
    pub show_partial_sum: bool,
    pub show_total: bool,
    pub padding: usize,
}

impl ReadableOptions {
    pub fn new() -> Self {
        Self {
            show_ids: false,
            show_timesheet: false,
            show_partial_sum: false,
            show_total: false,
            padding: 0,
        }
    }

    pub fn complete() -> Self {
        Self {
            show_ids: true,
            show_timesheet: true,
            show_partial_sum: true,
            show_total: true,
            padding: 0,
        }
    }
}

pub fn display_tasks(
    print_json: &bool,
    sheet: Option<&String>,
    start: Option<&String>,
    end: Option<&String>,
    ids: &bool,
    state: &State,
) -> Result<()> {
    // Getting the data
    let sheet = sheet.unwrap_or(&state.current_sheet);
    let mut entries = get_sheet_entries(sheet, &state.database)?;

    // Sorting
    entries.sort_by(|a, b| a.start.cmp(&b.start));

    // Filtering
    let start = start.map(|s| parse(s)).transpose()?.map(day_begin);

    let end = end.map(|e| parse(e)).transpose()?.map(day_end);

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
    match print_json {
        true => print_all_tasks_json(&entries)?,
        false => {
            let mut options = ReadableOptions::complete();
            options.show_ids = *ids;

            print_all_tasks_readable(sheet, &entries, &options);
        }
    };

    Ok(())
}

pub fn print_all_tasks_readable(sheet: &str, entries: &Vec<Entry>, options: &ReadableOptions) {
    if options.show_timesheet {
        println!("{} {}", "Timesheet:".bold(), sheet);
    }

    print_tasks_heading(options);

    let mut prev_date = None;
    let mut day_sum = Duration::zero();
    for entry in entries {
        let mut print_date = true;
        let mut print_partial = false;
        let is_same = prev_date.is_some() && is_same_day(prev_date.unwrap(), &entry.start);

        if is_same {
            print_date = false;
        } else if prev_date.is_some() {
            print_partial = true;
        }

        prev_date = Some(&entry.start);

        if print_partial && options.show_partial_sum {
            println!("{:<47}{}", "", format_duration(&day_sum));
            day_sum = Duration::zero();
        }

        day_sum = day_sum + entry.get_duration();

        print_task_readable(entry, print_date, options);
    }

    if options.show_partial_sum {
        println!("{:<47}{}", "", format_duration(&day_sum));
    }

    let total = entries.iter().map(|e| e.get_duration()).sum();
    if options.show_total {
        println!("{}", "-".repeat(67));
        println!("{:<47}{}", "Total".bold(), format_duration(&total));
    }
}

pub fn print_tasks_heading(options: &ReadableOptions) {
    let id_label = if options.show_ids {
        format!("{:<6}", "ID")
    } else {
        " ".repeat(6)
    };
    let date_label = format!("{:<18}", "Date");
    let start_label = format!("{:<11}", "Start");
    let end_label = format!("{:<10}", "End");
    let duration_label = format!("{:<12}", "Duration");
    let notes_label = "Notes";

    let full_label = format!(
        "{}{}{}{}{}{}",
        id_label, date_label, start_label, end_label, duration_label, notes_label
    );

    let pad = " ".repeat(options.padding);

    println!("{}{}", pad, full_label.bold());
}

pub fn print_task_readable(entry: &Entry, print_date: bool, options: &ReadableOptions) {
    let end = entry
        .end
        .map(|d| d.format("%H:%M:%S").to_string())
        .unwrap_or("".to_string());

    let id = match options.show_ids {
        // I can unwrap because all tasks are taken from the db and will have ids
        true => format!("{:<6}", entry.id.unwrap()),
        false => " ".repeat(6),
    };

    let date = match print_date {
        true => entry.start.format("%a %b %d, %Y").to_string(),
        false => "".to_string(),
    };

    let pad = " ".repeat(options.padding);

    println!(
        "{}{}{:<18}  {} - {}  {:<10}  {}",
        pad,
        id,
        date,
        entry.start.format("%H:%M:%S"),
        end,
        format_duration(&entry.get_duration()),
        entry.name
    )
}

pub fn print_all_tasks_json(entries: &Vec<Entry>) -> Result<()> {
    println!("{}", to_string_pretty(entries)?);

    Ok(())
}
