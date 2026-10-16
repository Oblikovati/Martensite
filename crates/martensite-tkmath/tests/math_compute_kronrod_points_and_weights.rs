//! Order-125 Kronrod points and weights memory regression.
#![allow(non_snake_case)]
use martensite_tkmath::math::KronrodPointsAndWeights;
#[test]
fn math_compute_kronrod_points_and_weights_compute_with_order125() {
    // Rust panics and constructor failures fail this test, matching the catch-all FAIL.
    let calculation: KronrodPointsAndWeights = KronrodPointsAndWeights::new(125).unwrap();
    assert!(
        calculation.is_done(),
        "Kronrod points and weights calculation should succeed"
    );
    let is_ok: bool = calculation.is_done();
    assert!(is_ok);
}
