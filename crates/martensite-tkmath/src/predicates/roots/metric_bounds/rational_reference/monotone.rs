//! A common strictly monotone projection excludes self folds throughout the homotopy.
use super::*;

#[cfg(test)]
pub(super) fn certify_projection(
    query: Query<'_>,
    direction: [f64; 2],
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<f64> {
    let weights = admitted_weights(query)?;
    let mut arithmetic = Arithmetic::new(query.policy, &mut cancelled);
    let source = capture(query.source, query.interval, weights, &mut arithmetic)?;
    let reference = capture_planar(query.reference, &mut arithmetic)?;
    let residual = compare(&source, &reference, &mut arithmetic)?;
    margin(&source, direction, residual.derivative, &mut arithmetic)
}

pub(super) fn margin(
    source: &Homogeneous,
    direction: [f64; 2],
    derivative_error: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<f64> {
    let projected = projected_numerator(source, direction, arithmetic)?;
    let projection = source_rate(&projected, &source.weight, arithmetic)?;
    let norm = arithmetic.scalar(direction[0].abs().max(direction[1].abs()))?;
    let magnitude = arithmetic.scalar(derivative_error)?;
    let error = arithmetic.multiply(&norm, &magnitude)?;
    let gap = arithmetic.subtract(&projection, &error)?;
    let certified = -arithmetic.upper_f64(-gap.lower)?;
    if certified <= 0. {
        return Err(Refusal::Monotonicity {
            projected_rate_lower: -arithmetic.upper_f64(-projection.lower)?,
            error_upper: arithmetic.upper_f64(error.upper)?,
        });
    }
    Ok(certified)
}

fn projected_numerator(
    source: &Homogeneous,
    [x, y]: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let x = arithmetic.scalar(x)?;
    let y = arithmetic.scalar(y)?;
    let a = bernstein::scale(&source.numerator[0], &x, arithmetic)?;
    let b = bernstein::scale(&source.numerator[1], &y, arithmetic)?;
    Ok(bernstein::sum(&a, &b, arithmetic)?)
}

fn source_rate(
    projection: &[Interval],
    weight: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let dx = derivative(projection, arithmetic)?;
    let dw = derivative(weight, arithmetic)?;
    let a = bernstein::product(&dx, weight, arithmetic)?;
    let b = bernstein::product(&dw, projection, arithmetic)?;
    let homogeneous = algebra::difference(&a, &b, arithmetic)?;
    let rate = algebra::hull(&homogeneous);
    let weights = algebra::hull(weight);
    let denominator = arithmetic.multiply(&weights, &weights)?;
    Ok(arithmetic.divide(&rate, &denominator)?)
}
