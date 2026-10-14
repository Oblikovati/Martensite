//! PR validation: lint the changed files, then build, lint and test only the
//! packages the change can affect.

use std::collections::BTreeSet;

use crate::affected::affected_packages;
use crate::cargo::{BuildTarget, CargoTasks, PackageSelection};
use crate::context::BuildContext;
use crate::error::BuildResult;
use crate::git::ChangedFile;
use crate::lint::{check_rust_format, check_whitespace};
use crate::workspace::CargoWorkspace;

/// Validates `HEAD` plus uncommitted work against the branch it will merge into.
///
/// ```ignore
/// validate_pull_request(&context, "origin/main", &BuildTarget::Host)?;
/// ```
pub fn validate_pull_request(
    context: &BuildContext<'_>,
    base: &str,
    target: &BuildTarget,
) -> BuildResult<()> {
    let merge_base = context.git.merge_base(base)?;
    let changed = context.git.changed_files(&merge_base)?;
    let summary = format!(
        "{} file(s) changed since {base} ({merge_base})",
        changed.len()
    );
    context.reporter.note(&summary);
    lint_changed_files(context, &merge_base, &changed)?;
    check_affected_packages(context, &changed, target)
}

fn check_affected_packages(
    context: &BuildContext<'_>,
    changed: &[ChangedFile],
    target: &BuildTarget,
) -> BuildResult<()> {
    let affected = context
        .reporter
        .step("Find affected packages", || affected(context, changed))?;
    if affected.is_empty() {
        context
            .reporter
            .note("No Rust package is affected; build and tests are skipped.");
        return Ok(());
    }
    let names: Vec<String> = affected.into_iter().collect();
    context
        .reporter
        .note(&format!("Affected packages: {}", names.join(", ")));
    check_packages(context, &PackageSelection::Packages(names), target)
}

/// Runs both file linters before failing, so one run shows every problem.
fn lint_changed_files(
    context: &BuildContext<'_>,
    merge_base: &str,
    changed: &[ChangedFile],
) -> BuildResult<()> {
    let reporter = &context.reporter;
    let whitespace = reporter.step("Check whitespace in changed files", || {
        check_whitespace(&context.git, merge_base, changed)
    });
    let format = reporter.step("Check formatting of changed Rust files", || {
        check_rust_format(context.runner, context.root(), changed)
    });
    whitespace.and(format)
}

fn affected(context: &BuildContext<'_>, changed: &[ChangedFile]) -> BuildResult<BTreeSet<String>> {
    let workspace = CargoWorkspace::load(context.runner, context.root())?;
    Ok(affected_packages(&workspace, changed))
}

fn check_packages(
    context: &BuildContext<'_>,
    selection: &PackageSelection,
    target: &BuildTarget,
) -> BuildResult<()> {
    let cargo = CargoTasks::new(context.runner, context.root());
    let reporter = &context.reporter;
    reporter.step("Clippy affected packages", || {
        cargo.clippy(selection, target)
    })?;
    reporter.step("Build affected packages", || {
        cargo.build_all_targets(selection, target)
    })?;
    reporter.step("Test affected packages", || cargo.test(selection, target))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::tests::context_at;
    use crate::fake_runner::RecordingRunner;
    use crate::workspace::tests::sample_metadata;
    use std::path::Path;

    const METADATA: &str = "cargo metadata --format-version 1 --no-deps --locked";

    fn runner_with_changes(diff: &str) -> RecordingRunner {
        RecordingRunner::default()
            .with_stdout("git rev-parse --show-toplevel", "/repo")
            .with_stdout("git merge-base origin/main HEAD", "base1")
            .with_stdout("git diff --name-status --no-renames -z base1", diff)
            .with_stdout(METADATA, &sample_metadata())
    }

    #[test]
    fn math_change_checks_math_and_its_dependent_kernel() {
        let runner = runner_with_changes("M\0crates/martensite-math/src/lib.rs\0");
        validate_pull_request(
            &context_at(&runner, Path::new("/repo")),
            "origin/main",
            &BuildTarget::Host,
        )
        .unwrap();
        let selection = "--package martensite-kernel --package martensite-math";
        let tail: Vec<_> = runner.commands().into_iter().rev().take(5).rev().collect();
        assert_eq!(
            tail,
            [
                "rustfmt --check -- crates/martensite-math/src/lib.rs".to_owned(),
                "cargo metadata --format-version 1 --no-deps --locked".to_owned(),
                format!("cargo clippy --locked {selection} --all-targets -- -D warnings"),
                format!("cargo build --locked {selection} --all-targets"),
                format!("cargo test --locked {selection} --no-fail-fast"),
            ]
        );
    }

    #[test]
    fn docs_only_change_skips_cargo_after_lint() {
        let runner = runner_with_changes("M\0docs/readme.md\0");
        validate_pull_request(
            &context_at(&runner, Path::new("/repo")),
            "origin/main",
            &BuildTarget::Host,
        )
        .unwrap();
        let commands = runner.commands();
        assert!(
            commands
                .iter()
                .all(|c| !c.starts_with("cargo clippy") && !c.starts_with("cargo test"))
        );
        assert!(commands.contains(&"git diff --check base1".to_owned()));
    }

    #[test]
    fn whitespace_failure_still_runs_rustfmt_then_fails() {
        let runner = runner_with_changes("M\0crates/martensite-math/src/lib.rs\0")
            .with_exit_code("git diff --check base1", 2);
        let context = context_at(&runner, Path::new("/repo"));
        assert!(validate_pull_request(&context, "origin/main", &BuildTarget::Host).is_err());
        assert!(
            runner
                .commands()
                .iter()
                .any(|c| c.starts_with("rustfmt --check"))
        );
        assert!(
            !runner
                .commands()
                .iter()
                .any(|c| c.starts_with("cargo build"))
        );
    }
}
