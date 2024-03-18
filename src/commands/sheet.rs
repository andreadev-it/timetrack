use anyhow::Result;

use crate::{
    database::update_sheet_name,
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

pub fn rename_sheet(name: &str, new_name: &str, state: &mut State) -> Result<()> {
    update_sheet_name(name, new_name, &state.database)?;

    if state.current_sheet == name {
        state.update_sheet_name(new_name)?;
    }

    println!(
        "{}",
        style_string("Sheet renamed succesfully.", Styles::Message)
    );

    Ok(())
}
