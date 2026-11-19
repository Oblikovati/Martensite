// SPDX-License-Identifier: GPL-2.0-only
use super::*;
const Q: [i64; 7] = [6400, 6410, 6398, 6432, 6407, 6444, 6464];
const C: [i64; 6] = [1, -3, 7, 11, -5, 13];
#[test]
fn all_129_degree_six_and_five_nodes_enclose_independent_exact_bernstein_values() {
    run(|arithmetic| {
        check_sequence(&Q, 128, arithmetic);
        check_sequence(&C, 128, arithmetic);
    });
}
#[test]
fn level_zero_and_one_seed_only_available_nodes_without_extrapolating() {
    run(|arithmetic| {
        for denominator in [2, 4] {
            check_sequence(&Q, denominator, arithmetic);
            check_sequence(&C, denominator, arithmetic);
        }
    });
}
fn check_sequence(coefficients: &[i64], denominator: u64, arithmetic: &mut Arithmetic<'_>) {
    let poles = capture(coefficients, arithmetic);
    let mut sequence = forward::ForwardDifference::new(&poles, denominator, arithmetic).unwrap();
    let mut visited = 0;
    for index in 0..=denominator {
        let expected = bernstein_value(coefficients, index, denominator);
        assert_contains(sequence.value(), &expected);
        visited += 1;
        if index < denominator {
            sequence.advance(arithmetic).unwrap();
        }
    }
    assert_eq!(visited, denominator + 1);
    let endpoint = ExactRatio::from_finite(*coefficients.last().unwrap() as f64);
    let [lower, upper] = exact_interval(sequence.value());
    assert_eq!(lower, endpoint);
    assert_eq!(upper, endpoint);
}
fn bernstein_value(coefficients: &[i64], index: u64, denominator: u64) -> ExactRatio {
    let degree = coefficients.len() - 1;
    let mut numerator = BigInt::from(0);
    for (power, coefficient) in coefficients.iter().enumerate() {
        let binomial = choose(degree, power);
        numerator += BigInt::from(*coefficient)
            * binomial
            * BigInt::from(index).pow(power as u32)
            * BigInt::from(denominator - index).pow((degree - power) as u32);
    }
    ExactRatio::new(numerator, BigInt::from(denominator).pow(degree as u32))
}
fn choose(degree: usize, power: usize) -> u64 {
    (0..power).fold(1, |value, index| {
        value * (degree - index) as u64 / (index + 1) as u64
    })
}
