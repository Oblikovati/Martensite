// SPDX-License-Identifier: GPL-2.0-only
//! Whole-span slant derivative bound.
use super::slant::*;
use super::*;
mod retained;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::derivative;
pub(super) use retained::retained_derivative;

pub(super) fn radial_lower(
    q: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, SlantFailure> {
    let q = positive_lower("squared radius", q, arithmetic)?;
    // q/(1+q) is no greater than sqrt(q) for every positive q.
    let one = arithmetic.scalar(1.)?;
    let denominator = arithmetic.add(&q, &one)?;
    let radius = arithmetic.divide(&q, &denominator)?;
    Ok(Interval {
        lower: radius.lower.clone(),
        upper: radius.lower,
    })
}

pub(super) fn fit_rate(
    numerator: &[Interval],
    weights: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, SlantFailure> {
    let da = derivative(numerator, arithmetic)?;
    let dw = derivative(weights, arithmetic)?;
    let first = product(&da, weights, arithmetic)?;
    let second = product(numerator, &dw, arithmetic)?;
    algebra::difference(&first, &second, arithmetic).map_err(SlantFailure::Polynomial)
}

pub(super) fn radial_derivative_bound(
    q: &[Interval],
    numerator: &[Interval],
    weights: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, SlantFailure> {
    let radius = radial_lower(q, arithmetic)?;
    let weight = positive_lower("weight", weights, arithmetic)?;
    let slant_error = slant_from_captured(q, numerator, weights, arithmetic)?;
    let rate = fit_rate(numerator, weights, arithmetic)?;
    let w2 = arithmetic.multiply(&weight, &weight)?;
    let fit_rate = arithmetic.divide(&magnitude_interval(&rate), &w2)?;
    let correction = arithmetic.multiply(&fit_rate, &slant_error)?;
    let correction = arithmetic.divide(&correction, &radius)?;
    // q' - 2 s s' = 2 r (r' - s') + 2 (r - s) s'.
    let residual = radial_residual(q, numerator, weights, &rate, arithmetic)?;
    let w3 = arithmetic.multiply(&w2, &weight)?;
    let denominator = arithmetic.multiply(&radius, &w3)?;
    let two = arithmetic.scalar(2.)?;
    let denominator = arithmetic.multiply(&denominator, &two)?;
    let first = arithmetic.divide(&magnitude_interval(&residual), &denominator)?;
    Ok(arithmetic.add(&first, &correction)?)
}

pub(super) fn radial_residual(
    q: &[Interval],
    numerator: &[Interval],
    weights: &[Interval],
    rate: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, SlantFailure> {
    let dq = derivative(q, arithmetic)?;
    let w2 = product(weights, weights, arithmetic)?;
    let w3 = product(&w2, weights, arithmetic)?;
    let first = product(&dq, &w3, arithmetic)?;
    let second = product(numerator, rate, arithmetic)?;
    let two = arithmetic.scalar(2.)?;
    let second = crate::predicates::roots::certificate_arithmetic::bernstein_bounds::scale(
        &second, &two, arithmetic,
    )?;
    algebra::difference(&first, &second, arithmetic).map_err(SlantFailure::Polynomial)
}

#[test]
pub(super) fn bound_covers_independent_linear_radius_derivative() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    let source = admit_source(&[[3., 0.], [3., 4.]], &mut arithmetic).unwrap();
    let q = squared_distance(&source, &mut arithmetic).unwrap();
    let a = capture(&[3., 5.], &mut arithmetic).unwrap();
    let w = capture(&[1., 1.], &mut arithmetic).unwrap();
    let bound = radial_derivative_bound(&q, &a, &w, &mut arithmetic).unwrap();
    let upper = DyadicBound::new(bound.upper, -128);
    assert_eq!(upper.compare_f64(2.).unwrap(), std::cmp::Ordering::Greater);
}

#[test]
pub(super) fn exact_linear_radius_has_zero_derivative_error() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    let source = admit_source(&[[3., 0.], [5., 0.]], &mut arithmetic).unwrap();
    let q = squared_distance(&source, &mut arithmetic).unwrap();
    let a = capture(&[3., 5.], &mut arithmetic).unwrap();
    let w = capture(&[1., 1.], &mut arithmetic).unwrap();
    let bound = radial_derivative_bound(&q, &a, &w, &mut arithmetic).unwrap();
    let upper = DyadicBound::new(bound.upper, -128);
    assert_eq!(upper.compare_f64(1e-25).unwrap(), std::cmp::Ordering::Less);
}

#[test]
pub(super) fn varying_weights_cover_known_endpoint_rate() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    let source = admit_source(&[[3., 0.], [3., 0.]], &mut arithmetic).unwrap();
    let q = squared_distance(&source, &mut arithmetic).unwrap();
    let a = capture(&[3., 8.], &mut arithmetic).unwrap();
    let w = capture(&[1., 2.], &mut arithmetic).unwrap();
    let bound = radial_derivative_bound(&q, &a, &w, &mut arithmetic).unwrap();
    let upper = DyadicBound::new(bound.upper, -128);
    assert_eq!(upper.compare_f64(2.).unwrap(), std::cmp::Ordering::Greater);
}

#[test]
pub(super) fn derivative_bound_covers_varying_weight_controls() {
    for (a, w) in [
        ([3., 5.], [1., 1.]),
        ([3., 8.], [1., 2.]),
        ([6., 4.], [2., 1.]),
    ] {
        check_sampled_rate(a, w);
    }
}

#[test]
pub(super) fn touching_axis_refuses_derivative_bound() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    let source = admit_source(&[[0., 0.], [3., 0.]], &mut arithmetic).unwrap();
    let q = squared_distance(&source, &mut arithmetic).unwrap();
    let a = capture(&[1., 3.], &mut arithmetic).unwrap();
    let w = capture(&[1., 1.], &mut arithmetic).unwrap();
    assert!(matches!(
        radial_derivative_bound(&q, &a, &w, &mut arithmetic),
        Err(SlantFailure::Nonpositive {
            role: "squared radius",
            ..
        })
    ));
}

#[test]
pub(super) fn derivative_calculation_refuses_exhaustion() {
    let mut available = || false;
    let mut captured = Arithmetic::new(ArithmeticPolicy::default(), &mut available);
    let (q, a, w) = derivative_fixture([3., 5.], [1., 1.], &mut captured);
    let mut available = || false;
    let mut limited = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 0,
        },
        &mut available,
    );
    assert!(matches!(
        radial_derivative_bound(&q, &a, &w, &mut limited),
        Err(SlantFailure::Arithmetic(
            BoundedArithmeticFailure::WorkLimit { .. }
        ))
    ));
}

#[test]
pub(super) fn derivative_calculation_refuses_cancellation() {
    let mut available = || false;
    let mut captured = Arithmetic::new(ArithmeticPolicy::default(), &mut available);
    let (q, a, w) = derivative_fixture([3., 5.], [1., 1.], &mut captured);
    let mut cancelled = || true;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    assert!(matches!(
        radial_derivative_bound(&q, &a, &w, &mut arithmetic),
        Err(SlantFailure::Arithmetic(
            BoundedArithmeticFailure::Cancelled
        ))
    ));
}

use crate::predicates::roots::metric_bounds::cylindrical_profile::algebra;

#[cfg(test)]
fn check_sampled_rate(a: [f64; 2], w: [f64; 2]) {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    let (q, numerator, weights) = derivative_fixture(a, w, &mut arithmetic);
    let bound = radial_derivative_bound(&q, &numerator, &weights, &mut arithmetic).unwrap();
    let upper = DyadicBound::new(bound.upper, -128);
    for index in 0..=100 {
        let t = f64::from(index) / 100.;
        let denominator = w[0] * (1. - t) + w[1] * t;
        let fit_rate = (a[1] * w[0] - a[0] * w[1]) / (denominator * denominator);
        let source_rate = 16. * t / (9. + 16. * t * t).sqrt();
        assert!(
            upper
                .compare_f64((source_rate - fit_rate).abs())
                .unwrap()
                .is_ge()
        );
    }
}

#[cfg(test)]
fn derivative_fixture(
    numerator: [f64; 2],
    weights: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> (Vec<Interval>, Vec<Interval>, Vec<Interval>) {
    let source = admit_source(&[[3., 0.], [3., 4.]], arithmetic).unwrap();
    let q = squared_distance(&source, arithmetic).unwrap();
    let a = capture(&numerator, arithmetic).unwrap();
    let w = capture(&weights, arithmetic).unwrap();
    (q, a, w)
}
