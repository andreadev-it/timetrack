mod config;
mod state;
mod database;
mod entry;
mod app;
mod utils;

use app::{start_task, stop_task, display_tasks, checkout_sheet, display_sheets, current_task};
use clap::{Parser, Subcommand};
use config::Config;
use database::{ensure_db_exists, connect_to_db, create_tables};
use langtime::parse;
use rusqlite::Connection;
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
    Out,
    Display {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        current_month: bool
    },
    Sheet { name: Option<String> },
    Current
}

fn main() -> Result<()> {
    let config = Config::build();

    setup(&config)?;

    let state = State::build(&config)?;

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
        Subcommands::Out => {
            stop_task(&state)?;
        },
        Subcommands::Display { json, current_month } => {
            display_tasks(json, current_month);
        },
        Subcommands::Sheet { name } => {
            match name {
                Some(s) => checkout_sheet(s),
                None => display_sheets()
            }
        },
        Subcommands::Current => {
            current_task(&state)?;
        }
    };

    Ok(())
}

fn setup(config: &Config) -> Result<()> {
    let db = connect_to_db(config)?;
    ensure_db_exists(config)?;
    create_tables(&db)?;

    Ok(())
}
