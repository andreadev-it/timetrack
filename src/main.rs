mod commands;
mod config;
mod database;
mod entry;
mod state;
mod style;
mod utils;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use commands::*;
use config::Config;
use database::{connect_to_db, create_tables, ensure_db_exists};
pub use entry::Entry;
use langtime::parse;
pub use state::State;

use crate::style::{style_string, Styles};

#[derive(Parser, Debug)]
#[command(author, version, about, infer_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    /// Checks into the current timesheet
    In {
        /// The task description
        task: Option<String>,
        /// The time and date this task was started. "15 minutes ago" and similar are also ok
        #[arg(short, long)]
        at: Option<String>,
    },
    /// Checks out of the current timesheet
    Out {
        /// The time and date this task has ended. "15 minutes ago" and similar are also ok
        #[arg(short, long)]
        at: Option<String>,
    },
    /// Displays the current timesheet
    Display {
        /// Show a JSON representation instead of a human-readable one
        #[arg(long)]
        json: bool,
        /// Show the tasks IDs
        #[arg(short, long)]
        ids: bool,
        /// Filter the tasks based on when they started
        #[arg(short, long)]
        start: Option<String>,
        /// Filter the tasks based on when they ended
        #[arg(short, long)]
        end: Option<String>,
        /// Just filter by whole days, do not take into account the time
        #[arg(short, long)]
        filter_by_date: bool,
        /// The timesheet to display, or the current one
        sheet: Option<String>,
    },
    /// Like `Display`, but for a specific month, or the current one
    Month {
        /// Show a JSON representation instead of a human-readable one
        #[arg(long)]
        json: bool,
        /// Show the tasks IDs
        #[arg(short, long)]
        ids: bool,
        /// The specific month to show. The format is yyyy-mm (e.g. "2024-03")
        #[arg(short, long)]
        month: Option<String>,
        /// The timesheet to display, or the current one
        sheet: Option<String>,
    },
    /// Change timesheet
    Sheet {
        name: String,
        #[arg(short, long)]
        rename: Option<String>,
    },
    /// List available timesheet
    List,
    /// Edit a task
    Edit {
        /// The ID of the task to edit
        #[arg(short, long)]
        id: Option<usize>,
        /// Set a new start date and time for this task
        #[arg(short, long)]
        start: Option<String>,
        /// Set a new end date and time for this task
        #[arg(short, long)]
        end: Option<String>,
        /// Move this task to a different timesheet
        #[arg(short, long)]
        move_to: Option<String>,
        /// The new task description
        notes: Option<String>,
    },
    /// Shows the active task for the current sheet
    Current,
    /// Removes a task or a whole timesheet
    Kill {
        #[command(flatten)]
        kill_args: KillArgs,
    },
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct KillArgs {
    /// The ID of the task to remove
    #[arg(long)]
    id: Option<usize>,
    /// The name of the timesheet to remove
    sheet: Option<String>,
}

fn main() {
    if let Err(e) = cli() {
        println!("{} {}", style_string("Error:", Styles::Error), e);
        std::process::exit(1);
    }
}

fn cli() -> Result<()> {
    let config = Config::build()?;

    setup(&config)?;

    let mut state = State::build(&config)?;

    let cli = Cli::parse();

    match &cli.command {
        Subcommands::In { task, at } => {
            let target_time = at.as_ref().map(|at| parse(at)).transpose()?;

            let task = task.as_ref();
            let default_task = "".to_string();
            let task = task.unwrap_or(&default_task);

            start_task(task, target_time, &state)?;
        }
        Subcommands::Out { at } => {
            let target_time = at.as_ref().map(|at| parse(at)).transpose()?;

            stop_task(target_time, &mut state)?;
        }
        Subcommands::Display {
            json,
            sheet,
            start,
            end,
            filter_by_date,
            ids,
        } => {
            display_tasks(
                json,
                sheet.as_ref(),
                start.as_ref().map(|s| parse(s)).transpose()?,
                end.as_ref().map(|e| parse(e)).transpose()?,
                filter_by_date,
                ids,
                &state,
            )?;
        }
        Subcommands::Month {
            json,
            ids,
            month,
            sheet,
        } => {
            display_month(json, ids, month.as_ref(), sheet.as_ref(), &mut state)?;
        }
        Subcommands::Sheet { name, rename } => match rename {
            None => checkout_sheet(name, &mut state)?,
            Some(new_name) => rename_sheet(name, new_name, &mut state)?,
        },
        Subcommands::List => {
            list_sheets(&state)?;
        }
        Subcommands::Current => {
            current_task(&state)?;
        }
        Subcommands::Edit {
            id,
            start,
            end,
            move_to,
            notes,
        } => {
            edit_task(id, start, end, move_to, notes, &mut state)?;
        }
        Subcommands::Kill { kill_args } => {
            if let Some(id) = &kill_args.id {
                kill_task(id, &mut state)?;
            } else if let Some(sheet) = &kill_args.sheet {
                kill_sheet(sheet, &mut state)?;
            }
        }
    };

    Ok(())
}

fn setup(config: &Config) -> Result<()> {
    ensure_db_exists(config)?;
    let db = connect_to_db(config)?;
    create_tables(&db)?;

    Ok(())
}
