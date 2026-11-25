//! The existing wrapped proofs need a polynomial in the original source parameter.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CurveIdentityDecision, MetricBezier, MetricCurve, MetricCurveChain, MetricEmbedding,
    compare_curve_chains,
};

#[derive(Debug)]
pub(in crate::surface_sketch::wrapped_profile) struct WeightedBezierSource {
    poles: Vec<[f64; 2]>,
    weights: Vec<f64>,
}

impl WeightedBezierSource {
    pub(super) fn orient(mut self, reversed: bool) -> Self {
        if reversed {
            self.poles.reverse();
            self.weights.reverse();
        }
        self
    }

    pub(in crate::surface_sketch::wrapped_profile) fn same_bits(&self, other: &Self) -> bool {
        self.weights.iter().copied().map(f64::to_bits).eq(other
            .weights
            .iter()
            .copied()
            .map(f64::to_bits))
            && self
                .poles
                .iter()
                .map(|p| p.map(f64::to_bits))
                .eq(other.poles.iter().map(|p| p.map(f64::to_bits)))
    }
}

pub(super) fn resolve(
    curve: ProfileCurveId,
    poles: &[[f64; 2]],
    weights: Option<&[f64]>,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Controls> {
    let Some(weights) = weights.filter(|row| row.iter().any(|&w| w != 1.)) else {
        return Ok((poles.to_vec(), None));
    };
    finite_controls(curve, poles)?;
    let source = WeightedBezierSource {
        poles: poles.to_vec(),
        weights: weights.to_vec(),
    };
    let polynomial = polynomial_alias(curve, poles, weights, cancelled)?;
    Ok((polynomial, Some(source)))
}

fn polynomial_alias(
    curve: ProfileCurveId,
    poles: &[[f64; 2]],
    weights: &[f64],
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Vec<[f64; 2]>> {
    if weights.iter().all(|&w| w == weights[0]) {
        return Ok(poles.to_vec());
    }
    let candidate = candidate(poles, weights);
    let decision = compare_curve_chains(
        chain(poles, Some(weights)),
        chain(&candidate, None),
        cancelled,
    )
    .map_err(|cause| ProfileInputFailure::WeightedProof {
        curve,
        cause: Box::new(cause),
    })?;
    match decision {
        CurveIdentityDecision::Identical => Ok(candidate),
        CurveIdentityDecision::Different {
            coordinate,
            coefficient,
        } => Err(ProfileInputFailure::NonPolynomial {
            curve,
            coordinate,
            coefficient,
        }),
    }
}

fn candidate(poles: &[[f64; 2]], weights: &[f64]) -> Vec<[f64; 2]> {
    let first = poles[0];
    let last = poles[poles.len() - 1];
    // A nonconstant denominator reduces the maximum quotient degree by at least one.
    // Endpoint derivatives determine the only possible quadratic; exact identity decides.
    if poles.len() <= 3 {
        return vec![first, last];
    }
    let rate = 1.5 * (weights[1] / weights[0]);
    let middle = std::array::from_fn(|axis| first[axis] + rate * (poles[1][axis] - first[axis]));
    vec![first, middle, last]
}

fn chain<'a>(poles: &'a [[f64; 2]], weights: Option<&'a [f64]>) -> MetricCurveChain<'a, 2> {
    MetricCurveChain {
        curve: MetricCurve::Bezier(MetricBezier { poles, weights }),
        embedding: MetricEmbedding {
            linear: [[1., 0.], [0., 1.], [0., 0.]],
            origin: [0.; 3],
            scale: 1.,
        },
        placements: &[],
    }
}
