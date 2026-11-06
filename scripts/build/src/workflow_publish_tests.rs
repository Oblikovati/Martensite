//! Publication status must bypass skipped ancestors without bypassing release gates (#350).
use crate::plan::{CiTrigger, plan_for, plan_outputs};

fn publish_job() -> String {
    // A Windows checkout has CRLF line endings.
    let workflow = include_str!("../../../.github/workflows/martensite.yml").replace("\r\n", "\n");
    workflow.split("\n  publish:\n").nth(1).unwrap().to_owned()
}

#[test]
fn publish_condition_overrides_ancestor_skip_and_requires_successful_gates() {
    let job = publish_job();
    assert!(job.contains("needs: [plan, checks, verify]"));
    for guard in [
        "!cancelled()",
        "github.event_name == 'push'",
        "github.ref == format('refs/tags/v{0}', needs.plan.outputs.version)",
        "needs.plan.result == 'success'",
        "needs.plan.outputs.publish == 'true'",
        "needs.checks.result == 'success'",
        "needs.verify.result == 'success'",
    ] {
        assert!(job.contains(guard), "missing publication guard {guard}");
    }
    assert!(!job.contains("needs.tests.result"));
}

#[test]
fn publication_keeps_only_job_scoped_write_permission_and_existing_publisher() {
    let job = publish_job();
    assert!(job.contains("permissions:\n      contents: write"));
    assert!(job.contains("run: cargo xtask ci publish --dist dist"));
    assert!(job.contains("GH_TOKEN: ${{ github.token }}"));
    assert!(!job.contains("gh release create"));
}

#[test]
fn only_tag_push_requests_publication_despite_equal_release_versions() {
    for (event, publish) in [
        ("push", "true"),
        ("workflow_dispatch", "false"),
        ("pull_request", "false"),
    ] {
        let trigger = CiTrigger {
            event: event.into(),
            git_ref: "refs/tags/v0.6.0".into(),
            base_sha: "a".repeat(40),
            dispatch_mode: "release".into(),
            dispatch_version: "0.6.0".into(),
        };
        let outputs = plan_outputs(&plan_for(&trigger).unwrap());
        let requested = &outputs.iter().find(|row| row.0 == "publish").unwrap().1;
        assert_eq!(requested, publish);
    }
}
