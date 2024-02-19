use anyhow::Result;
use colored::Colorize;

use crate::database::{remove_entries_by_sheet, remove_entry_by_id, get_all_sheets};
use crate::State;

pub fn kill_task(id: &usize, state: &mut State) -> Result<()> {
    remove_entry_by_id(id, &state.database)?;

    println!("{} {}", "Removed entry:".bold(), id);

    Ok(())
}

pub fn kill_sheet(sheet: &str, state: &mut State) -> Result<()> {
    let sheets = get_all_sheets(&state.database)?;

    // Guard for non-existent sheets
    if !sheets.iter().any(|s| s == sheet) {
        println!("{} {}", "Sheet not found:".bold(), sheet);
        return Ok(());
    }

    remove_entries_by_sheet(sheet, &state.database)?;
    
    if state.current_sheet == sheet {
        move_to_last_sheet(state)?;
    }

    println!("{} {}", "Removed sheet:".bold(), sheet);

    Ok(())
}

fn move_to_last_sheet(state: &mut State) -> Result<()> {
    // If there is a last sheet, move to it, otherwise move
    // to the first sheet available, or "default" if none exist.
    let sheets = get_all_sheets(&state.database)?;
    let last_sheet_exists = sheets
        .iter()
        .any(|s| s == &state.last_sheet);

    match last_sheet_exists {
        true => state.change_sheet(&state.last_sheet.clone())?,
        false => {
            match !sheets.is_empty() {
                true => state.change_sheet(&sheets[0])?,
                false => state.change_sheet("default")?
            };
        }
    };

    Ok(())
}
