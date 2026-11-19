// SPDX-License-Identifier: GPL-2.0-only
//! Independent Chebyshev endpoint and rational denominator controls.
use super::*;
fn execute(body: impl FnOnce(&mut Arithmetic<'_>)) {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 10000,
        },
        &mut never,
    );
    body(&mut arithmetic);
}
fn fit(arithmetic: &mut Arithmetic<'_>) -> FitCapture {
    let zero = arithmetic.scalar(0.).unwrap();
    let one = arithmetic.scalar(1.).unwrap();
    FitCapture {
        p: vec![zero.clone()],
        numerator: vec![one.clone(); 4],
        weights: vec![one.clone(); 4],
        p_hull: zero.clone(),
        weight_hull: one.clone(),
        b: one,
        p_rate: vec![zero],
    }
}
#[test]
fn unit_dual_markov_covers_independent_chebyshev_six_endpoint_derivative() {
    execute(|arithmetic| {
        // T6(2t-1)=2048t6-6144t5+6912t4-3584t3+840t2-72t+1.
        // Its unit-interval sup norm is1; the exact derivative at0 has magnitude72.
        let input = fit(arithmetic);
        let actual = vec![arithmetic.scalar(1.).unwrap(); 4];
        let bound = factor(&input, &actual, arithmetic).unwrap();
        let endpoint = arithmetic.scalar(72.).unwrap();
        assert!(bound.upper >= endpoint.upper);
        assert_eq!(bound.upper, endpoint.upper);
    });
}
#[test]
fn actual_denominator_rate_and_ratio_never_disappear() {
    execute(|arithmetic| {
        let input = fit(arithmetic);
        let actual = vec![
            arithmetic.scalar(1.).unwrap(),
            arithmetic.scalar(2.).unwrap(),
        ];
        let bound = factor(&input, &actual, arithmetic).unwrap();
        // Degree4 numerator, ratio<=2 and logarithmic actual-weight rate<=1.
        let independently_required = arithmetic.scalar(65.).unwrap();
        assert!(bound.upper >= independently_required.upper);
    });
}
#[test]
fn interval_crossing_zero_retains_the_true_square_minimum() {
    execute(|arithmetic| {
        let a = arithmetic.scalar(-2.).unwrap();
        let b = arithmetic.scalar(1.).unwrap();
        let range = Interval {
            lower: a.lower,
            upper: b.upper,
        };
        let square = numeric::square(&range, arithmetic).unwrap();
        assert_eq!(square.lower, 0.into());
        let four = arithmetic.scalar(4.).unwrap();
        assert!(square.upper >= four.upper);
    });
}
