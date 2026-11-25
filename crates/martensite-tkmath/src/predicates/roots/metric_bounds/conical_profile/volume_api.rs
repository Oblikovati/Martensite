// SPDX-License-Identifier: GPL-2.0-only
//! Conditional authored measure and represented signed-flux comparison.
use super::inverse::*;
use super::volume::*;
use super::volume_boundary::*;
use super::*;
use crate::predicates::roots::DyadicBound;
use crate::predicates::roots::metric_bounds::ConeClearanceOptions;

/// One authored polynomial contour and its material-facing orientation.
#[derive(Clone, Copy)]
pub struct ConicalAuthoredContour<'a> {
    /// Ordered polynomial spans; each has two through four developed-plane poles.
    pub spans: &'a [&'a [[f64; 2]]],
    /// Reverse the signed boundary integral for this contour.
    pub reverse: bool,
}

/// Inputs for a conditional authored-volume and signed-flux comparison.
#[derive(Clone, Copy)]
pub struct ConicalAuthoredFluxQuery<'a> {
    /// Material contours; the caller establishes their topology and orientation.
    pub contours: &'a [ConicalAuthoredContour<'a>],
    /// Cone semi-angle in radians, strictly between zero and pi/2.
    pub semi_angle: f64,
    /// Increasing signed normal-depth endpoints.
    pub depths: [f64; 2],
    /// Whether positive depth follows the outward normal.
    pub outward: bool,
    /// Simpson subdivision level, at most twelve.
    pub level: u8,
    /// Depth-clearance subdivision level, at most 64.
    pub clearance_level: u8,
    /// Maximum width of the authored-volume enclosure.
    pub volume_width: f64,
    /// Finite ordered represented signed-flux enclosure.
    pub flux: [f64; 2],
    /// Shared fixed-precision arithmetic allowance.
    pub arithmetic: ConeClearanceOptions,
}

/// Explicit authored quadrature behavior; both use the requested fixed Simpson panel count.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicalAuthoredFluxMethod {
    /// Original Bernstein subdivision, polynomial fourth derivative and repeated endpoints.
    LegacySimpsonV1,
    /// Newton differences and a scalar majorant when the global radius bound is positive.
    /// Otherwise retains the existing local Bernstein proof at the same fixed level.
    ConditionedSimpsonV2,
}

/// A conditional comparison; it does not establish body validity or flux provenance.
pub struct ConicalAuthoredFlux {
    pub(super) method: ConicalAuthoredFluxMethod,
    pub(super) volume: ConditionalVolumeEvidence,
    pub(super) flux: [u64; 2],
    pub(super) difference: DyadicBound,
    pub(super) operations: u32,
}

/// A refusal to certify the conditional comparison.
#[derive(Debug)]
pub struct ConicalAuthoredFluxError {
    pub(super) cause: Failure,
    pub(super) operations: u32,
}
/// Stage that refused the conditional comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicalAuthoredFluxStage {
    /// Input collections exceeded supported shape limits.
    Shape,
    /// Authored measure, clearance, or arithmetic could not be certified.
    Volume,
    /// Flux endpoints or the bounded comparison could not be certified.
    Comparison,
}
#[derive(Debug)]
pub(super) enum Failure {
    Shape { contours: usize, spans: usize },
    Volume(BoundaryFailure),
    Comparison(FluxComparisonFailure),
}
impl std::fmt::Display for ConicalAuthoredFluxError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "conditional authored flux after {} operations: ",
            self.operations
        )?;
        write!(out, "{}", self.cause)
    }
}
impl std::error::Error for ConicalAuthoredFluxError {}
impl ConicalAuthoredFluxError {
    /// The stage that refused; `error.stage()`.
    pub fn stage(&self) -> ConicalAuthoredFluxStage {
        match self.cause {
            Failure::Shape { .. } => ConicalAuthoredFluxStage::Shape,
            Failure::Volume(_) => ConicalAuthoredFluxStage::Volume,
            Failure::Comparison(_) => ConicalAuthoredFluxStage::Comparison,
        }
    }
    /// Number of arithmetic operations completed before refusal; `error.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Retained bounded-arithmetic cause, if present; `error.arithmetic_cause()`.
    pub fn arithmetic_cause(&self) -> Option<&BoundedArithmeticFailure> {
        match &self.cause {
            Failure::Comparison(FluxComparisonFailure::Arithmetic(cause))
            | Failure::Volume(BoundaryFailure::Arithmetic(cause))
            | Failure::Volume(BoundaryFailure::Options(SourceBoundFailure::Arithmetic(cause)))
            | Failure::Volume(BoundaryFailure::Span {
                cause: SourceBoundFailure::Arithmetic(cause),
                ..
            }) => Some(cause),
            _ => None,
        }
    }
    /// Actual authored contour/span for either integration or depth-clearance failure (#523).
    /// Global failures have no fabricated source position; `let location = error.span_location();`.
    pub fn span_location(&self) -> Option<(usize, usize)> {
        match &self.cause {
            Failure::Volume(BoundaryFailure::Span { contour, span, .. })
            | Failure::Volume(BoundaryFailure::DepthClearance { contour, span, .. }) => {
                Some((*contour, *span))
            }
            _ => None,
        }
    }
    /// Contour, span, and depth-clearance cause, if present; `error.depth_cause()`.
    pub fn depth_cause(&self) -> Option<(usize, usize, &super::ConicalDepthFailure)> {
        match &self.cause {
            Failure::Volume(BoundaryFailure::DepthClearance {
                contour,
                span,
                cause,
            }) => Some((*contour, *span, cause)),
            _ => None,
        }
    }
}
impl ConicalAuthoredFlux {
    /// Authored signed area enclosure; `evidence.area()`.
    pub fn area(&self) -> [DyadicBound; 2] {
        self.interval(&self.volume.area)
    }
    /// Inverse-slant integral enclosure; `evidence.inverse_slant()`.
    pub fn inverse_slant(&self) -> [DyadicBound; 2] {
        self.interval(&self.volume.inverse_slant)
    }
    /// Authored boundary clearance bounds for a span; `evidence.clearance(0)`.
    pub fn clearance(&self, index: usize) -> Option<[&DyadicBound; 2]> {
        self.volume
            .clearances
            .get(index)
            .map(|value| [&value.minimum_squared, &value.threshold_upper])
    }
    /// Work charged to authored-volume certification; `evidence.volume_operations()`.
    pub fn volume_operations(&self) -> u32 {
        self.volume.operations
    }
    pub(super) fn interval(&self, value: &Interval) -> [DyadicBound; 2] {
        let exponent = -i32::from(self.volume.policy.fractional_bits);
        [
            DyadicBound::new(value.lower.clone(), exponent),
            DyadicBound::new(value.upper.clone(), exponent),
        ]
    }
    /// Authored-volume enclosure in exact dyadic endpoints; `evidence.volume()`.
    pub fn volume(&self) -> [DyadicBound; 2] {
        let exponent = -i32::from(self.volume.policy.fractional_bits);
        [
            DyadicBound::new(self.volume.enclosure.lower.clone(), exponent),
            DyadicBound::new(self.volume.enclosure.upper.clone(), exponent),
        ]
    }
    /// Bound on the difference between the two supplied enclosures; `evidence.difference()`.
    pub fn difference(&self) -> &DyadicBound {
        &self.difference
    }
    /// Total bounded arithmetic work, including comparison; `evidence.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Selected fixed-panel algorithm version, including its documented fallback; `assert_eq!(proof.method(), ConicalAuthoredFluxMethod::LegacySimpsonV1);`.
    pub fn method(&self) -> ConicalAuthoredFluxMethod {
        self.method
    }
    /// Bind original sources, flux and policy under the legacy method; `evidence.matches(query)`.
    pub fn matches(&self, query: ConicalAuthoredFluxQuery<'_>) -> bool {
        self.matches_with_method(query, ConicalAuthoredFluxMethod::LegacySimpsonV1)
    }
    /// Bind the explicitly selected method and every original query bit.
    /// Example: `assert!(proof.matches_with_method(query, proof.method()));`.
    pub fn matches_with_method(
        &self,
        query: ConicalAuthoredFluxQuery<'_>,
        method: ConicalAuthoredFluxMethod,
    ) -> bool {
        self.method == method && self.matches_inputs(query)
    }
    fn matches_inputs(&self, query: ConicalAuthoredFluxQuery<'_>) -> bool {
        let Some(contours) = capture(query.contours) else {
            return false;
        };
        self.flux == query.flux.map(f64::to_bits)
            && self
                .volume
                .matches(&input(query, &contours), policy(query.arithmetic))
    }
}

/// Certify a conditional comparison; `certify_conical_authored_flux(query, || false)`.
///
/// The caller must prove the contours bound the intended material, the normal map
/// is injective, and the supplied flux belongs to the represented boundary.
pub fn certify_conical_authored_flux(
    query: ConicalAuthoredFluxQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalAuthoredFlux, ConicalAuthoredFluxError> {
    certify_selected(
        query,
        ConicalAuthoredFluxMethod::LegacySimpsonV1,
        &mut cancelled,
    )
}
/// Select fixed-panel authored quadrature explicitly, retaining original source and method binding.
/// Example: `certify_conical_authored_flux_with_method(query, ConicalAuthoredFluxMethod::ConditionedSimpsonV2, || false)?`.
pub fn certify_conical_authored_flux_with_method(
    query: ConicalAuthoredFluxQuery<'_>,
    method: ConicalAuthoredFluxMethod,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalAuthoredFlux, ConicalAuthoredFluxError> {
    certify_selected(query, method, &mut cancelled)
}
fn certify_selected(
    query: ConicalAuthoredFluxQuery<'_>,
    method: ConicalAuthoredFluxMethod,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<ConicalAuthoredFlux, ConicalAuthoredFluxError> {
    let contours = admitted_contours(query)?;
    let mut arithmetic = Arithmetic::new(policy(query.arithmetic), cancelled);
    let volume =
        selected_volume(&input(query, &contours), method, &mut arithmetic).map_err(|cause| {
            ConicalAuthoredFluxError {
                cause: Failure::Volume(cause),
                operations: arithmetic.operations(),
            }
        })?;
    let difference =
        compare_represented_flux(&volume, query.flux, &mut arithmetic).map_err(|cause| {
            ConicalAuthoredFluxError {
                cause: Failure::Comparison(cause),
                operations: arithmetic.operations(),
            }
        })?;
    let operations = arithmetic.operations();
    Ok(authored_flux(
        volume, query.flux, difference, operations, method,
    ))
}

fn selected_volume(
    input: &VolumeInput<'_>,
    method: ConicalAuthoredFluxMethod,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<ConditionalVolumeEvidence, BoundaryFailure> {
    match method {
        ConicalAuthoredFluxMethod::LegacySimpsonV1 => conditional_volume(input, arithmetic),
        ConicalAuthoredFluxMethod::ConditionedSimpsonV2 => {
            conditional_volume_using(input, arithmetic, conditioned_sum)
        }
    }
}
fn conditioned_sum(
    contours: &[Contour<'_>],
    level: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, BoundaryFailure> {
    sum_contours_using(
        contours,
        level,
        arithmetic,
        super::quadrature::conditioned::integrate_span,
    )
}

pub(super) fn policy(options: ConeClearanceOptions) -> ArithmeticPolicy {
    ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
    }
}
pub(super) fn capture<'a>(contours: &[ConicalAuthoredContour<'a>]) -> Option<Vec<Contour<'a>>> {
    if contours.is_empty() || contours.len() > 128 {
        return None;
    }
    let count = contours
        .iter()
        .try_fold(0_usize, |n, item| n.checked_add(item.spans.len()))?;
    if count == 0 || count > 128 {
        return None;
    }
    if contours.iter().any(|item| {
        item.spans.is_empty() || item.spans.iter().any(|span| !(2..=4).contains(&span.len()))
    }) {
        return None;
    }
    Some(capture_contours(contours))
}

pub(super) fn input<'a>(
    query: ConicalAuthoredFluxQuery<'_>,
    contours: &'a [Contour<'a>],
) -> VolumeInput<'a> {
    VolumeInput {
        contours,
        semi_angle: query.semi_angle,
        depths: query.depths,
        outward: query.outward,
        level: query.level,
        clearance_level: query.clearance_level,
        limit: query.volume_width,
    }
}

fn admitted_contours<'a>(
    query: ConicalAuthoredFluxQuery<'a>,
) -> Result<Vec<Contour<'a>>, ConicalAuthoredFluxError> {
    if !query.flux.iter().all(|value| value.is_finite()) || query.flux[0] > query.flux[1] {
        return Err(ConicalAuthoredFluxError {
            cause: Failure::Comparison(FluxComparisonFailure::InvalidEnclosure(query.flux)),
            operations: 0,
        });
    }
    let Some(contours) = capture(query.contours) else {
        return Err(shape_error(query));
    };
    Ok(contours)
}

fn shape_error(query: ConicalAuthoredFluxQuery<'_>) -> ConicalAuthoredFluxError {
    ConicalAuthoredFluxError {
        cause: Failure::Shape {
            contours: query.contours.len(),
            spans: query
                .contours
                .iter()
                .fold(0_usize, |n, item| n.saturating_add(item.spans.len())),
        },
        operations: 0,
    }
}

fn capture_contours<'a>(contours: &[ConicalAuthoredContour<'a>]) -> Vec<Contour<'a>> {
    contours
        .iter()
        .map(|item| Contour {
            spans: item.spans.to_vec(),
            reverse: item.reverse,
        })
        .collect()
}

fn authored_flux(
    volume: ConditionalVolumeEvidence,
    flux: [f64; 2],
    difference: DyadicBound,
    operations: u32,
    method: ConicalAuthoredFluxMethod,
) -> ConicalAuthoredFlux {
    ConicalAuthoredFlux {
        method,
        volume,
        flux: flux.map(f64::to_bits),
        difference,
        operations,
    }
}

impl std::fmt::Display for Failure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shape { contours, spans } => write!(
                out,
                "contours={contours}, spans={spans}; expected 1..=128 contours and spans, with 2..=4 poles per span"
            ),
            Self::Volume(cause) => write!(out, "authored volume: {cause}"),
            Self::Comparison(cause) => write!(out, "{cause}"),
        }
    }
}

#[cfg(test)]
#[path = "volume_api/location_tests.rs"]
mod location_tests;

#[cfg(test)]
#[path = "volume_api/conditioned_tests.rs"]
mod conditioned_tests;
