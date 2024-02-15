use anyhow::Result;

use crate::State;

pub fn checkout_sheet(name: &str, state: &mut State) -> Result<()> {
    state.change_sheet(name)?;

    println!("Switched to sheet {}", name);

    Ok(())
}
