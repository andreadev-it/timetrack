use anyhow::Result;
use tabled::builder::Builder;
use tabled::settings::{Style, Color};
use tabled::settings::object::Rows;
use tabled::settings::themes::Colorization;

use crate::database::get_all_sheets;
use crate::State;
use crate::style::{style_string, Styles};

pub fn list_sheets(state: &State) -> Result<()> {
    let mut sheets = get_all_sheets(&state.database)?;

    if sheets.is_empty() {
        sheets.push(state.current_sheet.to_string());
    }

    let mut builder = Builder::new();


    println!("{}", style_string("Timesheets:", Styles::Title));

    builder.push_record(vec![
        "Name",
        "Running",
        "Today",
        "Total time"
    ]);

    for sheet in sheets {
        let s = if sheet == state.current_sheet {
            format!("{}{}", "*", sheet)
        }
        else if sheet == state.last_sheet {
            format!("{}{}", "-", sheet)
        }
        else {
            format!("{}", sheet)
        };

        builder.push_record(vec![
            s,
            "".to_string(),
            "".to_string(),
            "".to_string()
        ]);
    }

    let mut table = builder.build();
    table.with(Style::empty());
    table.with(Colorization::exact([Color::BOLD], Rows::first()));

    println!("{}", table);


    Ok(())
}
