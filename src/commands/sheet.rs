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

    // Change to the previous sheet if the name is "-".
    let name = if name == "-" {
        state.last_sheet.clone()
    } else {
        name.to_string()
    };

    state.change_sheet(&name)?;

    println!(
        "{} {}",
        style_string("Switched to sheet:", Styles::Message),
        name
    );

    Ok(())
}
