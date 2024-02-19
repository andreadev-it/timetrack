use anyhow::Result;
use colored::Colorize;

use crate::database::get_all_sheets;
use crate::State;

pub fn list_sheets(state: &State) -> Result<()> {
    let mut sheets = get_all_sheets(&state.database)?;

    if sheets.is_empty() {
        sheets.push(state.current_sheet.to_string());
    }

    println!("{}", "Timesheets:".bold());

    for sheet in sheets {
        if sheet == state.current_sheet {
            println!("{} {}", sheet, "(active)".green().bold());
        } else if sheet == state.last_sheet {
            println!("{} {}", sheet, "(last)".blue());
        } else {
            println!("{}", sheet);
        }
    }

    Ok(())
}
