use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub(super) fn run() -> Run {
    Run {
        id: 21,
        run_attempt: 2,
        head_sha: "a".repeat(40),
        path: WORKFLOW.into(),
        event: "workflow_dispatch".into(),
        status: "completed".into(),
        conclusion: Some("success".into()),
        repository: Repository {
            full_name: "owner/repo".into(),
        },
        head_repository: Repository {
            full_name: "owner/repo".into(),
        },
    }
}

#[test]
fn only_same_repository_successful_exact_commit_rehearsals_qualify() {
    let valid = run();
    assert!(run_matches(&valid, "owner/repo", &"a".repeat(40)));
    assert!(!run_matches(&valid, "owner/other", &"a".repeat(40)));
    assert!(!run_matches(&valid, "owner/repo", &"b".repeat(40)));
    let mut fork = run();
    fork.head_repository.full_name = "fork/repo".into();
    assert!(!run_matches(&fork, "owner/repo", &"a".repeat(40)));
    let mut other = run();
    other.path = "other.yml".into();
    assert!(!run_matches(&other, "owner/repo", &"a".repeat(40)));
}

#[test]
fn non_rehearsal_events_and_unsuccessful_runs_are_refused() {
    for event in ["pull_request", "push"] {
        let mut candidate = run();
        candidate.event = event.into();
        assert!(!run_matches(&candidate, "owner/repo", &"a".repeat(40)));
    }
    for conclusion in [None, Some("failure"), Some("cancelled"), Some("skipped")] {
        let mut candidate = run();
        candidate.conclusion = conclusion.map(str::to_owned);
        assert!(!run_matches(&candidate, "owner/repo", &"a".repeat(40)));
    }
}

#[test]
fn every_required_job_must_complete_successfully_once() {
    let required = BTreeSet::from(["checks (linux)".into(), "release".into()]);
    let mut jobs = vec![successful_job("checks (linux)"), successful_job("release")];
    assert!(receipt::jobs_complete(&jobs, &required, false));
    jobs[1].status = "in_progress".into();
    jobs[1].conclusion = None;
    assert!(!receipt::jobs_complete(&jobs, &required, false));
    assert!(receipt::jobs_complete(&jobs, &required, true));
    jobs[0].conclusion = Some("cancelled".into());
    assert!(!receipt::jobs_complete(&jobs, &required, true));
    jobs.pop();
    assert!(!receipt::jobs_complete(&jobs, &required, true));
}

fn successful_job(name: &str) -> Job {
    Job {
        name: name.into(),
        status: "completed".into(),
        conclusion: Some("success".into()),
    }
}

#[test]
fn artifact_bytes_must_match_the_github_digest_and_size() {
    let bytes = b"authentic zip";
    let mut artifact = Artifact {
        id: 1,
        name: "dist-linux".into(),
        digest: format!("sha256:{:x}", Sha256::digest(bytes)),
        size_in_bytes: bytes.len() as u64,
        expired: false,
    };
    archive::verify_bytes(&artifact, bytes).unwrap();
    assert!(archive::verify_bytes(&artifact, b"substitution").is_err());
    artifact.size_in_bytes += 1;
    assert!(archive::verify_bytes(&artifact, bytes).is_err());
}

#[test]
fn artifact_entries_cannot_escape_or_duplicate_payloads() {
    assert_eq!(
        archive::names("m.crate\nm.crate.sha256\n").unwrap(),
        ["m.crate", "m.crate.sha256"]
    );
    for listing in [
        "../escape",
        "/absolute",
        "nested/file",
        "a\na",
        "a\n\nb",
        "a b",
        "a*",
        "",
        "a\rb",
        ".hidden",
    ] {
        assert!(archive::names(listing).is_err(), "{listing:?}");
    }
}
