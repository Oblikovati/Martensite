//! Explicit package versions and locked external identities accompany payload hashes.
use super::files;
use crate::{
    error::{BuildError, BuildResult, io_error},
    package_manifest::{PackageManifest, parse_table},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct InputMetadata {
    pub packages: BTreeMap<String, String>,
    pub locked_external: Vec<serde_json::Value>,
}

pub(super) fn inspect(root: &Path) -> BuildResult<InputMetadata> {
    Ok(InputMetadata {
        packages: packages(root)?,
        locked_external: locked_external(root)?,
    })
}

fn packages(root: &Path) -> BuildResult<BTreeMap<String, String>> {
    let root_table = parse_table(&read(root, "Cargo.toml")?)?;
    let workspace = root_table
        .get("workspace")
        .and_then(|v| v.get("package"))
        .and_then(|v| v.as_table())
        .cloned()
        .unwrap_or_default();
    let mut packages = BTreeMap::new();
    for entry in files::entries(root)?.iter().filter(|entry| {
        entry.archive_path.ends_with("Cargo.toml") && !entry.archive_path.starts_with("vendor/")
    }) {
        let text =
            fs::read_to_string(&entry.source).map_err(|cause| io_error(&entry.source, cause))?;
        if let Some(package) = PackageManifest::parse(&text, &workspace)? {
            packages.insert(package.name, package.version.to_string());
        }
    }
    Ok(packages)
}

fn locked_external(root: &Path) -> BuildResult<Vec<serde_json::Value>> {
    let lock = parse_table(&read(root, "Cargo.lock")?)?;
    lock.get("package")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
        .filter(|v| v.get("source").is_some())
        .map(serde_json::to_value)
        .collect::<Result<_, _>>()
        .map_err(|cause| BuildError::ReleaseRule(format!("SDK locked dependencies: {cause}")))
}

fn read(root: &Path, name: &str) -> BuildResult<String> {
    let path = root.join(name);
    fs::read_to_string(&path).map_err(|cause| io_error(&path, cause))
}
