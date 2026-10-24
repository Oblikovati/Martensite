//! Preserve committed configuration and lockfile while adding a local dependency source.
use super::files;
use crate::{
    archive,
    error::{BuildError, BuildResult, io_error},
    package_manifest::parse_table,
    process::{CommandRunner, CommandSpec},
};
use std::{fs, path::Path};

pub(super) fn prepare(runner: &dyn CommandRunner, stage: &Path) -> BuildResult<()> {
    if stage.join("vendor").exists() {
        return Err(BuildError::ReleaseRule(
            "SDK vendor/ already exists; expected a reserved empty generated dependency path"
                .into(),
        ));
    }
    super::paths::validate(stage)?;
    let lock = stage.join("Cargo.lock");
    let before = archive::sha256_hex(&lock)?;
    let config = read_config(stage)?;
    merge_config(&config, "")?;
    let replacement = runner.capture(&CommandSpec::new("cargo", stage).args([
        "vendor",
        "--locked",
        "--versioned-dirs",
        "vendor",
    ]))?;
    let merged = merge_config(&config, &replacement)?;
    files::write(stage, Path::new(".cargo/config.toml"), merged.as_bytes())?;
    verify_lock(&lock, &before)
}

fn verify_lock(lock: &Path, before: &str) -> BuildResult<()> {
    if before != archive::sha256_hex(lock)? {
        return Err(BuildError::ReleaseRule(
            "SDK Cargo.lock changed during vendoring; expected committed lock bytes".into(),
        ));
    }
    Ok(())
}

pub(super) fn read_config(root: &Path) -> BuildResult<String> {
    if root.join(".cargo/config").exists() {
        return Err(BuildError::ReleaseRule("SDK .cargo/config exists; expected only .cargo/config.toml to avoid source precedence ambiguity".into()));
    }
    let path = root.join(".cargo/config.toml");
    if !path.exists() {
        return Ok(String::new());
    }
    fs::read_to_string(&path).map_err(|cause| io_error(&path, cause))
}

fn merge_config(original: &str, replacement: &str) -> BuildResult<String> {
    let old = parse_table(original)?;
    let new = parse_table(replacement)?;
    if old.contains_key("source")
        || old.contains_key("patch")
        || new.keys().any(|key| key != "source")
    {
        return Err(BuildError::ReleaseRule("SDK Cargo config has source/patch conflicts; expected committed config without overrides and Cargo-generated source tables".into()));
    }
    Ok(format!("{original}\n{replacement}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_aliases_survive_and_source_overrides_are_rejected() {
        let alias = "[alias]\nxtask='run -p build'\n";
        let replacement = "[source.crates-io]\nreplace-with='vendored-sources'\n[source.vendored-sources]\ndirectory='vendor'\n";
        assert!(merge_config(alias, replacement).unwrap().starts_with(alias));
        assert!(merge_config(replacement, replacement).is_err());
        assert!(merge_config("[patch.crates-io]", replacement).is_err());
        assert!(merge_config(alias, "[build]\nrustc='elsewhere'").is_err());
    }
}
