use anyhow::Result;
use colored::Colorize;

use crate::State;
use crate::database::get_all_sheets;


pub fn list_sheets(state: &State) -> Result<()> {
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
