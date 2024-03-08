use anyhow::Result;

use crate::{
    style::{style_string, Styles},
    State,
};

pub fn checkout_sheet(name: &str, state: &mut State) -> Result<()> {
    // Guard to check if I'm already on that sheet
    if state.current_sheet == name {
        println!(
            "{} {}",
            style_string("Already on sheet:", Styles::Message),
            name
        );
        return Ok(());
    }

    state.change_sheet(name)?;

    println!(
        "{} {}",
        style_string("Switched to sheet:", Styles::Message),
        name
    );

    Ok(())
}
