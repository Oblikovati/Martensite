//! Legacy filling retains replay behavior after shared row construction moves (#464).
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomBezierCurve, GeomSurface};
use martensite_tkgeomalgo::geom_fill::{GeomFillBezierCurves, GeomFillFillingStyle};
#[path = "support/geom_fill_rows.rs"]
mod rows;
fn curve(y: f64, weights: &[f64]) -> Handle<GeomBezierCurve> {
    let points = rows::row(&[(0., y, 0.), (1., y, 0.)]);
    Handle::new(GeomBezierCurve::with_weights(&points, &rows::reals(weights)).unwrap())
}
#[test]
fn legacy_fill_still_demotes_near_unit_surface_weights() {
    let down = f64::from_bits(1f64.to_bits() - 1);
    let curves = [curve(0., &[1., down]), curve(1., &[1., down])];
    let fill =
        GeomFillBezierCurves::with_two(&curves[0], &curves[1], GeomFillFillingStyle::StretchStyle)
            .unwrap();
    assert!(!fill.surface().is_u_rational());
    assert!(!fill.surface().is_v_rational());
    assert_eq!(fill.surface().weight(2, 1).unwrap(), 1.);
}
#[test]
fn legacy_constant_rows_keep_unit_coefficients_and_polynomial_geometry() {
    let curves = [curve(0., &[2., 2.]), curve(1., &[2., 2.])];
    let fill =
        GeomFillBezierCurves::with_two(&curves[0], &curves[1], GeomFillFillingStyle::StretchStyle)
            .unwrap();
    assert_eq!(curves[0].weight(1).unwrap(), 1.);
    assert_eq!(fill.surface().weight(2, 2).unwrap(), 1.);
    assert_eq!(fill.surface().value(0.25, 0.75).unwrap().x(), 0.25);
}
#[test]
fn legacy_unequal_degrees_are_raised_without_mutating_caller_curves() {
    let curves = [curve(0., &[1., 0.25]), curve(1., &[1., 0.25])];
    curves[1].increase(2).unwrap();
    let first = curves[0].poles();
    let fill =
        GeomFillBezierCurves::with_two(&curves[0], &curves[1], GeomFillFillingStyle::StretchStyle)
            .unwrap();
    assert_eq!(fill.surface().u_degree(), 2);
    assert_eq!(curves[0].degree(), 1);
    assert_eq!(curves[0].poles().as_slice(), first.as_slice());
}
#[test]
fn legacy_rational_rows_keep_their_actual_poles_and_weights() {
    let curves = [curve(0., &[1., 0.25]), curve(1., &[1., 0.25])];
    let fill =
        GeomFillBezierCurves::with_two(&curves[0], &curves[1], GeomFillFillingStyle::StretchStyle)
            .unwrap();
    for col in 1..=2 {
        for row in 1..=2 {
            assert_eq!(
                fill.surface().pole(row, col).unwrap(),
                curves[col as usize - 1].pole(row).unwrap()
            );
            assert_eq!(
                fill.surface().weight(row, col).unwrap(),
                curves[col as usize - 1].weight(row).unwrap()
            );
        }
    }
}
