// SPDX-License-Identifier: GPL-2.0-only
//! #544: the owned selector binds its native transaction before graph consumption.
use super::work_tests::lens_input;
use super::*;
use crate::sketch_region_certification::GraphPolicyMismatch;
use crate::sketch_region_prism::region_certificate::loop_region::{
    LoopRegionCause, LoopRegionFailure,
};
fn prepared(version: u32) -> PreparedProfile {
    let mut request = owned_graph_request(lens_input(Some(17)));
    request.version = version;
    prepare_owned_profile(request, &mut NeverStop)
        .unwrap_or_else(|_| panic!("owned graph version={version}"))
}
#[test]
fn owned_filtered_graph_keeps_its_actual_selector_and_signs() {
    let prepared = prepared(4);
    assert_eq!(prepared.request.version, 4);
    let paired = prepared.bridge.census.paired.as_ref().unwrap();
    assert_eq!(paired.selected_signs.version, 2);
    assert_eq!(paired.requested.signs, paired.selected_signs);
    assert_eq!(paired.requested.version, 2);
    assert_eq!(paired.exclusion_version, Some(1));
    assert!(!paired.exclusions.is_empty());
    let certificate = prepared.inspect_plane_graph(|| false).unwrap();
    let area = &certificate.faces[0].area;
    assert!((area.representative() - 4. / 3.).abs() <= area.absolute_error_bound() + f64::EPSILON);
}
#[test]
fn owned_graph_refuses_unknown_seven_before_native_work() {
    let mut request = owned_graph_request(lens_input(None));
    request.version = 7;
    let failure = prepare_owned_profile(request, &mut NeverStop)
        .err()
        .unwrap();
    assert!(failure.pair_work.is_none());
    assert!(matches!(
        failure.cause,
        ProfilePreparationCause::Admission(
            super::super::admission::ProfileAdmissionCause::UnsupportedVersion { actual: 7, .. }
        )
    ));
}
#[test]
fn owned_graph_refuses_both_directions_of_selector_relabeling_without_polling() {
    for (original, changed) in [(3, 4), (4, 3), (2, 4), (4, 2)] {
        let mut prepared = prepared(original);
        prepared.request.version = changed;
        let mut polls = 0;
        let failure = prepared
            .inspect_plane_graph(|| {
                polls += 1;
                true
            })
            .unwrap_err();
        assert_eq!(polls, 0);
        assert_relabel(&prepared, failure, original, changed);
    }
}
#[test]
fn owned_filtered_graph_refuses_retained_selected_sign_policy_mutation() {
    let mut prepared = prepared(4);
    prepared
        .bridge
        .census
        .paired
        .as_mut()
        .unwrap()
        .selected_signs
        .version = 1;
    let failure = prepared.inspect_plane_graph(|| false).unwrap_err();
    assert_diagnostic(&prepared, failure, &[2, 3, 4], |diagnostic| {
        let actual = diagnostic.actual.unwrap();
        assert_eq!(diagnostic.expected.unwrap().selected_signs.version, 2);
        assert_eq!(actual.pair_policy.signs.version, 2);
        assert_eq!(actual.selected_signs.version, 1);
    });
}
fn assert_relabel(
    prepared: &PreparedProfile,
    failure: LoopRegionFailure,
    original: u32,
    changed: u32,
) {
    assert_diagnostic(prepared, failure, &[2, 3, 4], |diagnostic| {
        assert_eq!(diagnostic.requested_graph_version, changed);
        let expected = diagnostic.expected.unwrap();
        let actual = diagnostic.actual.unwrap();
        assert_eq!(expected.pair_policy.version, 2);
        assert_eq!(actual.pair_policy.version, 2);
        assert_eq!(expected.exclusion_version, (changed != 2).then_some(1));
        assert_eq!(actual.exclusion_version, (original != 2).then_some(1));
        assert_eq!(
            expected.selected_signs.version,
            if changed == 4 { 2 } else { 1 }
        );
        assert_eq!(
            actual.selected_signs.version,
            if original == 4 { 2 } else { 1 }
        );
    });
}
fn assert_diagnostic(
    prepared: &PreparedProfile,
    failure: LoopRegionFailure,
    admitted_versions: &[u32],
    check: impl FnOnce(&GraphPolicyMismatch),
) {
    let LoopRegionCause::GraphPolicy(diagnostic) = &failure.cause else {
        panic!("typed graph policy refusal");
    };
    assert_eq!(diagnostic.admitted_graph_versions, admitted_versions);
    check(diagnostic);
    let expected = format!("expected: {:?}", diagnostic.expected);
    let actual = format!("actual: {:?}", diagnostic.actual);
    let version = format!(
        "requested_graph_version: {}",
        diagnostic.requested_graph_version
    );
    let ProfileProofCause::Incomplete(text) = inspection_refusal(prepared, failure).cause else {
        panic!("outward graph refusal");
    };
    assert!(text.contains(&version), "{text}");
    assert!(text.contains(&expected), "{text}");
    assert!(text.contains(&actual), "{text}");
    let admitted = format!("admitted_graph_versions: {admitted_versions:?}");
    assert!(text.contains(&admitted), "{text}");
}
#[test]
fn scalar_policy_mismatches_keep_actual_and_expected_values_outward() {
    for mutation in 0..3 {
        let mut prepared = prepared(4);
        mutate_scalar(&mut prepared, mutation);
        let failure = prepared.inspect_plane_graph(|| false).unwrap_err();
        assert_diagnostic(&prepared, failure, &[2, 3, 4], |diagnostic| {
            assert_scalar(diagnostic, mutation)
        });
    }
}
fn mutate_scalar(prepared: &mut PreparedProfile, mutation: usize) {
    let census = &mut prepared.bridge.census;
    let paired = census.paired.as_mut().unwrap();
    match mutation {
        0 => paired.requested.roots.max_bisections -= 1,
        1 => paired.requested.requested_world_error *= 2.,
        _ => {
            census.requested.max_world_error = -0.;
            paired.requested.requested_world_error = 0.;
        }
    }
}
fn assert_scalar(diagnostic: &GraphPolicyMismatch, mutation: usize) {
    let expected = diagnostic.expected.unwrap().pair_policy;
    let actual = diagnostic.actual.unwrap().pair_policy;
    match mutation {
        0 => {
            assert_eq!(expected.roots.max_bisections, 17);
            assert_eq!(actual.roots.max_bisections, 16);
        }
        1 => {
            assert_eq!(expected.requested_world_error, 1e-8);
            assert_eq!(actual.requested_world_error, 2e-8);
        }
        _ => {
            assert_eq!(expected.requested_world_error.to_bits(), (-0_f64).to_bits());
            assert_eq!(actual.requested_world_error.to_bits(), 0_f64.to_bits());
        }
    }
}
#[test]
fn unknown_retained_selector_reports_admitted_versions_and_observed_policy() {
    let mut prepared = prepared(4);
    prepared.request.version = 7;
    let failure = prepared.inspect_plane_graph(|| false).unwrap_err();
    assert_diagnostic(&prepared, failure, &[2, 3, 4, 5, 6], |diagnostic| {
        assert_eq!(diagnostic.requested_graph_version, 7);
        assert!(diagnostic.expected.is_none());
        assert_eq!(diagnostic.actual.unwrap().selected_signs.version, 2);
    });
}

#[test]
fn graph_policy_bit_binding_does_not_readmit_or_validate_a_stale_receipt() {
    let mut prepared = prepared(4);
    let census = &mut prepared.bridge.census;
    let same_nan = f64::from_bits(0x7ff8000000000001);
    census.requested.max_world_error = same_nan;
    census
        .paired
        .as_mut()
        .unwrap()
        .requested
        .requested_world_error = same_nan;
    // Admission is upstream; binding preserves the original bit comparison, not float equality.
    assert!(super::super::selection::bind(4, &prepared.bridge.census).is_ok());
    assert!(prepared.inspect_plane_graph(|| false).is_err());
}
