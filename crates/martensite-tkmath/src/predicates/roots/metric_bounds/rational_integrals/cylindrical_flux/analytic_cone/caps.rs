// SPDX-License-Identifier: GPL-2.0-only
use super::*;
mod coefficients;
pub(super) mod moments;
pub(super) use coefficients::coefficients;

#[derive(Clone, Debug)]
pub(super) struct Density {
    constant: ContributionBounds,
    meridian: ContributionBounds,
}

pub(super) fn accumulate(
    sum: &mut publish::BoundsSum,
    index: usize,
    span: AnalyticConicalFluxSpan<'_>,
    densities: &[Density; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<()> {
    for (depth, density) in densities.iter().enumerate() {
        let moments = moments::integrate(span.cap_pcurves[depth], cancelled).map_err(|cause| {
            AnalyticConicalFluxFailure::Cap {
                span: index,
                depth,
                cause: Box::new(cause),
            }
        })?;
        let contribution = contribution(density, &moments);
        sum.accumulate(
            publish::reversed_bounds(contribution, span.cap_reversed[depth]),
            0,
        );
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
