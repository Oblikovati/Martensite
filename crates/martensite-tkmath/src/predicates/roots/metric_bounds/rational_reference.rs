// SPDX-License-Identifier: GPL-2.0-only
//! Bound a reference's value and local derivative against one positive rational source interval.
use super::{
    CylindricalProfileFailure, CylindricalProfileInterval, CylindricalProfileOptions, MetricBezier,
    cylindrical_profile::algebra,
};
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_bounds as bernstein,
    bernstein_curve::{capture_planar, derivative, restrict},
    bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
    bounded_failure::BoundedArithmeticFailure,
};
mod bound_refinement;
mod monotone;
mod reference_fit;

#[cfg(test)]
#[derive(Clone, Copy)]
struct Query<'a> {
    source: MetricBezier<'a, 2>,
    reference: &'a [[f64; 2]],
    interval: CylindricalProfileInterval,
    limits: [f64; 2],
    policy: ArithmeticPolicy,
}

/// Certified L1 error in the developed frame and the span's local parameter.
#[derive(Clone, Copy, Debug)]
pub struct RationalReferenceBounds {
    /// Maximum developed position discrepancy.
    pub position: f64,
    /// Maximum derivative discrepancy in the local span parameter.
    pub derivative: f64,
    /// Cumulative arithmetic work through the span.
    pub operations: u32,
}
type Bounds = RationalReferenceBounds;

/// A fit failed a prescribed bounded arithmetic or geometric obligation.
#[derive(Debug)]
#[non_exhaustive]
pub enum RationalReferenceFailure {
    /// Unsupported source or reference representation.
    Shape {
        /// Source pole count.
        source: usize,
        /// Supplied weight count.
        weights: Option<usize>,
        /// Reference pole count.
        reference: usize,
    },
    /// A source weight was not positive and finite.
    Weight {
        /// Source control index.
        index: usize,
        /// Actual weight.
        value: f64,
    },
    /// Requested position or derivative limit was negative or nonfinite.
    Limits {
        /// Requested position limit.
        position: f64,
        /// Requested derivative limit.
        derivative: f64,
    },
    /// The reference exceeded its requested proof budget.
    Exceeds {
        /// Achieved conservative bounds.
        achieved: Bounds,
        /// Requested position and derivative limits.
        limits: [f64; 2],
    },
    /// The projection could not stay strictly increasing through the homotopy.
    Monotonicity {
        /// Certified lower bound on the original projected rate.
        projected_rate_lower: f64,
        /// Certified upper bound on the projected derivative error.
        error_upper: f64,
    },
    /// Arithmetic policy outside the supported range.
    Policy {
        /// Requested fractional precision.
        fractional_bits: u16,
        /// Requested operation count.
        operations: u32,
    },
    /// Uniform subdivision level above eight.
    Level(u8),
    /// Original bounded arithmetic cause.
    Arithmetic(BoundedArithmeticFailure),
    /// Original polynomial enclosure cause.
    Polynomial(CylindricalProfileFailure),
}
type Refusal = RationalReferenceFailure;

impl From<CylindricalProfileFailure> for Refusal {
    fn from(cause: CylindricalProfileFailure) -> Self {
        match cause {
            CylindricalProfileFailure::Arithmetic(cause) => Self::Arithmetic(*cause),
            other => Self::Polynomial(other),
        }
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Polynomial(cause) => cause.fmt(f),
            other => write!(f, "{other:?}"),
        }
    }
}
impl std::error::Error for Refusal {}

impl From<BoundedArithmeticFailure> for Refusal {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}

type Checked<T> = Result<T, Refusal>;

/// A uniform positive-rational reference fit with a certified monotone projection.
#[derive(Clone, Copy)]
pub struct RationalReferenceRequest<'a> {
    /// Original rational Bézier in its own normalized parameter.
    pub source: MetricBezier<'a, 2>,
    /// Uniform dyadic subdivision level from zero through eight.
    pub level: u8,
    /// L1 position and local derivative error limits.
    pub limits: [f64; 2],
    /// Direction of strictly increasing source-to-reference projection.
    pub direction: [f64; 2],
    /// Fixed-grid arithmetic policy shared across the entire fit.
    pub options: CylindricalProfileOptions,
}

/// The bound policy for a fixed set of fitted cubic spans.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RationalReferenceProof {
    /// Whole-span Bernstein bound used by the original fit.
    WholeSpanV1,
    /// Bisect the residual proof, without adding a geometry span, if the whole-span bound exceeds a limit.
    BisectResidualV2,
}

pub use reference_fit::RationalReferenceSpan;

/// Fit and certify a same-parameter cubic reference over the full source.
/// Example: `let pieces = fit_rational_bezier_reference(request, || false)?;`.
pub fn fit_rational_bezier_reference(
    request: RationalReferenceRequest<'_>,
    cancelled: impl FnMut() -> bool + Send,
) -> Checked<Vec<RationalReferenceSpan>> {
    fit_rational_bezier_reference_with_proof(
        request,
        RationalReferenceProof::WholeSpanV1,
        cancelled,
    )
}

/// Fit the same cubic controls under the selected versioned residual proof.
/// The bisected proof charges every operation to the request's original arithmetic allowance.
/// Example: `fit_rational_bezier_reference_with_proof(request, RationalReferenceProof::BisectResidualV2, || false)?`.
pub fn fit_rational_bezier_reference_with_proof(
    request: RationalReferenceRequest<'_>,
    proof: RationalReferenceProof,
    cancelled: impl FnMut() -> bool + Send,
) -> Checked<Vec<RationalReferenceSpan>> {
    reference_fit::fit(request, proof, cancelled)
}

#[cfg(test)]
fn certify(query: Query<'_>, mut cancelled: impl FnMut() -> bool + Send) -> Checked<Bounds> {
    let weights = admitted_weights(query)?;
    let mut arithmetic = Arithmetic::new(query.policy, &mut cancelled);
    let homogeneous = capture(query.source, query.interval, weights, &mut arithmetic)?;
    let reference = capture_planar(query.reference, &mut arithmetic)?;
    let bounds = compare(&homogeneous, &reference, &mut arithmetic)?;
    accept(bounds, query.limits)
}

#[cfg(test)]
fn admitted_weights(query: Query<'_>) -> Checked<&[f64]> {
    admit_policy(query.limits, query.policy)?;
    checked_shape(query.source, query.reference.len())
}

fn admit_policy(limits: [f64; 2], policy: ArithmeticPolicy) -> Checked<()> {
    let [position, derivative] = limits;
    if !limits
        .into_iter()
        .all(|value| value.is_finite() && value >= 0.)
    {
        return Err(Refusal::Limits {
            position,
            derivative,
        });
    }
    let ArithmeticPolicy {
        fractional_bits,
        max_operations,
    } = policy;
    if !(8..=4096).contains(&fractional_bits) || max_operations > 1_000_000 {
        return Err(Refusal::Policy {
            fractional_bits,
            operations: max_operations,
        });
    }
    Ok(())
}

fn checked_shape(source: MetricBezier<'_, 2>, reference: usize) -> Checked<&[f64]> {
    let count = source.poles.len();
    let weights = source.weights.filter(|weights| weights.len() == count);
    let Some(weights) =
        weights.filter(|_| (2..=17).contains(&count) && (2..=4).contains(&reference))
    else {
        return Err(Refusal::Shape {
            source: count,
            weights: source.weights.map(<[f64]>::len),
            reference,
        });
    };
    if let Some((index, &value)) = weights
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite() || **value <= 0.)
    {
        return Err(Refusal::Weight { index, value });
    }
    Ok(weights)
}

fn accept(bounds: Bounds, limits: [f64; 2]) -> Checked<Bounds> {
    if bounds.position > limits[0] || bounds.derivative > limits[1] {
        return Err(Refusal::Exceeds {
            achieved: bounds,
            limits,
        });
    }
    Ok(bounds)
}

struct Homogeneous {
    numerator: [Vec<Interval>; 2],
    weight: Vec<Interval>,
}

fn capture(
    source: MetricBezier<'_, 2>,
    span: CylindricalProfileInterval,
    weights: &[f64],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Homogeneous> {
    let mut control = [Vec::new(), Vec::new(), Vec::new()];
    for (&[x, y], &weight) in source.poles.iter().zip(weights) {
        let weight = arithmetic.scalar(weight)?;
        let x = arithmetic.scalar(x)?;
        let y = arithmetic.scalar(y)?;
        control[0].push(arithmetic.multiply(&x, &weight)?);
        control[1].push(arithmetic.multiply(&y, &weight)?);
        control[2].push(weight);
    }
    let [x, y, w] = control;
    let mut interval =
        |controls: &[Interval]| restrict(controls, span.index(), span.level(), arithmetic);
    Ok(Homogeneous {
        numerator: [interval(&x)?, interval(&y)?],
        weight: interval(&w)?,
    })
}

fn compare(
    source: &Homogeneous,
    reference: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Bounds> {
    let minimum = bernstein::lower(&source.weight);
    let weight = Interval {
        lower: minimum.clone(),
        upper: minimum,
    };
    let squared = arithmetic.multiply(&weight, &weight)?;
    let mut combined = [arithmetic.zero(), arithmetic.zero()];
    for (axis, control) in reference.iter().enumerate() {
        let [position, rate] =
            axis_bounds(control, &source.numerator[axis], &source.weight, arithmetic)?;
        for (sum, value) in combined.iter_mut().zip([position, rate]) {
            *sum = arithmetic.add(sum, &value)?;
        }
    }
    let position = arithmetic.divide(&combined[0], &weight)?;
    let rate = arithmetic.divide(&combined[1], &squared)?;
    Ok(Bounds {
        position: arithmetic.upper_f64(position.upper)?,
        derivative: arithmetic.upper_f64(rate.upper)?,
        operations: arithmetic.operations(),
    })
}

fn axis_bounds(
    reference: &[Interval],
    numerator: &[Interval],
    weight: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Interval; 2]> {
    // Subtract pointwise in the source parameter before evaluating either norm bound.
    let [position, rate] = residual_terms(reference, numerator, weight, arithmetic)?;
    Ok([algebra::magnitude(&position), algebra::magnitude(&rate)])
}

fn residual_terms(
    reference: &[Interval],
    numerator: &[Interval],
    weight: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    let reference = bernstein::product(reference, weight, arithmetic)?;
    let residual = algebra::difference(&reference, numerator, arithmetic)?;
    let dp = derivative(&residual, arithmetic)?;
    let dw = derivative(weight, arithmetic)?;
    let left = bernstein::product(&dp, weight, arithmetic)?;
    let right = bernstein::product(&residual, &dw, arithmetic)?;
    let rate = algebra::difference(&left, &right, arithmetic)?;
    Ok([residual, rate])
}

#[cfg(test)]
mod tests;
