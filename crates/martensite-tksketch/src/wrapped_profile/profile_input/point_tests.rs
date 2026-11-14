//! Split-corner identity uses the original directed curve parameter, never coordinates.
use super::*;
fn directions(reversed: bool) -> (CurveDirections, SurfaceCurveId) {
    let (_, mut profile, mut selection) = fixture();
    let curve = SurfaceCurveId(profile.curves[0].id().0);
    if reversed {
        reverse_source(&mut profile.curves[0]);
        selection.outer.uses[0].reversed = true;
    }
    (
        checked_input(&profile, &selection).unwrap().directions,
        curve,
    )
}

#[test]
fn reversed_split_points_map_to_exact_original_parameters_and_reduce_once() {
    let (directions, curve) = directions(true);
    for (numerator, level, expected_numerator, expected_level) in [
        (1, 3, 7, 3),
        (2, 3, 3, 2),
        (2, 2, 1, 1),
        (1, 63, 9_223_372_036_854_775_807, 63),
    ] {
        let point = directions.source_point(curve, numerator, level).unwrap();
        assert_eq!(
            (point.numerator, point.level),
            (expected_numerator, expected_level)
        );
        assert_eq!(point.curve, curve);
    }
}

#[test]
fn forward_split_points_keep_original_values_with_canonical_dyadic_identity() {
    let (directions, curve) = directions(false);
    for (numerator, level, expected_numerator, expected_level) in [(1, 3, 1, 3), (2, 3, 1, 2)] {
        let point = directions.source_point(curve, numerator, level).unwrap();
        assert_eq!(
            (point.numerator, point.level),
            (expected_numerator, expected_level)
        );
    }
}

#[test]
fn split_point_refuses_endpoints_outside_parameters_and_unselected_sources() {
    let (directions, curve) = directions(false);
    for (numerator, level) in [(0, 3), (8, 3), (9, 3), (1, 0), (1, 64), (1, 255)] {
        let result = directions
            .source_point(curve, numerator, level)
            .inspect_err(assert_diagnostic);
        assert!(matches!(result, Err(ProfileInputFailure::Point { .. })));
    }
    let result = directions
        .source_point(SurfaceCurveId(u64::MAX), 1, 3)
        .inspect_err(assert_diagnostic);
    assert!(matches!(result, Err(ProfileInputFailure::UnknownSource(_))));
}
