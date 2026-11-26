//! Regression tests for overflow- and underflow-safe normalization (#550).
use martensite_tkmath::gp::{Direction2, Direction3, Vector2, Vector3, Xy, Xyz};

fn assert_unit_3d(coords: [f64; 3], expected: [f64; 3]) {
    for (actual, exp) in coords.iter().zip(expected.iter()) {
        assert!(actual.is_finite(), "component must be finite: {coords:?}");
        assert!(
            (actual - exp).abs() <= 1e-15,
            "expected {expected:?}, got {coords:?}"
        );
    }
    let norm = (coords[0] * coords[0] + coords[1] * coords[1] + coords[2] * coords[2]).sqrt();
    assert!(
        (norm - 1.0).abs() <= 1e-15,
        "non-unit norm {norm} for {coords:?}"
    );
}

fn assert_unit_2d(coords: [f64; 2], expected: [f64; 2]) {
    for (actual, exp) in coords.iter().zip(expected.iter()) {
        assert!(actual.is_finite(), "component must be finite: {coords:?}");
        assert!(
            (actual - exp).abs() <= 1e-15,
            "expected {expected:?}, got {coords:?}"
        );
    }
    let norm = (coords[0] * coords[0] + coords[1] * coords[1]).sqrt();
    assert!(
        (norm - 1.0).abs() <= 1e-15,
        "non-unit norm {norm} for {coords:?}"
    );
}

#[test]
fn direction3_normalizes_extreme_large_and_small_finite_vectors() {
    let d_max = Direction3::new(f64::MAX, 0.0, 0.0).unwrap();
    assert_eq!(*d_max.coordinates(), [1.0, 0.0, 0.0]);

    let inv_sqrt3 = 1.0 / 3.0_f64.sqrt();
    let d_diag_max = Direction3::new(f64::MAX, -f64::MAX, f64::MAX).unwrap();
    assert_unit_3d(
        *d_diag_max.coordinates(),
        [inv_sqrt3, -inv_sqrt3, inv_sqrt3],
    );

    let d_small = Direction3::new(0.0, 1e-200, 0.0).unwrap();
    assert_eq!(*d_small.coordinates(), [0.0, 1.0, 0.0]);

    let d_min_pos = Direction3::new(0.0, 0.0, -f64::MIN_POSITIVE).unwrap();
    assert_eq!(*d_min_pos.coordinates(), [0.0, 0.0, -1.0]);

    let d_subnormal = Direction3::new(f64::from_bits(1), 0.0, 0.0).unwrap();
    assert_eq!(*d_subnormal.coordinates(), [1.0, 0.0, 0.0]);
}

#[test]
fn direction3_rejects_zero_and_nonfinite_inputs() {
    assert!(Direction3::new(0.0, 0.0, 0.0).is_err());
    assert!(Direction3::new(-0.0, 0.0, -0.0).is_err());
    assert!(Direction3::new(f64::INFINITY, 0.0, 0.0).is_err());
    assert!(Direction3::new(f64::NEG_INFINITY, 1.0, 0.0).is_err());
    assert!(Direction3::new(f64::NAN, 1.0, 0.0).is_err());
}

#[test]
fn shared_gp_normalization_paths_handle_extreme_scales() {
    let d2_max = Direction2::new(f64::MAX, 0.0).unwrap();
    assert_unit_2d([d2_max.x(), d2_max.y()], [1.0, 0.0]);
    let d2_small = Direction2::new(0.0, -1e-250).unwrap();
    assert_unit_2d([d2_small.x(), d2_small.y()], [0.0, -1.0]);
    assert!(Direction2::new(f64::INFINITY, 0.0).is_err());
    assert!(Direction2::new(f64::NAN, 1.0).is_err());

    let v3_max = Vector3::new(f64::MAX, 0.0, 0.0).normalized().unwrap();
    assert_unit_3d([v3_max.x(), v3_max.y(), v3_max.z()], [1.0, 0.0, 0.0]);
    let v3_small = Vector3::new(0.0, 1e-250, 0.0).normalized().unwrap();
    assert_unit_3d([v3_small.x(), v3_small.y(), v3_small.z()], [0.0, 1.0, 0.0]);

    let xyz_max = Xyz::new(0.0, 0.0, f64::MAX).normalized().unwrap();
    assert_unit_3d([xyz_max.x(), xyz_max.y(), xyz_max.z()], [0.0, 0.0, 1.0]);

    let v2_max = Vector2::new(f64::MAX, f64::MAX).normalized().unwrap();
    let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
    assert_unit_2d([v2_max.x(), v2_max.y()], [inv_sqrt2, inv_sqrt2]);

    let xy_small = Xy::new(-1e-250, 0.0).normalized().unwrap();
    assert_unit_2d([xy_small.x(), xy_small.y()], [-1.0, 0.0]);
}
