//! Exact native rows and legacy replay remain separate construction contracts (#464).
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
fn near_unit_boundary_rows_remain_bit_identical() {
    let down = f64::from_bits(1f64.to_bits() - 1);
    let curves = [curve(0., &[1., down]), curve(1., &[1., down])];
    let fill = GeomFillBezierCurves::ruled_from_stored_boundaries(&curves[0], &curves[1]).unwrap();
    let surface = fill.surface();
    for col in 1..=2 {
        for row in 1..=2 {
            let input = curves[col as usize - 1].get().unwrap();
            assert_eq!(
                surface.weight(row, col).unwrap().to_bits(),
                input.weight(row).unwrap().to_bits()
            );
            assert_eq!(surface.pole(row, col).unwrap(), input.pole(row).unwrap());
        }
    }
    assert!(surface.is_u_rational());
    assert!(!surface.is_v_rational());
}
#[test]
fn constant_curve_weights_are_already_canonical_before_filling() {
    let curves = [curve(0., &[2., 2.]), curve(1., &[2., 2.])];
    for input in &curves {
        assert!(!input.is_rational());
        assert_eq!(input.weight(1).unwrap(), 1.);
        assert_eq!(input.weight(2).unwrap(), 1.);
    }
    let fill = GeomFillBezierCurves::ruled_from_stored_boundaries(&curves[0], &curves[1]).unwrap();
    assert_eq!(fill.surface().weight(1, 1).unwrap(), 1.);
    assert_eq!(fill.surface().weight(2, 2).unwrap(), 1.);
}
#[test]
fn retained_rows_match_the_independent_linear_rational_parameterization() {
    let curves = [curve(0., &[1., 0.25]), curve(1., &[1., 0.25])];
    let fill = GeomFillBezierCurves::ruled_from_stored_boundaries(&curves[0], &curves[1]).unwrap();
    let surface = fill.surface();
    for (u, v) in [(0.25, 0.75), (0.5, 0.25)] {
        let point = surface.value(u, v).unwrap();
        assert!((point.x() - u * 0.25 / (1. - u + u * 0.25)).abs() < 2e-16);
        assert!((point.y() - v).abs() < 2e-16);
    }
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
fn unequal_degree_and_null_boundaries_are_refused_without_source_edits() {
    let first = curve(0., &[1., 0.25]);
    let second = curve(1., &[1., 0.25]);
    second.increase(2).unwrap();
    let before = first.poles();
    assert!(GeomFillBezierCurves::ruled_from_stored_boundaries(&first, &second).is_err());
    assert!(
        GeomFillBezierCurves::ruled_from_stored_boundaries(&first, &Handle::default()).is_err()
    );
    assert_eq!(first.poles().as_slice(), before.as_slice());
}
