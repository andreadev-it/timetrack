use anyhow::Result;
use chrono::{Duration, Local};
use tabled::builder::Builder;
use tabled::settings::object::Rows;
use tabled::settings::themes::Colorization;
use tabled::settings::{Color, Style};

use crate::database::{get_all_sheets, get_sheet_entries};
use crate::style::{style_string, Styles};
use crate::utils::{format_duration, time_from_now};
use crate::State;

pub fn list_sheets(state: &State) -> Result<()> {
    let mut sheets = get_all_sheets(&state.database)?;

    if sheets.is_empty() {
        sheets.push(state.current_sheet.to_string());
    }

    let mut builder = Builder::new();

    println!("{}", style_string("Timesheets:", Styles::Title));

    builder.push_record(vec!["Name", "Running", "Today", "Total time"]);

    for sheet in sheets {
        let entries = get_sheet_entries(&sheet, &state.database)?;

        let running = entries.iter().find(|e| e.end.is_none());
        let running_time = match running {
            Some(e) => time_from_now(&e.start),
            None => Duration::seconds(0),
        };

        let today_total: Duration = entries
            .iter()
            .filter(|e| e.start.date_naive() == Local::now().date_naive())
            .map(|e| e.end.unwrap_or(Local::now()) - e.start)
            .sum();

        let total: Duration = entries
            .iter()
            .map(|e| e.end.unwrap_or(Local::now()) - e.start)
            .sum();

        let s = if sheet == state.current_sheet {
            format!("{}{}", "*", sheet)
        } else if sheet == state.last_sheet {
            format!("{}{}", "-", sheet)
        } else {
            format!("{}", sheet)
        };

        builder.push_record(vec![
            s,
            format_duration(&running_time),
            format_duration(&today_total),
            format_duration(&total),
        ]);
    }

    let mut table = builder.build();
    table.with(Style::empty());
    table.with(Colorization::exact([Color::BOLD], Rows::first()));

    println!("{}", table);

    Ok(())
}
