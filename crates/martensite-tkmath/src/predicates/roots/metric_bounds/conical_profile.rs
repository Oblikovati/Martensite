// SPDX-License-Identifier: GPL-2.0-only
//! Bounded conical source-span clearance under normal offsets.
use super::*;
type Checked<T> = Result<T, BoundedArithmeticFailure>;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_bounds::{lower, product, sum},
    bounded_dyadic::{Arithmetic, Interval},
};
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_curve::capture_planar, bounded_dyadic::ArithmeticPolicy,
};

/// Invalid authored polynomial or incomplete bounded capture.
#[derive(Debug)]
pub enum ConicalSourceFailure {
    /// Number of poles, expected two through four.
    PoleCount(usize),
    /// A source coordinate was not finite.
    NonFinite {
        /// Source pole index.
        pole: usize,
        /// Coordinate axis index.
        axis: usize,
        /// Supplied value.
        value: f64,
    },
    /// Bounded capture did not complete.
    Arithmetic(BoundedArithmeticFailure),
}
fn admit_source(
    poles: &[[f64; 2]],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Vec<Interval>; 2], ConicalSourceFailure> {
    if !(2..=4).contains(&poles.len()) {
        return Err(ConicalSourceFailure::PoleCount(poles.len()));
    }
    for (pole, point) in poles.iter().enumerate() {
        admit_source_point(pole, point)?;
    }
    capture_planar(poles, arithmetic).map_err(ConicalSourceFailure::Arithmetic)
}
fn squared_distance(
    coordinates: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let x = product(&coordinates[0], &coordinates[0], arithmetic)?;
    let y = product(&coordinates[1], &coordinates[1], arithmetic)?;
    let count = x.len().max(y.len());
    let x = elevate_constant(x, count);
    let y = elevate_constant(y, count);
    sum(&x, &y, arithmetic)
}
fn elevate_constant(values: Vec<Interval>, count: usize) -> Vec<Interval> {
    if values.len() == 1 {
        return vec![values[0].clone(); count];
    }
    values
}
/// Certified squared distance and arithmetic work for one source span.
#[derive(Debug)]
pub struct ConicalDistanceProof {
    minimum_squared: crate::predicates::roots::DyadicBound,
    fractional_bits: u16,
    operations: u32,
    leaves: usize,
}
#[derive(Debug)]
enum DistanceOutcome {
    Proven(ConicalDistanceProof),
    Unresolved { index: u64, depth: u8 },
    InvalidThreshold(Interval),
}
type DistanceCell = ([Vec<Interval>; 2], u64, u8);
struct DistanceSearch {
    pending: Vec<DistanceCell>,
    minimum: Option<num_bigint::BigInt>,
    leaves: usize,
}
fn proves_positive(
    source: [Vec<Interval>; 2],
    max_depth: u8,
    threshold: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<DistanceOutcome> {
    if threshold.lower < 0.into() {
        return Ok(DistanceOutcome::InvalidThreshold(threshold.clone()));
    }
    let mut search = DistanceSearch {
        pending: vec![(source, 0, 0)],
        minimum: None,
        leaves: 0,
    };
    while let Some(cell) = search.pending.pop() {
        if let Some(unresolved) = search.visit(cell, max_depth, threshold, arithmetic)? {
            return Ok(unresolved);
        }
    }
    Ok(finish_distance_proof(
        search.minimum,
        search.leaves,
        arithmetic,
    ))
}
impl DistanceSearch {
    fn visit(
        &mut self,
        (coordinates, index, depth): DistanceCell,
        max_depth: u8,
        threshold: &Interval,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<Option<DistanceOutcome>> {
        let bound = lower(&squared_distance(&coordinates, arithmetic)?);
        if bound > threshold.upper {
            self.minimum = Some(
                self.minimum
                    .take()
                    .map_or(bound.clone(), |value| value.min(bound)),
            );
            self.leaves += 1;
            return Ok(None);
        }
        if depth == max_depth || depth == 64 {
            return Ok(Some(DistanceOutcome::Unresolved { index, depth }));
        }
        let [left, right] = bisect_planar(&coordinates, arithmetic)?;
        self.pending.push((right, index * 2 + 1, depth + 1));
        self.pending.push((left, index * 2, depth + 1));
        Ok(None)
    }
}
fn finish_distance_proof(
    minimum: Option<num_bigint::BigInt>,
    leaves: usize,
    arithmetic: &Arithmetic<'_>,
) -> DistanceOutcome {
    let Some(minimum_squared) = minimum else {
        return DistanceOutcome::Unresolved { index: 0, depth: 0 };
    };
    DistanceOutcome::Proven(ConicalDistanceProof {
        minimum_squared: DyadicBound::new(
            minimum_squared,
            -i32::from(arithmetic.fractional_bits()),
        ),
        fractional_bits: arithmetic.fractional_bits(),
        operations: arithmetic.operations(),
        leaves,
    })
}

fn bisect_planar(
    coordinates: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[[Vec<Interval>; 2]; 2]> {
    use crate::predicates::roots::certificate_arithmetic::bernstein_curve::bisect;
    let [x0, x1] = bisect(&coordinates[0], arithmetic)?;
    let [y0, y1] = bisect(&coordinates[1], arithmetic)?;
    Ok([[x0, y0], [x1, y1]])
}
fn required_slant_squared(
    sine: &Interval,
    cosine: &Interval,
    signed_depth: &Interval,
    clearance: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let normal_offset = arithmetic.multiply(signed_depth, cosine)?;
    let required = arithmetic.subtract(clearance, &normal_offset)?;
    let nonnegative = Interval {
        lower: required.lower.max(0.into()),
        upper: required.upper.max(0.into()),
    };
    let slant = arithmetic.divide(&nonnegative, sine)?;
    arithmetic.multiply(&slant, &slant)
}
/// Typed input or incomplete proof cause for conical span clearance.
#[derive(Debug)]
pub enum ConicalDepthFailure {
    /// Requested subdivision level exceeds dyadic address capacity.
    InvalidDepthLimit {
        /// Requested level.
        requested: u8,
    },
    /// Precision or operation limit is outside the supported range.
    InvalidPolicy {
        /// Requested fractional precision.
        fractional_bits: u16,
        /// Requested operation allowance.
        max_operations: u32,
    },
    /// The given dyadic subspan could not be resolved.
    Unresolved {
        /// Numerator of the subspan's left endpoint.
        index: u64,
        /// Subspan's power-of-two denominator exponent.
        depth: u8,
    },
    /// The clearance threshold was invalid.
    InvalidThreshold([DyadicBound; 2]),
    /// Depths were nonfinite or not increasing.
    InvalidDepths([f64; 2]),
    /// The cone or radial-clearance inputs were invalid.
    InvalidConeInputs {
        /// Requested semi-angle.
        semi_angle: f64,
        /// Worst signed depth.
        depth: f64,
        /// Requested radial clearance.
        clearance: f64,
    },
    /// Source capture failure.
    Source(ConicalSourceFailure),
    /// Bounded arithmetic failure.
    Arithmetic(BoundedArithmeticFailure),
    /// Trigonometric enclosure failure.
    Trigonometric(crate::predicates::roots::metric_bounds::CylindricalProfileFailure),
}
#[derive(Debug)]
struct DepthEvaluation {
    outcome: DistanceOutcome,
    sine: [crate::predicates::roots::DyadicBound; 2],
    cosine: [crate::predicates::roots::DyadicBound; 2],
    threshold: [crate::predicates::roots::DyadicBound; 2],
}
fn exact_enclosure(interval: &Interval, bits: u16) -> [crate::predicates::roots::DyadicBound; 2] {
    use crate::predicates::roots::DyadicBound;
    let exponent = -i32::from(bits);
    [
        DyadicBound::new(interval.lower.clone(), exponent),
        DyadicBound::new(interval.upper.clone(), exponent),
    ]
}
fn prove_conical_depth_clearance(
    poles: &[[f64; 2]],
    semi_angle: f64,
    worst_signed_depth: f64,
    clearance: f64,
    max_depth: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<DepthEvaluation, ConicalDepthFailure> {
    admit_cone_inputs(semi_angle, worst_signed_depth, clearance)?;
    let source = admit_source(poles, arithmetic).map_err(ConicalDepthFailure::Source)?;
    let (sine, cosine, threshold) =
        cone_intervals(semi_angle, worst_signed_depth, clearance, arithmetic)?;
    let outcome = proves_positive(source, max_depth, &threshold, arithmetic)
        .map_err(ConicalDepthFailure::Arithmetic)?;
    let bits = arithmetic.fractional_bits();
    Ok(DepthEvaluation {
        outcome,
        sine: exact_enclosure(&sine[0], bits),
        cosine: exact_enclosure(&cosine[0], bits),
        threshold: exact_enclosure(&threshold, bits),
    })
}
fn cone_intervals(
    semi_angle: f64,
    depth: f64,
    clearance: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Vec<Interval>, Vec<Interval>, Interval), ConicalDepthFailure> {
    use crate::predicates::roots::metric_bounds::cylindrical_profile::trigonometric;
    let angle = arithmetic
        .scalar(semi_angle)
        .map_err(ConicalDepthFailure::Arithmetic)?;
    let [cosine, sine] =
        trigonometric::enclose(&[angle], arithmetic).map_err(ConicalDepthFailure::Trigonometric)?;
    let depth = arithmetic
        .scalar(depth)
        .map_err(ConicalDepthFailure::Arithmetic)?;
    let clearance = arithmetic
        .scalar(clearance)
        .map_err(ConicalDepthFailure::Arithmetic)?;
    let threshold = required_slant_squared(&sine[0], &cosine[0], &depth, &clearance, arithmetic)
        .map_err(ConicalDepthFailure::Arithmetic)?;
    Ok((sine, cosine, threshold))
}
fn admit_cone_inputs(
    semi_angle: f64,
    worst_signed_depth: f64,
    clearance: f64,
) -> Result<(), ConicalDepthFailure> {
    if !semi_angle.is_finite()
        || semi_angle <= 0.
        || semi_angle >= std::f64::consts::FRAC_PI_2
        || !worst_signed_depth.is_finite()
        || !clearance.is_finite()
        || clearance < 0.
    {
        return Err(ConicalDepthFailure::InvalidConeInputs {
            semi_angle,
            depth: worst_signed_depth,
            clearance,
        });
    }
    Ok(())
}
fn prove_conical_depth_interval(
    poles: &[[f64; 2]],
    semi_angle: f64,
    depths: [f64; 2],
    outward: bool,
    clearance: f64,
    max_depth: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<DepthEvaluation, ConicalDepthFailure> {
    if !depths[0].is_finite() || !depths[1].is_finite() || depths[0] >= depths[1] {
        return Err(ConicalDepthFailure::InvalidDepths(depths));
    }
    let worst = if outward { depths[0] } else { -depths[1] };
    prove_conical_depth_clearance(poles, semi_angle, worst, clearance, max_depth, arithmetic)
}
/// Source-bound conical span clearance evidence.
#[derive(Debug)]
pub struct ConeReceipt {
    source_bits: Vec<[u64; 2]>,
    semi_angle_bits: u64,
    depths_bits: [u64; 2],
    outward: bool,
    clearance_bits: u64,
    requested_depth: u8,
    proof: ConicalDistanceProof,
    sine: [crate::predicates::roots::DyadicBound; 2],
    cosine: [crate::predicates::roots::DyadicBound; 2],
    threshold: [crate::predicates::roots::DyadicBound; 2],
}
fn prove_owned_cone_interval(
    poles: &[[f64; 2]],
    semi_angle: f64,
    depths: [f64; 2],
    outward: bool,
    clearance: f64,
    max_depth: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<ConeReceipt, ConicalDepthFailure> {
    let outcome = prove_conical_depth_interval(
        poles, semi_angle, depths, outward, clearance, max_depth, arithmetic,
    )?;
    let proof = completed_depth_proof(outcome.outcome, arithmetic.fractional_bits())?;
    Ok(ConeReceipt {
        source_bits: poles.iter().map(|point| point.map(f64::to_bits)).collect(),
        semi_angle_bits: semi_angle.to_bits(),
        depths_bits: depths.map(f64::to_bits),
        outward,
        clearance_bits: clearance.to_bits(),
        requested_depth: max_depth,
        proof,
        sine: outcome.sine,
        cosine: outcome.cosine,
        threshold: outcome.threshold,
    })
}
fn completed_depth_proof(
    outcome: DistanceOutcome,
    bits: u16,
) -> Result<ConicalDistanceProof, ConicalDepthFailure> {
    match outcome {
        DistanceOutcome::Proven(proof) => Ok(proof),
        DistanceOutcome::Unresolved { index, depth } => {
            Err(ConicalDepthFailure::Unresolved { index, depth })
        }
        DistanceOutcome::InvalidThreshold(value) => Err(ConicalDepthFailure::InvalidThreshold(
            exact_enclosure(&value, bits),
        )),
    }
}

fn magnitude_interval(values: &[Interval]) -> Interval {
    Interval {
        lower: 0.into(),
        upper: crate::predicates::roots::certificate_arithmetic::bernstein_bounds::magnitude(
            values,
        ),
    }
}

mod actual_certificate;
pub(super) mod angle;
pub use actual_certificate::*;
mod angular;
mod boundary;
mod boundary_cause;
mod boundary_chart;
pub use boundary_chart::{
    certify_conical_boundary_in_chart, certify_conical_boundary_in_chart_with_refinement,
};
mod boundary_kind;
mod boundary_snapshot;
mod boundary_transition;
pub use boundary_transition::{
    ConicalBoundaryZRowError, ConicalBoundaryZRowEvidence, ConicalBoundaryZRowFailure,
    ConicalBoundaryZRowTransition, certify_conical_boundary_z_rows,
};
mod chart_domain;
mod combined;
mod developed;
mod inverse;
mod joins;
mod original;
mod quadrature;
mod quadrature_certificate;
mod refinement;
pub use refinement::ConicalPrimitiveRefinement;
mod reference;
mod request;
mod sector;
mod sector_api;
mod slant;
mod slant_derivative;
mod source_index;
mod spatial;
mod storage;
mod storage_api;
mod storage_pair;
mod volume;
mod volume_api;
mod volume_boundary;

pub use angle::{
    ConicalAuthoredContour, ConicalAuthoredFlux, ConicalAuthoredFluxError,
    ConicalAuthoredFluxQuery, ConicalAuthoredFluxStage, ConicalBoundaryError,
    ConicalBoundaryEvidence, ConicalBoundaryFailure, ConicalBoundaryFailureKind,
    ConicalBoundaryProofFailure, ConicalBoundaryQuery, ConicalBoundaryStage,
    ConicalJoinFailureKind, ConicalJoinProofFailure, ConicalJoinSpan, ConicalJoinsEvidence,
    ConicalJoinsFailure, ConicalJoinsQuery, ConicalOriginalFailure, ConicalOriginalSpan,
    certify_conical_authored_flux, certify_conical_boundary, certify_conical_joins,
};
pub use chart_domain::{
    ConicalChartDomainBound, ConicalChartDomainError, ConicalChartDomainEvidence,
    ConicalChartDomainFailure, ConicalChartDomainQuery, ConicalChartDomainRequirement,
    ConicalChartDomainVersion, ConicalChartPlan, ConicalChartPlanError, ConicalChartPlanFailure,
    ConicalChartPlanQuery, ConicalSubspanChartError, certify_conical_chart_domain,
    certify_conical_chart_domain_with_version, certify_conical_subspan_chart,
    plan_conical_subspan_charts,
};
pub use reference::{
    ConicalAtlasReferenceDomain, ConicalAtlasReferenceQuery, ConicalChartReferenceQuery,
    ConicalDecodedFamilyError, ConicalDecodedFamilyFailure, ConicalDecodedFillFailure,
    ConicalReferenceError, ConicalReferenceEvidence, ConicalReferenceFailure,
    ConicalReferenceMember, ConicalReferenceQuery, certify_conical_atlas_reference,
    certify_conical_chart_reference, certify_conical_chart_reference_with_refinement,
    certify_conical_decoded_family, certify_conical_decoded_family_with_fill,
    certify_conical_reference,
};
pub use request::{
    ConeClearanceOptions, ConeRequest, ConeRequestError, ConeRequestReceipt, certify_cone_request,
};
pub use sector_api::{
    ConicalSectorEvidence, ConicalSectorFailure, ConicalSectorQuery, ConicalSectorWedge,
    certify_conical_sector,
};
pub use storage::{
    ConicalStorageError, ConicalStorageEvidence, ConicalStorageFailure, ConicalStorageQuery,
    certify_conical_storage,
};

mod boundary_evidence;
mod cancellation;

#[cfg(test)]
mod leaf_tests;

fn admit_source_point(pole: usize, point: &[f64; 2]) -> Result<(), ConicalSourceFailure> {
    for (axis, value) in point.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(ConicalSourceFailure::NonFinite { pole, axis, value });
        }
    }
    Ok(())
}

pub use joins::{
    ConicalCapClearanceRelation, ConicalCapFamilyHomotopyError, ConicalCapHomotopyEvidence,
    ConicalCapHomotopyFailure, ConicalCapJoinSpan, ConicalCapJoinsQuery, ConicalCapNormalError,
    ConicalCapNormalEvidence, ConicalCapVolumeError, ConicalCapVolumeEvidence,
    ConicalCapVolumeFailure, ConicalChartJoinsError, ConicalDecodedCapEvidence,
    certify_conical_cap_family_homotopy, certify_conical_cap_joins, certify_conical_joins_in_chart,
    certify_conical_joins_in_chart_with_refinement,
};

mod trim_bridge;
pub use trim_bridge::{
    ConicalChartTrimPairError, ConicalChartTrimPairFailure, certify_conical_trim_pair_in_chart,
    certify_conical_trim_pair_in_chart_with_refinement,
};

pub(super) mod developed_boundary;
pub use developed_boundary::{ConicalDevelopedBoundaryEvidence, ConicalDevelopedBoundarySpan};
pub use trim_bridge::{
    certify_conical_trim_pair_with_developed_boundary,
    certify_conical_trim_pair_with_developed_boundary_and_refinement,
};

pub use volume_api::{ConicalAuthoredFluxMethod, certify_conical_authored_flux_with_method};
