//! Finite failure sequences prove completion precedes resource destruction.
use super::*;
use std::collections::VecDeque;
struct FakeRetirementDispatch {
    results: VecDeque<Option<i32>>,
    events: Vec<&'static str>,
}
impl FakeRetirementDispatch {
    fn new(results: impl IntoIterator<Item = Option<i32>>) -> Self {
        // Exhaustion is a fake-definition error, never a synthesized successful wait.
        Self {
            results: results.into_iter().collect(),
            events: Vec::new(),
        }
    }
}
impl CompletionDispatch for FakeRetirementDispatch {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        self.events.push("wait");
        match self
            .results
            .pop_front()
            .expect("an explicit wait result for each actual attempt")
        {
            None => Ok(()),
            Some(code) => Err(RenderError::device_backend(
                "fake retirement wait",
                code,
                format!("result {code}"),
            )),
        }
    }
    fn retry_pause(&mut self) {
        // This named fake observes the spacing request without wall-clock delay.
        self.events.push("pause");
    }
}
#[test]
fn finite_unknown_failures_are_spaced_before_successful_retirement() {
    let mut calls = FakeRetirementDispatch::new([Some(-1), Some(-2), None]);
    let error = retire_until_stopped(&mut calls).unwrap_err();
    assert_eq!(calls.events, ["wait", "pause", "wait", "pause", "wait"]);
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
    assert!(calls.results.is_empty());
}
#[test]
fn actual_loss_stops_retry_without_turning_failed_execution_into_success() {
    let mut calls = FakeRetirementDispatch::new([Some(-1), Some(-4)]);
    let error = retire_until_stopped(&mut calls).unwrap_err();
    assert!(contains_device_loss(&error));
    assert_eq!(calls.events, ["wait", "pause", "wait"]);
    assert_eq!(error.device_failure().unwrap().result_code(), Some(-1));
}
#[test]
fn successful_first_wait_needs_no_retry_pause() {
    let mut calls = FakeRetirementDispatch::new([None]);
    retire_until_stopped(&mut calls).unwrap();
    assert_eq!(calls.events, ["wait"]);
    assert!(calls.results.is_empty());
}
#[test]
fn repeated_unknown_results_keep_bounded_first_and_latest_causes() {
    let mut calls = FakeRetirementDispatch::new([Some(-1), Some(-2), Some(-13), None]);
    let error = retire_until_stopped(&mut calls).unwrap_err();
    assert_eq!(error.device_failure().unwrap().result_code(), Some(-1));
    let latest = error.observed_failure().unwrap();
    assert_eq!(latest.device_failure().unwrap().result_code(), Some(-13));
    assert!(latest.observed_failure().is_none());
}
