// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::certificate_arithmetic::ExactRatio;
use num_bigint::BigInt;
mod forward_grid;
mod majorants;

fn run<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> (T, u32) {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 1_000_000,
        },
        &mut never,
    );
    let result = body(&mut arithmetic);
    (result, arithmetic.operations())
}
fn capture(values: &[i64], arithmetic: &mut Arithmetic<'_>) -> Vec<Interval> {
    values
        .iter()
        .map(|value| arithmetic.scalar(*value as f64).unwrap())
        .collect()
}
fn exact_interval(value: &Interval) -> [ExactRatio; 2] {
    let scale = BigInt::from(1) << 128usize;
    [
        ExactRatio::new(value.lower.clone(), scale.clone()),
        ExactRatio::new(value.upper.clone(), scale),
    ]
}
fn assert_contains(interval: &Interval, expected: &ExactRatio) {
    let [lower, upper] = exact_interval(interval);
    assert!(&lower <= expected && &upper >= expected);
}
#[test]
fn unresolved_global_radius_retains_the_existing_local_bernstein_fallback() {
    let poles = [[-1., 1.], [1., 1.]];
    let (old, old_work) =
        run(|arithmetic| super::super::integrate_span(&poles, 2, arithmetic).unwrap());
    let (new, new_work) = run(|arithmetic| integrate_span(&poles, 2, arithmetic).unwrap());
    assert_eq!(old, new);
    assert_eq!(old_work, new_work);
}
#[test]
fn scalar_and_streaming_proofs_enclose_the_same_nontrivial_authored_curve_integral() {
    let poles = [[80., 0.], [81., 2.], [82., -1.], [83., 1.]];
    let (old, old_work) =
        run(|arithmetic| super::super::integrate_span(&poles, 6, arithmetic).unwrap());
    let (new, work) = run(|arithmetic| integrate_span(&poles, 6, arithmetic).unwrap());
    assert!(new.lower <= old.upper && old.lower <= new.upper);
    assert!(work < old_work);
}
