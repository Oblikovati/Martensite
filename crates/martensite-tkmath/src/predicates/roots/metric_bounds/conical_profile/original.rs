// SPDX-License-Identifier: GPL-2.0-only
//! Original dyadic span to rounded child under an explicitly matched angle lift.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::lower;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::derivative;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::restrict;
use crate::predicates::roots::metric_bounds::cylindrical_profile::algebra;

/// Original poles and their exact dyadic interval on the unit source parameter.
#[derive(Clone, Copy)]
pub struct ConicalOriginalSpan<'a> {
    /// Original authored polynomial Bézier poles.
    pub poles: &'a [[f64; 2]],
    /// Zero-based dyadic interval index.
    pub index: u64,
    /// Dyadic interval level, at most 63.
    pub level: u8,
}
/// Invalid or incomplete original-to-child comparison.
#[derive(Debug)]
pub enum ConicalOriginalFailure {
    /// The original interval is not representable.
    Interval {
        /// Supplied dyadic interval index.
        index: u64,
        /// Supplied dyadic interval level.
        level: u8,
    },
    /// Original authored poles were invalid or capture did not complete.
    Source(ConicalSourceFailure),
    /// A bounded polynomial operation failed.
    Polynomial(CylindricalProfileFailure),
    /// Squared child radius did not have a positive lower bound.
    Radius {
        /// Certified squared-radius lower endpoint.
        lower: DyadicBound,
    },
    /// Bounded arithmetic could not complete.
    Arithmetic(BoundedArithmeticFailure),
}
impl From<BoundedArithmeticFailure> for ConicalOriginalFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
impl std::fmt::Display for ConicalOriginalFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Interval { index, level } => write!(
                f,
                "interval={index}/{level}; expected level <= 63 and index < 2^level"
            ),
            Self::Source(cause) => write!(f, "original source: {cause:?}"),
            Self::Polynomial(cause) => write!(f, "original residual: {cause:?}"),
            Self::Radius { lower } => {
                write!(f, "squared child radius lower={lower:?}; expected positive")
            }
            Self::Arithmetic(cause) => write!(f, "original arithmetic: {cause:?}"),
        }
    }
}
impl std::error::Error for ConicalOriginalFailure {}
pub(super) struct OriginalEvidence {
    pub(super) bound: Interval,
    pub(super) angular_derivative: Interval,
    pub(super) anchor: AnchorEvidence,
    pub(super) comparison: Option<OriginalComparison>,
}
#[derive(Clone, Copy, Debug)]
pub(super) struct AnchorEvidence {
    pub(super) exact: bool,
    pub(super) turn_adjustment: Option<i8>,
}
pub(super) struct AxisEvidence {
    pub(super) error: Interval,
    pub(super) derivative_error: Interval,
    pub(super) exact: bool,
    pub(super) positive: bool,
    pub(super) nonnegative: bool,
    pub(super) negative: bool,
    pub(super) source_nonnegative: bool,
    pub(super) source_negative: bool,
    pub(super) child_nonnegative: bool,
    pub(super) child_negative: bool,
}
pub(super) fn bound(
    original: ConicalOriginalSpan<'_>,
    child: &[Vec<Interval>; 2],
    q: &[Interval],
    sine: &Interval,
    depths: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<OriginalEvidence, ConicalOriginalFailure> {
    admit_interval(original)?;
    let source =
        admit_source(original.poles, arithmetic).map_err(ConicalOriginalFailure::Source)?;
    let displacement = displacement(&source, child, original, arithmetic)?;
    let radius = minimum_radius(q, arithmetic)?;
    let gap = arithmetic.subtract(&radius, &displacement.delta)?;
    let scale = angular_scale(child, sine, depths, arithmetic)?;
    let angle = arithmetic.divide(&displacement.delta, &gap)?;
    let angular = arithmetic.multiply(&scale, &angle)?;
    let angular_derivative =
        derivative_bound(child, &displacement, &radius, &gap, sine, arithmetic)?;
    Ok(OriginalEvidence {
        bound: arithmetic.add(&displacement.delta, &angular)?,
        angular_derivative,
        anchor: displacement.anchor,
        comparison: Some(original_comparison(displacement, gap, angle)),
    })
}
pub(super) struct OriginalComparison {
    pub(super) displacement: DisplacementEvidence,
    pub(super) gap: Interval,
    pub(super) angle: Interval,
}
pub(super) struct DisplacementEvidence {
    pub(super) delta: Interval,
    pub(super) derivative: Interval,
    pub(super) anchor: AnchorEvidence,
}
pub(super) fn admit_interval(
    original: ConicalOriginalSpan<'_>,
) -> Result<(), ConicalOriginalFailure> {
    if original.level > 63 || original.index >= (1_u64 << original.level) {
        return Err(ConicalOriginalFailure::Interval {
            index: original.index,
            level: original.level,
        });
    }
    Ok(())
}
pub(super) fn displacement(
    source: &[Vec<Interval>; 2],
    child: &[Vec<Interval>; 2],
    original: ConicalOriginalSpan<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<DisplacementEvidence, ConicalOriginalFailure> {
    let x = axis_error(&source[0], &child[0], original, arithmetic)?;
    let y = axis_error(&source[1], &child[1], original, arithmetic)?;
    let same_branch = x.positive || y.nonnegative || y.negative;
    let anchor = AnchorEvidence {
        exact: x.exact && y.exact,
        turn_adjustment: turn_adjustment(&x, &y, same_branch),
    };
    Ok(DisplacementEvidence {
        delta: arithmetic.add(&x.error, &y.error)?,
        derivative: arithmetic.add(&x.derivative_error, &y.derivative_error)?,
        anchor,
    })
}
pub(super) fn turn_adjustment(x: &AxisEvidence, y: &AxisEvidence, same: bool) -> Option<i8> {
    if same {
        return Some(0);
    }
    if !x.negative {
        return None;
    }
    if y.source_nonnegative && y.child_negative {
        return Some(1);
    }
    if y.source_negative && y.child_nonnegative {
        return Some(-1);
    }
    None
}
pub(super) fn axis_error(
    source: &[Interval],
    child: &[Interval],
    original: ConicalOriginalSpan<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<AxisEvidence, ConicalOriginalFailure> {
    let restricted = restrict(source, original.index, original.level, arithmetic)?;
    let exact_anchor = exact_same_point(&restricted[0], &child[0]);
    let residual = algebra::difference(&restricted, child, arithmetic)
        .map_err(ConicalOriginalFailure::Polynomial)?;
    let derivative_error = magnitude_interval(&derivative(&residual, arithmetic)?);
    let zero = 0.into();
    Ok(AxisEvidence {
        error: magnitude_interval(&residual),
        derivative_error,
        exact: exact_anchor,
        positive: restricted[0].lower > zero && child[0].lower > zero,
        nonnegative: restricted[0].lower >= zero && child[0].lower >= zero,
        negative: restricted[0].upper < zero && child[0].upper < zero,
        source_nonnegative: restricted[0].lower >= zero,
        source_negative: restricted[0].upper < zero,
        child_nonnegative: child[0].lower >= zero,
        child_negative: child[0].upper < zero,
    })
}

// grad(arg z) changes by at most |p-c|/(|p||c|); all quantities use the child's local parameter.
pub(super) fn derivative_bound(
    child: &[Vec<Interval>; 2],
    displacement: &DisplacementEvidence,
    radius: &Interval,
    gap: &Interval,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalOriginalFailure> {
    let dx = magnitude_interval(&derivative(&child[0], arithmetic)?);
    let dy = magnitude_interval(&derivative(&child[1], arithmetic)?);
    let speed = arithmetic.add(&dx, &dy)?;
    let first = arithmetic.divide(&displacement.derivative, gap)?;
    let numerator = arithmetic.multiply(&speed, &displacement.delta)?;
    let denominator = arithmetic.multiply(radius, gap)?;
    let second = arithmetic.divide(&numerator, &denominator)?;
    let total = arithmetic.add(&first, &second)?;
    Ok(arithmetic.divide(&total, sine)?)
}

pub(super) fn radial_derivative_bound(
    child: &[Vec<Interval>; 2],
    displacement: &DisplacementEvidence,
    gap: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalOriginalFailure> {
    let dx = magnitude_interval(&derivative(&child[0], arithmetic)?);
    let dy = magnitude_interval(&derivative(&child[1], arithmetic)?);
    let speed = arithmetic.add(&dx, &dy)?;
    let two = arithmetic.scalar(2.)?;
    let change = arithmetic.multiply(&displacement.delta, &two)?;
    let change = arithmetic.multiply(&change, &speed)?;
    let change = arithmetic.divide(&change, gap)?;
    Ok(arithmetic.add(&displacement.derivative, &change)?)
}

pub(super) fn polar_from_comparison(
    comparison: OriginalComparison,
    child: &[Vec<Interval>; 2],
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Interval, Interval, Interval, AnchorEvidence), ConicalOriginalFailure> {
    let angle = arithmetic.divide(&comparison.angle, sine)?;
    let rate =
        radial_derivative_bound(child, &comparison.displacement, &comparison.gap, arithmetic)?;
    Ok((
        comparison.displacement.delta,
        angle,
        rate,
        comparison.displacement.anchor,
    ))
}
pub(super) fn minimum_radius(
    q: &[Interval],
    arithmetic: &Arithmetic<'_>,
) -> Result<Interval, ConicalOriginalFailure> {
    let value = lower(q);
    let fraction = i64::from(arithmetic.fractional_bits());
    if value <= 0.into() {
        return Err(ConicalOriginalFailure::Radius {
            lower: DyadicBound::new(value, -(fraction as i32)),
        });
    }
    // q >= 2^(bits-1-f); rounding the root exponent down is conservative.
    let exponent = fraction + (value.bits() as i64 - 1 - fraction).div_euclid(2);
    let minimum = num_bigint::BigInt::from(1) << exponent as usize;
    Ok(Interval {
        lower: minimum.clone(),
        upper: minimum,
    })
}
pub(super) fn angular_scale(
    child: &[Vec<Interval>; 2],
    sine: &Interval,
    depths: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let x = magnitude_interval(&child[0]);
    let y = magnitude_interval(&child[1]);
    let radius = arithmetic.add(&x, &y)?;
    let depth = arithmetic.scalar(depths[0].abs().max(depths[1].abs()))?;
    let offset = arithmetic.divide(&depth, sine)?;
    arithmetic.add(&radius, &offset)
}

fn exact_same_point(source: &Interval, child: &Interval) -> bool {
    source.lower == source.upper && child.lower == child.upper && source.lower == child.lower
}

fn original_comparison(
    displacement: DisplacementEvidence,
    gap: Interval,
    angle: Interval,
) -> OriginalComparison {
    OriginalComparison {
        displacement,
        gap,
        angle,
    }
}
