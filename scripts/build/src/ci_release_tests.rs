//! Release shard selection uses the previous release, never the PR base.

use super::*;
use crate::{context::tests::context_at, fake_runner::RecordingRunner};
use std::{fs, path::Path};

struct ReleaseScopeFixture {
    directory: tempfile::TempDir,
    runner: RecordingRunner,
}

impl ReleaseScopeFixture {
    fn new(changed: &str) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        fs::write(root.join("Cargo.toml"), "[workspace]").unwrap();
        for (name, version) in [("kernel", "0.3.1"), ("math", "0.3.0")] {
            let path = root.join(format!("crates/martensite-{name}"));
            fs::create_dir_all(&path).unwrap();
            let manifest = format!("[package]\nname='martensite-{name}'\nversion='{version}'");
            fs::write(path.join("Cargo.toml"), manifest).unwrap();
        }
        let runner = release_runner(root, changed);
        Self { directory, runner }
    }
}

fn release_runner(root: &Path, changed: &str) -> RecordingRunner {
    let runner = RecordingRunner::default()
        .with_stdout("git rev-parse --show-toplevel", &root.display().to_string())
        .with_stdout(
            "cargo metadata --format-version 1 --no-deps --locked",
            &crate::workspace::tests::metadata_at(&root.display().to_string(), "0.3.1"),
        )
        .with_stdout("git tag --list v*", "v0.3.0")
        .with_stdout("git show v0.3.0:Cargo.toml", "[workspace]")
        .with_stdout("git diff --name-status --no-renames -z v0.3.0", changed)
        .with_stdout(
            "git ls-tree -r --name-only -z v0.3.0",
            "crates/martensite-kernel/Cargo.toml\0crates/martensite-math/Cargo.toml\0",
        );
    crate::published_history::fixtures::seed_releases(previous_manifests(runner), "v0.3.0")
}

fn previous_manifests(mut runner: RecordingRunner) -> RecordingRunner {
    for name in ["kernel", "math"] {
        runner = runner.with_stdout(
            &format!("git show v0.3.0:crates/martensite-{name}/Cargo.toml"),
            &format!("[package]\nname='martensite-{name}'\nversion='0.3.0'"),
        );
    }
    runner
}

#[test]
fn leaf_release_shard_skips_unchanged_base_package_on_every_platform() {
    let fixture = ReleaseScopeFixture::new("M\0crates/martensite-kernel/src/lib.rs\0");
    let context = context_at(&fixture.runner, fixture.directory.path());
    let plan = CiPlan::Release {
        version: semver::Version::new(0, 3, 1),
        publish: false,
    };
    let outputs = matrix_outputs(&context, &plan).unwrap();
    let matrix: serde_json::Value = serde_json::from_str(&outputs[1].1).unwrap();
    assert_eq!(matrix["include"].as_array().unwrap().len(), 4);
    assert_leaf_tests_on_each_platform(&context, &plan, &fixture.runner);
    assert!(
        fixture
            .runner
            .commands()
            .iter()
            .all(|command| !command.contains("merge-base"))
    );
}

fn assert_leaf_tests_on_each_platform(
    context: &BuildContext<'_>,
    plan: &CiPlan,
    runner: &RecordingRunner,
) {
    for platform in crate::targets::RELEASE_TARGETS {
        let target = BuildTarget::Triple(platform.triple.into());
        test_shard(context, plan, 0, &target).unwrap();
        let command = format!(
            "cargo test --locked --package martensite-kernel --target {} --no-fail-fast",
            platform.triple
        );
        assert!(runner.commands().contains(&command));
    }
}

#[test]
fn empty_release_scope_has_no_test_jobs_and_rejects_shard_execution() {
    let fixture = unchanged_release();
    let context = context_at(&fixture.runner, fixture.directory.path());
    let plan = CiPlan::Release {
        version: semver::Version::new(0, 3, 1),
        publish: true,
    };
    let outputs = matrix_outputs(&context, &plan).unwrap();
    assert_eq!(outputs[0].1, "false");
    assert_eq!(outputs[1].1, "{\"include\":[]}");
    let target = BuildTarget::Triple(crate::targets::PRIMARY_TARGET.triple.into());
    assert!(test_shard(&context, &plan, 0, &target).is_err());
    let commands = fixture.runner.commands();
    assert!(
        commands
            .iter()
            .all(|command| !command.starts_with("cargo test"))
    );
}

fn unchanged_release() -> ReleaseScopeFixture {
    let fixture = ReleaseScopeFixture::new("M\0README.md\0");
    let manifest = fixture
        .directory
        .path()
        .join("crates/martensite-kernel/Cargo.toml");
    fs::write(
        manifest,
        "[package]\nname='martensite-kernel'\nversion='0.3.0'",
    )
    .unwrap();
    fixture
}
