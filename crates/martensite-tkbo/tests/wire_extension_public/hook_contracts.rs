//! Default delegation and custom check effects must reach the actual stored receiver.
use super::{fixture, support::*};
use martensite_tkbo::bop_algo::{ProgressCompletion, WireSplitter, WireSplitterHooks};
use martensite_tkernel::{message::ProgressRange, standard::RuntimeTyped};
use std::sync::atomic::Ordering;

#[test]
fn actual_custom_check_supplies_missing_input_before_preparation() {
    let mut hooks = AuthoredWireHooks::new(WireSplitter::new(), WireCheckPolicy::SupplyEmpty);
    let outcome = hooks.perform_with_progress(&ProgressRange::default());
    assert_eq!(outcome, ProgressCompletion::Completed);
    assert_eq!(hooks.checks, 1);
    assert!(hooks.receiver.wes().unwrap().shapes().is_empty());
    assert!(!hooks.receiver.context().is_null());
}

#[test]
fn delegated_default_preserves_missing_input_report_and_stops_early() {
    let mut hooks = AuthoredWireHooks::new(WireSplitter::new(), WireCheckPolicy::Delegate);
    let (indicator, polls) = counted_indicator(usize::MAX);
    let outcome = hooks.perform_with_progress(&indicator.start());
    require_report_failure(outcome);
    assert!(
        hooks
            .receiver
            .options()
            .has_error(martensite_tkbo::bop_algo::AlertNullInputShapes::static_type())
    );
    assert!(hooks.receiver.context().is_null());
    assert_eq!(polls.polls.load(Ordering::Acquire), 0);
    assert_eq!(hooks.checks, 1);
}

#[test]
fn deliberate_report_error_stops_before_context_blocks_and_worker_polls() {
    let (receiver, _) = fixture::split_square_splitter(false);
    let mut hooks = AuthoredWireHooks::new(receiver, WireCheckPolicy::ReportError);
    let (indicator, polls) = counted_indicator(usize::MAX);
    let outcome = hooks.perform_with_progress(&indicator.start());
    require_report_failure(outcome);
    assert!(hooks.receiver.context().is_null());
    assert!(hooks.receiver.wes().unwrap().shapes().is_empty());
    assert_eq!(polls.polls.load(Ordering::Acquire), 0);
}

#[test]
fn returned_custom_failure_keeps_exact_type_message_and_stack() {
    let mut hooks =
        AuthoredWireHooks::new(fixture::empty_splitter(), WireCheckPolicy::ReturnFailure);
    let cause = require_kernel_failure(
        hooks.perform_with_progress(&ProgressRange::default()),
        "Standard_DomainError",
    );
    assert_eq!(cause.message(), "authored wire input check");
    assert_eq!(cause.stack_string(), "wire-owned-stack");
    assert!(hooks.receiver.context().is_null());
    assert_eq!(hooks.checks, 1);
}

#[test]
fn stable_external_delegation_retains_actual_irregular_input_incidences() {
    for parallel in [false, true] {
        let (receiver, inputs) = fixture::split_square_splitter(parallel);
        let mut hooks = AuthoredWireHooks::new(receiver, WireCheckPolicy::Delegate);
        assert_eq!(
            hooks.perform_with_progress(&ProgressRange::default()),
            ProgressCompletion::Completed
        );
        super::helper_contracts::assert_authored_triangles(&fixture::input_identity_records(
            &hooks.receiver,
            &inputs,
        ));
        assert_eq!(hooks.checks, 1);
    }
}

#[test]
fn whole_entry_clears_the_actual_preexisting_report_without_changing_options() {
    let mut hooks = AuthoredWireHooks::new(fixture::empty_splitter(), WireCheckPolicy::Delegate);
    hooks.receiver.options_mut().set_run_parallel(true);
    add_error(&hooks.receiver, false);
    assert_eq!(
        hooks.perform_with_progress(&ProgressRange::default()),
        ProgressCompletion::Completed
    );
    assert!(!hooks.receiver.options().has_errors());
    assert!(hooks.receiver.options().run_parallel());
}

#[test]
fn a_send_trait_object_uses_the_same_bound_default_program() {
    let mut hooks = AuthoredWireHooks::new(fixture::empty_splitter(), WireCheckPolicy::Delegate);
    let erased: &mut dyn WireSplitterHooks = &mut hooks;
    assert_eq!(
        erased.perform_with_progress(&ProgressRange::default()),
        ProgressCompletion::Completed
    );
    assert_eq!(hooks.checks, 1);
}
