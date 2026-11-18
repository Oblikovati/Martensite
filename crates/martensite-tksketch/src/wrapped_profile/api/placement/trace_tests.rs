//! These are inequality controls, not fabricated examples of an invalid production body.
use super::*;
fn span() -> CurveSpan {
    CurveSpan {
        curve: SurfaceCurveId(17),
        level: 2,
        index: 1,
    }
}
fn checked(residual: f64, stretch: f64, evaluation: f64, limit: f64) -> Checked<f64> {
    enforce(
        SurfaceRegionId(5),
        span(),
        1,
        transport(residual, stretch, evaluation)?,
        limit,
    )
}
#[test]
fn at_limit_stretch_refuses_with_retained_owner_span_depth_and_limit() {
    assert!(checked(0.5, 1. + f64::EPSILON, 0., 0.5).is_err());
    let failure = checked(0.5, 1.125, 0., 0.5).unwrap_err();
    let super::super::super::error::Cause::Placement(cause) = failure.cause.as_ref() else {
        panic!("expected typed physical trace refusal");
    };
    assert!(matches!(cause.as_ref(), PlacementFailure::TraceTolerance {
        region: SurfaceRegionId(5), span: actual_span, depth: 1, actual, retained_limit: 0.5,
    } if *actual_span == span() && *actual >= 0.5625));
}
#[test]
fn identity_and_exact_zero_preserve_equality_without_spurious_inflation() {
    assert_eq!(checked(0.5, 1., 0., 0.5).unwrap(), 0.5);
    assert_eq!(checked(0., 1., 0., 0.).unwrap(), 0.);
}
#[test]
fn both_independent_evaluation_errors_are_charged_even_for_zero_residual() {
    assert!(transport(0., 1., 0.125).unwrap() >= 0.25);
    assert!(checked(0., 1., 0.125, 0.125).is_err());
    assert!(checked(0.125, 1., 0.125, 0.5).is_ok());
}
#[test]
fn underflow_remains_positive_and_overflow_or_invalid_bounds_refuse() {
    let tiny = f64::from_bits(1);
    assert_eq!(transport(tiny, 0.5, 0.).unwrap(), tiny);
    for (residual, stretch, evaluation) in [
        (f64::MAX, 2., 0.),
        (0., 1., f64::MAX),
        (f64::NAN, 1., 0.),
        (1., -1., 0.),
        (1., 1., -1.),
    ] {
        assert!(transport(residual, stretch, evaluation).is_err());
    }
    assert!(checked(0., 1., 0., f64::NAN).is_err());
    assert!(checked(0., 1., 0., -1.).is_err());
}
fn identity_budget(origin: f64, translation: f64) -> CertifiedFrameSpatialBudget {
    let frame = certify_frame_transport(FrameTransportInput {
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        stored: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    })
    .unwrap();
    frame
        .spatial_budget(FrameSpatialInput {
            origin: [origin, 0., 0.],
            translation: [translation, 0., 0.],
            point_norm: 2.,
            spatial_limit: 16.,
        })
        .unwrap()
}
#[test]
fn ideal_origin_error_does_not_enter_a_same_stored_frame_trace() {
    let first = identity_budget(0., 0.);
    let shifted_ideal = identity_budget(1., 0.);
    assert!(shifted_ideal.translation_error_bound() > first.translation_error_bound());
    assert_eq!(
        transport(0.5, 1., first.evaluation_error_bound()).unwrap(),
        transport(0.5, 1., shifted_ideal.evaluation_error_bound()).unwrap()
    );
}
#[test]
fn common_large_translation_cancels_in_real_arithmetic_but_not_in_rounded_evaluations() {
    let translation = 9_007_199_254_740_992.;
    let budget = identity_budget(translation, translation);
    assert_eq!(budget.translation_error_bound(), 0.);
    assert!(budget.evaluation_error_bound() > 0.);
    let observed = ((translation + 2.) - (translation + 1.)).abs();
    assert!(observed > 1.);
    assert!(observed <= transport(1., 1., budget.evaluation_error_bound()).unwrap());
}
