//! Shared PR/release scope selects only affected sanitizer owners (#410).
use super::tests::fixture;
use super::{matrix_outputs, require_success, sanitizer_controls};
use crate::{
    context::tests::context_at, fake_runner::RecordingRunner, plan::CiPlan,
    workspace::tests::sample_metadata,
};
use std::path::Path;

fn sanitizer_fixture(path: &str) -> (RecordingRunner, CiPlan) {
    let (runner, plan) = fixture(path);
    let metadata = sample_metadata()
        .replace("martensite-math", "martensite-tkg2d")
        .replace("martensite-kernel", "martensite-tkg3d");
    (
        runner.with_stdout(
            "cargo metadata --format-version 1 --no-deps --locked",
            &metadata,
        ),
        plan,
    )
}

#[test]
fn sanitizer_scope_tracks_dependencies_and_build_inputs() {
    use crate::thread_sanitizer_selection::Control::{Cache2d, Cache3d};
    for (path, expected) in [
        ("README.md", vec![]),
        ("crates/martensite-tkg3d/src/lib.rs", vec![Cache3d]),
        ("crates/martensite-tkg2d/src/lib.rs", vec![Cache2d, Cache3d]),
        ("rust-toolchain.toml", vec![Cache2d, Cache3d]),
        (".cargo/config.toml", vec![Cache2d, Cache3d]),
    ] {
        let (runner, plan) = sanitizer_fixture(path);
        let context = context_at(&runner, Path::new("/repo"));
        assert_eq!(sanitizer_controls(&context, &plan).unwrap(), expected);
        let outputs = matrix_outputs(&context, &plan).unwrap();
        assert_eq!(
            outputs[2],
            ("has-sanitizer", (!expected.is_empty()).to_string())
        );
    }
}

#[test]
fn sanitizer_selection_uses_pinned_pr_base() {
    let sha = "a".repeat(40);
    let (runner, _) = sanitizer_fixture("crates/martensite-tkg3d/src/lib.rs");
    let runner = runner.with_stdout(&format!("git merge-base {sha} HEAD"), "base1");
    let plan = CiPlan::PullRequest { base: sha };
    let context = context_at(&runner, Path::new("/repo"));
    assert_eq!(
        sanitizer_controls(&context, &plan).unwrap(),
        vec![crate::thread_sanitizer_selection::Control::Cache3d]
    );
    assert!(
        !runner
            .commands()
            .iter()
            .any(|command| command.contains("origin/main"))
    );
}

#[test]
fn sanitizer_initial_release_uses_release_package_scope() {
    let (runner, _) = sanitizer_fixture("README.md");
    let runner = crate::published_history::fixtures::seed_releases(runner, "");
    let plan = CiPlan::Release {
        version: semver::Version::new(0, 3, 6),
        publish: false,
    };
    let context = context_at(&runner, Path::new("/repo"));
    assert_eq!(
        sanitizer_controls(&context, &plan).unwrap(),
        vec![
            crate::thread_sanitizer_selection::Control::Cache2d,
            crate::thread_sanitizer_selection::Control::Cache3d,
        ]
    );
}

#[test]
fn sanitizer_aggregate_requires_exact_planned_job_status() {
    for (planned, status, accepted) in [
        (false, "skipped", true),
        (false, "success", false),
        (true, "success", true),
        (true, "skipped", false),
        (true, "failure", false),
        (false, "failure", false),
        (true, "cancelled", false),
        (false, "cancelled", false),
    ] {
        assert_eq!(
            require_success("success", status, planned).is_ok(),
            accepted
        );
        assert!(require_success("failure", status, planned).is_err());
    }
}

struct AdvancingManualBase {
    runner: RecordingRunner,
    baseline: String,
}

impl AdvancingManualBase {
    fn new() -> Self {
        let baseline = "a".repeat(40);
        let (runner, _) = sanitizer_fixture(".github/workflows/martensite.yml");
        let runner = runner
            .with_stdout_sequence(
                "git merge-base origin/main HEAD",
                vec![baseline.clone(), "b".repeat(40)],
            )
            .with_stdout(&format!("git merge-base {baseline} HEAD"), &baseline)
            .with_stdout(
                &format!("git diff --name-status --no-renames -z {baseline}"),
                "M\0.github/workflows/martensite.yml\0",
            );
        Self { runner, baseline }
    }

    fn trigger(&self, pinned: bool) -> crate::plan::CiTrigger {
        crate::plan::CiTrigger {
            event: "workflow_dispatch".into(),
            dispatch_mode: "pr".into(),
            base_sha: if pinned {
                self.baseline.clone()
            } else {
                String::new()
            },
            ..crate::plan::CiTrigger::default()
        }
    }
}

#[test]
fn manual_workflow_only_selection_survives_main_advancing_after_plan() {
    let fixture = AdvancingManualBase::new();
    let context = context_at(&fixture.runner, Path::new("/repo"));
    let manual = crate::plan::plan_for(&fixture.trigger(false)).unwrap();
    let pinned = crate::plan::pin_comparison(&context, manual.clone()).unwrap();
    let outputs = crate::plan::plan_outputs(&pinned);
    assert!(outputs.contains(&("base-sha", fixture.baseline.clone())));
    let selections = matrix_outputs(&context, &pinned).unwrap();
    assert!(selections.contains(&("has-tests", "false".into())));
    assert!(selections.contains(&("has-sanitizer", "true".into())));
    let execution = crate::plan::plan_for(&fixture.trigger(true)).unwrap();
    assert_eq!(
        sanitizer_controls(&context, &execution).unwrap(),
        crate::thread_sanitizer_selection::CONTROLS
    );
    assert!(sanitizer_controls(&context, &manual).unwrap().is_empty());
}

#[test]
fn workflow_consumers_receive_the_planners_comparison_commit() {
    let workflow = include_str!("../../../.github/workflows/martensite.yml");
    assert!(workflow.contains("base-sha: ${{ steps.plan.outputs.base-sha }}"));
    for job in ["checks", "tests", "thread-sanitizer"] {
        let section = workflow.split(&format!("\n  {job}:\n")).nth(1).unwrap();
        let environment = section.split("    steps:").next().unwrap();
        assert!(
            environment.contains("MARTENSITE_CI_BASE_SHA: ${{ needs.plan.outputs.base-sha }}"),
            "{job}"
        );
    }
}
