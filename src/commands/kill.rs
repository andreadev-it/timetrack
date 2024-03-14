use anyhow::Result;

use crate::database::{
    get_all_sheets, get_entry_by_id, remove_entries_by_sheet, remove_entry_by_id,
};
use crate::style::{style_string, Styles};
use crate::utils::confirm_action;
use crate::State;

pub fn kill_task(id: &usize, state: &mut State) -> Result<()> {
    let entry = get_entry_by_id(id, &state.database)?;

    // Guard for non-existent entries
    if entry.is_none() {
        println!(
            "{} {}",
            style_string("Entry not found. Id:", Styles::Message),
            id
        );
        return Ok(());
    }

    let entry = entry.unwrap();

    if !confirm_action(&format!(
        "Are you sure you want to remove entry {}?",
        entry.name
    )) {
        return Ok(());
    }

    remove_entry_by_id(id, &state.database)?;

    println!("{} {}", style_string("Removed entry:", Styles::Message), id);

    Ok(())
}

pub fn kill_sheet(sheet: &str, state: &mut State) -> Result<()> {
    let sheets = get_all_sheets(&state.database)?;

    // Guard for non-existent sheets
    if !sheets.iter().any(|s| s == sheet) {
        println!(
            "{} {}",
            style_string("Sheet not found:", Styles::Message),
            sheet
        );
        return Ok(());
    }

    if !confirm_action(&format!("Are you sure you want to remove sheet {}?", sheet)) {
        return Ok(());
    }

    remove_entries_by_sheet(sheet, &state.database)?;

    // Check edge cases for sheets that are in use
    if state.current_sheet == sheet {
        move_to_last_sheet(state)?;
    } else if state.last_sheet == sheet {
        state.last_sheet = "default".to_string()
    }

    println!(
        "{} {}",
        style_string("Removed sheet:", Styles::Message),
        sheet
    );

    Ok(())
}

fn move_to_last_sheet(state: &mut State) -> Result<()> {
    // If there is a last sheet, move to it, otherwise move
    // to the first sheet available, or "default" if none exist.
    let sheets = get_all_sheets(&state.database)?;
    let last_sheet_exists = sheets.iter().any(|s| s == &state.last_sheet);

    match last_sheet_exists {
        true => state.change_sheet(&state.last_sheet.clone())?,
        false => {
            match !sheets.is_empty() {
                true => state.change_sheet(&sheets[0])?,
                false => state.change_sheet("default")?,
            };
        }
    };

    Ok(())
}
