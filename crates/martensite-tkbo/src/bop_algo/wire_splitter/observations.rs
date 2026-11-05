//! Emit the same authored receiver matrix as the independent native producer.
use super::{WireSplitter, fixture::split_square_splitter};
use crate::bop_algo::builder_area::progress::classify_area_progress;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion, WireEdgeSet};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressIndicator;
#[path = "observation_hooks.rs"]
mod hooks;
use hooks::{WireObservationHooks, WireObservationTrace};

type WireProfile = (&'static str, bool, usize, usize);

fn profiles() -> Vec<WireProfile> {
    let mut profiles = Vec::new();
    for parallel in [false, true] {
        for kind in ["missing", "empty", "regular", "irregular"] {
            profiles.push((kind, parallel, 0, 0));
        }
        for (cancel, throw) in [(1, 0), (2, 0), (4, 0), (0, 4)] {
            profiles.push(("irregular", parallel, cancel, throw));
        }
        for (cancel, throw) in [(0, 0), (1, 0), (2, 0), (3, 0), (0, 3)] {
            profiles.push(("prepared", parallel, cancel, throw));
        }
    }
    profiles
}

fn receiver(kind: &str, parallel: bool) -> (WireSplitter, Vec<Shape>) {
    match kind {
        "missing" => (WireSplitter::new(), Vec::new()),
        "empty" => selected_receiver(parallel, 0),
        "regular" => selected_receiver(parallel, 4),
        "irregular" | "prepared" => split_square_splitter(parallel),
        other => {
            panic!("unknown profile {other}; expected missing/empty/regular/irregular/prepared")
        }
    }
}

pub(super) fn selected_receiver(parallel: bool, prefix_count: usize) -> (WireSplitter, Vec<Shape>) {
    let (mut receiver, mut inputs) = split_square_splitter(parallel);
    inputs.truncate(prefix_count);
    let mut wes = WireEdgeSet::new();
    wes.set_face(receiver.wes().unwrap().face());
    for input in &inputs {
        wes.add_start_element(input);
    }
    receiver.set_wes(wes);
    (receiver, inputs)
}

fn perform_profile(
    receiver: &mut WireSplitter,
    kind: &str,
    indicator: &ProgressIndicator,
) -> ProgressCompletion {
    let range = indicator.start();
    if kind != "prepared" {
        return receiver.perform_with_progress(&range);
    }
    receiver.prepare_blocks().unwrap();
    let observed = ProgressCancellation::default();
    let result = receiver.make_wires(&range, &observed);
    classify_area_progress(
        crate::bop_algo::ProgressOperation::WireSplitter,
        receiver.options(),
        result,
        &observed,
    )
}

fn wire_rows(receiver: &WireSplitter, inputs: &[Shape]) -> Vec<Vec<[usize; 2]>> {
    if receiver.wes().is_none() {
        return Vec::new();
    }
    super::contract::numeric_rows(&super::fixture::input_identity_records(receiver, inputs))
}

fn report_text(receiver: &WireSplitter) -> (String, String) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    receiver.options().dump_errors(&mut errors).unwrap();
    receiver.options().dump_warnings(&mut warnings).unwrap();
    (
        String::from_utf8(errors).unwrap(),
        String::from_utf8(warnings).unwrap(),
    )
}

fn failure_fields(outcome: &ProgressCompletion) -> String {
    let (kind, message, stack) = match outcome {
        ProgressCompletion::Failed {
            cause: Some(cause), ..
        } => (
            cause.exception_type(),
            cause.message(),
            cause.stack_string(),
        ),
        _ => ("", "", ""),
    };
    format!("\"failure_type\":{kind:?},\"failure_message\":{message:?},\"failure_stack\":{stack:?}")
}

fn receiver_json(
    receiver: &WireSplitter,
    inputs: &[Shape],
    outcome: &ProgressCompletion,
) -> String {
    let rows = wire_rows(receiver, inputs);
    let (errors, warnings) = report_text(receiver);
    let failure = failure_fields(outcome);
    let has_errors = usize::from(receiver.options().has_errors());
    let completion = format!("{outcome:?}");
    format!(
        "\"completion\":{completion:?},\"has_errors\":{has_errors},\"errors\":{errors:?},\"warnings\":{warnings:?},\"wires\":{rows:?},{failure}"
    )
}

fn emit_profile(
    profile: WireProfile,
    receiver: &WireSplitter,
    inputs: &[Shape],
    observation: (
        &ProgressIndicator,
        &WireObservationTrace,
        &ProgressCompletion,
    ),
) {
    let (kind, parallel, cancel, throw) = profile;
    let fields = receiver_json(receiver, inputs, observation.2);
    let observations = observation.1.json_fields();
    let position = observation.0.get_position();
    println!(
        "WIRE_OBSERVATION {{\"profile\":[{kind:?},{},{cancel},{throw}],\"position\":{position},{observations},{fields}}}",
        usize::from(parallel)
    );
}

fn run_profile(profile: WireProfile) {
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
    let outcome = perform_profile(&mut receiver, kind, &indicator);
    assert_raw_profile(profile, &receiver, &outcome);
    assert_scope_cleanup(profile, &trace);
    emit_profile(profile, &receiver, &inputs, (&indicator, &trace, &outcome));
}

fn assert_raw_profile(profile: WireProfile, receiver: &WireSplitter, outcome: &ProgressCompletion) {
    let status = match outcome {
        ProgressCompletion::Completed => "completed",
        ProgressCompletion::Cancelled { .. } => "cancelled",
        ProgressCompletion::Failed { cause: Some(_), .. } => "kernel failure",
        ProgressCompletion::Failed { cause: None, .. } => "report failure",
        other => panic!("unexpected completion {other:?}; expected qualified raw status"),
    };
    let expected = expected_status(profile);
    assert_eq!(status, expected);
    assert_eq!(
        receiver.wes().map_or(0, |wes| wes.shapes().len()),
        expected_loops(profile)
    );
}

fn expected_status(profile: WireProfile) -> &'static str {
    let (kind, _, cancel, throw) = profile;
    match (kind, cancel, throw) {
        ("missing", _, _) => "report failure",
        (_, 1.., _) => "cancelled",
        (_, _, 1..) => "kernel failure",
        _ => "completed",
    }
}

fn expected_loops(profile: WireProfile) -> usize {
    let (kind, _, cancel, throw) = profile;
    match (kind, cancel, throw) {
        ("missing" | "empty", _, _) | (_, _, 1..) | (_, 1 | 2, _) => 0,
        ("regular", _, _) => 1,
        (_, 3 | 4, _) => 2,
        _ => 4,
    }
}

fn require_owned_range_send<T: Send>() {
    let witness: fn(T) = std::mem::drop::<T>;
    std::hint::black_box(witness);
}

fn assert_scope_cleanup(profile: WireProfile, trace: &WireObservationTrace) {
    let (kind, parallel, _, throw) = profile;
    if parallel || throw == 0 {
        return;
    }
    let expected = if kind == "prepared" { 3 } else { 4 };
    assert_eq!(
        trace.last_depth(),
        expected,
        "unstarted task must close its native parent range"
    );
}

fn require_observation_bounds<T: Send + Sync>() {
    let witness: fn(T) = std::mem::drop::<T>;
    std::hint::black_box(witness);
}

#[test]
fn actual_public_and_protected_wire_receiver_matrix() {
    require_owned_range_send::<martensite_tkernel::message::ProgressRange<'static>>();
    require_observation_bounds::<WireObservationTrace>();
    require_observation_bounds::<WireObservationHooks>();
    for profile in profiles() {
        run_profile(profile);
    }
}

#[path = "extension_matrix.rs"]
mod extension_matrix;
