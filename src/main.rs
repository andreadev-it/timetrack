mod config;
mod state;
mod database;
mod entry;
mod utils;
mod commands;

use commands::*;
use clap::{Parser, Subcommand, Args};
use config::Config;
use database::{ensure_db_exists, connect_to_db, create_tables};
use langtime::parse;
use anyhow::{Result, Context};
pub use state::State;
pub use entry::Entry;

#[derive(Parser, Debug)]
#[command(author, version, about, infer_subcommands=true)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    In { 
        task: Option<String>, 
        #[arg(short, long)]
        at: Option<String> 
    },
    Out {
        #[arg(short, long)]
        at: Option<String>
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
        sheet: Option<String>
    },
    Sheet { name: Option<String> },
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
        notes: Option<String>
    },
    Current,
    Kill {
        #[command(flatten)]
        kill_args: KillArgs
    }
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct KillArgs {
    #[arg(long)]
    id: Option<usize>,
    sheet: Option<String>
}

fn main() -> Result<()> {
    let config = Config::build();

    setup(&config)?;

    let mut state = State::build(&config)?;

    let cli = Cli::parse();

    match &cli.command {
        Subcommands::In { task, at }=> {
            let target_time = at.as_ref()
                .and_then(|at| Some(parse(&at)))
                .transpose()?;

            let task = task.as_ref();
            let default_task = "".to_string();
            let task = task.unwrap_or(&default_task);

            start_task(task, target_time, &state)?;
        },
        Subcommands::Out { at } => {
            let target_time = at.as_ref()
                .and_then(|at| Some(parse(&at)))
                .transpose()?;

            stop_task(target_time, &mut state)?;
        },
        Subcommands::Display { json, sheet, start, end, ids } => {
            display_tasks(
                json,
                sheet.as_ref(),
                start.as_ref(),
                end.as_ref(),
                &ids,
                &state
            )?;
        },
        Subcommands::Sheet { name } => {
            match name {
                Some(s) => checkout_sheet(s, &mut state)?,
                None => list_sheets(&state)?
            }
        },
        Subcommands::List => {
            list_sheets(&state)?;
        },
        Subcommands::Current => {
            current_task(&state)?;
        },
        Subcommands::Edit { id, start, end, move_to, notes } => {
            edit_task(id, start, end, move_to, notes, &mut state)?;
        },
        Subcommands::Kill { kill_args } => {
            if let Some(id) = &kill_args.id {
                kill_task(&id, &mut state)?;
            }
            else if let Some(sheet) = &kill_args.sheet {
                kill_sheet(&sheet, &mut state)?;
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
