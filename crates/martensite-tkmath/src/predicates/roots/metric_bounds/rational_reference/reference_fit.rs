//! Assign each dyadic endpoint once and certify the actual rounded cubic controls.
use super::*;

/// A cubic and the evidence binding it to one original span.
pub struct RationalReferenceSpan {
    /// Exact interval in the original normalized parameter.
    pub interval: CylindricalProfileInterval,
    /// Cubic controls in the developed plane.
    pub poles: [[f64; 2]; 4],
    /// Certified same-parameter displacement and derivative errors.
    pub bounds: RationalReferenceBounds,
    /// Strictly positive lower derivative bound for the homotopy projection.
    pub projection_rate_lower: f64,
}

pub(super) fn fit(
    request: RationalReferenceRequest<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<Vec<RationalReferenceSpan>> {
    let RationalReferenceRequest {
        source,
        level,
        limits,
        options,
        ..
    } = request;
    if level > 8 {
        return Err(Refusal::Level(level));
    }
    let policy = ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
    };
    admit_policy(limits, policy)?;
    let weights = checked_shape(source, 4)?;
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    let mut start = source.poles[0];
    (0..(1_u64 << level))
        .map(|index| {
            let interval = CylindricalProfileInterval::new(index, level)?;
            let span = fit_span(request, weights, interval, start, &mut arithmetic)?;
            start = span.poles[3];
            Ok(span)
        })
        .collect()
}

fn fit_span(
    request: RationalReferenceRequest<'_>,
    weights: &[f64],
    interval: CylindricalProfileInterval,
    start: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<RationalReferenceSpan> {
    let source = request.source;
    let native = capture(source, interval, weights, arithmetic)?;
    let end = if interval.index() + 1 == (1_u64 << interval.level()) {
        source.poles[source.poles.len() - 1]
    } else {
        end_point(&native, arithmetic)?
    };
    let poles = fit_poles(&native, [start, end], arithmetic)?;
    let reference = capture_planar(&poles, arithmetic)?;
    let mut bounds = accept(compare(&native, &reference, arithmetic)?, request.limits)?;
    let projection_rate_lower =
        monotone::margin(&native, request.direction, bounds.derivative, arithmetic)?;
    bounds.operations = arithmetic.operations();
    Ok(RationalReferenceSpan {
        interval,
        poles,
        bounds,
        projection_rate_lower,
    })
}

fn end_point(native: &Homogeneous, arithmetic: &mut Arithmetic<'_>) -> Checked<[f64; 2]> {
    let mut end = [0.; 2];
    let last = native.weight.len() - 1;
    for (axis, result) in end.iter_mut().enumerate() {
        let value = arithmetic.divide(&native.numerator[axis][last], &native.weight[last])?;
        *result = arithmetic.upper_f64(value.upper)?;
    }
    Ok(end)
}

fn fit_poles(
    native: &Homogeneous,
    [start, end]: [[f64; 2]; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[[f64; 2]; 4]> {
    let dw = derivative(&native.weight, arithmetic)?;
    let mut inner = [[0.; 2]; 2];
    for (axis, controls) in native.numerator.iter().enumerate() {
        let dn = derivative(controls, arithmetic)?;
        for (side, point) in [start, end].into_iter().enumerate() {
            let index = if side == 0 { 0 } else { controls.len() - 1 };
            let d_index = index.saturating_sub(1);
            let point = arithmetic.scalar(point[axis])?;
            let rate = endpoint_rate(
                &controls[index],
                &dn[d_index],
                &native.weight[index],
                &dw[d_index],
                arithmetic,
            )?;
            let third = arithmetic.fraction(&rate, 1, 3)?;
            let control = if side == 0 {
                arithmetic.add(&point, &third)?
            } else {
                arithmetic.subtract(&point, &third)?
            };
            inner[side][axis] = arithmetic.upper_f64(control.upper)?;
        }
    }
    Ok([start, inner[0], inner[1], end])
}

fn endpoint_rate(
    value: &Interval,
    derivative: &Interval,
    weight: &Interval,
    weight_derivative: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let left = arithmetic.multiply(derivative, weight)?;
    let right = arithmetic.multiply(value, weight_derivative)?;
    let numerator = arithmetic.subtract(&left, &right)?;
    let denominator = arithmetic.multiply(weight, weight)?;
    Ok(arithmetic.divide(&numerator, &denominator)?)
}
