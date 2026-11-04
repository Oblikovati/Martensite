//! Ordered CPU dispatch proves the production wait loop returns before release only after qualification.
use super::*;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
struct FakeRetirementOwner(Arc<Mutex<Vec<&'static str>>>);
impl Drop for FakeRetirementOwner {
    fn drop(&mut self) {
        // This observable release must follow the qualifying wait, not its unknown predecessor.
        self.0.lock().unwrap().push("release");
    }
}
struct FakeOwnerCompletionCalls {
    outcomes: VecDeque<Option<i32>>,
    trace: Arc<Mutex<Vec<&'static str>>>,
}
impl CompletionDispatch for FakeOwnerCompletionCalls {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        self.trace.lock().unwrap().push("wait");
        match self
            .outcomes
            .pop_front()
            .expect("one explicit outcome for each required completion call")
        {
            None => Ok(()),
            Some(code) => Err(RenderError::device_backend(
                "synthetic owner wait",
                code,
                format!("result {code}"),
            )),
        }
    }
    fn retry_pause(&mut self) {
        // No wall-clock dependency is needed for the finite event-order reference.
        self.trace.lock().unwrap().push("pause");
    }
}
fn finite_owner_retirement(last: Option<i32>) -> Vec<&'static str> {
    let trace = Arc::new(Mutex::new(Vec::new()));
    let owner = FakeRetirementOwner(trace.clone());
    let mut calls = FakeOwnerCompletionCalls {
        outcomes: [Some(-1), last].into(),
        trace: trace.clone(),
    };
    assert!(retire_until_stopped(&mut calls).is_err());
    drop(owner);
    trace.lock().unwrap().clone()
}
#[test]
fn finite_unknown_then_success_or_loss_never_releases_the_owner_early() {
    for last in [None, Some(-4)] {
        let events = finite_owner_retirement(last);
        assert_eq!(events, ["wait", "pause", "wait", "release"]);
    }
}
