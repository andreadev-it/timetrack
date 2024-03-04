mod commands;
mod config;
mod database;
mod entry;
mod state;
mod utils;
mod style;

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
    In {
        task: Option<String>,
        #[arg(short, long)]
        at: Option<String>,
    },
    Out {
        #[arg(short, long)]
        at: Option<String>,
    },
    Display {
        #[arg(long)]
        json: bool,
        #[arg(short, long)]
        ids: bool,
        #[arg(short, long)]
        start: Option<String>,
        #[arg(short, long)]
        end: Option<String>,
        #[arg(short, long)]
        filter_by_date: bool,
        sheet: Option<String>,
    },
    Month {
        #[arg(long)]
        json: bool,
        #[arg(short, long)]
        ids: bool,
        #[arg(short, long)]
        month: Option<String>,
        sheet: Option<String>
    },
    Sheet {
        name: Option<String>,
    },
    List,
    Edit {
        #[arg(short, long)]
        id: Option<usize>,
        #[arg(short, long)]
        start: Option<String>,
        #[arg(short, long)]
        end: Option<String>,
        #[arg(short, long)]
        move_to: Option<String>,
        notes: Option<String>,
    },
    Current,
    Kill {
        #[command(flatten)]
        kill_args: KillArgs,
    },
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct KillArgs {
    #[arg(long)]
    id: Option<usize>,
    sheet: Option<String>,
}

fn main() {
    match cli() {
        Ok(_) => {}
        Err(e) => {
            println!(
                "{} {}",
                style_string("Error:", Styles::Error),
                e
            );
            std::process::exit(1);
        }
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
        },
        Subcommands::Month { json, ids, month, sheet } => {
            display_month(
                json,
                ids,
                month.as_ref(),
                sheet.as_ref(),
                &mut state
            )?;
        }
        Subcommands::Sheet { name } => match name {
            Some(s) => checkout_sheet(s, &mut state)?,
            None => list_sheets(&state)?,
        },
        Subcommands::List => {
            list_sheets(&state)?;
        },
        Subcommands::Current => {
            current_task(&state)?;
        },
        Subcommands::Edit {
            id,
            start,
            end,
            move_to,
            notes,
        } => {
            edit_task(id, start, end, move_to, notes, &mut state)?;
        },
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
