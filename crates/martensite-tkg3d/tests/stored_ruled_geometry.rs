//! Owned-state admission and native patch correspondence for #464.
use martensite_tkernel::ncollection::Array1;
use martensite_tkg3d::geom::{
    GeomBezierSurface, GeomCurve, GeomSurface, StoredBezierBoundary, StoredRuledGeometry,
};
use martensite_tkmath::gp::Point3;

fn row(y: f64, values: &[f64]) -> StoredBezierBoundary {
    let mut poles = Array1::with_bounds(1, values.len() as i32).unwrap();
    let mut weights = Array1::with_bounds(1, values.len() as i32).unwrap();
    for (index, &weight) in values.iter().enumerate() {
        poles
            .set_value(index as i32 + 1, Point3::new(index as f64, y, 0.))
            .unwrap();
        weights.set_value(index as i32 + 1, weight).unwrap();
    }
    StoredBezierBoundary { poles, weights }
}
fn patch(weights: &[f64]) -> StoredRuledGeometry {
    let rows = [row(0., weights), row(1., weights)];
    GeomBezierSurface::ruled_with_stored_boundaries(rows).unwrap()
}
fn check_rows(patch: &StoredRuledGeometry) {
    for (index, boundary) in patch.boundaries.iter().enumerate() {
        for pole in 1..=boundary.nb_poles() {
            assert_eq!(
                patch.surface.pole(pole, index as i32 + 1).unwrap(),
                boundary.pole(pole).unwrap()
            );
            assert_eq!(
                patch
                    .surface
                    .weight(pole, index as i32 + 1)
                    .unwrap()
                    .to_bits(),
                boundary.weight(pole).unwrap().to_bits()
            );
        }
    }
}
#[test]
fn near_unit_boundaries_and_surface_keep_the_same_actual_coefficient_rows() {
    let geometry = patch(&[1., f64::from_bits(1f64.to_bits() - 1)]);
    check_rows(&geometry);
    assert!(geometry.surface.is_u_rational());
    assert!(!geometry.surface.is_v_rational());
    assert_eq!(
        geometry.surface.weight(2, 1).unwrap().to_bits(),
        1f64.to_bits() - 1
    );
}
#[test]
fn constant_rows_use_the_existing_curve_canonicalization_once() {
    let geometry = patch(&[2., 2.]);
    check_rows(&geometry);
    assert_eq!(geometry.boundaries[0].weight(1).unwrap(), 1.);
    assert_eq!(geometry.surface.weight(2, 2).unwrap(), 1.);
    assert!(!geometry.surface.is_u_rational());
    assert!(!geometry.surface.is_v_rational());
}
#[test]
fn complete_native_patch_matches_the_independent_rational_parameterization() {
    let geometry = patch(&[1., 0.25]);
    for u in [0., 0.25, 0.5, 1.] {
        let expected = u * 0.25 / (1. - u + u * 0.25);
        assert!((geometry.surface.value(u, 0.5).unwrap().x() - expected).abs() < 2e-16);
        for (index, curve) in geometry.boundaries.iter().enumerate() {
            assert_eq!(
                geometry.surface.value(u, index as f64).unwrap(),
                curve.value(u).unwrap()
            );
        }
    }
}
#[test]
fn original_nonfinite_weights_are_refused_even_when_native_curves_would_demote_them() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            GeomBezierSurface::ruled_with_stored_boundaries([
                row(0., &[value; 2]),
                row(1., &[1.; 2])
            ])
            .is_err()
        );
    }
}
#[test]
fn malformed_or_unequal_counts_are_refused_before_native_handles_escape() {
    assert!(
        GeomBezierSurface::ruled_with_stored_boundaries([row(0., &[1.]), row(1., &[1.])]).is_err()
    );
    assert!(
        GeomBezierSurface::ruled_with_stored_boundaries([row(0., &[1., 1.]), row(1., &[1.; 3])])
            .is_err()
    );
}
#[test]
fn nonfinite_poles_are_refused_before_native_handles_escape() {
    let mut invalid = row(0., &[1., 1.]);
    invalid
        .poles
        .set_value(1, Point3::new(f64::NAN, 0., 0.))
        .unwrap();
    assert!(
        GeomBezierSurface::ruled_with_stored_boundaries([invalid, row(1., &[1., 1.])]).is_err()
    );
}
#[test]
fn curves_and_surface_have_independent_mutable_owners_after_assembly() {
    let geometry = patch(&[1., 0.25]);
    let original = geometry.surface.pole(1, 1).unwrap();
    geometry.boundaries[0]
        .set_pole(1, Point3::new(5., 6., 7.))
        .unwrap();
    assert_eq!(geometry.surface.pole(1, 1).unwrap(), original);
    geometry
        .surface
        .set_pole(1, 2, Point3::new(8., 9., 10.))
        .unwrap();
    assert_eq!(
        geometry.boundaries[1].pole(1).unwrap(),
        Point3::new(0., 1., 0.)
    );
}
#[test]
fn owned_input_and_native_patch_are_send_and_sync() {
    let input = row(0., &[1., 1.]);
    let _: &(dyn Send + Sync) = &input;
    let geometry = patch(&[1., 1.]);
    let _: &(dyn Send + Sync) = &geometry;
}

#[test]
fn readonly_shared_input_buffers_cannot_alias_the_native_patch() {
    static POLES: [Point3; 2] = [Point3::new(0., 0., 0.), Point3::new(1., 0., 0.)];
    static WEIGHTS: [f64; 2] = [1., 0.25];
    let first = StoredBezierBoundary {
        poles: Array1::from_shared(&POLES, 1),
        weights: Array1::from_shared(&WEIGHTS, 1),
    };
    let geometry =
        GeomBezierSurface::ruled_with_stored_boundaries([first, row(1., &WEIGHTS)]).unwrap();
    geometry.boundaries[0]
        .set_pole(1, Point3::new(5., 6., 7.))
        .unwrap();
    assert_eq!(POLES[0], Point3::new(0., 0., 0.));
    assert_eq!(geometry.surface.pole(1, 1).unwrap(), POLES[0]);
}
