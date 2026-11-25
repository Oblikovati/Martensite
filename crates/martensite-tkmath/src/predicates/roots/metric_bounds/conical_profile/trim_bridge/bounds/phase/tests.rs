// SPDX-License-Identifier: GPL-2.0-only
use super::*;
fn execute(body: impl FnOnce(&mut Arithmetic<'_>)) {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut never);
    body(&mut arithmetic);
}
fn captured(values: &[f64], arithmetic: &mut Arithmetic<'_>) -> Vec<Interval> {
    values
        .iter()
        .map(|value| arithmetic.scalar(*value).unwrap())
        .collect()
}
fn upper(bound: &Interval, arithmetic: &Arithmetic<'_>) -> f64 {
    arithmetic.upper_f64(bound.upper.clone()).unwrap()
}
fn hermite() -> [f64; 4] {
    let angle = 2. * martensite_cmath::atan(0.25);
    [0., 1. / 6., angle - 0.5 / (3. * 1.0625), angle]
}
#[test]
fn native_cubic_hermite_cancellation_is_tighter_than_absolute_residual_mean() {
    execute(|arithmetic| {
        let u = captured(&hermite(), arithmetic);
        let tangent = captured(&[0., 0.25], arithmetic);
        let residual = native_angular_residual(&u, &tangent, arithmetic).unwrap();
        let mean = angular::absolute_coefficient_mean(&residual.numerator, arithmetic).unwrap();
        let zero = arithmetic.zero();
        let bound = conditioned::bound(&residual, &zero, arithmetic).unwrap();
        assert!(upper(&bound, arithmetic) < upper(&mean, arithmetic));
        check_hermite_samples(upper(&bound, arithmetic));
    });
}
fn check_hermite_samples(bound: f64) {
    let controls = hermite();
    for index in 0..=100 {
        let t = f64::from(index) / 100.;
        let u = cubic(controls, t);
        let actual = (u - 2. * martensite_cmath::atan(0.25 * t)).abs();
        assert!(actual <= bound);
    }
}
fn cubic(poles: [f64; 4], t: f64) -> f64 {
    let s = 1. - t;
    poles[0] * s * s * s
        + 3. * poles[1] * s * s * t
        + 3. * poles[2] * s * t * t
        + poles[3] * t * t * t
}
#[test]
fn crossing_zero_tangent_uses_the_exact_positive_denominator_floor() {
    execute(|arithmetic| {
        let tangent = captured(&[-0.5, 0.5], arithmetic);
        let u = captured(&[0., 0.], arithmetic);
        let residual = native_angular_residual(&u, &tangent, arithmetic).unwrap();
        assert!(polynomial::lower(&residual.denominator) < residual.unit.lower);
        let phase = arithmetic
            .scalar(-2. * martensite_cmath::atan(0.5))
            .unwrap();
        let anchor = arithmetic.subtract(&u[0], &phase).unwrap();
        let bound = conditioned::bound(&residual, &anchor, arithmetic).unwrap();
        assert!(upper(&bound, arithmetic) >= 2. * martensite_cmath::atan(0.5));
    });
}
#[test]
fn varying_positive_denominator_contains_the_independent_endpoint_change() {
    execute(|arithmetic| {
        let tangent = captured(&[1., 2.], arithmetic);
        let u = captured(&[0., 0.], arithmetic);
        let residual = native_angular_residual(&u, &tangent, arithmetic).unwrap();
        let zero = arithmetic.zero();
        let bound = conditioned::bound(&residual, &zero, arithmetic).unwrap();
        let expected = 2. * (martensite_cmath::atan(2.) - martensite_cmath::atan(1.));
        assert!(upper(&bound, arithmetic) >= expected);
        assert!(upper(&bound, arithmetic) <= 1.);
    });
}
