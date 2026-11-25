//! Refuse unsupported representations before geometry construction or endpoint indexing.
use super::*;
fn spline(profile: &mut SketchProfile) -> &mut ProfileCurve {
    &mut profile.curves[1]
}
#[test]
fn identical_weights_preserve_controls_and_invalid_weights_refuse() {
    let (request, mut profile, selection) = fixture();
    let ProfileCurve::Spline { weights, poles, .. } = spline(&mut profile) else {
        panic!("spline fixture")
    };
    *weights = Some(vec![1.; poles.len()]);
    assert_geometry(&checked_input(&profile, &selection).unwrap(), &request);
    let ProfileCurve::Spline { weights, .. } = spline(&mut profile) else {
        panic!("spline fixture")
    };
    weights.as_mut().unwrap().fill(4.);
    assert_geometry(&checked_input(&profile, &selection).unwrap(), &request);
    let ProfileCurve::Spline { weights, .. } = spline(&mut profile) else {
        panic!("spline fixture")
    };
    weights.as_mut().unwrap()[1] = -0.5;
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::Weight { .. })
    ));
}
#[test]
fn spline_shape_refuses_extreme_degree_and_missing_poles() {
    let (_, mut profile, selection) = fixture();
    let ProfileCurve::Spline { degree, .. } = spline(&mut profile) else {
        panic!("spline fixture")
    };
    *degree = u32::MAX;
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::Spline { .. })
    ));
    let ProfileCurve::Spline { degree, poles, .. } = spline(&mut profile) else {
        panic!("spline fixture")
    };
    *degree = 2;
    poles.clear();
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::Spline { .. })
    ));
}
#[test]
fn internal_or_nonunit_knots_are_not_reinterpreted_as_a_single_span() {
    let (_, mut profile, selection) = fixture();
    let ProfileCurve::Spline { knots, .. } = spline(&mut profile) else {
        panic!("spline fixture")
    };
    knots[2] = 0.5;
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::Knot { .. })
    ));
}
#[test]
fn conic_sources_are_refused_in_the_polynomial_bridge() {
    let (_, mut profile, selection) = fixture();
    let id = profile.curves[1].id();
    profile.curves[1] = ProfileCurve::Circle {
        id,
        center: [0.; 2],
        radius: 1.,
    };
    assert!(matches!(
        checked_input(&profile, &selection),
        Err(ProfileInputFailure::Unsupported(_))
    ));
}
