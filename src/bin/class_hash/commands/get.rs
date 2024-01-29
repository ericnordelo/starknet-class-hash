use anyhow::{Ok, Result};
use clap::Parser;
use num::bigint::BigInt;
use prettytable::{format, Table};

use crate::commands::CliCommand;
use class_hash::artifacts::get_artifacts;
use class_hash::get_class_hashes;
use class_hash::scarb::{clean, compile, get_scarb_manifest, print_compiler_versions};

#[derive(Parser, Debug)]
pub struct Get {
    #[clap(
        short,
        long,
        default_value_t = true,
        help = "Compile the project before computing the hashes"
    )]
    pub compile: bool,
}

impl CliCommand for Get {
    /// Get the class hashes from the project contracts.
    fn run(&self) -> Result<()> {
        let _ = get_scarb_manifest()?;

        if self.compile {
            clean()?;
            compile()?;
        }

        let artifacts = get_artifacts()?;
        let class_hashes = get_class_hashes(artifacts)?;

        println!();
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(row![bFg->"Contract:", bFg->"Class hashes:"]);

        for (i, artifact) in class_hashes.iter().enumerate() {
            if i > 0 {
                table.add_empty_row();
            }
            let name = artifact.0;
            let hashes = artifact.1;
            let sierra = if hashes.0.is_empty() {
                "_".to_string()
            } else {
                format!("0x{:x}", hashes.0.parse::<BigInt>()?)
            };
            let casm = if hashes.1.is_empty() {
                "_".to_string()
            } else {
                format!("0x{:x}", hashes.1.parse::<BigInt>()?)
            };
            table.add_row(row![bFy->name, format!("Sierra: {}", sierra)]);
            table.add_row(row!["", format!("Casm: {}", casm)]);
        }
        table.printstd();

        println!();
        print_compiler_versions()?;

        Ok(())
    }
}
