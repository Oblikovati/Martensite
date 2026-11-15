//! Independent coefficient, directional and ownership controls for #464.
use martensite_tkernel::ncollection::Array2;
use martensite_tkg3d::geom::{BezierSurfaceCoefficientError, GeomBezierSurface, GeomSurface};
use martensite_tkmath::gp::Point3;

fn poles() -> Array2<'static, Point3> {
    let mut grid = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for u in 1..=2 {
        for v in 1..=2 {
            grid.set_value(u, v, Point3::new(f64::from(u - 1), f64::from(v - 1), 0.))
                .unwrap();
        }
    }
    grid
}
fn weights(values: [f64; 4]) -> Array2<'static, f64> {
    let mut grid = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for (index, value) in values.into_iter().enumerate() {
        grid.set_value(index as i32 / 2 + 1, index as i32 % 2 + 1, value)
            .unwrap();
    }
    grid
}
fn check_weights(surface: &GeomBezierSurface, expected: [f64; 4]) {
    for (index, value) in expected.into_iter().enumerate() {
        assert_eq!(
            surface
                .weight(index as i32 / 2 + 1, index as i32 % 2 + 1)
                .unwrap()
                .to_bits(),
            value.to_bits()
        );
    }
}
#[test]
fn near_unit_rows_survive_construction_and_copy_without_epsilon_demotion() {
    let down = f64::from_bits(1f64.to_bits() - 1);
    let values = [1., 1., down, down];
    let surface = GeomBezierSurface::from_stored_coefficients(&poles(), &weights(values)).unwrap();
    assert!(surface.is_u_rational());
    assert!(!surface.is_v_rational());
    check_weights(&surface, values);
    check_weights(&GeomBezierSurface::copy_of(&surface).unwrap(), values);
    let legacy = GeomBezierSurface::with_weights(&poles(), &weights(values)).unwrap();
    check_weights(&legacy, [1.; 4]);
}
#[test]
fn directional_flags_and_evaluation_follow_the_actual_weight_coordinates() {
    let u = GeomBezierSurface::from_stored_coefficients(&poles(), &weights([1., 1., 0.25, 0.25]))
        .unwrap();
    let v = GeomBezierSurface::from_stored_coefficients(&poles(), &weights([1., 0.25, 1., 0.25]))
        .unwrap();
    assert_eq!((u.is_u_rational(), u.is_v_rational()), (true, false));
    assert_eq!((v.is_u_rational(), v.is_v_rational()), (false, true));
    for t in [0., 0.25, 0.5, 1.] {
        let expected = t * 0.25 / (1. - t + t * 0.25);
        assert!((u.value(t, 0.5).unwrap().x() - expected).abs() <= 2e-16);
        assert!((v.value(0.5, t).unwrap().y() - expected).abs() <= 2e-16);
    }
}
#[test]
fn unit_grids_keep_both_polynomial_flags() {
    let surface = GeomBezierSurface::from_stored_coefficients(&poles(), &weights([1.; 4])).unwrap();
    assert_eq!(
        (surface.is_u_rational(), surface.is_v_rational()),
        (false, false)
    );
    check_weights(&surface, [1.; 4]);
}
#[test]
fn directional_derivatives_match_the_independent_rational_quotient() {
    let surface =
        GeomBezierSurface::from_stored_coefficients(&poles(), &weights([1., 1., 0.25, 0.25]))
            .unwrap();
    for u in [0., 0.25, 0.5, 1.] {
        let derivative = surface.d1(u, 0.5).unwrap();
        let denominator = 1. - u + u * 0.25;
        assert!((derivative.d1u.x() - 0.25 / (denominator * denominator)).abs() < 1e-14);
        assert!(derivative.d1u.y().abs() < 1e-14);
        assert!(derivative.d1v.x().abs() < 1e-14);
        assert!((derivative.d1v.y() - 1.).abs() < 1e-14);
    }
}
#[test]
fn constant_nonunit_weights_have_a_structured_refusal_and_native_category() {
    let error = GeomBezierSurface::from_stored_coefficients(&poles(), &weights([2.; 4]))
        .err()
        .unwrap();
    assert!(matches!(
        error,
        BezierSurfaceCoefficientError::ConstantNonUnitWeights { weight: 2. }
    ));
    let native = error.into_failure();
    assert_eq!(
        native.exception_type(),
        BezierSurfaceCoefficientError::CONSTANT_NON_UNIT_EXCEPTION
    );
    assert!(native.to_string().contains("0x4000000000000000"));
}
#[test]
fn malformed_grids_and_nonfinite_values_are_refused() {
    for value in [0., -1., f64::NAN, f64::INFINITY] {
        assert!(
            GeomBezierSurface::from_stored_coefficients(&poles(), &weights([1., 1., value, 1.]))
                .is_err()
        );
    }
    let wrong = Array2::with_bounds(1, 3, 1, 2).unwrap();
    assert!(GeomBezierSurface::from_stored_coefficients(&poles(), &wrong).is_err());
    let mut invalid = poles();
    invalid
        .set_value(1, 1, Point3::new(f64::NAN, 0., 0.))
        .unwrap();
    assert!(GeomBezierSurface::from_stored_coefficients(&invalid, &weights([1.; 4])).is_err());
}
#[test]
fn construction_and_copy_own_their_coefficients_in_both_directions() {
    let mut input = weights([1., 1., 0.25, 0.25]);
    let original = GeomBezierSurface::from_stored_coefficients(&poles(), &input).unwrap();
    let copied = GeomBezierSurface::copy_of(&original).unwrap();
    input.set_value(2, 1, 0.5).unwrap();
    original.set_weight(2, 1, 0.75).unwrap();
    copied.set_pole(1, 1, Point3::new(5., 6., 7.)).unwrap();
    assert_eq!(copied.weight(2, 1).unwrap(), 0.25);
    assert_eq!(original.pole(1, 1).unwrap(), Point3::new(0., 0., 0.));
}
#[test]
fn coefficient_error_is_send_and_sync() {
    let error = BezierSurfaceCoefficientError::ConstantNonUnitWeights { weight: 2. };
    let _: &(dyn Send + Sync) = &error;
}
fn native_message(error: BezierSurfaceCoefficientError) -> String {
    match error {
        BezierSurfaceCoefficientError::Native(cause) => cause.message().to_owned(),
        other => panic!("expected native validation cause, got {other:?}"),
    }
}
#[test]
fn retained_diagnostics_keep_column_major_native_floor_precedence() {
    let error = GeomBezierSurface::from_stored_coefficients(&poles(), &weights([1., 0., -1., 1.]))
        .err()
        .unwrap();
    assert_eq!(
        native_message(error),
        "Geom_BezierSurface: Weights(2, 1) = -1; expected > gp::Resolution()"
    );
}
#[test]
fn retained_diagnostics_keep_nonfinite_pole_before_nonfinite_weight() {
    let mut invalid = poles();
    invalid
        .set_value(1, 1, Point3::new(f64::NAN, 0., 0.))
        .unwrap();
    let error = GeomBezierSurface::from_stored_coefficients(
        &invalid,
        &weights([f64::INFINITY, 1., 1., 1.]),
    )
    .err()
    .unwrap();
    assert_eq!(
        native_message(error),
        "Bezier pole [NaN, 0.0, 0.0]; expected finite coordinates"
    );
}
#[test]
fn retained_diagnostics_keep_the_first_row_major_nonfinite_weight() {
    let error = GeomBezierSurface::from_stored_coefficients(
        &poles(),
        &weights([f64::NAN, f64::INFINITY, 1., 1.]),
    )
    .err()
    .unwrap();
    assert_eq!(
        native_message(error),
        "Bezier weight NaN; expected finite positive weight"
    );
}
#[test]
fn every_coordinate_rejects_nan_and_both_infinities() {
    for axis in 0..3 {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut xyz = [0.; 3];
            xyz[axis] = value;
            let mut invalid = poles();
            invalid
                .set_value(1, 1, Point3::new(xyz[0], xyz[1], xyz[2]))
                .unwrap();
            let error = GeomBezierSurface::from_stored_coefficients(&invalid, &weights([1.; 4]))
                .err()
                .unwrap();
            assert_eq!(
                native_message(error),
                format!("Bezier pole {xyz:?}; expected finite coordinates")
            );
        }
    }
}
