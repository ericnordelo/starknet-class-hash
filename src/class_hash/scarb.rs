use anyhow::{bail, Context, Ok, Result};
use camino::Utf8PathBuf;
use std::env;
use std::fs;
use std::process::Command;

const MANIFEST_FILE_NAME: &str = "Scarb.toml";

pub fn get_scarb_manifest() -> Result<Utf8PathBuf> {
    let manifest =
        try_find_manifest_of_pwd().context("Error while trying to retrieve Scarb.toml")?;
    if manifest.is_none() {
        bail!("Could not find Scarb.toml in the current directory");
    }
    Ok(manifest.unwrap())
}

pub fn compile() -> Result<()> {
    let status = Command::new("scarb")
        .arg("build")
        .status()
        .context("Error while trying to compile the project")?;

    if status.success() {
        Ok(())
    } else {
        bail!("Compilation errors where detected. Try running `scarb build` manually.")
    }
}

pub fn print_compiler_versions() -> Result<()> {
    let status = Command::new("scarb")
        .arg("--version")
        .status()
        .context("Error while trying to get the compiler versions")?;

    if status.success() {
        Ok(())
    } else {
        bail!("Compilation errors where detected. Try running `scarb build` manually.")
    }
}

fn try_find_manifest_of_pwd() -> Result<Option<Utf8PathBuf>> {
    let path = fs::canonicalize(env::current_dir()?)?;
    let pwd = Utf8PathBuf::try_from(path)?;
    let manifest = pwd.as_path().join(MANIFEST_FILE_NAME);

    if manifest.is_file() {
        return Ok(Some(manifest));
    }
    Ok(None)
}
