//! Named API fixture exercises discovery and restoration without GitHub writes.
use super::{
    Artifact, RECEIPT, WORKFLOW, archive_tests::ArtifactRunner, find_rehearsal, receipt,
    request::ValidationRequest, scope,
};
use crate::{
    context::BuildContext,
    fake_runner::RecordingRunner,
    git::GitRepo,
    process::{CommandRunner, CommandSpec, ProcessRunner},
    report::{LogFlavor, StepReporter},
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};

struct RehearsalApiFixture {
    directory: tempfile::TempDir,
    run: Value,
    jobs: Value,
    receipt: Value,
    dist: Artifact,
    dist_bytes: Vec<u8>,
}

impl RehearsalApiFixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let payload = "owned source payload";
        let hash = format!("{:x}  payload.crate\n", Sha256::digest(payload.as_bytes()));
        let dist_bytes = make_zip(
            directory.path(),
            json!({"payload.crate": payload, "payload.crate.sha256": hash}),
        );
        let dist = artifact(11, "dist-x86_64-unknown-linux-gnu", &dist_bytes);
        Self {
            directory,
            run: run_json(),
            jobs: json!({"jobs": [job("checks (linux)"), job("release")]}),
            receipt: json!({"schema":1, "version":"0.13.1", "commit":"a".repeat(40), "run":21, "attempt":2, "artifacts":[dist], "scope":scope::fixture()}),
            dist,
            dist_bytes,
        }
    }

    fn request(&self) -> ValidationRequest {
        ValidationRequest {
            repo: "owner/repo".into(),
            head: "a".repeat(40),
            version: semver::Version::new(0, 13, 1),
            scope: scope::fixture(),
            jobs: BTreeSet::from(["checks (linux)".into(), "release".into()]),
        }
    }

    fn runner(&self) -> ArtifactRunner {
        let receipt_bytes = make_zip(
            self.directory.path(),
            json!({RECEIPT: self.receipt.to_string()}),
        );
        let receipt = artifact(12, "release-validation-v0.13.1-attempt-2", &receipt_bytes);
        let runner = RecordingRunner::default().with_stdout("git rev-parse --show-toplevel", &self.directory.path().display().to_string())
            .with_stdout(&format!("gh api repos/owner/repo/actions/workflows/martensite.yml/runs?event=workflow_dispatch&head_sha={}&status=success&per_page=100&page=1", "a".repeat(40)), &json!({"workflow_runs":[self.run]}).to_string())
            .with_stdout("gh api repos/owner/repo/actions/runs/21", &self.run.to_string())
            .with_stdout("gh api repos/owner/repo/actions/runs/21/artifacts?per_page=100&page=1", &json!({"artifacts":[self.dist, receipt]}).to_string())
            .with_stdout("gh api repos/owner/repo/actions/runs/21/attempts/2/jobs?filter=all&per_page=100&page=1", &self.jobs.to_string())
            .with_bytes("gh api repos/owner/repo/actions/artifacts/12/zip", receipt_bytes)
            .with_bytes("gh api repos/owner/repo/actions/artifacts/11/zip", self.dist_bytes.clone());
        ArtifactRunner(runner)
    }

    fn context<'a>(&self, runner: &'a ArtifactRunner) -> BuildContext<'a> {
        BuildContext {
            runner,
            git: GitRepo::discover(runner, self.directory.path()).unwrap(),
            reporter: StepReporter::new(LogFlavor::Terminal),
        }
    }
}

fn make_zip(root: &Path, files: Value) -> Vec<u8> {
    let input = root.join("entries.json");
    let zip = root.join("generated.zip");
    fs::write(&input, files.to_string()).unwrap();
    let script = "import sys,zipfile,json\nwith open(sys.argv[1]) as f: entries=json.load(f)\nwith zipfile.ZipFile(sys.argv[2],'w') as z:\n for name,body in entries.items(): z.writestr(name,body)";
    ProcessRunner
        .run(&CommandSpec::new("python3", root).args([
            "-I",
            "-c",
            script,
            &input.display().to_string(),
            &zip.display().to_string(),
        ]))
        .unwrap();
    fs::read(zip).unwrap()
}

fn artifact(id: u64, name: &str, bytes: &[u8]) -> Artifact {
    Artifact {
        id,
        name: name.into(),
        digest: format!("sha256:{:x}", Sha256::digest(bytes)),
        size_in_bytes: bytes.len() as u64,
        expired: false,
    }
}

fn run_json() -> Value {
    json!({"id":21, "run_attempt":2, "head_sha":"a".repeat(40), "path":WORKFLOW, "event":"workflow_dispatch", "status":"completed", "conclusion":"success", "repository":{"full_name":"owner/repo"}, "head_repository":{"full_name":"owner/repo"}})
}

fn job(name: &str) -> Value {
    json!({"name":name, "status":"completed", "conclusion":"success"})
}

#[test]
fn discovery_then_restore_reuses_exact_checked_payload_without_publication() {
    let fixture = RehearsalApiFixture::new();
    let runner = fixture.runner();
    let context = fixture.context(&runner);
    let request = fixture.request();
    assert_eq!(find_rehearsal(&context, &request).unwrap(), Some(21));
    let dist = fixture.directory.path().join("restored");
    receipt::restore_request(&context, &request, 21, &dist).unwrap();
    assert_eq!(
        fs::read_to_string(dist.join("payload.crate")).unwrap(),
        "owned source payload"
    );
    assert!(
        !runner
            .0
            .commands()
            .iter()
            .any(|command| command.starts_with("gh release"))
    );
}

#[test]
fn ineligible_evidence_falls_back_without_restoring_unvalidated_payload() {
    for failure in [
        "pr",
        "sha",
        "failure",
        "version",
        "scope",
        "controls",
        "attempt",
        "missing-job",
        "expired",
        "substitution",
    ] {
        assert_refused(failure);
    }
}

fn assert_refused(failure: &str) {
    let mut fixture = RehearsalApiFixture::new();
    invalidate(&mut fixture, failure);
    let runner = fixture.runner();
    let context = fixture.context(&runner);
    assert_eq!(
        find_rehearsal(&context, &fixture.request()).unwrap(),
        None,
        "{failure}"
    );
    let dist = fixture.directory.path().join("restored");
    assert!(
        receipt::restore_request(&context, &fixture.request(), 21, &dist).is_err(),
        "{failure}"
    );
    assert!(!dist.join("payload.crate").exists());
}

fn invalidate(fixture: &mut RehearsalApiFixture, failure: &str) {
    match failure {
        "pr" => fixture.run["event"] = "pull_request".into(),
        "sha" => fixture.run["head_sha"] = "b".repeat(40).into(),
        "failure" => fixture.run["conclusion"] = "failure".into(),
        "version" => fixture.receipt["version"] = "0.13.2".into(),
        "controls" => fixture.receipt["scope"]["sanitizer_controls"][0] = "groups".into(),
        "scope" => fixture.receipt["scope"]["tests"][1] = "different".into(),
        "attempt" => fixture.receipt["attempt"] = 1.into(),
        "missing-job" => fixture.jobs["jobs"].as_array_mut().unwrap().clear(),
        "expired" => fixture.dist.expired = true,
        _ => fixture.dist.id += 1,
    }
}
