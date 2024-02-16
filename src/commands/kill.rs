use anyhow::Result;
use colored::Colorize;

use crate::database::{remove_entries_by_sheet, remove_entry_by_id};
use crate::State;

pub fn kill_task(id: &usize, state: &mut State) -> Result<()> {
    remove_entry_by_id(id, &state.database)?;

    println!("{} {}", "Removed entry:".bold(), id);

    Ok(())
}

pub fn kill_sheet(sheet: &str, state: &mut State) -> Result<()> {
    remove_entries_by_sheet(sheet, &state.database)?;

    println!("{} {}", "Removed sheet:".bold(), sheet);

    Ok(())
}
