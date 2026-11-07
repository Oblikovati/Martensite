use super::*;

fn fixture() -> (Receipt, Run, Vec<Artifact>) {
    let run = super::super::tests::run();
    let artifacts = vec![Artifact {
        id: 9,
        name: "dist-x86_64-unknown-linux-gnu".into(),
        digest: format!("sha256:{}", "a".repeat(64)),
        size_in_bytes: 100,
        expired: false,
    }];
    let receipt = Receipt {
        schema: 1,
        version: "0.13.1".into(),
        commit: run.head_sha.clone(),
        run: run.id,
        attempt: run.run_attempt,
        artifacts: artifacts.clone(),
        scope: super::super::scope::fixture(),
    };
    (receipt, run, artifacts)
}

#[test]
fn receipt_requires_same_version_commit_attempt_and_immutable_artifact_set() {
    let version = semver::Version::new(0, 13, 1);
    let (receipt, run, artifacts) = fixture();
    assert_eq!(
        check_receipt(receipt, &run, &version, &artifacts).unwrap(),
        artifacts
    );
    for field in [
        "version", "commit", "run", "attempt", "schema", "artifact", "expired",
    ] {
        assert_changed_receipt_refused(field, &version);
    }
}

fn assert_changed_receipt_refused(field: &str, version: &semver::Version) {
    let (mut receipt, run, mut artifacts) = fixture();
    match field {
        "version" => receipt.version = "0.13.2".into(),
        "commit" => receipt.commit = "b".repeat(40),
        "run" => receipt.run += 1,
        "attempt" => receipt.attempt += 1,
        "schema" => receipt.schema += 1,
        "artifact" => artifacts[0].id += 1,
        _ => artifacts[0].expired = true,
    }
    assert!(
        check_receipt(receipt, &run, version, &artifacts).is_err(),
        "{field}"
    );
}

#[test]
fn required_jobs_come_from_current_plan_including_conditional_arm_gate() {
    let matrix = serde_json::json!({"include": [{"target": "x86_64-unknown-linux-gnu", "shard": 0}, {"target": "x86_64-pc-windows-msvc", "shard": 1}]});
    let mut outputs = vec![
        ("test-matrix", matrix.to_string()),
        ("renderer-admission", "true".into()),
        ("has-sanitizer", "true".into()),
    ];
    let names = job_names(&outputs).unwrap();
    for expected in [
        "plan",
        "release",
        "renderer-arm64",
        "thread sanitizer (x86_64-unknown-linux-gnu)",
        "checks (aarch64-apple-darwin)",
        "release tests (x86_64-pc-windows-msvc, shard 1)",
    ] {
        assert!(names.contains(expected), "{expected}");
    }
    outputs[1].1 = "false".into();
    assert!(!job_names(&outputs).unwrap().contains("renderer-arm64"));
    outputs[2].1 = "false".into();
    assert!(
        !job_names(&outputs)
            .unwrap()
            .contains("thread sanitizer (x86_64-unknown-linux-gnu)")
    );
    outputs[2].1 = "unknown".into();
    assert!(job_names(&outputs).is_err());
    outputs.pop();
    assert!(job_names(&outputs).is_err());
    assert!(job_names(&[]).is_err());
}

#[test]
fn distribution_rejects_duplicates_expiry_and_missing_primary() {
    let (_, _, mut artifacts) = fixture();
    assert!(distribution(&artifacts).is_ok());
    artifacts.push(artifacts[0].clone());
    assert!(distribution(&artifacts).is_err());
    artifacts.pop();
    artifacts[0].name = "dist-other".into();
    assert!(distribution(&artifacts).is_err());
}
