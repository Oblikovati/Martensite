//! Regression tests for overflow- and underflow-safe normalization (#550).
use martensite_tkmath::gp::{Direction3, normalize_finite};

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
fn normalize_finite_handles_extreme_large_and_small_vectors() {
    let d_max = normalize_finite([f64::MAX, 0.0, 0.0]).unwrap();
    assert_eq!(d_max, [1.0, 0.0, 0.0]);

    let inv_sqrt3 = 1.0 / 3.0_f64.sqrt();
    let d_diag_max = normalize_finite([f64::MAX, -f64::MAX, f64::MAX]).unwrap();
    assert_unit_3d(d_diag_max, [inv_sqrt3, -inv_sqrt3, inv_sqrt3]);

    let d_small = normalize_finite([0.0, 1e-200, 0.0]).unwrap();
    assert_eq!(d_small, [0.0, 1.0, 0.0]);

    let d_min_pos = normalize_finite([0.0, 0.0, -f64::MIN_POSITIVE]).unwrap();
    assert_eq!(d_min_pos, [0.0, 0.0, -1.0]);

    let d_subnormal = normalize_finite([f64::from_bits(1), 0.0, 0.0]).unwrap();
    assert_eq!(d_subnormal, [1.0, 0.0, 0.0]);
}

#[test]
fn normalize_finite_rejects_zero_and_nonfinite_inputs() {
    assert!(normalize_finite([0.0, 0.0, 0.0]).is_none());
    assert!(normalize_finite([-0.0, 0.0, -0.0]).is_none());
    assert!(normalize_finite([f64::INFINITY, 0.0, 0.0]).is_none());
    assert!(normalize_finite([f64::NEG_INFINITY, 1.0, 0.0]).is_none());
    assert!(normalize_finite([f64::NAN, 1.0, 0.0]).is_none());
    assert!(Direction3::new(0.0, 0.0, 0.0).is_err());
}

#[test]
fn normalize_finite_handles_2d_extreme_scales() {
    let d2_max = normalize_finite([f64::MAX, 0.0]).unwrap();
    assert_unit_2d(d2_max, [1.0, 0.0]);
    let d2_small = normalize_finite([0.0, -1e-250]).unwrap();
    assert_unit_2d(d2_small, [0.0, -1.0]);
    let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
    let d2_diag = normalize_finite([f64::MAX, f64::MAX]).unwrap();
    assert_unit_2d(d2_diag, [inv_sqrt2, inv_sqrt2]);
    assert!(normalize_finite([f64::INFINITY, 0.0]).is_none());
    assert!(normalize_finite([f64::NAN, 1.0]).is_none());
}
