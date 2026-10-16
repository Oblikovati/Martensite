//! File-level lint of a change set: whitespace and rustfmt run only on the
//! files a PR touches, so unrelated legacy formatting never blocks a PR.

use std::path::Path;

use crate::error::{BuildError, BuildResult};
use crate::git::{ChangedFile, GitRepo};
use crate::process::{CommandRunner, CommandSpec};

/// Keeps each rustfmt command line well under the Windows 32 KiB limit.
const RUSTFMT_BATCH: usize = 64;

/// Checks all toolkit source tokens when a change can affect the unsafe inventory.
/// Example: `check_unsafe_inventory(runner, root, changed)?`.
pub fn check_unsafe_inventory(
    runner: &dyn CommandRunner,
    root: &Path,
    changed: &[ChangedFile],
) -> BuildResult<()> {
    if !changed
        .iter()
        .any(|file| unsafe_inventory_input(&file.path))
    {
        return Ok(());
    }
    runner.run(&CommandSpec::new("cargo", root).args([
        "test",
        "--locked",
        "--package",
        "martensite-build",
        "unsafe_inventory",
    ]))
}

fn unsafe_inventory_input(path: &str) -> bool {
    (path.starts_with("crates/") && path.ends_with(".rs"))
        || path.starts_with("scripts/build/")
        || path.starts_with("docs/features/safe-toolkits/")
        || path.ends_with("Cargo.toml")
        || path == "Cargo.lock"
}

/// Fails when the change set adds trailing whitespace or conflict markers.
///
/// ```ignore
/// check_whitespace(&git, &merge_base, &changed)?;
/// ```
pub fn check_whitespace(
    git: &GitRepo<'_>,
    base_commit: &str,
    changed: &[ChangedFile],
) -> BuildResult<()> {
    if git.whitespace_clean(base_commit, changed)? {
        return Ok(());
    }
    Err(BuildError::LintFailed(format!(
        "whitespace errors in files changed since {base_commit} (listed above); expected none"
    )))
}

/// Runs `rustfmt --check` on every changed, still-existing `.rs` file and
/// reports all unformatted batches, not only the first.
pub fn check_rust_format(
    runner: &dyn CommandRunner,
    root: &Path,
    changed: &[ChangedFile],
) -> BuildResult<()> {
    let sources = rust_sources(changed);
    if unformatted_batches(runner, root, &sources)? == 0 {
        return Ok(());
    }
    Err(BuildError::LintFailed(format!(
        "rustfmt reported differences in {} changed Rust file(s) (diff above); run `cargo fmt --all`",
        sources.len()
    )))
}

fn unformatted_batches(
    runner: &dyn CommandRunner,
    root: &Path,
    sources: &[&str],
) -> BuildResult<usize> {
    let mut failed = 0;
    for batch in sources.chunks(RUSTFMT_BATCH) {
        let spec = CommandSpec::new("rustfmt", root)
            .args(["--check", "--"])
            .args(batch.iter().copied());
        failed += usize::from(runner.exit_code(&spec)? != 0);
    }
    Ok(failed)
}

fn rust_sources(changed: &[ChangedFile]) -> Vec<&str> {
    changed
        .iter()
        .filter(|file| !file.deleted && file.path.ends_with(".rs"))
        .map(|file| file.path.as_str())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_runner::RecordingRunner;

    fn file(path: &str, deleted: bool) -> ChangedFile {
        ChangedFile {
            path: path.into(),
            deleted,
            untracked: false,
        }
    }

    #[test]
    fn unsafe_inventory_runs_for_source_deletions_and_baseline_changes() {
        for path in [
            "crates/example/src/lib.rs",
            "docs/features/safe-toolkits/baseline.json",
            "Cargo.lock",
        ] {
            let runner = RecordingRunner::default();
            check_unsafe_inventory(&runner, Path::new("/r"), &[file(path, true)]).unwrap();
            assert_eq!(
                runner.commands(),
                ["cargo test --locked --package martensite-build unsafe_inventory"]
            );
        }
    }

    #[test]
    fn unsafe_inventory_skips_unrelated_docs() {
        let runner = RecordingRunner::default();
        check_unsafe_inventory(&runner, Path::new("/r"), &[file("README.md", false)]).unwrap();
        assert!(runner.commands().is_empty());
    }

    #[test]
    fn rustfmt_checks_only_existing_rust_files() {
        let runner = RecordingRunner::default();
        let changed = [
            file("a.rs", false),
            file("gone.rs", true),
            file("README.md", false),
        ];
        check_rust_format(&runner, Path::new("/r"), &changed).unwrap();
        assert_eq!(runner.commands(), ["rustfmt --check -- a.rs"]);
    }

    #[test]
    fn no_rust_changes_runs_no_rustfmt() {
        let runner = RecordingRunner::default();
        check_rust_format(&runner, Path::new("/r"), &[file("docs/x.md", false)]).unwrap();
        assert!(runner.commands().is_empty());
    }

    #[test]
    fn unformatted_files_fail_with_a_fix_hint() {
        let runner = RecordingRunner::default().with_exit_code("rustfmt --check -- a.rs", 1);
        let error =
            check_rust_format(&runner, Path::new("/r"), &[file("a.rs", false)]).unwrap_err();
        assert!(error.to_string().contains("cargo fmt --all"));
    }

    #[test]
    fn many_files_are_split_into_batches() {
        let runner = RecordingRunner::default();
        let changed: Vec<_> = (0..RUSTFMT_BATCH + 1)
            .map(|i| file(&format!("f{i}.rs"), false))
            .collect();
        check_rust_format(&runner, Path::new("/r"), &changed).unwrap();
        assert_eq!(runner.commands().len(), 2);
    }
}
