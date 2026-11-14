//! Geometric chart groups retain canonical source spans and shared authored/split endpoints.
use super::*;

#[test]
fn long_authored_curve_gets_multiple_charts_with_shared_canonical_corners() {
    let input = near_full_request(0., 6.);
    let prepared = prepare(&input).unwrap();
    let contour = &prepared.loops[0];
    assert!(contour.len() > 8);
    assert!(contour.iter().any(|piece| piece.chart_center > 3.));
    for pair in contour.windows(2) {
        assert_eq!(pair[0].end, pair[1].start);
        assert_eq!(pair[0].poles.last(), pair[1].poles.first());
    }
    assert!(
        contour
            .iter()
            .all(|piece| piece.span.level <= input.max_fit_level)
    );
}

#[test]
fn existing_fixture_retains_its_nineteen_geometric_pieces() {
    let prepared = prepare(&super::super::tests::request()).unwrap();
    assert_eq!(prepared.loops.iter().map(Vec::len).sum::<usize>(), 19);
    for (id, expected) in [(10, 1), (11, 2), (12, 3)] {
        let piece = prepared.loops[0]
            .iter()
            .find(|p| p.span.curve.0 == id)
            .unwrap();
        assert_eq!(piece.span.level, expected);
    }
}

fn near_full_request(start: f64, width: f64) -> Request {
    let mut input = super::super::tests::request();
    input.radius = 1.;
    input.depths = [-0.1, 0.2];
    input.tolerance = 1e-3;
    let end = start + width;
    input.loops = vec![super::super::tests::contour(
        100,
        &[
            &[[start, 0.], [end, 0.]],
            &[[end, 0.], [end, 1.]],
            &[[end, 1.], [start, 1.]],
            &[[start, 1.], [start, 0.]],
        ],
    )];
    input
}

#[test]
fn six_radian_profile_freezes_and_proves_original_interval_conformance() {
    let input = near_full_request(0., 6.);
    let result = super::super::pipeline::wrap_extrude(&input).unwrap();
    assert!(result.member.report.is_valid());
    assert!((result.member.volume_estimate - 1.89).abs() < input.tolerance);
    assert_periodic_margin(&result);
    assert_eq!(result.member.storage.len(), result.clearance.spans.len());
    assert!(!result.bytes.is_empty());
}

#[test]
fn wide_source_is_refused_by_periodic_evidence_before_native_construction() {
    let input = near_full_request(0., 6.4);
    let failure = super::super::pipeline::wrap_extrude(&input).err().unwrap();
    assert_eq!(failure.stage, Stage::Clearance);
    assert!(matches!(failure.cause, RefusalCause::Clearance(_)));
}

#[test]
fn multiple_charts_cross_the_cylinder_parameter_seam_without_splitting_identity() {
    let input = near_full_request(6., 6.);
    let result = super::super::pipeline::wrap_extrude(&input).unwrap();
    assert!(result.member.report.is_valid());
    assert!((result.member.volume_estimate - 1.89).abs() < input.tolerance);
    assert!(
        result
            .member
            .storage
            .iter()
            .any(|piece| piece.span.curve == SurfaceCurveId(100))
    );
    assert_periodic_margin(&result);
}

fn assert_periodic_margin(result: &super::super::pipeline::Unqualified) {
    let margin = result.clearance.periodic.separation_lower_bound();
    assert!(margin.compare_f64(0.28).unwrap().is_gt());
}

#[test]
fn helical_material_spans_two_turns_with_two_certified_images_and_a_tail() {
    let input = helical_request();
    let result = super::super::pipeline::wrap_extrude(&input).unwrap();
    assert!(result.member.report.is_valid());
    assert!((result.member.volume_estimate - 2.205).abs() < input.tolerance);
    assert_eq!(result.clearance.periodic.images().len(), 2);
    assert_eq!(result.clearance.periodic.tail_start(), 3);
    assert!(
        result
            .clearance
            .periodic
            .separation_lower_bound()
            .compare_f64(4. * input.tolerance)
            .unwrap()
            .is_ge()
    );
}

fn helical_request() -> Request {
    let mut input = near_full_request(0., 14.);
    input.tolerance = 1e-2;
    input.loops = vec![super::super::tests::contour(
        100,
        &[
            &[[0., 0.], [14., 14.]],
            &[[14., 14.], [14., 14.5]],
            &[[14., 14.5], [0., 0.5]],
            &[[0., 0.5], [0., 0.]],
        ],
    )];
    input
}
