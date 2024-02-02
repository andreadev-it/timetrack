mod config;
mod state;
mod database;
mod entry;
mod app;
mod utils;

use std::usize;

use app::{start_task, stop_task, display_tasks, checkout_sheet, display_sheets, current_task};
use clap::{Parser, Subcommand};
use config::Config;
use database::{ensure_db_exists, connect_to_db, create_tables};
use langtime::parse;
use anyhow::{Result, Context};
use state::State;

#[derive(Parser, Debug)]
#[command(author, version, about, infer_subcommands=true)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    In { 
        task: String, 
        #[arg(long)]
        at: Option<String> 
    },
    Out {
        #[arg(long)]
        at: Option<String>
    },
    Display {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        month: Option<String>,
        sheet: Option<String>
    },
    Sheet { name: Option<String> },
    List,
    Edit {
        #[arg(long)]
        id: usize,
        #[arg(long)]
        start: Option<String>,
        #[arg(long)]
        end: Option<String>,
        notes: Option<String>
    },
    Current,
    Kill {
        #[arg(long)]
        id: Option<usize>,
        sheet: String
    }
}

fn main() -> Result<()> {
    let config = Config::build();

    setup(&config)?;

    let mut state = State::build(&config)?;

    let cli = Cli::parse();

    match &cli.command {
        Subcommands::In { task, at }=> {
            let mut target_time = None;

            if let Some(at) = at {
                target_time = Some(
                    parse(at).context("The specified time couldn't be recognized.")?
                );
            }

            start_task(task, target_time, &state)?;
        },
        Subcommands::Out { at } => {
            let mut target_time = None;

            if let Some(at) = at {
                target_time = Some(
                    parse(at).context("The specified time couldn't be recognized.")?
                );
            }

            stop_task(target_time, &mut state)?;
        },
        Subcommands::Display { json, month, sheet } => {
            display_tasks(json, month, sheet, &state)?;
        },
        Subcommands::Sheet { name } => {
            match name {
                Some(s) => checkout_sheet(s, &mut state)?,
                None => display_sheets(&state)?
            }
        },
        Subcommands::List => {
            display_sheets(&state)?;
        },
        Subcommands::Current => {
            current_task(&state)?;
        },
        Subcommands::Edit { id, start, end, notes } => {
            todo!();
        },
        Subcommands::Kill { id, sheet } => {
            todo!();
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
