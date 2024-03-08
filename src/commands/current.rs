use anyhow::Result;

use crate::database::running_entry;
use crate::style::{style_string, Styles};
use crate::utils::{format_duration, time_from_now};
use crate::State;

pub fn current_task(state: &State) -> Result<()> {
    let entry = running_entry(&state.database, &state.current_sheet)?;

    match entry {
        None => {
            println!(
                "{} {}",
                style_string("On sheet:", Styles::Title),
                style_string(&state.current_sheet, Styles::Primary)
            );

            println!(
                "{}",
                style_string("There is no active task.", Styles::Message)
            )
        }
        Some(e) => {
            let elapsed = time_from_now(&e.start);

            println!(
                "{} {}",
                style_string("Sheet:", Styles::Title),
                style_string(&e.sheet, Styles::Primary)
            );

            println!(
                "{} ({})",
                style_string(&e.name, Styles::Secondary),
                format_duration(&elapsed)
            );
        }
    }

    Ok(())
}
