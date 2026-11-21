//! Captures scoped compiler diagnostics during #592 without issuing a full-workspace receipt.
use std::collections::BTreeSet;
use std::path::Path;

use crate::context::BuildContext;
use crate::error::{BuildResult, io_error};
use crate::process::CommandSpec;
use crate::targets::PRIMARY_TARGET;

use super::{
    baseline, collect_diagnostics, invalidate_evidence, inventory_environment, provenance,
};

pub(crate) fn capture_migration(
    context: &BuildContext<'_>,
    base: &str,
    libraries: &BTreeSet<String>,
    all_targets: &BTreeSet<String>,
) -> BuildResult<()> {
    let triple = PRIMARY_TARGET.triple;
    let path = baseline::baseline_path(Path::new(""), triple)?;
    let approved = context.git.file_at(base, &path.to_string_lossy())?;
    for (flag, packages) in [("--lib", libraries), ("--all-targets", all_targets)] {
        let directory =
            Path::new("target/unsafe-inventory/migration").join(flag.trim_start_matches('-'));
        invalidate_evidence(&context.root().join(&directory))?;
        if !packages.is_empty() {
            context
                .reporter
                .step("Collect scoped unsafe diagnostics", || {
                    capture_group(context, flag, packages, &directory, &approved)
                })?;
        }
    }
    Ok(())
}

fn capture_group(
    context: &BuildContext<'_>,
    flag: &str,
    packages: &BTreeSet<String>,
    directory: &Path,
    approved: &str,
) -> BuildResult<()> {
    let root = context.root();
    let triple = PRIMARY_TARGET.triple;
    let command = scoped_command(root, triple, flag, packages, directory);
    let diagnostics = collect_diagnostics(context.runner, root, &command, directory)?;
    let provenance = provenance::capture(context.runner, root, &command)?;
    let report = serde_json::json!({
        "target": triple, "provenance": provenance, "packages": packages, "cargo_scope": flag,
        "coverage": "partial migration; compare emitted group counts with the base's reviewed workspace ceiling",
        "diagnostics": diagnostics
    });
    let path = root.join(directory).join("latest.json");
    let bytes = serde_json::to_vec_pretty(&report)
        .map_err(|error| super::output_error(error.to_string()))?;
    std::fs::write(&path, bytes).map_err(|error| io_error(&path, error))?;
    baseline::verify_migration_ceiling(
        approved,
        triple,
        &provenance.compiler,
        &diagnostics.workspace_summary,
    )
}

fn scoped_command(
    root: &Path,
    triple: &str,
    flag: &str,
    packages: &BTreeSet<String>,
    directory: &Path,
) -> CommandSpec {
    let args = packages.iter().flat_map(|package| ["--package", package]);
    let command = CommandSpec::new("cargo", root)
        .args([
            "check",
            "--locked",
            flag,
            "--target",
            triple,
            "--message-format=json",
            "--target-dir",
        ])
        .arg(directory.join("build").to_string_lossy())
        .args(args);
    inventory_environment(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expanded_scope_uses_only_selected_crates_and_never_sets_workspace() {
        let packages = BTreeSet::from(["tkernel".to_owned(), "tkmath".to_owned()]);
        let command = scoped_command(
            Path::new("/repo"),
            "target",
            "--lib",
            &packages,
            Path::new("scoped"),
        );
        assert_eq!(
            command.display(),
            "cargo check --locked --lib --target target --message-format=json --target-dir scoped/build --package tkernel --package tkmath"
        );
    }
}
