// SPDX-License-Identifier: GPL-2.0-only
use super::*;
#[test]
fn constant_radius_times_quartic_has_exact_fourth_derivative_twelve() {
    run(|arithmetic| {
        let q = capture(&[4], arithmetic);
        let c = capture(&[0, 0, 0, 0, 1], arithmetic);
        let proof = majorant::bound(&q, &c, arithmetic).unwrap();
        assert_eq!(
            proof.fourth_derivative_upper.upper,
            BigInt::from(12) << 128usize
        );
    });
}
#[test]
fn linear_radius_inverse_matches_the_independent_factorial_fourth_derivative() {
    run(|arithmetic| {
        let q = capture(&[6400, 6480, 6561], arithmetic);
        let c = capture(&[1], arithmetic);
        let proof = majorant::bound(&q, &c, arithmetic).unwrap();
        let expected = ExactRatio::new(24.into(), BigInt::from(80).pow(5));
        assert_contains(&proof.fourth_derivative_upper, &expected);
        let upper = exact_interval(&proof.fourth_derivative_upper)[1].clone();
        assert!(upper < expected.multiply(&ExactRatio::from_finite(16.)));
    });
}
#[test]
fn varying_numerator_uses_the_full_fourth_leibniz_sum() {
    run(|arithmetic| {
        let q = capture(&[6400, 6480, 6561], arithmetic);
        let c = capture(&[0, 0, 1], arithmetic);
        let proof = majorant::bound(&q, &c, arithmetic).unwrap();
        // t²/(80+t)=t-80+6400/(80+t), so the maximum fourth derivative is24/80³.
        let expected = ExactRatio::new(24.into(), BigInt::from(80).pow(3));
        assert_contains(&proof.fourth_derivative_upper, &expected);
    });
}
