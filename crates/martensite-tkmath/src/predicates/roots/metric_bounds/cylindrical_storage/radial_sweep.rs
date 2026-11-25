// SPDX-License-Identifier: GPL-2.0-only
//! Prove a strict radial derivative on the actual common-weight ruled net.
mod contract;
#[cfg(test)]
mod tests;

use super::{Arithmetic, Interval, admission, arithmetic_policy, capture, polynomial::*};
use CylindricalRadialSweepFailure as Failure;
pub use contract::*;

/// Certify the signed derivative of squared radius over the entire unit ruled patch.
///
/// The caller binds the actual stored side, ordered rails, and axis frame.
/// This proof covers radial motion, not contact or topology.
/// Example: `let proof = certify_cylindrical_radial_sweep(query, || false)?;`.
pub fn certify_cylindrical_radial_sweep(
    query: CylindricalRadialSweepQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalRadialSweep, CylindricalRadialSweepError> {
    execute(query, &mut cancelled).map_err(|cause| CylindricalRadialSweepError {
        requested_minimum: query.minimum,
        requested_options: query.options,
        sense: query.sense,
        cause,
    })
}

fn execute(
    query: CylindricalRadialSweepQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<CertifiedCylindricalRadialSweep, Failure> {
    admit(&query)?;
    let mut arithmetic = Arithmetic::new(arithmetic_policy(query.options), cancelled);
    let lower = derivative_lower(query, &mut arithmetic)?;
    if lower <= query.minimum {
        return Err(Failure::Uncertified { lower });
    }
    Ok(CertifiedCylindricalRadialSweep::new(
        query,
        lower,
        arithmetic.operations(),
    ))
}

fn admit(query: &CylindricalRadialSweepQuery<'_>) -> Result<(), Failure> {
    admission::admit_policy(query.options)?;
    for (depth, rail) in query.rails.into_iter().enumerate() {
        admission::admit_curve(rail, depth, query.options.max_degree)?;
    }
    if !query.axis_origin.iter().all(|value| value.is_finite()) {
        return Err(Failure::AxisOrigin(query.axis_origin));
    }
    if !query.minimum.is_finite() || query.minimum < 0. {
        return Err(Failure::Minimum(query.minimum));
    }
    admit_common_weights(&query.rails)
}

fn admit_common_weights(rails: &[super::MetricBezier<'_, 3>; 2]) -> Result<(), Failure> {
    let counts = rails.map(|rail| rail.poles.len());
    if counts[0] != counts[1] {
        return Err(Failure::PoleCounts(counts));
    }
    for index in 0..counts[0] {
        let actual = rails.map(|rail| rail.weights.map_or(1., |weights| weights[index]));
        if actual[0].to_bits() != actual[1].to_bits() {
            return Err(Failure::CommonWeights { index, actual });
        }
    }
    Ok(())
}

fn derivative_lower(
    query: CylindricalRadialSweepQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<f64, Failure> {
    use CylindricalRadialSense::{Inward, Outward};
    // For S=A+sD, the derivative of |S_xy|² is 2(A_xy·D_xy+s|D_xy|²).
    // Its signed minimum is at s=0 for outward motion and s=1 for inward motion.
    let [inner, outer] = match query.sense {
        Outward => query.rails,
        Inward => [query.rails[1], query.rails[0]],
    };
    let inner = capture(inner, arithmetic)?;
    let outer = capture(outer, arithmetic)?;
    let [x, y] = query.axis_origin;
    let x = component(&inner[0], &outer[0], &inner[3], x, arithmetic)?;
    let y = component(&inner[1], &outer[1], &inner[3], y, arithmetic)?;
    let numerator = sum(&x, &y, arithmetic)?;
    let denominator = product(&inner[3], &inner[3], arithmetic)?;
    ratio_lower(&numerator, &denominator, arithmetic)
}

fn component(
    inner: &[Interval],
    outer: &[Interval],
    weights: &[Interval],
    origin: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, Failure> {
    let origin = arithmetic.scalar(origin)?;
    let offset = scale(weights, &origin, arithmetic)?;
    let relative = difference(inner, &offset, arithmetic)?;
    let delta = difference(outer, inner, arithmetic)?;
    Ok(product(&relative, &delta, arithmetic)?)
}

fn ratio_lower(
    numerator: &[Interval],
    denominator: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<f64, Failure> {
    // Same-degree Bernstein polynomials with positive denominator coefficients
    // have a ratio in the convex hull of the corresponding coefficient ratios.
    let ratios = numerator
        .iter()
        .zip(denominator)
        .map(|(n, d)| {
            let doubled = arithmetic.add(n, n)?;
            arithmetic.divide(&doubled, d)
        })
        .collect::<Result<Vec<_>, _>>()?;
    // Rounding the negation upward rounds the lower endpoint downward.
    Ok(-arithmetic.upper_f64(-lower(&ratios))?)
}
