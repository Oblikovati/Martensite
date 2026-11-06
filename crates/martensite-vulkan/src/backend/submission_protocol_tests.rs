//! Named driver failures prove queue/fence ordering without a physical device.
use super::*;
use std::collections::BTreeMap;
#[derive(Default)]
struct FakeSubmissionDispatch {
    failures: BTreeMap<&'static str, i32>,
    events: Vec<&'static str>,
}
impl FakeSubmissionDispatch {
    fn call(&mut self, stage: &'static str) -> Result<(), RenderError> {
        self.events.push(stage);
        match self.failures.get(stage) {
            Some(code) => Err(RenderError::device_backend(
                stage,
                *code,
                format!("fake {stage}: {code}"),
            )),
            None => Ok(()),
        }
    }
}
impl SubmissionDispatch for FakeSubmissionDispatch {
    fn end_commands(&mut self) -> Result<(), RenderError> {
        // This boundary models the actual end operation, before queue submission.
        self.call("end")
    }
    fn submit_commands(&mut self) -> Result<(), RenderError> {
        // Failed submit never grants the protocol ownership of queued work.
        self.call("submit")
    }
    fn wait_fence(&mut self) -> Result<(), RenderError> {
        // A wait is meaningful only after the recorded submit succeeded.
        self.call("fence")
    }
}
impl CompletionDispatch for FakeSubmissionDispatch {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        // Idle is the fallback completion observation, not a second submission.
        self.call("idle")
    }
    fn retry_pause(&mut self) {
        // Normal explicit retirement is one attempt; only blocking Drop requests pauses.
        self.events.push("pause");
    }
}
#[test]
fn one_shot_submission_rejects_a_second_finish_before_dispatch() {
    let protocol = SubmissionProtocol::default();
    let mut calls = FakeSubmissionDispatch::default();
    protocol.finish(&mut calls).unwrap();
    assert_eq!(protocol.phase(), SubmissionPhase::Retired);
    assert!(protocol.finish(&mut calls).is_err());
    protocol.retire(&mut calls).unwrap();
    assert_eq!(calls.events, ["end", "submit", "fence"]);
}
#[test]
fn failed_end_or_submit_never_waits_an_unsignalled_fence() {
    for (stage, expected) in [("end", vec!["end"]), ("submit", vec!["end", "submit"])] {
        let protocol = SubmissionProtocol::default();
        let mut calls = FakeSubmissionDispatch::default();
        calls.failures.insert(stage, -1);
        assert!(protocol.finish(&mut calls).is_err());
        assert_eq!(protocol.phase(), SubmissionPhase::NotSubmitted);
        protocol.retire(&mut calls).unwrap();
        assert_eq!(calls.events, expected);
    }
}
#[test]
fn failed_fence_with_successful_idle_preserves_primary_and_proves_retirement() {
    let protocol = SubmissionProtocol::default();
    let mut calls = FakeSubmissionDispatch::default();
    calls.failures.insert("fence", -1);
    let error = protocol.finish(&mut calls).unwrap_err();
    assert_eq!(error.device_failure().unwrap().operation(), "fence");
    assert!(error.observed_failure().is_none());
    assert_eq!(protocol.phase(), SubmissionPhase::Retired);
    assert_eq!(calls.events, ["end", "submit", "fence", "idle"]);
}
#[test]
fn unknown_idle_failure_remains_incomplete_and_retains_both_causes() {
    let protocol = SubmissionProtocol::default();
    let mut calls = FakeSubmissionDispatch::default();
    calls.failures.extend([("fence", -1), ("idle", -2)]);
    let error = protocol.finish(&mut calls).unwrap_err();
    assert_eq!(protocol.phase(), SubmissionPhase::RetirementIncomplete);
    assert_eq!(error.device_failure().unwrap().result_code(), Some(-1));
    assert_eq!(
        error
            .observed_failure()
            .unwrap()
            .device_failure()
            .unwrap()
            .result_code(),
        Some(-2)
    );
    calls.failures.remove("idle");
    protocol.retire(&mut calls).unwrap();
    assert_eq!(protocol.phase(), SubmissionPhase::Retired);
}
#[test]
fn actual_device_loss_proves_execution_stopped_without_claiming_success() {
    for (fence_code, idle_code) in [(-4, None), (-1, Some(-4))] {
        let protocol = SubmissionProtocol::default();
        let mut calls = FakeSubmissionDispatch::default();
        calls.failures.insert("fence", fence_code);
        calls.failures.extend(idle_code.map(|code| ("idle", code)));
        let error = protocol.finish(&mut calls).unwrap_err();
        assert!(contains_device_loss(&error));
        assert_eq!(protocol.phase(), SubmissionPhase::Retired);
        assert_eq!(calls.events.contains(&"idle"), idle_code.is_some());
    }
}
#[test]
fn preexisting_recording_is_refused_after_another_operation_loses_device() {
    let protocol = SubmissionProtocol::default();
    let lifecycle = DeviceLifecycle::default();
    let mut calls = FakeSubmissionDispatch::default();
    let loss = RenderError::device_backend("another submission", -4, "lost".into());
    assert!(lifecycle.observe::<()>(Err(loss)).is_err());
    let error = protocol
        .finish_admitted(&lifecycle, &mut calls)
        .unwrap_err();
    assert_eq!(
        error.device_failure().unwrap().state(),
        Some(crate::RenderDeviceState::Lost)
    );
    assert!(calls.events.is_empty());
    assert_eq!(protocol.phase(), SubmissionPhase::Recording);
}
#[test]
fn unknown_completion_blocks_the_next_actual_submission_dispatch() {
    let first = SubmissionProtocol::default();
    let second = SubmissionProtocol::default();
    let lifecycle = DeviceLifecycle::default();
    let mut calls = FakeSubmissionDispatch::default();
    calls.failures.extend([("fence", -1), ("idle", -2)]);
    assert!(first.finish_admitted(&lifecycle, &mut calls).is_err());
    let before = calls.events.clone();
    assert!(second.finish_admitted(&lifecycle, &mut calls).is_err());
    assert_eq!(calls.events, before);
    assert_eq!(
        lifecycle.state(),
        crate::RenderDeviceState::RetirementIncomplete
    );
}
#[test]
fn unclassified_failed_submit_requires_idle_proof_without_waiting_its_fence() {
    let protocol = SubmissionProtocol::default();
    let mut calls = FakeSubmissionDispatch::default();
    calls.failures.extend([("submit", -13), ("idle", -1)]);
    assert!(protocol.finish(&mut calls).is_err());
    assert_eq!(protocol.phase(), SubmissionPhase::RetirementIncomplete);
    assert_eq!(calls.events, ["end", "submit", "idle"]);
}
#[test]
fn blocking_teardown_reaches_retired_only_after_qualified_idle() {
    let protocol = SubmissionProtocol::default();
    let mut calls = FakeSubmissionDispatch::default();
    calls.failures.extend([("fence", -1), ("idle", -2)]);
    assert!(protocol.finish(&mut calls).is_err());
    assert_eq!(protocol.phase(), SubmissionPhase::RetirementIncomplete);
    calls.failures.remove("idle");
    protocol.retire_blocking(&mut calls).unwrap();
    assert_eq!(protocol.phase(), SubmissionPhase::Retired);
    assert_eq!(calls.events, ["end", "submit", "fence", "idle", "idle"]);
}
