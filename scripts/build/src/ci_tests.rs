//! Dependency-aware PR test jobs and their required aggregate gate (#250).

use std::collections::BTreeSet;

use crate::cargo::{BuildTarget, CargoTasks, PackageSelection};
use crate::context::BuildContext;
use crate::error::{BuildError, BuildResult};
use crate::plan::CiPlan;
use crate::workspace::CargoWorkspace;

fn selected_packages(context: &BuildContext<'_>, plan: &CiPlan) -> BuildResult<BTreeSet<String>> {
    let CiPlan::PullRequest { base } = plan else {
        return Err(BuildError::Usage(format!(
            "package test plan {plan:?}; expected PullRequest"
        )));
    };
    let merge_base = context.git.merge_base(base)?;
    let changed = context.git.changed_files(&merge_base)?;
    let workspace = CargoWorkspace::load(context.runner, context.root())?;
    crate::release_impact::pull_request_packages(context, &workspace, &merge_base, &changed)
}

/// Emits each affected package exactly once, including an empty docs-only matrix.
///
/// ```ignore
/// let outputs = matrix_outputs(&context, &plan)?;
/// ```
pub fn matrix_outputs(
    context: &BuildContext<'_>,
    plan: &CiPlan,
) -> BuildResult<Vec<(&'static str, String)>> {
    let packages = match plan {
        CiPlan::PullRequest { .. } => selected_packages(context, plan)?,
        CiPlan::Release { .. } => BTreeSet::new(),
    };
    Ok(vec![
        ("has-tests", (!packages.is_empty()).to_string()),
        (
            "test-matrix",
            serde_json::json!({"package": packages}).to_string(),
        ),
    ])
}

/// Revalidates membership before running unit, integration and documentation tests.
///
/// ```ignore
/// test_package(&context, &plan, "martensite-tkmath", &BuildTarget::Host)?;
/// ```
pub fn test_package(
    context: &BuildContext<'_>,
    plan: &CiPlan,
    package: &str,
    target: &BuildTarget,
) -> BuildResult<()> {
    if !selected_packages(context, plan)?.contains(package) {
        return Err(BuildError::Usage(format!(
            "test package {package:?}; expected a member of the affected PR package set"
        )));
    }
    CargoTasks::new(context.runner, context.root()).test(
        &PackageSelection::Packages(vec![package.to_owned()]),
        target,
    )
}

/// Fails closed on failed, cancelled or unexpectedly skipped matrix jobs.
///
/// ```ignore
/// require_success("success", "success", true)?;
/// ```
pub fn require_success(checks: &str, tests: &str, has_tests: bool) -> BuildResult<()> {
    let expected_tests = if has_tests { "success" } else { "skipped" };
    if checks == "success" && tests == expected_tests {
        return Ok(());
    }
    Err(BuildError::Usage(format!(
        "PR jobs: checks={checks:?}, tests={tests:?}; expected checks=success, tests={expected_tests}"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::tests::context_at;
    use crate::fake_runner::RecordingRunner;
    use crate::workspace::tests::sample_metadata;
    use std::path::Path;

    fn fixture(path: &str) -> (RecordingRunner, CiPlan) {
        let runner = RecordingRunner::default()
            .with_stdout("git merge-base origin/main HEAD", "base1")
            .with_stdout(
                "git diff --name-status --no-renames -z base1",
                &format!("M\0{path}\0"),
            )
            .with_stdout(
                "cargo metadata --format-version 1 --no-deps --locked",
                &sample_metadata(),
            );
        (
            runner,
            CiPlan::PullRequest {
                base: "origin/main".into(),
            },
        )
    }

    #[test]
    fn matrix_covers_affected_packages_once_without_base_dependencies() {
        for (path, expected) in [
            (
                "crates/martensite-kernel/src/lib.rs",
                vec!["martensite-kernel"],
            ),
            (
                "crates/martensite-math/src/lib.rs",
                vec!["martensite-kernel", "martensite-math"],
            ),
            ("README.md", vec![]),
        ] {
            assert_matrix(path, expected);
        }
    }

    fn assert_matrix(path: &str, expected: Vec<&str>) {
        let (runner, plan) = fixture(path);
        let outputs = matrix_outputs(&context_at(&runner, Path::new("/repo")), &plan).unwrap();
        assert_eq!(outputs[0].1, (!expected.is_empty()).to_string());
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&outputs[1].1).unwrap(),
            serde_json::json!({"package": expected})
        );
    }

    #[test]
    fn package_job_runs_full_test_command_and_rejects_unaffected_package() {
        let (runner, plan) = fixture("crates/martensite-kernel/src/lib.rs");
        let context = context_at(&runner, Path::new("/repo"));
        test_package(&context, &plan, "martensite-kernel", &BuildTarget::Host).unwrap();
        assert!(
            runner
                .commands()
                .contains(&"cargo test --locked --package martensite-kernel --no-fail-fast".into())
        );
        assert!(test_package(&context, &plan, "martensite-math", &BuildTarget::Host).is_err());
    }

    #[test]
    fn aggregate_rejects_missing_failed_and_cancelled_jobs() {
        assert!(require_success("success", "success", true).is_ok());
        assert!(require_success("success", "skipped", false).is_ok());
        for status in ["failure", "cancelled", "skipped", ""] {
            assert!(require_success(status, "success", true).is_err());
            assert!(require_success("success", status, true).is_err());
        }
    }

    #[test]
    fn release_has_no_pr_matrix_and_cannot_run_package_jobs() {
        let runner = RecordingRunner::default();
        let context = context_at(&runner, Path::new("/repo"));
        let plan = CiPlan::Release {
            version: semver::Version::new(0, 3, 6),
            publish: false,
        };
        assert_eq!(matrix_outputs(&context, &plan).unwrap()[0].1, "false");
        assert!(test_package(&context, &plan, "martensite-kernel", &BuildTarget::Host).is_err());
        assert_eq!(runner.commands(), ["git rev-parse --show-toplevel"]);
    }
}

#[cfg(test)]
mod bookkeeping_tests {
    use super::*;
    use crate::{context::tests::context_at, fake_runner::RecordingRunner};

    struct ManifestBookkeepingFixture {
        root: tempfile::TempDir,
        runner: RecordingRunner,
    }

    impl ManifestBookkeepingFixture {
        fn new() -> Self {
            let root = tempfile::tempdir().unwrap();
            let directory = root.path().join("crates/martensite-math");
            std::fs::create_dir_all(&directory).unwrap();
            std::fs::write(root.path().join("Cargo.toml"), "[workspace]").unwrap();
            std::fs::write(
                directory.join("Cargo.toml"),
                "[package]\nname='martensite-math'\nversion='0.3.1'",
            )
            .unwrap();
            let runner = bookkeeping_runner(root.path());
            Self { root, runner }
        }
    }

    fn bookkeeping_runner(root: &std::path::Path) -> RecordingRunner {
        RecordingRunner::default()
            .with_stdout("git rev-parse --show-toplevel", &root.display().to_string())
            .with_stdout("git merge-base origin/main HEAD", "base")
            .with_stdout(
                "git diff --name-status --no-renames -z base",
                "M\0crates/martensite-math/Cargo.toml\0",
            )
            .with_stdout(
                "cargo metadata --format-version 1 --no-deps --locked",
                &crate::workspace::tests::metadata_at(&root.display().to_string(), "0.3.1"),
            )
            .with_stdout("git show base:Cargo.toml", "[workspace]")
            .with_stdout(
                "git ls-tree -r --name-only -z base",
                "crates/martensite-math/Cargo.toml\0",
            )
            .with_stdout(
                "git show base:crates/martensite-math/Cargo.toml",
                "[package]\nname='martensite-math'\nversion='0.3.0'",
            )
    }

    #[test]
    fn version_only_manifest_does_not_add_a_base_dependency_to_pr_tests() {
        let fixture = ManifestBookkeepingFixture::new();
        let context = context_at(&fixture.runner, fixture.root.path());
        let plan = CiPlan::PullRequest {
            base: "origin/main".into(),
        };
        assert_eq!(matrix_outputs(&context, &plan).unwrap()[0].1, "false");
    }
}
