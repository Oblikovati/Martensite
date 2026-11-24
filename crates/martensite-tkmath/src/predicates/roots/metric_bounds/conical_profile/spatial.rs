// SPDX-License-Identifier: GPL-2.0-only
//! Shared-source candidate for a bounded spatial map discrepancy.
use super::angle::*;
use super::angular;
use super::slant;
use super::*;
use num_bigint::BigInt;

pub(super) struct SpatialRequest<'a> {
    pub(super) source: &'a [[f64; 2]],
    pub(super) original: Option<ConicalOriginalSpan<'a>>,
    pub(super) angular_fit: &'a [f64],
    pub(super) slant_numerator: &'a [f64],
    pub(super) weights: &'a [f64],
    pub(super) anchor: AnchorRequest,
    pub(super) depths: [f64; 2],
    pub(super) outward: bool,
    pub(super) limit: f64,
}
pub(super) struct SpatialEvidence {
    pub(super) bound: Interval,
    pub(super) original: Interval,
    pub(super) anchor: super::original::AnchorEvidence,
    pub(super) angular_derivative: Interval,
    pub(super) original_angular_derivative: Interval,
}
pub(super) struct SpatialCapture {
    pub(super) source: [Vec<Interval>; 2],
    pub(super) q: Vec<Interval>,
    pub(super) numerator: Vec<Interval>,
    pub(super) weights: Vec<Interval>,
    pub(super) angular: Interval,
    pub(super) slant: Interval,
    pub(super) sine: Interval,
    pub(super) comparison: Option<super::original::OriginalComparison>,
}
#[derive(Debug)]
pub(super) enum SpatialFailure {
    Domain { depths: [f64; 2], limit: f64 },
    Source(ConicalSourceFailure),
    Original(ConicalOriginalFailure),
    Angular(angular::AngularSpanFailure),
    Slant(slant::SlantFailure),
    Arithmetic(BoundedArithmeticFailure),
    Uncertified { upper: DyadicBound, limit: f64 },
}
impl From<BoundedArithmeticFailure> for SpatialFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}

pub(super) fn certify_spatial_span(
    request: &SpatialRequest<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<SpatialEvidence, SpatialFailure> {
    let (evidence, _) = certify_spatial_captured(request, arithmetic)?;
    Ok(evidence)
}
pub(super) fn certify_spatial_captured(
    request: &SpatialRequest<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(SpatialEvidence, SpatialCapture), SpatialFailure> {
    admit_spatial(request)?;
    admit_components(request)?;
    let source = admit_source(request.source, arithmetic).map_err(SpatialFailure::Source)?;
    let q = squared_distance(&source, arithmetic)?;
    let (angular, mut original, sine) = angular_component(request, &source, &q, arithmetic)?;
    let (slant, numerator, weights) = slant_component(request, &q, arithmetic)?;
    let radius = max_offset_radius(&numerator, &weights, request.depths, arithmetic)?;
    let angular_distance = arithmetic.multiply(&radius, &angular.bound)?;
    let bound = arithmetic.add(&slant, &angular_distance)?;
    let comparison = original.comparison.take();
    let evidence = accept_bound(bound, original, angular.derivative, request, arithmetic)?;
    let radial = (numerator, weights);
    let errors = (angular.bound, slant);
    let capture = capture_span(source, q, radial, errors, sine, comparison);
    Ok((evidence, capture))
}

pub(super) fn admit_components(request: &SpatialRequest<'_>) -> Result<(), SpatialFailure> {
    let angular_request = angular::AngularSpanRequest {
        source: request.source,
        fit: request.angular_fit,
        anchor: request.anchor,
    };
    let slant_request = slant::SlantRequest {
        numerator: request.slant_numerator,
        weights: request.weights,
    };
    angular::admit_angular_request(&angular_request).map_err(SpatialFailure::Angular)?;
    slant::admit_slant(&slant_request).map_err(SpatialFailure::Slant)
}
pub(super) fn angular_component(
    request: &SpatialRequest<'_>,
    source: &[Vec<Interval>; 2],
    q: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<
    (
        angular::AngularEvidence,
        super::original::OriginalEvidence,
        Interval,
    ),
    SpatialFailure,
> {
    let (anchor, sine) = anchor_with_sine(&request.anchor, arithmetic)
        .map_err(|cause| SpatialFailure::Angular(angular::AngularSpanFailure::Initial(cause)))?;
    let fit =
        angular::capture_fit(request.angular_fit, arithmetic).map_err(SpatialFailure::Angular)?;
    let original = original_component(request, source, q, &sine, arithmetic)?;
    let angular = angular::angular_from_captured(source, &fit, q, anchor, &sine, arithmetic)
        .map_err(SpatialFailure::Angular)?;
    Ok((angular, original, sine))
}
pub(super) type SlantComponents = (Interval, Vec<Interval>, Vec<Interval>);

pub(super) fn slant_component(
    request: &SpatialRequest<'_>,
    q: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<SlantComponents, SpatialFailure> {
    let numerator =
        slant::capture(request.slant_numerator, arithmetic).map_err(SpatialFailure::Slant)?;
    let weights = slant::capture(request.weights, arithmetic).map_err(SpatialFailure::Slant)?;
    let error = slant::slant_from_captured(q, &numerator, &weights, arithmetic)
        .map_err(SpatialFailure::Slant)?;
    Ok((error, numerator, weights))
}

pub(super) fn admit_spatial(request: &SpatialRequest<'_>) -> Result<(), SpatialFailure> {
    if !request.limit.is_finite()
        || request.limit <= 0.
        || !request.depths.iter().all(|v| v.is_finite())
        || request.depths[0] >= request.depths[1]
    {
        return Err(SpatialFailure::Domain {
            depths: request.depths,
            limit: request.limit,
        });
    }
    Ok(())
}
pub(super) fn max_offset_radius(
    numerator: &[Interval],
    weights: &[Interval],
    depths: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, SpatialFailure> {
    let maximum = numerator
        .iter()
        .fold(BigInt::from(0), |value, coefficient| {
            value.max(coefficient.upper.clone())
        });
    let numerator = Interval {
        lower: 0.into(),
        upper: maximum,
    };
    let weight =
        slant::positive_lower("weight", weights, arithmetic).map_err(SpatialFailure::Slant)?;
    let maximum_slant = arithmetic.divide(&numerator, &weight)?;
    // Both maps use the same normal sense; absolute depth bounds either orientation.
    let maximum_depth = arithmetic.scalar(depths[0].abs().max(depths[1].abs()))?;
    Ok(arithmetic.add(&maximum_slant, &maximum_depth)?)
}
pub(super) fn accept_bound(
    bound: Interval,
    original: super::original::OriginalEvidence,
    angular_derivative: Interval,
    request: &SpatialRequest<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<SpatialEvidence, SpatialFailure> {
    let limit = arithmetic.scalar(request.limit)?;
    if bound.upper > limit.lower {
        return Err(SpatialFailure::Uncertified {
            upper: DyadicBound::new(bound.upper, -i32::from(arithmetic.fractional_bits())),
            limit: request.limit,
        });
    }
    Ok(SpatialEvidence {
        bound,
        original: original.bound,
        anchor: original.anchor,
        original_angular_derivative: original.angular_derivative,
        angular_derivative,
    })
}

impl std::fmt::Display for SpatialFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Domain { depths, limit } => write!(
                f,
                "depths={depths:?}, limit={limit}; expected finite increasing depths and positive limit"
            ),
            Self::Source(cause) => write!(f, "source: {cause:?}"),
            Self::Original(cause) => write!(f, "original: {cause}"),
            Self::Angular(cause) => write!(f, "angular: {cause}"),
            Self::Slant(cause) => write!(f, "slant: {cause}"),
            Self::Arithmetic(cause) => write!(f, "arithmetic: {cause:?}"),
            Self::Uncertified { upper, limit } => write!(
                f,
                "spatial upper={upper:?}, limit={limit}; expected upper at most limit"
            ),
        }
    }
}

impl std::error::Error for SpatialFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Original(cause) => Some(cause),
            _ => None,
        }
    }
}

fn original_component(
    request: &SpatialRequest<'_>,
    source: &[Vec<Interval>; 2],
    q: &[Interval],
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<super::original::OriginalEvidence, SpatialFailure> {
    let original = match request.original {
        Some(original) => {
            super::original::bound(original, source, q, sine, request.depths, arithmetic)
                .map_err(SpatialFailure::Original)?
        }
        None => super::original::OriginalEvidence {
            bound: arithmetic.zero(),
            angular_derivative: arithmetic.zero(),
            anchor: super::original::AnchorEvidence {
                exact: true,
                turn_adjustment: Some(0),
            },
            comparison: None,
        },
    };
    Ok(original)
}

fn capture_span(
    source: [Vec<Interval>; 2],
    q: Vec<Interval>,
    radial: (Vec<Interval>, Vec<Interval>),
    errors: (Interval, Interval),
    sine: Interval,
    comparison: Option<super::original::OriginalComparison>,
) -> SpatialCapture {
    SpatialCapture {
        source,
        q,
        numerator: radial.0,
        weights: radial.1,
        angular: errors.0,
        slant: errors.1,
        sine,
        comparison,
    }
}
