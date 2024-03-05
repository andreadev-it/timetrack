use anyhow::Result;
use chrono::{Duration, DateTime, Local};
use langtime::parse;
use serde_json::to_string_pretty;
use tabled::builder::Builder;
use tabled::settings::object::Rows;
use tabled::settings::{Style, Color, Padding, Border};
use colored::Colorize;
use tabled::settings::themes::Colorization;

use crate::database::get_sheet_entries;
use crate::style::{style_string, Styles};
use crate::utils::{day_begin, day_end, format_duration, is_same_day};
use crate::Entry;
use crate::State;

pub struct ReadableOptions {
    pub show_ids: bool,
    pub show_timesheet: bool,
    pub show_partial_sum: bool,
    pub show_total: bool,
    pub show_headings: bool,
    pub padding: usize,
}

impl ReadableOptions {
    pub fn new() -> Self {
        Self {
            show_ids: false,
            show_timesheet: false,
            show_partial_sum: false,
            show_total: false,
            show_headings: false,
            padding: 0,
        }
    }

    pub fn complete() -> Self {
        Self {
            show_ids: true,
            show_timesheet: true,
            show_partial_sum: true,
            show_total: true,
            show_headings: true,
            padding: 0,
        }
    }
}

pub fn display_tasks(
    print_json: &bool,
    sheet: Option<&String>,
    start: Option<DateTime<Local>>,
    end: Option<DateTime<Local>>,
    filter_by_date: &bool,
    ids: &bool,
    state: &State,
) -> Result<()> {
    // Getting the data
    let sheet = sheet.unwrap_or(&state.current_sheet);
    let mut entries = get_sheet_entries(sheet, &state.database)?;

    if entries.is_empty() {
        println!(
            "{} {}",
            style_string("No sheet found with name:", Styles::Message),
            sheet
        );
        return Ok(())
    }

    // Sorting
    entries.sort_by(|a, b| a.start.cmp(&b.start));

    // Filtering
    let mut start = start;
    let mut end = end;

    if *filter_by_date {
        start = start.map(day_begin);
        end = end.map(day_end);
    }

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
        println!(
            "{} {}",
            style_string("Timesheet:", Styles::Title),
            sheet
        );
    }

    let mut builder = Builder::new();

    if options.show_headings {
        let h_id = match options.show_ids {
            true => "ID",
            false => ""
        };
        
        let headings = vec![h_id, "Date", "Start", "End", "Duration", "Task"];
        builder.push_record(headings);
    }

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
            builder.push_record(vec!["", "", "", "", &format_duration(&day_sum)]);
            day_sum = Duration::zero();
        }

        day_sum = day_sum + entry.get_duration();

        let id = match options.show_ids {
            true => entry.id.unwrap().to_string(),
            false => "".to_string()
        };

        let date = match print_date {
            true => entry.start.format("%a %b %d, %Y").to_string(),
            false => "".to_string()
        };

        let start = entry.start.format("%H:%M:%S").to_string();

        let end = match entry.end {
            Some(d) => d.format("%H:%M:%S").to_string(),
            None => "".to_string(),
        };

        builder.push_record(vec![
            &id,
            &date,
            &start,
            &end,
            &format_duration(&entry.get_duration()),
            &entry.name
        ]);
    }

    if options.show_partial_sum {
        builder.push_record(vec!["", "", "", "", &format_duration(&day_sum)]);
    }

    let total = entries.iter().map(|e| e.get_duration()).sum();
    if options.show_total {
        builder.push_record(vec!["Total", "", "", "", &format_duration(&total)]);
    }

    let mut table = builder.build();
    table.with(Style::empty());
    table.with(Padding::new(2, 2, 0, 0));

    if options.show_headings {
        table.with(Colorization::exact([Color::BOLD], Rows::first()));
    }

    if options.show_total {
        table.with(Colorization::exact([Color::BOLD], Rows::last()));
        table.modify(Rows::last(), Border::new().set_top('-'));
    }

    println!("{}", table);
}

pub fn print_all_tasks_json(entries: &Vec<Entry>) -> Result<()> {
    println!("{}", to_string_pretty(entries)?);

    Ok(())
}
