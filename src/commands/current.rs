use anyhow::Result;

use crate::database::running_entries;
use crate::style::{style_string, Styles};
use crate::utils::{format_duration, time_from_now};
use crate::State;

pub fn current_task(state: &State) -> Result<()> {
    let mut entries = running_entries(&state.database)?;

    entries.sort_unstable();

    println!(
        "{}{}",
        style_string("Currently on sheet: ", Styles::Title),
        style_string(&state.current_sheet, Styles::Primary)
    );

    // Early return if there is no active task
    if entries.is_empty() {
        println!(
            "{}",
            style_string("There is no active task.", Styles::Message)
        );

        return Ok(());
    }

    // There will always be only one active entry for each sheet
    println!("{}", style_string("Active tasks:", Styles::Title));
    for entry in &entries {
        println!(
            "{}: {} ({})",
            style_string(&entry.sheet, Styles::Primary),
            style_string(&entry.name, Styles::Secondary),
            format_duration(&time_from_now(&entry.start))
        );
    }

    Ok(())
}
