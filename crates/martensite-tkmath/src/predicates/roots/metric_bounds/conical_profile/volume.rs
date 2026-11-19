// SPDX-License-Identifier: GPL-2.0-only
//! Conditional authored normal-map volume; material and injectivity are caller obligations.
use super::inverse::*;
use super::volume_boundary::*;
use super::*;
use crate::predicates::roots::metric_bounds::cylindrical_profile::trigonometric;

pub use super::volume_api::ConicalAuthoredContour;
pub use super::volume_api::ConicalAuthoredFlux;
pub use super::volume_api::ConicalAuthoredFluxError;
pub use super::volume_api::ConicalAuthoredFluxQuery;
pub use super::volume_api::ConicalAuthoredFluxStage;
pub use super::volume_api::certify_conical_authored_flux;

pub(super) type Shape = SourceBits;
pub(super) struct VolumeInput<'a> {
    pub(super) contours: &'a [Contour<'a>],
    pub(super) semi_angle: f64,
    pub(super) depths: [f64; 2],
    pub(super) outward: bool,
    pub(super) level: u8,
    pub(super) clearance_level: u8,
    pub(super) limit: f64,
}
pub(super) struct DepthEvidence {
    pub(super) minimum_squared: crate::predicates::roots::DyadicBound,
    pub(super) threshold_upper: crate::predicates::roots::DyadicBound,
}
pub(super) struct ConditionalVolumeEvidence {
    pub(super) enclosure: Interval,
    pub(super) area: Interval,
    pub(super) inverse_slant: Interval,
    pub(super) source: Shape,
    pub(super) reversals: Vec<bool>,
    pub(super) angle_bits: u64,
    pub(super) depth_bits: [u64; 2],
    pub(super) outward: bool,
    pub(super) level: u8,
    pub(super) clearance_level: u8,
    pub(super) clearances: Vec<DepthEvidence>,
    pub(super) limit_bits: u64,
    pub(super) policy: ArithmeticPolicy,
    pub(super) operations: u32,
}
#[derive(Debug)]
pub(super) enum FluxComparisonFailure {
    InvalidEnclosure([f64; 2]),
    Precision { authored: u16, comparison: u16 },
    Arithmetic(BoundedArithmeticFailure),
}

// The result bounds the difference of any two values in the supplied intervals;
// interpreting the represented flux as body volume requires separate validity proof.
pub(super) fn compare_represented_flux(
    authored: &ConditionalVolumeEvidence,
    flux: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<crate::predicates::roots::DyadicBound, FluxComparisonFailure> {
    admit_comparison(authored, flux, arithmetic)?;
    let lower = arithmetic
        .scalar(flux[0])
        .map_err(FluxComparisonFailure::Arithmetic)?;
    let upper = arithmetic
        .scalar(flux[1])
        .map_err(FluxComparisonFailure::Arithmetic)?;
    let authored_upper = singleton_bound(&authored.enclosure.upper);
    let authored_lower = singleton_bound(&authored.enclosure.lower);
    let left = arithmetic
        .subtract(&authored_upper, &lower)
        .map_err(FluxComparisonFailure::Arithmetic)?;
    let right = arithmetic
        .subtract(&upper, &authored_lower)
        .map_err(FluxComparisonFailure::Arithmetic)?;
    Ok(crate::predicates::roots::DyadicBound::new(
        left.upper.max(right.upper),
        -i32::from(arithmetic.fractional_bits()),
    ))
}
impl ConditionalVolumeEvidence {
    pub(super) fn matches(&self, input: &VolumeInput<'_>, policy: ArithmeticPolicy) -> bool {
        self.angle_bits == input.semi_angle.to_bits()
            && self.depth_bits == input.depths.map(f64::to_bits)
            && self.outward == input.outward
            && self.level == input.level
            && self.clearance_level == input.clearance_level
            && self.limit_bits == input.limit.to_bits()
            && self.policy == policy
            && self.source == source_bits(input.contours)
            && self
                .reversals
                .iter()
                .copied()
                .eq(input.contours.iter().map(|item| item.reverse))
    }
}
pub(super) fn conditional_volume(
    input: &VolumeInput<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<ConditionalVolumeEvidence, BoundaryFailure> {
    conditional_volume_using(input, arithmetic, sum_contours)
}
pub(super) fn conditional_volume_using(
    input: &VolumeInput<'_>,
    arithmetic: &mut Arithmetic<'_>,
    integrate: impl FnOnce(&[Contour<'_>], u8, &mut Arithmetic<'_>) -> Result<Interval, BoundaryFailure>,
) -> Result<ConditionalVolumeEvidence, BoundaryFailure> {
    admit_volume(input, arithmetic)?;
    let start = arithmetic.operations();
    let clearances = prove_boundary_clearance(input, arithmetic)?;
    let area = area_sum(input.contours, arithmetic)?;
    if area.lower <= 0.into() {
        return Err(BoundaryFailure::UnresolvedArea);
    }
    let inverse = integrate(input.contours, input.level, arithmetic)?;
    if inverse.lower <= 0.into() {
        return Err(BoundaryFailure::UnresolvedInverse);
    }
    let enclosure = compose_volume(input, &area, &inverse, arithmetic)?;
    check_width(&enclosure, input.limit, arithmetic).map_err(BoundaryFailure::Options)?;
    arithmetic
        .check_cancelled()
        .map_err(BoundaryFailure::Arithmetic)?;
    Ok(volume_evidence(
        input, arithmetic, start, clearances, area, inverse, enclosure,
    ))
}
pub(super) fn admit_volume(
    input: &VolumeInput<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), BoundaryFailure> {
    admit_options(input.limit, arithmetic).map_err(BoundaryFailure::Options)?;
    if input.level > 12 {
        return Err(BoundaryFailure::Options(SourceBoundFailure::InvalidLevel(
            input.level,
        )));
    }
    if input.clearance_level > 64 {
        return Err(BoundaryFailure::InvalidClearanceLevel(
            input.clearance_level,
        ));
    }
    admit_support(input)?;
    admit_contours(input.contours, arithmetic)
}
pub(super) fn prove_boundary_clearance(
    input: &VolumeInput<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<DepthEvidence>, BoundaryFailure> {
    let mut receipts = Vec::new();
    for (contour, item) in input.contours.iter().enumerate() {
        for (span, poles) in item.spans.iter().enumerate() {
            let receipt = span_clearance(input, poles, contour, span, arithmetic)?;
            receipts.push(receipt);
        }
    }
    Ok(receipts)
}
pub(super) fn area_sum(
    contours: &[Contour<'_>],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, BoundaryFailure> {
    let mut total = arithmetic.zero();
    for (index, contour) in contours.iter().enumerate() {
        let area = area_contour(index, contour, arithmetic)?;
        total = arithmetic
            .add(&total, &area)
            .map_err(BoundaryFailure::Arithmetic)?;
    }
    Ok(total)
}
pub(super) fn area_contour(
    index: usize,
    contour: &Contour<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, BoundaryFailure> {
    let mut total = arithmetic.zero();
    for (span, poles) in contour.spans.iter().enumerate() {
        let value = area_span(poles, arithmetic).map_err(|cause| BoundaryFailure::Span {
            contour: index,
            span,
            cause,
        })?;
        total = arithmetic
            .add(&total, &value)
            .map_err(BoundaryFailure::Arithmetic)?;
    }
    if contour.reverse {
        total = arithmetic
            .subtract(&arithmetic.zero(), &total)
            .map_err(BoundaryFailure::Arithmetic)?;
    }
    Ok(total)
}
pub(super) fn area_span(
    poles: &[[f64; 2]],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, SourceBoundFailure> {
    let source = super::admit_source(poles, arithmetic).map_err(SourceBoundFailure::Source)?;
    let coefficients =
        cross_polynomial(&source, poles.len(), arithmetic).map_err(SourceBoundFailure::from)?;
    let mut sum = arithmetic.zero();
    for coefficient in &coefficients {
        sum = arithmetic
            .add(&sum, coefficient)
            .map_err(SourceBoundFailure::Arithmetic)?;
    }
    arithmetic
        .fraction(&sum, 1, 2 * coefficients.len() as u64)
        .map_err(SourceBoundFailure::Arithmetic)
}
pub(super) fn compose_volume(
    input: &VolumeInput<'_>,
    area: &Interval,
    inverse: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, BoundaryFailure> {
    let (delta, half_square_difference) = depth_factors(input.depths, arithmetic)?;
    let cotangent = cotangent(input.semi_angle, arithmetic)?;
    let baseline = arithmetic
        .multiply(&delta, area)
        .map_err(BoundaryFailure::Arithmetic)?;
    let coefficient = arithmetic
        .multiply(&half_square_difference, &cotangent)
        .map_err(BoundaryFailure::Arithmetic)?;
    let correction = arithmetic
        .multiply(&coefficient, inverse)
        .map_err(BoundaryFailure::Arithmetic)?;
    if input.outward {
        arithmetic.add(&baseline, &correction)
    } else {
        arithmetic.subtract(&baseline, &correction)
    }
    .map_err(BoundaryFailure::Arithmetic)
}
pub(super) fn depth_factors(
    depths: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Interval, Interval), BoundaryFailure> {
    let low = arithmetic
        .scalar(depths[0])
        .map_err(BoundaryFailure::Arithmetic)?;
    let high = arithmetic
        .scalar(depths[1])
        .map_err(BoundaryFailure::Arithmetic)?;
    let delta = arithmetic
        .subtract(&high, &low)
        .map_err(BoundaryFailure::Arithmetic)?;
    let sum = arithmetic
        .add(&high, &low)
        .map_err(BoundaryFailure::Arithmetic)?;
    let product = arithmetic
        .multiply(&delta, &sum)
        .map_err(BoundaryFailure::Arithmetic)?;
    let half = arithmetic
        .fraction(&product, 1, 2)
        .map_err(BoundaryFailure::Arithmetic)?;
    Ok((delta, half))
}
pub(super) fn cotangent(
    angle: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, BoundaryFailure> {
    let angle = arithmetic
        .scalar(angle)
        .map_err(BoundaryFailure::Arithmetic)?;
    let [cosine, sine] =
        trigonometric::enclose(&[angle], arithmetic).map_err(BoundaryFailure::Trigonometric)?;
    arithmetic
        .divide(&cosine[0], &sine[0])
        .map_err(BoundaryFailure::Arithmetic)
}

use super::quadrature_certificate::{admit_options, check_width};

fn admit_comparison(
    authored: &ConditionalVolumeEvidence,
    flux: [f64; 2],
    arithmetic: &Arithmetic<'_>,
) -> Result<(), FluxComparisonFailure> {
    if !flux.iter().all(|value| value.is_finite()) || flux[0] > flux[1] {
        return Err(FluxComparisonFailure::InvalidEnclosure(flux));
    }
    if arithmetic.fractional_bits() != authored.policy.fractional_bits {
        return Err(FluxComparisonFailure::Precision {
            authored: authored.policy.fractional_bits,
            comparison: arithmetic.fractional_bits(),
        });
    }
    Ok(())
}

fn volume_evidence(
    input: &VolumeInput<'_>,
    arithmetic: &Arithmetic<'_>,
    start: u32,
    clearances: Vec<DepthEvidence>,
    area: Interval,
    inverse: Interval,
    enclosure: Interval,
) -> ConditionalVolumeEvidence {
    ConditionalVolumeEvidence {
        enclosure,
        area,
        inverse_slant: inverse,
        source: source_bits(input.contours),
        reversals: input.contours.iter().map(|item| item.reverse).collect(),
        angle_bits: input.semi_angle.to_bits(),
        depth_bits: input.depths.map(f64::to_bits),
        outward: input.outward,
        level: input.level,
        clearance_level: input.clearance_level,
        clearances,
        limit_bits: input.limit.to_bits(),
        policy: arithmetic.policy(),
        operations: arithmetic.operations() - start,
    }
}

fn admit_support(input: &VolumeInput<'_>) -> Result<(), BoundaryFailure> {
    if !input.semi_angle.is_finite()
        || input.semi_angle <= 0.
        || input.semi_angle >= std::f64::consts::FRAC_PI_2
    {
        return Err(BoundaryFailure::InvalidSupport(input.semi_angle));
    }
    if !input.depths.iter().all(|d| d.is_finite()) || input.depths[0] >= input.depths[1] {
        return Err(BoundaryFailure::InvalidDepths(input.depths));
    }
    Ok(())
}

fn span_clearance(
    input: &VolumeInput<'_>,
    poles: &[[f64; 2]],
    contour: usize,
    span: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<DepthEvidence, BoundaryFailure> {
    let receipt = super::prove_owned_cone_interval(
        poles,
        input.semi_angle,
        input.depths,
        input.outward,
        0.,
        input.clearance_level,
        arithmetic,
    )
    .map_err(|cause| BoundaryFailure::DepthClearance {
        contour,
        span,
        cause: Box::new(cause),
    })?;
    Ok(DepthEvidence {
        minimum_squared: receipt.proof().minimum_squared().clone(),
        threshold_upper: receipt.threshold()[1].clone(),
    })
}

fn singleton_bound(value: &num_bigint::BigInt) -> Interval {
    Interval {
        lower: value.clone(),
        upper: value.clone(),
    }
}

impl std::fmt::Display for FluxComparisonFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEnclosure(value) => {
                write!(out, "flux={value:?}; expected finite ordered endpoints")
            }
            Self::Precision {
                authored,
                comparison,
            } => write!(
                out,
                "authored precision={authored}, comparison precision={comparison}; expected equal"
            ),
            Self::Arithmetic(cause) => write!(out, "comparison arithmetic: {cause:?}"),
        }
    }
}
