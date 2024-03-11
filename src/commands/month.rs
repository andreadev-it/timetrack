use anyhow::Result;
use chrono::Local;

use crate::commands::display_tasks;
use crate::utils::get_month_boundaries;
use crate::State;

pub fn display_month(
    json: &bool,
    ids: &bool,
    month: Option<&String>,
    sheet: Option<&String>,
    state: &mut State,
) -> Result<()> {
    let now = Local::now().format("%Y-%m").to_string();
    let month = month.unwrap_or(&now);
    let (start, end) = get_month_boundaries(month)?;

    display_tasks(json, sheet, Some(start), Some(end), &true, ids, state)
}
