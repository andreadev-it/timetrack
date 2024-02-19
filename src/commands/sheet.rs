use anyhow::Result;
use colored::Colorize;

use crate::State;

pub fn checkout_sheet(name: &str, state: &mut State) -> Result<()> {

    // Guard to check if I'm already on that sheet
    if state.current_sheet == name {
        println!("{} {}", "Already on sheet:".bold(), name);
        return Ok(());
    }

    state.change_sheet(name)?;

    println!("{} {}","Switched to sheet:".bold(), name);

    Ok(())
}
