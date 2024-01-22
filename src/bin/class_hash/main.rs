#[macro_use]
extern crate prettytable;

mod cli;
mod commands;

use anyhow::{Ok, Result};
use clap::Parser;

use cli::Cli;
use commands::CliCommand;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        cli::Commands::Get(cmd) => {
            cmd.run()?;
        }
    };
    Ok(())
}
