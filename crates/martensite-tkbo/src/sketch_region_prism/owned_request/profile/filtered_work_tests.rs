// SPDX-License-Identifier: GPL-2.0-only
//! #544: the owned selector binds its native transaction before graph consumption.
use super::work_tests::lens_input;
use super::*;
use crate::sketch_region_prism::region_certificate::loop_region::LoopRegionCause;
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
fn owned_graph_refuses_unknown_five_before_native_work() {
    let mut request = owned_graph_request(lens_input(None));
    request.version = 5;
    let failure = prepare_owned_profile(request, &mut NeverStop)
        .err()
        .unwrap();
    assert!(failure.pair_work.is_none());
    assert!(matches!(
        failure.cause,
        ProfilePreparationCause::Admission(
            super::super::admission::ProfileAdmissionCause::UnsupportedVersion { actual: 5, .. }
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
        assert!(matches!(failure.cause, LoopRegionCause::Obligation { .. }));
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
    assert!(matches!(
        prepared.inspect_plane_graph(|| false).unwrap_err().cause,
        LoopRegionCause::Obligation { .. }
    ));
}
