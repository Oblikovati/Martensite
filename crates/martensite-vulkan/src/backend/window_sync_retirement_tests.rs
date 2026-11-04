//! The actual pending-fence driver must not substitute device idle for presentation completion.
use super::*;
use std::collections::VecDeque;
struct FakeWindowFenceDispatch {
    outcomes: VecDeque<Option<i32>>,
    reset_failure: Option<i32>,
    events: Vec<&'static str>,
}
impl FakeWindowFenceDispatch {
    fn new(outcomes: impl IntoIterator<Item = Option<i32>>) -> Self {
        // A missing outcome fails the fake rather than synthesizing completion.
        Self {
            outcomes: outcomes.into_iter().collect(),
            reset_failure: None,
            events: Vec::new(),
        }
    }
}
impl CompletionDispatch for FakeWindowFenceDispatch {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        self.events.push("fence");
        match self
            .outcomes
            .pop_front()
            .expect("one explicit outcome per owned-fence attempt")
        {
            Some(code) => Err(RenderError::device_backend(
                "fake owned fence",
                code,
                format!("result {code}"),
            )),
            None => Ok(()),
        }
    }
    fn retry_pause(&mut self) {
        // This fake records the nonbusy request without introducing test latency.
        self.events.push("pause");
    }
}
impl WindowFenceDispatch for FakeWindowFenceDispatch {
    fn reset_completed(&mut self) -> Result<(), RenderError> {
        self.events.push("reset");
        match self.reset_failure {
            Some(code) => Err(RenderError::device_backend(
                "fake completed reset",
                code,
                format!("result {code}"),
            )),
            None => Ok(()),
        }
    }
}
#[test]
fn no_pending_fence_never_dispatches_or_waits_an_unsignalled_handle() {
    let mut calls = FakeWindowFenceDispatch::new([]);
    let mut pending = false;
    wait_pending_fence(&mut calls, &mut pending).unwrap();
    retire_pending_fence(&mut calls, &mut pending).unwrap();
    assert!(calls.events.is_empty());
}
#[test]
fn successful_owned_fence_is_cleared_before_reset() {
    let mut calls = FakeWindowFenceDispatch::new([None]);
    let mut pending = true;
    wait_pending_fence(&mut calls, &mut pending).unwrap();
    assert!(!pending);
    assert_eq!(calls.events, ["fence", "reset"]);
}
#[test]
fn unknown_owned_fence_failure_keeps_ownership_and_does_not_reset() {
    let mut calls = FakeWindowFenceDispatch::new([Some(-1)]);
    let mut pending = true;
    assert!(wait_pending_fence(&mut calls, &mut pending).is_err());
    assert!(pending);
    assert_eq!(calls.events, ["fence"]);
}
#[test]
fn actual_owned_fence_loss_stops_ownership_but_remains_an_error() {
    let mut calls = FakeWindowFenceDispatch::new([Some(-4)]);
    let mut pending = true;
    let error = wait_pending_fence(&mut calls, &mut pending).unwrap_err();
    assert!(contains_device_loss(&error));
    assert!(!pending);
    assert_eq!(calls.events, ["fence"]);
}
#[test]
fn reset_failure_cannot_recreate_pending_execution_after_successful_wait() {
    let mut calls = FakeWindowFenceDispatch::new([None]);
    calls.reset_failure = Some(-1);
    let mut pending = true;
    assert!(wait_pending_fence(&mut calls, &mut pending).is_err());
    assert!(!pending);
    assert_eq!(calls.events, ["fence", "reset"]);
}
#[test]
fn blocking_owner_waits_its_exact_fence_until_success_or_loss() {
    for final_code in [None, Some(-4)] {
        let mut calls = FakeWindowFenceDispatch::new([Some(-1), final_code]);
        let mut pending = true;
        assert!(retire_pending_fence(&mut calls, &mut pending).is_err());
        assert!(!pending);
        assert_eq!(calls.events, ["fence", "pause", "fence"]);
    }
}
