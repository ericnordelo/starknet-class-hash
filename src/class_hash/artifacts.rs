use anyhow::{Ok, Result};
use camino::{Utf8Path, Utf8PathBuf};
use std::env;
use std::fs;

#[derive(Debug)]
pub struct Artifacts {
    pub sierra: Vec<Utf8PathBuf>,
    pub casm: Vec<Utf8PathBuf>,
}

pub fn get_artifacts() -> Result<Artifacts> {
    let path = fs::canonicalize(env::current_dir()?)?;
    let pwd = Utf8PathBuf::try_from(path)?;
    let artifacts_dir = pwd.as_path().join("target").join("dev");
    let mut artifacts = Artifacts {
        sierra: Vec::new(),
        casm: Vec::new(),
    };

    for entry in Utf8Path::read_dir_utf8(artifacts_dir.as_path())? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if path.file_name().unwrap().ends_with(".contract_class.json") {
                artifacts.sierra.push(Utf8PathBuf::try_from(path)?);
            } else if path
                .file_name()
                .unwrap()
                .ends_with(".compiled_contract_class.json")
            {
                artifacts.casm.push(Utf8PathBuf::try_from(path)?);
            }
        }
    }
    Ok(artifacts)
}
