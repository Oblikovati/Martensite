// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::RootIsolationError;
mod coefficients;
pub(super) mod moments;
pub(super) use coefficients::coefficients;

#[derive(Clone, Debug)]
pub(super) struct Density {
    constant: ContributionBounds,
    meridian: ContributionBounds,
}

pub(super) fn accumulate(
    sum: &mut super::super::super::conical_sum::ConicalSum,
    index: usize,
    span: AnalyticConicalFluxSpan<'_>,
    densities: &[Density; 2],
    operations: &mut u32,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Checked<()> {
    let shared = same_poles(span.cap_pcurves);
    let mut cached = None;
    let mut polls = 0;
    for (depth, density) in densities.iter().enumerate() {
        let poles = span.cap_pcurves[depth];
        let local;
        let moments = if let Some(first) = cached.as_ref() {
            replay_moment_polls(index, polls, cancelled)?;
            first
        } else if shared {
            cached.insert(measured_moments(index, depth, poles, &mut || {
                polls += 1;
                cancelled()
            })?)
        } else {
            local = measured_moments(index, depth, poles, cancelled)?;
            &local
        };
        let contribution = contribution(density, moments);
        sum.accumulate(
            publish::reversed_bounds(contribution, span.cap_reversed[depth]),
            0,
            operations,
            cancelled,
        )
        .map_err(|cause| AnalyticConicalFluxFailure::Arithmetic {
            span: Some(index),
            depth: Some(depth),
            cause: Box::new(cause),
        })?;
    }
    Ok(())
}

fn same_poles([first, last]: [&[[f64; 2]]; 2]) -> bool {
    first
        .iter()
        .map(|pole| pole.map(f64::to_bits))
        .eq(last.iter().map(|pole| pole.map(f64::to_bits)))
}

fn replay_moment_polls(
    index: usize,
    polls: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<()> {
    // Preserve each cap's ordered cancellation points between the independent contributions.
    for _ in 0..polls {
        if cancelled() {
            return Err(AnalyticConicalFluxFailure::Cap {
                span: index,
                depth: 1,
                cause: Box::new(PolynomialIntegralFailure::from(
                    RootIsolationError::Cancelled,
                )),
            });
        }
    }
    Ok(())
}

fn contribution(density: &Density, moments: &[ExactRatio; 2]) -> ContributionBounds {
    let constant = scaled(&density.constant, &moments[0]);
    let meridian = scaled(&density.meridian, &moments[1]);
    ContributionBounds {
        lower: constant.lower.add(&meridian.lower),
        upper: constant.upper.add(&meridian.upper),
    }
}

fn scaled(bounds: &ContributionBounds, factor: &ExactRatio) -> ContributionBounds {
    let [lower, upper] = if factor.sign() == std::cmp::Ordering::Less {
        [&bounds.upper, &bounds.lower]
    } else {
        [&bounds.lower, &bounds.upper]
    };
    ContributionBounds {
        lower: lower.multiply(factor),
        upper: upper.multiply(factor),
    }
}

fn measured_moments(
    index: usize,
    depth: usize,
    poles: &[[f64; 2]],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<[ExactRatio; 2]> {
    moments::integrate(poles, cancelled).map_err(|cause| AnalyticConicalFluxFailure::Cap {
        span: index,
        depth,
        cause: Box::new(cause),
    })
}
