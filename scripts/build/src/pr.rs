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

/// One independent group of shared PR checks. CI runs each as its own job so
/// the slowest group, not their sum, bounds the checks (#473).
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum CheckStage {
    /// File linters, the source unsafe ratchet and the compiler unsafe inventory.
    Lint,
    /// Clippy over the affected packages.
    Clippy,
    /// A debug build of every target of the affected packages.
    Build,
}

impl CheckStage {
    /// Every stage, in the order a local run executes them.
    pub const ALL: [Self; 3] = [Self::Lint, Self::Clippy, Self::Build];

    /// The `--stage` value that selects this stage.
    ///
    /// ```ignore
    /// assert_eq!(CheckStage::Clippy.name(), "clippy");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            Self::Lint => "lint",
            Self::Clippy => "clippy",
            Self::Build => "build",
        }
    }
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
    validate_pr_stages(context, base, target, &CheckStage::ALL, run_tests)
}

/// Runs only `stages` of the shared PR checks, plus tests when requested.
///
/// ```ignore
/// validate_pr_stages(&context, "origin/main", &target, &[CheckStage::Clippy], false)?;
/// ```
pub fn validate_pr_stages(
    context: &BuildContext<'_>,
    base: &str,
    target: &BuildTarget,
    stages: &[CheckStage],
    run_tests: bool,
) -> BuildResult<()> {
    crate::compiler_inventory::invalidate_previous_evidence(context.root())?;
    let merge_base = context.git.merge_base(base)?;
    let changed = context.git.changed_files(&merge_base)?;
    let summary = format!(
        "{} file(s) changed since {base} ({merge_base})",
        changed.len()
    );
    context.reporter.note(&summary);
    if stages.contains(&CheckStage::Lint) {
        lint_changed_files(context, &merge_base, &changed)?;
        check_expanded_inventory(context, &changed, target)?;
    }
    let scope = CheckScope { stages, run_tests };
    check_affected_packages(context, &merge_base, &changed, target, scope)
}

/// Which package-level checks this run performs.
#[derive(Clone, Copy)]
struct CheckScope<'a> {
    stages: &'a [CheckStage],
    run_tests: bool,
}

impl CheckScope<'_> {
    fn has(&self, stage: CheckStage) -> bool {
        self.stages.contains(&stage)
    }
}

// Kept separate from the source ratchet: compiler capture cleans a dedicated build (#411).
fn check_expanded_inventory(
    context: &BuildContext<'_>,
    changed: &[ChangedFile],
    target: &BuildTarget,
) -> BuildResult<()> {
    if !changed
        .iter()
        .any(|file| crate::inventory_impact::expanded_input(&file.path))
    {
        crate::compiler_inventory::invalidate_previous_evidence(context.root())?;
        context
            .reporter
            .note("Expanded unsafe inventory skipped: no relevant input changed.");
        return Ok(());
    }
    context
        .reporter
        .step("Collect expanded unsafe diagnostics", || {
            crate::compiler_inventory::capture_inventory(context.runner, context.root(), target)
        })
}

#[cfg(test)]
#[path = "pr_inventory_tests.rs"]
mod inventory_tests;

struct PackageChecks {
    build: BTreeSet<String>,
    test: BTreeSet<String>,
}

fn check_affected_packages(
    context: &BuildContext<'_>,
    reference: &str,
    changed: &[ChangedFile],
    target: &BuildTarget,
    scope: CheckScope<'_>,
) -> BuildResult<()> {
    let checks = context.reporter.step("Find affected packages", || {
        affected(context, reference, changed)
    })?;
    let rustfmt_changed = changed.iter().any(|file| file.path == "rustfmt.toml");
    if scope.has(CheckStage::Lint) && rustfmt_changed {
        let workspace = CargoWorkspace::load(context.runner, context.root())?;
        CargoTasks::new(context.runner, context.root())
            .format_check_packages(&workspace.names())?;
    }
    if checks.build.is_empty() {
        context
            .reporter
            .note("No Rust package is affected; build and tests are skipped.");
        return Ok(());
    }
    run_package_checks(context, checks, target, scope)
}

fn run_package_checks(
    context: &BuildContext<'_>,
    checks: PackageChecks,
    target: &BuildTarget,
    scope: CheckScope<'_>,
) -> BuildResult<()> {
    let build = PackageSelection::Packages(checks.build.into_iter().collect());
    check_packages(context, &build, target, scope)?;
    if !scope.run_tests || checks.test.is_empty() {
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
    scope: CheckScope<'_>,
) -> BuildResult<()> {
    let cargo = CargoTasks::new(context.runner, context.root());
    let reporter = &context.reporter;
    if scope.has(CheckStage::Clippy) {
        reporter.step("Clippy affected packages", || {
            cargo.clippy(selection, target)
        })?;
    }
    if scope.has(CheckStage::Build) {
        reporter.step("Build affected packages", || {
            cargo.build_all_targets(selection, target)
        })?;
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
        assert!(commands.contains(&"git -c core.safecrlf=false diff --check base1".to_owned()));
    }

    const MATH_CHANGE: &str = "M\0crates/martensite-math/src/lib.rs\0";

    fn stage_commands(stage: CheckStage, diff: &str) -> Vec<String> {
        let runner = runner_with_changes(diff);
        let context = context_at(&runner, Path::new("/repo"));
        validate_pr_stages(&context, "origin/main", &BuildTarget::Host, &[stage], false).unwrap();
        runner.commands()
    }

    #[test]
    fn stage_names_parse_back_to_their_stage() {
        use clap::ValueEnum;
        for stage in CheckStage::ALL {
            assert_eq!(CheckStage::from_str(stage.name(), false), Ok(stage));
        }
    }

    #[test]
    fn each_ci_stage_runs_only_its_own_checks() {
        let ran =
            |commands: &[String], prefix: &str| commands.iter().any(|c| c.starts_with(prefix));
        let clippy = stage_commands(CheckStage::Clippy, MATH_CHANGE);
        assert!(ran(&clippy, "cargo clippy"));
        assert!(!ran(&clippy, "cargo build") && !ran(&clippy, "git -c core.safecrlf=false diff"));
        let build = stage_commands(CheckStage::Build, MATH_CHANGE);
        assert!(ran(&build, "cargo build") && !ran(&build, "cargo clippy"));
        assert!(!ran(&build, "cargo test"));
    }

    #[test]
    fn lint_stage_lints_files_without_package_checks() {
        // A documentation edit selects no package, so only the linters can run here.
        let commands = stage_commands(CheckStage::Lint, "M\0docs/readme.md\0");
        assert!(commands.contains(&"git -c core.safecrlf=false diff --check base1".to_owned()));
        assert!(
            commands
                .iter()
                .all(|c| !c.starts_with("cargo clippy") && !c.starts_with("cargo build"))
        );
    }

    #[test]
    fn delegated_tests_preserve_clippy_and_build() {
        let runner = RecordingRunner::default();
        check_packages(
            &context_at(&runner, Path::new("/repo")),
            &PackageSelection::Packages(vec!["martensite-math".into()]),
            &BuildTarget::Host,
            CheckScope {
                stages: &CheckStage::ALL,
                run_tests: false,
            },
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
            .with_exit_code("git -c core.safecrlf=false diff --check base1", 2);
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
    fn lint_configuration_runner() -> RecordingRunner {
        RecordingRunner::default()
            .with_stdout("git rev-parse --show-toplevel", "/repo")
            .with_stdout("git merge-base origin/main HEAD", "base")
            .with_stdout(
                "git diff --name-status --no-renames -z base",
                "M\0clippy.toml\0",
            )
            .with_stdout(
                "cargo metadata --format-version 1 --no-deps --locked",
                &crate::workspace::tests::sample_metadata(),
            )
    }

    #[test]
    fn lint_setting_change_checks_workspace_without_running_behavioral_suites() {
        let runner = lint_configuration_runner();
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
