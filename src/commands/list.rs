use anyhow::Result;

use crate::database::get_all_sheets;
use crate::State;
use crate::style::{style_string, Styles};

pub fn list_sheets(state: &State) -> Result<()> {
    let mut sheets = get_all_sheets(&state.database)?;

    if sheets.is_empty() {
        sheets.push(state.current_sheet.to_string());
    }

    println!("{}", style_string("Timesheets:", Styles::Title));

    for sheet in sheets {
        if sheet == state.current_sheet {
            println!("{} {}", sheet, style_string("(active)", Styles::Primary));
        } else if sheet == state.last_sheet {
            println!("{} {}", sheet, style_string("(last)", Styles::Secondary));
        } else {
            println!("{}", sheet);
        }
    }

    Ok(())
}
