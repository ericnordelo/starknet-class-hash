pub mod artifacts;
pub mod scarb;
pub mod types;

use anyhow::{Ok, Result};
use artifacts::Artifacts;
use camino::Utf8PathBuf;
use starknet::core::types::contract::{CompiledClass, SierraClass};
use std::collections::HashMap;

/// Returns a HashMap of the contract name and its class hashes (sierra and casm).
pub fn get_class_hashes(artifacts: Artifacts) -> Result<HashMap<String, (String, String)>> {
    let mut class_hashes = HashMap::new();

    for file in artifacts.sierra {
        let artifact_name = get_artifact_name(&file);
        let class_hash = get_class_hash(file.as_str())?;

        class_hashes.insert(artifact_name, (class_hash, String::new()));
    }

    for file in artifacts.casm {
        let artifact_name = get_artifact_name(&file);
        let class_hash = get_class_hash(file.as_str())?;

        class_hashes
            .entry(artifact_name)
            .and_modify(|e| e.1 = class_hash.clone())
            .or_insert((String::new(), class_hash));
    }

    Ok(class_hashes)
}

fn get_class_hash(file: &str) -> Result<String> {
    let class_hash = if let std::result::Result::Ok(class) =
        serde_json::from_reader::<_, SierraClass>(std::fs::File::open(file)?)
    {
        class.class_hash()?.to_string()
    } else if let std::result::Result::Ok(class) =
        serde_json::from_reader::<_, CompiledClass>(std::fs::File::open(file)?)
    {
        class.class_hash()?.to_string()
    } else {
        anyhow::bail!("Failed to parse contract artifact");
    };

    Ok(class_hash)
}

fn get_artifact_name(artifact: &Utf8PathBuf) -> String {
    artifact
        .file_name()
        .unwrap()
        .split(".")
        .collect::<Vec<&str>>()[0]
        .to_string()
}
