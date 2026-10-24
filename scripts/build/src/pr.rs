//! PR validation: lint the changed files, then build, lint and test only the
//! packages the change can affect.

use std::collections::BTreeSet;

use crate::cargo::{BuildTarget, CargoTasks, PackageSelection};
use crate::context::BuildContext;
use crate::error::BuildResult;
use crate::git::ChangedFile;
use crate::lint::{check_rust_format, check_unsafe_inventory, check_whitespace};
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
    validate_pr_checks(context, base, target, true)
}

/// Runs the shared PR checks; CI delegates tests to package jobs.
///
/// ```ignore
/// validate_pr_checks(&context, "origin/main", &BuildTarget::Host, false)?;
/// ```
pub fn validate_pr_checks(
    context: &BuildContext<'_>,
    base: &str,
    target: &BuildTarget,
    run_tests: bool,
) -> BuildResult<()> {
    let merge_base = context.git.merge_base(base)?;
    let changed = context.git.changed_files(&merge_base)?;
    let summary = format!(
        "{} file(s) changed since {base} ({merge_base})",
        changed.len()
    );
    context.reporter.note(&summary);
    lint_changed_files(context, &merge_base, &changed)?;
    if changed
        .iter()
        .any(|file| crate::lint::unsafe_inventory_input(&file.path))
    {
        context
            .reporter
            .step("Collect expanded unsafe diagnostics", || {
                crate::compiler_inventory::capture_inventory(context.runner, context.root(), target)
            })?;
    }
    check_affected_packages(context, &merge_base, &changed, target, run_tests)
}

struct PackageChecks {
    build: BTreeSet<String>,
    test: BTreeSet<String>,
}

fn check_affected_packages(
    context: &BuildContext<'_>,
    reference: &str,
    changed: &[ChangedFile],
    target: &BuildTarget,
    run_tests: bool,
) -> BuildResult<()> {
    let checks = context.reporter.step("Find affected packages", || {
        affected(context, reference, changed)
    })?;
    if changed.iter().any(|file| file.path == "rustfmt.toml") {
        CargoTasks::new(context.runner, context.root()).format_check_all()?;
    }
    if checks.build.is_empty() {
        context
            .reporter
            .note("No Rust package is affected; build and tests are skipped.");
        return Ok(());
    }
    run_package_checks(context, checks, target, run_tests)
}

fn run_package_checks(
    context: &BuildContext<'_>,
    checks: PackageChecks,
    target: &BuildTarget,
    run_tests: bool,
) -> BuildResult<()> {
    let build = PackageSelection::Packages(checks.build.into_iter().collect());
    check_packages(context, &build, target, false)?;
    if !run_tests || checks.test.is_empty() {
        return Ok(());
    }
    let names: Vec<_> = checks.test.into_iter().collect();
    context
        .reporter
        .note(&format!("Test packages: {}", names.join(", ")));
    context.reporter.step("Test affected packages", || {
        CargoTasks::new(context.runner, context.root())
            .test(&PackageSelection::Packages(names), target)
    })
}

/// Runs both file linters before failing, so one run shows every problem.
fn lint_changed_files(
    context: &BuildContext<'_>,
    merge_base: &str,
    changed: &[ChangedFile],
) -> BuildResult<()> {
    let reporter = &context.reporter;
    reporter.step("Check toolkit unsafe inventory", || {
        check_unsafe_inventory(context.runner, context.root(), changed)
    })?;
    let whitespace = reporter.step("Check whitespace in changed files", || {
        check_whitespace(&context.git, merge_base, changed)
    });
    let format = reporter.step("Check formatting of changed Rust files", || {
        check_rust_format(context.runner, context.root(), changed)
    });
    whitespace.and(format)
}

fn affected(
    context: &BuildContext<'_>,
    reference: &str,
    changed: &[ChangedFile],
) -> BuildResult<PackageChecks> {
    let workspace = CargoWorkspace::load(context.runner, context.root())?;
    let test =
        crate::release_impact::pull_request_packages(context, &workspace, reference, changed)?;
    let build = if changed
        .iter()
        .any(|file| matches!(file.path.as_str(), "clippy.toml" | "rustfmt.toml"))
    {
        workspace.names().into_iter().collect()
    } else {
        test.clone()
    };
    Ok(PackageChecks { build, test })
}

fn check_packages(
    context: &BuildContext<'_>,
    selection: &PackageSelection,
    target: &BuildTarget,
    run_tests: bool,
) -> BuildResult<()> {
    let cargo = CargoTasks::new(context.runner, context.root());
    let reporter = &context.reporter;
    reporter.step("Clippy affected packages", || {
        cargo.clippy(selection, target)
    })?;
    reporter.step("Build affected packages", || {
        cargo.build_all_targets(selection, target)
    })?;
    if run_tests {
        reporter.step("Test affected packages", || cargo.test(selection, target))?;
    }
    Ok(())
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
        let directory = tempfile::tempdir().unwrap();
        crate::compiler_inventory::baseline::install_empty_fixture(directory.path());
        let runner = runner_with_changes("M\0crates/martensite-math/src/lib.rs\0")
            .with_stdout("git rev-parse --show-toplevel", directory.path().to_str().unwrap())
            .with_stdout("rustc -vV", "host: test-target\n")
            .with_stdout("git rev-parse HEAD", &"a".repeat(40))
            .with_stdout("cargo check --locked --workspace --all-targets --target test-target --message-format=json --target-dir target/unsafe-inventory/build",
                "{\"reason\":\"build-finished\",\"success\":true}\n");
        validate_pull_request(
            &context_at(&runner, directory.path()),
            "origin/main",
            &BuildTarget::Host,
        )
        .unwrap();
        let selection = "--package martensite-kernel --package martensite-math";
        let tail: Vec<_> = runner.commands().into_iter().rev().take(4).rev().collect();
        assert_eq!(
            tail,
            [
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
    fn delegated_tests_preserve_clippy_and_build() {
        let runner = RecordingRunner::default();
        check_packages(
            &context_at(&runner, Path::new("/repo")),
            &PackageSelection::Packages(vec!["martensite-math".into()]),
            &BuildTarget::Host,
            false,
        )
        .unwrap();
        assert_eq!(
            runner.commands(),
            [
                "git rev-parse --show-toplevel",
                "cargo clippy --locked --package martensite-math --all-targets -- -D warnings",
                "cargo build --locked --package martensite-math --all-targets",
            ]
        );
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

#[cfg(test)]
mod lint_setting_tests {
    use super::*;
    use crate::{context::tests::context_at, fake_runner::RecordingRunner};
    #[test]
    fn lint_setting_change_checks_workspace_without_running_behavioral_suites() {
        let runner = RecordingRunner::default()
            .with_stdout("git rev-parse --show-toplevel", "/repo")
            .with_stdout("git merge-base origin/main HEAD", "base")
            .with_stdout(
                "git diff --name-status --no-renames -z base",
                "M\0clippy.toml\0",
            )
            .with_stdout(
                "cargo metadata --format-version 1 --no-deps --locked",
                &crate::workspace::tests::sample_metadata(),
            );
        validate_pull_request(
            &context_at(&runner, std::path::Path::new("/repo")),
            "origin/main",
            &BuildTarget::Host,
        )
        .unwrap();
        assert!(runner.commands().iter().any(|line| line.starts_with(
            "cargo clippy --locked --package martensite-kernel --package martensite-math"
        )));
        assert!(
            !runner
                .commands()
                .iter()
                .any(|line| line.starts_with("cargo test"))
        );
    }
}
