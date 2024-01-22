use crate::commands::Get;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "class_hash")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Get the class hashes from the project contracts.")]
    Get(Get),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
