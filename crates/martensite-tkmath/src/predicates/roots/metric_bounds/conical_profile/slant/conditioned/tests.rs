// SPDX-License-Identifier: GPL-2.0-only
use super::*;
fn execute<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 100_000,
        },
        &mut never,
    );
    body(&mut arithmetic)
}
fn capture(values: &[f64], arithmetic: &mut Arithmetic<'_>) -> Vec<Interval> {
    super::super::capture(values, arithmetic).unwrap()
}
fn check_oracle(residual: &[Interval]) {
    let degree = residual.len() - 1;
    let denominator = num_bigint::BigInt::from(degree * (degree - 1));
    let grid: num_bigint::BigInt = num_bigint::BigInt::from(1) << 128;
    for (k, value) in residual.iter().enumerate() {
        let numerator = -3 * (degree * (degree - 1)) as i64
            + 6 * (k * (degree - 1)) as i64
            + 9 * (k * k.saturating_sub(1)) as i64;
        let exact = num_bigint::BigInt::from(numerator) * &grid;
        assert!(&value.lower * &denominator <= exact);
        assert!(&value.upper * &denominator >= exact);
    }
}
#[test]
fn elevated_coefficients_enclose_independent_quadratic_rational_oracle() {
    execute(|arithmetic| {
        let q = capture(&[1.; 7], arithmetic);
        let n = capture(&[1., 2., 3., 4.], arithmetic);
        let w = capture(&[2.; 4], arithmetic);
        let coefficients = residual(&q, &n, &w, arithmetic).unwrap();
        assert_eq!(coefficients.len(), 13);
        check_oracle(&coefficients);
    });
}
#[test]
fn zero_residual_and_nonunit_constant_weight_are_outward_enclosed() {
    execute(|arithmetic| {
        let q = capture(&[1.; 7], arithmetic);
        let n = capture(&[2.; 4], arithmetic);
        let w = capture(&[2.; 4], arithmetic);
        let coefficients = residual(&q, &n, &w, arithmetic).unwrap();
        assert_eq!(coefficients.len(), 13);
        assert!(
            coefficients
                .iter()
                .all(|v| v.lower <= 0.into() && v.upper >= 0.into())
        );
    });
}
#[test]
fn actual_conditioned_work_is_less_without_reducing_final_degree() {
    execute(|arithmetic| {
        let q = capture(&[1.; 7], arithmetic);
        let n = capture(&[1., 2., 3., 4.], arithmetic);
        let w = capture(&[2.; 4], arithmetic);
        let start = arithmetic.operations();
        let new = residual(&q, &n, &w, arithmetic).unwrap();
        let reduced_work = arithmetic.operations() - start;
        let start = arithmetic.operations();
        let old = slant_residual(&q, &n, &w, arithmetic).unwrap();
        assert!(reduced_work < arithmetic.operations() - start);
        assert_eq!(new.len(), old.len());
        check_oracle(&old);
    });
}
#[test]
fn changed_or_nonpoint_weights_retain_the_exact_general_execution() {
    execute(|arithmetic| {
        let mut weights = capture(&[1.; 4], arithmetic);
        weights[3] = arithmetic.scalar(1_f64.next_up()).unwrap();
        check_fallback(&weights, arithmetic);
        let point = arithmetic.scalar(1.).unwrap();
        let rounded = Interval {
            lower: point.lower.clone(),
            upper: point.upper + 1,
        };
        let weights = std::array::from_fn::<_, 4, _>(|_| rounded.clone());
        check_fallback(&weights, arithmetic);
    });
}
fn check_fallback(weights: &[Interval], arithmetic: &mut Arithmetic<'_>) {
    let q = capture(&[1.; 7], arithmetic);
    let n = capture(&[1., 2., 3., 4.], arithmetic);
    let start = arithmetic.operations();
    let new = residual(&q, &n, weights, arithmetic).unwrap();
    let new_work = arithmetic.operations() - start;
    let start = arithmetic.operations();
    let old = slant_residual(&q, &n, weights, arithmetic).unwrap();
    assert_eq!(new, old);
    assert_eq!(new_work, arithmetic.operations() - start);
}
#[test]
fn nonpositive_weights_still_fail_existing_positive_admission() {
    execute(|arithmetic| {
        let q = capture(&[1.; 7], arithmetic);
        let n = capture(&[1., 2., 3., 4.], arithmetic);
        for c in [0., -1.] {
            let w = capture(&[c; 4], arithmetic);
            assert!(matches!(
                slant_in_chart(&q, &n, &w, arithmetic),
                Err(SlantFailure::Nonpositive { .. })
            ));
        }
    });
}
#[test]
fn unequal_reduced_degrees_retain_the_original_final_basis() {
    execute(|arithmetic| {
        let q = capture(&[1.; 3], arithmetic);
        let n = capture(&[1., 2., 3., 4.], arithmetic);
        let w = capture(&[2.; 4], arithmetic);
        let coefficients = residual(&q, &n, &w, arithmetic).unwrap();
        assert_eq!(coefficients.len(), 9);
        check_oracle(&coefficients);
    });
}
