use super::{
    CaseFailure, CaseRunner, CaseSpec, FactorProfile,
    artifacts::{FilesystemReceipts, ReceiptRecorder},
    context::CaseContext,
    harness_tests::successful_output,
    launch::{ChildLauncher, ChildObservation, ChildRequest},
    runner::RunEnvironment,
};
use std::{
    cell::Cell,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub(crate) struct OwnedArtifactRoot(PathBuf);

impl OwnedArtifactRoot {
    pub(crate) fn new() -> Self {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        for collision in 0..64 {
            let path = std::env::temp_dir().join(format!(
                "martensite-runner-control-{}-{stamp}-{collision}",
                std::process::id()
            ));
            if fs::create_dir(&path).is_ok() {
                return Self(path);
            }
        }
        panic!("64 control directory collisions: expected an exclusive artifact root")
    }

    pub(crate) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for OwnedArtifactRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Clone, Copy)]
pub(super) enum ChildScenario {
    Success,
    LaunchFailure,
    ExitFailure,
    Signal,
    MissingEntry,
    MissingCompletion,
    WrongCompletion,
    ZeroCases,
    ExtraCases,
    Ignored,
    Malformed,
    Truncated,
}

pub(super) struct NamedFakeChild {
    scenario: ChildScenario,
    launches: Cell<usize>,
}

impl NamedFakeChild {
    pub(super) fn new(scenario: ChildScenario) -> Self {
        Self {
            scenario,
            launches: Cell::new(0),
        }
    }

    fn records(&self, request: &ChildRequest<'_>) -> Result<(), io::Error> {
        if !matches!(self.scenario, ChildScenario::MissingEntry) {
            fs::write(
                request.context.directory.join("entry.txt"),
                &request.context.request,
            )?;
        }
        if matches!(self.scenario, ChildScenario::MissingCompletion) {
            return Ok(());
        }
        let complete = if matches!(self.scenario, ChildScenario::WrongCompletion) {
            b"wrong identity and budget".as_slice()
        } else {
            &request.context.request
        };
        fs::write(request.context.directory.join("complete.txt"), complete)
    }

    fn stdout(&self, identity: &str) -> Vec<u8> {
        let text = String::from_utf8(successful_output(identity, "original stdout")).unwrap();
        match self.scenario {
            ChildScenario::ZeroCases => text
                .replace("running 1 test", "running 0 tests")
                .into_bytes(),
            ChildScenario::ExtraCases => text
                .replace("running 1 test", "running 2 tests")
                .into_bytes(),
            ChildScenario::Ignored => text.replace("0 ignored", "1 ignored").into_bytes(),
            ChildScenario::Malformed => vec![0xff],
            ChildScenario::Truncated => text.as_bytes()[..text.len() / 2].to_vec(),
            _ => text.into_bytes(),
        }
    }
}

impl ChildLauncher for NamedFakeChild {
    fn launch(&self, request: &ChildRequest<'_>) -> Result<ChildObservation, io::Error> {
        self.launches.set(self.launches.get() + 1);
        if matches!(self.scenario, ChildScenario::LaunchFailure) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "named launcher refusal",
            ));
        }
        self.records(request)?;
        let (code, signal) = fake_status(self.scenario);
        Ok(ChildObservation {
            code,
            signal,
            stdout: self.stdout(request.spec.identity),
            stderr: b"original stderr\n".to_vec(),
        })
    }
}

struct RefusingReceiptRecorder;

impl ReceiptRecorder for RefusingReceiptRecorder {
    fn write(&self, path: &Path, _bytes: &[u8]) -> Result<(), io::Error> {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("named recorder refuses {}", path.display()),
        ))
    }
}

fn spec() -> CaseSpec<'static> {
    CaseSpec::new("original::point", 60, FactorProfile::RequireUnset)
}

#[test]
fn successful_named_child_retains_raw_logs_without_running_parent_body() {
    let root = OwnedArtifactRoot::new();
    let launcher = NamedFakeChild::new(ChildScenario::Success);
    let body_ran = Cell::new(false);
    let receipt = CaseRunner::new(root.path(), None)
        .run_with(
            spec(),
            || {
                body_ran.set(true);
                Ok(())
            },
            &RunEnvironment::default(),
            &launcher,
            &FilesystemReceipts,
        )
        .unwrap();
    assert!(!body_ran.get());
    assert_eq!(launcher.launches.get(), 1);
    assert_retained_receipt(&receipt);
}

#[test]
fn named_launch_protocol_exit_and_signal_failures_never_accept_a_case() {
    for scenario in rejected_scenarios() {
        let root = OwnedArtifactRoot::new();
        let launcher = NamedFakeChild::new(scenario);
        assert!(invoke_named_child(&root, &launcher, &FilesystemReceipts).is_err());
        assert_eq!(launcher.launches.get(), 1);
    }
}

#[test]
fn recorder_refusal_keeps_original_exit_and_raw_diagnostics_primary() {
    let root = OwnedArtifactRoot::new();
    let launcher = NamedFakeChild::new(ChildScenario::ExitFailure);
    let error = invoke_named_child(&root, &launcher, &RefusingReceiptRecorder).unwrap_err();
    assert!(matches!(
        *error.failure,
        CaseFailure::ChildExit {
            code: Some(42),
            signal: None
        }
    ));
    assert_eq!(error.recording_failures.len(), 3);
    assert!(error.to_string().contains("original stderr"));
    assert!(error.to_string().contains("original stdout"));
}

#[test]
fn successful_child_cannot_pass_when_receipt_recording_fails() {
    let root = OwnedArtifactRoot::new();
    let launcher = NamedFakeChild::new(ChildScenario::Success);
    let error = CaseRunner::new(root.path(), None)
        .run_with(
            spec(),
            || Ok(()),
            &RunEnvironment::default(),
            &launcher,
            &RefusingReceiptRecorder,
        )
        .unwrap_err();
    assert!(matches!(*error.failure, CaseFailure::Io { .. }));
    assert!(error.to_string().contains("retain raw child logs"));
}

#[test]
fn present_factor_including_empty_is_rejected_without_launch_or_body() {
    for factor in ["", "2", "1+1"] {
        assert_rejected_factor(factor);
    }
}

#[test]
fn invalid_spec_and_unknown_marker_fail_before_policy_or_fixture() {
    let root = OwnedArtifactRoot::new();
    let launcher = NamedFakeChild::new(ChildScenario::Success);
    for invalid in invalid_specs() {
        assert!(
            CaseRunner::new(root.path(), None)
                .run_with(
                    invalid,
                    || panic!("fixture"),
                    &RunEnvironment::default(),
                    &launcher,
                    &FilesystemReceipts
                )
                .is_err()
        );
    }
    assert_unknown_marker_rejected(&root, &launcher);
    assert_eq!(launcher.launches.get(), 0);
}

#[test]
fn child_entry_is_single_use_and_bound_to_identity_and_budget() {
    let root = OwnedArtifactRoot::new();
    let context = CaseContext::create(root.path(), spec()).unwrap();
    assert_wrong_bindings_rejected(&context);
    context.claim_entry().unwrap();
    assert!(context.claim_entry().is_err());
    context.complete().unwrap();
    assert!(context.complete().is_err());
    context.require_record("entry.txt").unwrap();
    context.require_same_executable().unwrap();
}

#[test]
fn receipt_json_escaping_keeps_controls_quotes_and_unicode() {
    assert_eq!(
        super::artifacts::json_string("\"\\\n\tλ"),
        "\"\\\"\\\\\\u000a\\u0009λ\""
    );
    assert_eq!(super::context::hex(&[0, 255, 16]), "00ff10");
}

fn fake_status(scenario: ChildScenario) -> (Option<i32>, Option<i32>) {
    match scenario {
        ChildScenario::ExitFailure => (Some(42), None),
        ChildScenario::Signal => (None, Some(15)),
        _ => (Some(0), None),
    }
}

fn invoke_named_child(
    root: &OwnedArtifactRoot,
    launcher: &NamedFakeChild,
    recorder: &dyn ReceiptRecorder,
) -> Result<super::CaseReceipt, super::CaseRunError> {
    CaseRunner::new(root.path(), None).run_with(
        spec(),
        || panic!("parent body must not run"),
        &RunEnvironment::default(),
        launcher,
        recorder,
    )
}

fn rejected_scenarios() -> [ChildScenario; 11] {
    [
        ChildScenario::LaunchFailure,
        ChildScenario::ExitFailure,
        ChildScenario::Signal,
        ChildScenario::MissingEntry,
        ChildScenario::MissingCompletion,
        ChildScenario::WrongCompletion,
        ChildScenario::ZeroCases,
        ChildScenario::ExtraCases,
        ChildScenario::Ignored,
        ChildScenario::Malformed,
        ChildScenario::Truncated,
    ]
}

fn assert_retained_receipt(receipt: &super::CaseReceipt) {
    let stdout = fs::read_to_string(receipt.directory.join("stdout.log")).unwrap();
    assert!(stdout.contains("original stdout"));
    assert_eq!(
        fs::read(receipt.directory.join("stderr.log")).unwrap(),
        b"original stderr\n"
    );
    let summary = fs::read_to_string(receipt.directory.join("receipt.json")).unwrap();
    assert!(summary.contains("\"accepted\":true"));
}

fn assert_rejected_factor(factor: &str) {
    let root = OwnedArtifactRoot::new();
    let launcher = NamedFakeChild::new(ChildScenario::Success);
    let environment = RunEnvironment {
        factor: Some(factor.into()),
        marker: None,
    };
    let error = CaseRunner::new(root.path(), None)
        .run_with(
            spec(),
            || panic!("body must not run"),
            &environment,
            &launcher,
            &FilesystemReceipts,
        )
        .unwrap_err();
    assert!(matches!(*error.failure, CaseFailure::FactorPresent(_)));
    assert_eq!(launcher.launches.get(), 0);
}

fn invalid_specs() -> [CaseSpec<'static>; 5] {
    [
        CaseSpec::new("", 60, FactorProfile::RequireUnset),
        CaseSpec::new("--ignored", 60, FactorProfile::RequireUnset),
        CaseSpec::new("point", 0, FactorProfile::RequireUnset),
        CaseSpec::new("space case", 60, FactorProfile::RequireUnset),
        CaseSpec::new("control\ncase", 60, FactorProfile::RequireUnset),
    ]
}

fn assert_unknown_marker_rejected(root: &OwnedArtifactRoot, launcher: &NamedFakeChild) {
    let environment = RunEnvironment {
        marker: Some(root.path().join("unknown").into_os_string()),
        factor: None,
    };
    assert!(
        CaseRunner::new(root.path(), None)
            .run_with(
                spec(),
                || panic!("fixture"),
                &environment,
                launcher,
                &FilesystemReceipts
            )
            .is_err()
    );
}

fn assert_wrong_bindings_rejected(context: &CaseContext) {
    for wrong in [
        CaseSpec::new("other", 60, FactorProfile::RequireUnset),
        CaseSpec::new("original::point", 61, FactorProfile::RequireUnset),
    ] {
        assert!(CaseContext::load(context.directory.clone(), wrong).is_err());
    }
}
