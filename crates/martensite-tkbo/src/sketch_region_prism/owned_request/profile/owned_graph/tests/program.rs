// SPDX-License-Identifier: GPL-2.0-only
//! V5 preserves actual integral-program diagnostics through the private owned epoch.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    IntegralProgramFailure, IntegralProgramOptions, IntegralProgramVersion, RationalIntegralOptions,
};
pub(super) fn options() -> IntegralProgramOptions {
    IntegralProgramOptions {
        version: IntegralProgramVersion::V1,
        integration: RationalIntegralOptions::default(),
        max_components: 8,
        max_total_subintervals: 4096,
    }
}
fn selected(options: IntegralProgramOptions) -> OwnedProfileRequest {
    let mut request = request();
    request.version = super::super::super::admission::COMPOSITE_GRAPH_PROFILE_VERSION;
    request.integral_program = Some(options);
    request
}
fn live_program(
    request: OwnedProfileRequest,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Result<GraphRegionCertificate, super::super::super::certify::ProfileProofRefusal> {
    let prepared = super::super::super::pipeline::prepare_owned_profile(
        request,
        &mut super::super::super::certify::PollProgress(cancelled),
    )
    .map_err(super::super::super::certify::preparation_refusal)?;
    let options = prepared.request.integral_program.unwrap();
    super::super::super::regions::inspect_program_graph(
        &prepared.request,
        &prepared.edges,
        &prepared.bridge,
        cancelled,
        options,
        SourceAccess::Live,
    )
    .map_err(|failure| super::super::super::certify::program_inspection_refusal(&prepared, failure))
}
#[test]
fn composite_owned_graph_keeps_live_program_bytes_trace_and_linear_source_captures() {
    let mut captures = 0;
    let mut actual_trace = Trace::default();
    let actual = run_observed(selected(options()), &mut actual_trace, |count| {
        captures = count
    })
    .unwrap();
    let mut expected_trace = Trace::default();
    let prepared = super::super::super::pipeline::prepare_owned_profile(
        selected(options()),
        &mut expected_trace,
    )
    .unwrap_or_else(|_| panic!("actual composite live fixture preparation"));
    let expected = live_prepared(&prepared).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
    assert_eq!(actual_trace.checkpoints, expected_trace.checkpoints);
    assert_eq!(captures, 6 * selected(options()).edges.len());
    assert!(actual.programs.is_some());
}
fn live_prepared(
    prepared: &super::super::super::PreparedProfile,
) -> Result<GraphRegionCertificate, Box<crate::sketch_region_prism::ProgramGraphFailure>> {
    super::super::super::regions::inspect_program_graph(
        &prepared.request,
        &prepared.edges,
        &prepared.bridge,
        || false,
        prepared.request.integral_program.unwrap(),
        SourceAccess::Live,
    )
}
#[test]
fn zero_program_budget_retains_live_actual_options_consumption_and_pair_work() {
    check_rational_cells();
    let mut options = options();
    options.max_total_subintervals = 0;
    let actual = run_with_cancel(rational_profile(options), &mut || false).unwrap_err();
    let expected = live_program(rational_profile(options), &mut || false).unwrap_err();
    assert_eq!(actual, expected);
    let program = actual.program.as_ref().unwrap();
    assert_eq!(program.options, options);
    assert_eq!(program.consumed, 0);
    assert!(matches!(
        &program.numerical_cause().unwrap().cause,
        IntegralProgramFailure::AggregateWork { limit: 0, .. }
    ));
    assert!(actual.work.is_some());
    assert!(actual.root_failure.is_none());
    assert!(!actual.cancelled);
}
#[test]
fn composite_final_actual_cancellation_preserves_live_cause_program_and_work() {
    let total = completed_polls();
    let mut actual_polls = 0;
    let actual = run_with_cancel(selected(options()), &mut || {
        actual_polls += 1;
        actual_polls == total
    })
    .unwrap_err();
    let mut live_polls = 0;
    let expected = live_program(selected(options()), &mut || {
        live_polls += 1;
        live_polls == total
    })
    .unwrap_err();
    assert_eq!(actual, expected);
    assert!(actual.cancelled);
    assert!(actual.program.is_some());
    assert_eq!((actual_polls, live_polls), (total, total));
}
fn completed_polls() -> usize {
    let mut polls = 0;
    run_with_cancel(selected(options()), &mut || {
        polls += 1;
        false
    })
    .unwrap();
    polls
}
#[test]
fn earlier_composite_pair_refusal_precedes_later_cancel_without_owned_capture() {
    let mut prepared =
        super::super::super::pipeline::prepare_owned_profile(selected(options()), &mut Continue)
            .unwrap_or_else(|_| panic!("actual composite first-refusal fixture"));
    prepared.bridge.census.domains[0].occurrences = [0, 1];
    let captures = std::cell::Cell::new(0);
    let access = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &captures);
    let mut polls = 0;
    let actual = refuse_before_late_cancel(&prepared, &access, &mut polls);
    let expected = live_prepared(&prepared).unwrap_err();
    check_earlier_pair(&prepared, actual, expected);
    assert_eq!(polls, 2);
    assert_eq!(captures.get(), 0);
}
fn check_earlier_pair(
    prepared: &super::super::super::PreparedProfile,
    actual: Box<crate::sketch_region_prism::ProgramGraphFailure>,
    expected: Box<crate::sketch_region_prism::ProgramGraphFailure>,
) {
    assert_eq!(actual, expected);
    assert_eq!(actual.consumed, 0);
    assert!(matches!(actual.failure.cause, crate::sketch_region_prism::region_certificate::loop_region::LoopRegionCause::PairUnavailable { .. }));
    let refusal = super::super::super::certify::program_inspection_refusal(prepared, actual);
    assert!(!refusal.cancelled);
    assert!(refusal.work.is_some());
    assert!(refusal.program.is_some());
}

fn refuse_before_late_cancel(
    prepared: &super::super::super::PreparedProfile,
    access: &OwnedGraphAccess<'_>,
    polls: &mut usize,
) -> Box<crate::sketch_region_prism::ProgramGraphFailure> {
    super::super::super::regions::inspect_program_graph(
        &prepared.request,
        &prepared.edges,
        &prepared.bridge,
        || {
            *polls += 1;
            *polls == 3
        },
        options(),
        SourceAccess::Owned(access),
    )
    .unwrap_err()
}

fn rational_profile(options: IntegralProgramOptions) -> OwnedProfileRequest {
    let mut request = selected(options);
    // Only positive interior weight changes: the same four edges still close at their endpoints.
    request.edges[0].world_weights = Some(vec![1., 1.25, 1.]);
    request
}
fn check_rational_cells() {
    let request = rational_profile(options());
    assert!(
        request.edges[0]
            .world_weights
            .as_ref()
            .unwrap()
            .windows(2)
            .any(|pair| pair[0] != pair[1])
    );
    let proof = run_with_cancel(request, &mut || false).unwrap();
    assert!(proof.programs.as_ref().unwrap().consumed > 0);
}
#[test]
fn zero_program_cells_complete_actual_polynomial_profiles_with_live_evidence() {
    let mut options = options();
    options.max_total_subintervals = 0;
    let actual = run_with_cancel(selected(options), &mut || false).unwrap();
    let expected = live_program(selected(options), &mut || false).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual.programs.as_ref().unwrap().consumed, 0);
    assert_eq!(actual.programs.as_ref().unwrap().options, options);
}

#[test]
fn geometric_v6_owned_graph_keeps_live_program_and_native_contact_evidence() {
    let mut request = selected(options());
    request.version = super::super::super::admission::GEOMETRIC_GRAPH_PROFILE_VERSION;
    let mut captures = 0;
    let mut actual_trace = Trace::default();
    let actual =
        run_observed(request.clone(), &mut actual_trace, |count| captures = count).unwrap();
    let mut expected_trace = Trace::default();
    let prepared =
        super::super::super::pipeline::prepare_owned_profile(request, &mut expected_trace)
            .unwrap_or_else(|_| panic!("V6 native profile"));
    let expected = live_prepared(&prepared).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual_trace.checkpoints, expected_trace.checkpoints);
    assert_eq!(captures, 6 * prepared.request.edges.len());
    assert!(
        prepared
            .bridge
            .inspection
            .contacts
            .iter()
            .all(|row| row.bounded_construction.is_some())
    );
}
