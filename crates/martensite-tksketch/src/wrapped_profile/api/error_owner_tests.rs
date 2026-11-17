//! Exact owner context comes from retained causes, never the selection list (#448; cone sibling #523).
use super::*;

fn two_regions(reverse: bool) -> WrappedNormalRequest {
    let mut input = operation_tests::request();
    if reverse {
        input.selections.reverse();
    }
    input
}
fn assert_context(
    failure: &WrappedNormalFailure,
    stage: WrappedStage,
    class: WrappedFailureClass,
    entities: &[HistorySource],
) {
    assert_eq!(failure.stage(), stage, "{failure}");
    assert_public(failure, class, entities);
}

#[test]
fn global_support_and_depth_refusals_have_no_selected_owner() {
    for reverse in [false, true] {
        let mut input = two_regions(reverse);
        input.support.chart_origin = 1.;
        assert_context(
            &refused(&input),
            WrappedStage::Admission,
            WrappedFailureClass::Unsupported,
            &[],
        );
        input.support.chart_origin = 0.;
        input.depths = [1., 0.];
        assert_context(
            &refused(&input),
            WrappedStage::Admission,
            WrappedFailureClass::InvalidInput,
            &[],
        );
    }
}

#[test]
fn global_vertex_index_refusal_has_only_its_actual_vertex_under_permutation() {
    for reverse in [false, true] {
        let mut input = two_regions(reverse);
        let vertex = input.profile.vertices[0];
        input.profile.vertices.push(vertex);
        assert_context(
            &refused(&input),
            WrappedStage::Input,
            WrappedFailureClass::InvalidInput,
            &[HistorySource::Vertex(SurfaceVertexId(vertex.id.0))],
        );
    }
}

#[test]
fn global_curve_index_refusal_has_only_its_actual_curve_under_permutation() {
    for reverse in [false, true] {
        let mut input = two_regions(reverse);
        let curve = input.profile.curves[0].clone();
        let id = curve.id();
        input.profile.curves.push(curve);
        assert_context(
            &refused(&input),
            WrappedStage::Input,
            WrappedFailureClass::InvalidInput,
            &[HistorySource::Curve(SurfaceCurveId(id.0))],
        );
    }
}

#[test]
fn per_region_profile_refusal_keeps_the_same_owner_under_selection_permutation() {
    for reverse in [false, true] {
        let mut input = two_regions(false);
        let owner = input.selections[1].region;
        input.selections[1].outer.uses[0].curve = ProfileCurveId(u64::MAX);
        if reverse {
            input.selections.reverse();
        }
        assert_context(
            &refused(&input),
            WrappedStage::Input,
            WrappedFailureClass::InvalidInput,
            &[
                HistorySource::Region(owner),
                HistorySource::Curve(SurfaceCurveId(u64::MAX)),
            ],
        );
    }
}

#[test]
fn global_budget_policy_does_not_fabricate_selected_owners() {
    for reverse in [false, true] {
        let mut input = two_regions(reverse);
        input.options.volume = WrappedVolumePolicy::Refuse(-1.);
        assert_context(
            &refused(&input),
            WrappedStage::Admission,
            WrappedFailureClass::InvalidInput,
            &[],
        );
    }
}

#[test]
fn aggregate_volume_bound_refusal_does_not_fabricate_a_region() {
    let mut input = two_regions(false);
    input.options.volume = WrappedVolumePolicy::Refuse(0.);
    let failure = refused(&input);
    assert_context(
        &failure,
        WrappedStage::Construction,
        WrappedFailureClass::Incomplete,
        &[],
    );
    assert!(
        matches!(failure.cause.as_ref(), error::Cause::Family(family::FamilyFailure::Budget(cause))
        if matches!(cause.as_ref(), publication_budget::BudgetFailure::Limit { region: None, span: None, .. }))
    );
}

#[test]
fn global_physical_frame_refusal_keeps_no_guessed_region() {
    for reverse in [false, true] {
        let mut input = two_regions(reverse);
        input.support.origin[0] = f64::INFINITY;
        let failure = refused(&input);
        assert_context(
            &failure,
            WrappedStage::Placement,
            WrappedFailureClass::InvalidInput,
            &[],
        );
        assert!(
            matches!(failure.cause.as_ref(), error::Cause::Placement(cause)
            if matches!(cause.as_ref(), placement::PlacementFailure::InputFrame { .. }))
        );
    }
}

#[test]
fn cancelled_global_planar_work_has_no_selected_owner() {
    let input = two_regions(false);
    let stop = input.selections.len() + 6;
    let mut polls = 0;
    let failure = normal_extrude_profile_with_cancel(&input, || {
        polls += 1;
        polls == stop
    })
    .err()
    .unwrap();
    assert_context(
        &failure,
        WrappedStage::Construction,
        WrappedFailureClass::Cancelled,
        &[],
    );
    assert!(failure.planar_work_failure().unwrap().is_cancelled());
    assert_eq!(polls, stop);
}

#[test]
fn explicit_chart_global_angle_refusal_has_no_selected_owner() {
    for reverse in [false, true] {
        let mut input = two_regions(reverse);
        input.support.chart_origin = f64::INFINITY;
        let failure = normal_extrude_profile_on_chart(&input).err().unwrap();
        assert_context(
            &failure,
            WrappedStage::Placement,
            WrappedFailureClass::InvalidInput,
            &[],
        );
        assert!(
            matches!(failure.cause.as_ref(), error::Cause::Placement(cause)
            if matches!(cause.as_ref(), placement::PlacementFailure::Rotation(_)))
        );
    }
}
