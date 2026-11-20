// SPDX-License-Identifier: GPL-2.0-only
//! Conditional single-turn proof for a material enclosed by one convex wedge.
use super::*;
use crate::predicates::roots::certificate_arithmetic::ExactRatio;
use crate::predicates::roots::certificate_arithmetic::bounded_dyadic::ArithmeticPolicy;
use crate::predicates::roots::metric_bounds::cylindrical_profile::trigonometric;

/// The strict developed-plane quarter-turn wedge established by a sector receipt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicalSectorWedge {
    /// `x > |y|`.
    PositiveX,
    /// `y > |x|`.
    PositiveY,
    /// `-x > |y|`; the caller must supply a continuous negative-axis angle lift.
    NegativeX,
    /// `-y > |x|`.
    NegativeY,
}

pub(super) struct SectorEvidence {
    pub(super) source: Vec<Vec<[u64; 2]>>,
    pub(super) angle: u64,
    pub(super) policy: ArithmeticPolicy,
    pub(super) operations: u32,
    pub(super) wedge: u8,
    pub(super) margin: DyadicBound,
}
impl SectorEvidence {
    pub(super) fn wedge(&self) -> Option<ConicalSectorWedge> {
        match self.wedge {
            1 => Some(ConicalSectorWedge::PositiveX),
            2 => Some(ConicalSectorWedge::PositiveY),
            4 => Some(ConicalSectorWedge::NegativeX),
            8 => Some(ConicalSectorWedge::NegativeY),
            _ => None,
        }
    }
    pub(super) fn matches(
        &self,
        spans: &[&[[f64; 2]]],
        angle: f64,
        policy: ArithmeticPolicy,
    ) -> bool {
        self.angle == angle.to_bits()
            && self.policy == policy
            && self.source.len() == spans.len()
            && self.source.iter().zip(spans).all(|(expected, actual)| {
                expected.len() == actual.len()
                    && expected
                        .iter()
                        .copied()
                        .eq(actual.iter().map(|point| point.map(f64::to_bits)))
            })
    }
    pub(super) fn admits_bound(&self, displacement: &DyadicBound) -> bool {
        let displacement = ExactRatio::from_dyadic(displacement);
        if displacement.sign().is_lt() {
            return false;
        }
        let required = displacement.multiply(&ExactRatio::from_finite(2.));
        ExactRatio::from_dyadic(&self.margin) > required
    }
}
#[derive(Debug)]
/// A sector request could not establish the sufficient wedge condition.
pub enum SectorFailure {
    /// The requested precision or work limit is outside the supported range.
    Policy {
        /// Requested fractional precision.
        fractional_bits: u16,
        /// Requested arithmetic operation limit.
        max_operations: u32,
    },
    /// Number of supplied spans is outside the supported range.
    SpanCount(usize),
    /// A span has an unsupported number of control points.
    PoleCount {
        /// Index of the span.
        span: usize,
        /// Number of supplied points.
        count: usize,
    },
    /// A supplied point contains a nonfinite coordinate.
    Point {
        /// Index of the span.
        span: usize,
        /// Index of the point in the span.
        pole: usize,
        /// Supplied point.
        value: [f64; 2],
    },
    /// The semi-angle is nonfinite or outside the open interval `(0, pi/2)`.
    Angle(f64),
    /// A common strict wedge could not be established.
    UnresolvedWedge {
        /// Index of the span at which the test became unresolved.
        span: usize,
        /// Index of the point at which the test became unresolved.
        pole: usize,
    },
    /// The required lower bound on the sine was not certified.
    UnresolvedSine,
    /// Trigonometric enclosure was incomplete.
    Trigonometric(CylindricalProfileFailure),
    /// Bounded arithmetic was incomplete.
    Arithmetic(BoundedArithmeticFailure),
}
impl std::fmt::Display for SectorFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Policy {
                fractional_bits,
                max_operations,
            } => policy_message(out, *fractional_bits, *max_operations),
            Self::SpanCount(count) => write!(out, "spans={count}; expected 1..=128"),
            Self::PoleCount { span, count } => {
                write!(out, "span={span}, poles={count}; expected 2..=4")
            }
            Self::Point { span, pole, value } => point_message(out, *span, *pole, *value),
            Self::Angle(angle) => write!(out, "semi-angle={angle}; expected finite in (0,pi/2)"),
            Self::UnresolvedWedge { span, pole } => wedge_message(out, *span, *pole),
            Self::UnresolvedSine => write!(out, "sin(semi-angle) > 1/4 not certified"),
            Self::Trigonometric(cause) => write!(out, "sector trigonometry: {cause:?}"),
            Self::Arithmetic(cause) => write!(out, "sector arithmetic: {cause:?}"),
        }
    }
}
impl std::error::Error for SectorFailure {}

// Conditional on a bounded material region with this complete boundary, a
// consistent lift on the chosen wedge, and positive radius over its depth range.
pub(super) fn certify(
    spans: &[&[[f64; 2]]],
    angle: f64,
    policy: ArithmeticPolicy,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<SectorEvidence, SectorFailure> {
    admit(spans, angle, policy)?;
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    prove_sine(angle, &mut arithmetic)?;
    let (wedge, margin) = common_wedge(spans, &mut arithmetic)?;
    arithmetic
        .check_cancelled()
        .map_err(SectorFailure::Arithmetic)?;
    Ok(SectorEvidence {
        source: source_bits(spans),
        angle: angle.to_bits(),
        policy,
        operations: arithmetic.operations(),
        wedge,
        margin: DyadicBound::new(
            margin.unwrap_or_default(),
            -i32::from(policy.fractional_bits),
        ),
    })
}
pub(super) fn admit(
    spans: &[&[[f64; 2]]],
    angle: f64,
    policy: ArithmeticPolicy,
) -> Result<(), SectorFailure> {
    admit_policy(policy)?;
    if spans.is_empty() || spans.len() > 128 {
        return Err(SectorFailure::SpanCount(spans.len()));
    }
    admit_geometry(spans, angle)
}
pub(super) fn admit_policy(policy: ArithmeticPolicy) -> Result<(), SectorFailure> {
    if !(8..=4096).contains(&policy.fractional_bits) || policy.max_operations > 1_000_000 {
        return Err(SectorFailure::Policy {
            fractional_bits: policy.fractional_bits,
            max_operations: policy.max_operations,
        });
    }
    Ok(())
}
pub(super) fn admit_geometry(spans: &[&[[f64; 2]]], angle: f64) -> Result<(), SectorFailure> {
    if !angle.is_finite() || angle <= 0. || angle >= std::f64::consts::FRAC_PI_2 {
        return Err(SectorFailure::Angle(angle));
    }
    admit_span_points(spans)
}
pub(super) fn prove_sine(angle: f64, arithmetic: &mut Arithmetic<'_>) -> Result<(), SectorFailure> {
    let phase = arithmetic
        .scalar(angle)
        .map_err(SectorFailure::Arithmetic)?;
    let sine =
        trigonometric::enclose(&[phase], arithmetic).map_err(SectorFailure::Trigonometric)?;
    let quarter = arithmetic.scalar(0.25).map_err(SectorFailure::Arithmetic)?;
    if sine[1][0].lower <= quarter.upper {
        return Err(SectorFailure::UnresolvedSine);
    }
    Ok(())
}
pub(super) fn point_wedge(
    point: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(u8, num_bigint::BigInt), SectorFailure> {
    let x = arithmetic
        .scalar(point[0])
        .map_err(SectorFailure::Arithmetic)?;
    let y = arithmetic
        .scalar(point[1])
        .map_err(SectorFailure::Arithmetic)?;
    let lower = arithmetic
        .subtract(&x, &y)
        .map_err(SectorFailure::Arithmetic)?;
    let upper = arithmetic.add(&x, &y).map_err(SectorFailure::Arithmetic)?;
    Ok(wedge_from_axes(lower, upper))
}

pub(super) fn source_bits(spans: &[&[[f64; 2]]]) -> Vec<Vec<[u64; 2]>> {
    spans
        .iter()
        .map(|poles| poles.iter().map(|point| point.map(f64::to_bits)).collect())
        .collect()
}

fn admit_span_points(spans: &[&[[f64; 2]]]) -> Result<(), SectorFailure> {
    for (span, points) in spans.iter().enumerate() {
        if !(2..=4).contains(&points.len()) {
            return Err(SectorFailure::PoleCount {
                span,
                count: points.len(),
            });
        }
        admit_finite_points(span, points)?;
    }
    Ok(())
}

fn admit_finite_points(span: usize, points: &[[f64; 2]]) -> Result<(), SectorFailure> {
    for (pole, value) in points.iter().copied().enumerate() {
        if value.iter().any(|v| !v.is_finite()) {
            return Err(SectorFailure::Point { span, pole, value });
        }
    }
    Ok(())
}

fn interval_sign(value: &Interval) -> i8 {
    if value.lower > 0.into() {
        return 1;
    }
    if value.upper < 0.into() {
        return -1;
    }
    0
}

fn wedge_from_axes(lower: Interval, upper: Interval) -> (u8, num_bigint::BigInt) {
    let first = interval_sign(&lower);
    let second = interval_sign(&upper);
    let first_margin = if first > 0 { lower.lower } else { -lower.upper };
    let second_margin = if second > 0 {
        upper.lower
    } else {
        -upper.upper
    };
    let wedge = match (first, second) {
        (1, 1) => 0b0001,
        (-1, 1) => 0b0010,
        (-1, -1) => 0b0100,
        (1, -1) => 0b1000,
        _ => 0,
    };
    (wedge, first_margin.min(second_margin))
}

pub(super) fn common_wedge(
    spans: &[&[[f64; 2]]],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(u8, Option<num_bigint::BigInt>), SectorFailure> {
    let mut wedge = 0b1111;
    let mut margin: Option<num_bigint::BigInt> = None;
    for (span, poles) in spans.iter().enumerate() {
        for (pole, point) in poles.iter().enumerate() {
            let (point_wedge, point_margin) = point_wedge(*point, arithmetic)?;
            wedge &= point_wedge;
            admit_common_wedge(wedge, span, pole)?;
            margin = Some(margin.map_or(point_margin.clone(), |old| old.min(point_margin)));
        }
    }
    Ok((wedge, margin))
}

fn admit_common_wedge(wedge: u8, span: usize, pole: usize) -> Result<(), SectorFailure> {
    if wedge == 0 {
        return Err(SectorFailure::UnresolvedWedge { span, pole });
    }
    Ok(())
}

fn policy_message(
    out: &mut std::fmt::Formatter<'_>,
    fractional_bits: u16,
    max_operations: u32,
) -> std::fmt::Result {
    write!(
        out,
        "sector precision={fractional_bits}, work={max_operations}; expected supported precision and work"
    )
}

fn point_message(
    out: &mut std::fmt::Formatter<'_>,
    span: usize,
    pole: usize,
    value: [f64; 2],
) -> std::fmt::Result {
    write!(
        out,
        "span={span}, pole={pole}, point={value:?}; expected finite coordinates"
    )
}

fn wedge_message(out: &mut std::fmt::Formatter<'_>, span: usize, pole: usize) -> std::fmt::Result {
    write!(
        out,
        "span={span}, pole={pole}; strict wedge membership not certified"
    )
}
