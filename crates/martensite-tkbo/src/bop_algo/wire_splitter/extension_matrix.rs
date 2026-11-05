//! The new public routes must match the same frozen records as private baseline routes.
use super::*;
use crate::bop_algo::{WireOperationContext, WireSplitterHooks};

#[test]
fn public_wire_profiles_retain_complete_actual_receiver_observations() {
    for profile in profiles() {
        run_public_wire_profile(profile);
    }
}

fn run_public_wire_profile(profile: WireProfile) {
    let (kind, parallel, cancel, throw) = profile;
    let trace = WireObservationTrace::default();
    let hooks = WireObservationHooks::new(
        trace.clone(),
        cancel,
        throw,
        if kind == "prepared" { 3 } else { 4 },
    );
    let indicator = ProgressIndicator::new(Box::new(hooks));
    let (mut receiver, inputs) = receiver(kind, parallel);
    receiver.options_mut().set_run_parallel(parallel);
    let outcome = public_wire_outcome(&mut receiver, kind, &indicator);
    assert_raw_profile(profile, &receiver, &outcome);
    assert_scope_cleanup(profile, &trace);
    emit_profile(profile, &receiver, &inputs, (&indicator, &trace, &outcome));
}

fn public_wire_outcome(
    receiver: &mut WireSplitter,
    kind: &str,
    indicator: &ProgressIndicator,
) -> ProgressCompletion {
    let range = indicator.start();
    if kind != "prepared" {
        return WireSplitterHooks::perform_with_progress(receiver, &range);
    }
    let mut operation = WireOperationContext::new();
    receiver
        .prepare_wires_extension(&mut operation)
        .unwrap()
        .make_wires(&range)
}
