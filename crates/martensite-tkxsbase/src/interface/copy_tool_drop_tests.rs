//! Cached copy inputs must be released before the model clears its checks.
use super::{
    InterfaceCheck, InterfaceCopyTool, InterfaceGeneralLib, InterfaceModelReports,
    test_norm::fake_model,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, SharedState, StandardTransient},
};
use std::sync::Arc;

struct CheckObservingCopyInput {
    transient: StandardTransient,
    model_check: Handle<InterfaceCheck>,
    observations: Arc<SharedState<Vec<bool>>>,
}
impl_transient!(
    CheckObservingCopyInput,
    "CheckObservingCopyInput",
    StandardTransient,
    transient
);
impl Drop for CheckObservingCopyInput {
    fn drop(&mut self) {
        self.observations
            .write()
            .push(self.model_check.has_failed());
    }
}

fn observing_input(
    check: &Handle<InterfaceCheck>,
    observations: &Arc<SharedState<Vec<bool>>>,
) -> Handle<StandardTransient> {
    Handle::new(CheckObservingCopyInput {
        transient: StandardTransient::default(),
        model_check: check.clone(),
        observations: observations.clone(),
    })
    .cast()
}

fn cached_copy_input(
    observations: &Arc<SharedState<Vec<bool>>>,
) -> (
    InterfaceCopyTool,
    Handle<StandardTransient>,
    Handle<InterfaceCheck>,
) {
    let model = fake_model();
    let check = model.global_check(true);
    check.add_fail_text("owned lifecycle marker", "");
    let input = observing_input(&check, observations);
    let mut tool = InterfaceCopyTool::with_lib(&model, &InterfaceGeneralLib::default()).unwrap();
    let mut result = Handle::default();
    assert!(!tool.copy(&input, &mut result, false, false).unwrap());
    assert!(result.is_null());
    (tool, input, check)
}

#[test]
fn cached_unrecognized_input_is_released_before_model_global_checks() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let (tool, input, check) = cached_copy_input(&observations);
    drop(input);
    assert!(observations.read().is_empty());
    drop(tool);
    assert_eq!(*observations.read(), vec![true]);
    assert!(!check.has_failed());
}

#[test]
fn external_copy_input_owner_delays_release_until_after_model_cleanup() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let (tool, input, check) = cached_copy_input(&observations);
    drop(tool);
    assert!(observations.read().is_empty());
    assert!(!check.has_failed());
    drop(input);
    assert_eq!(*observations.read(), vec![false]);
}
