// SPDX-License-Identifier: GPL-2.0-only
//! Compose the winding-qualified reference and separate the decoded solid members.
use super::super::{joins::CapBinding, trim_bridge::bounds::captured_storage};
use super::*;
use crate::predicates::roots::DyadicBound;
mod fill;
pub use fill::ConicalDecodedFillFailure;

use std::sync::Arc;
pub(super) struct DecodedFamily {
    members: Vec<ConicalDecodedCapEvidence>,
    solid_gap: Option<DyadicBound>,
    joins: Vec<Arc<CapBinding>>,
    separated_fill: bool,
}
impl std::fmt::Debug for DecodedFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecodedFamily")
            .field("members", &self.members)
            .field("solid_gap", &self.solid_gap)
            .finish()
    }
}
/// Stage and arithmetic work of an incomplete decoded cone family.
#[derive(Debug)]
pub struct ConicalDecodedFamilyError {
    /// Index of the failed member, if the refusal was member-local.
    pub member: Option<usize>,
    /// Exact arithmetic spent before refusal.
    pub operations: u32,
    /// Typed failed obligation.
    pub cause: ConicalDecodedFamilyFailure,
}
/// The decoded reference could not be bound, measured, or separated in space.
#[derive(Debug)]
pub enum ConicalDecodedFamilyFailure {
    /// Complete interpolation and periodic boundary binding.
    Homotopy(Box<ConicalCapFamilyHomotopyError>),
    /// A particular member's full-depth measures.
    Member(Box<ConicalCapNormalError>),
    /// Family separation or arithmetic policy.
    Family(ConicalReferenceFailure),
    /// Exact winding of the decoded endpoint failed.
    Fill(ConicalDecodedFillFailure),
}
impl std::fmt::Display for ConicalDecodedFamilyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "decoded cone member={:?}, work={}, obligation={:?}",
            self.member, self.operations, self.cause
        )
    }
}
impl std::error::Error for ConicalDecodedFamilyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.cause {
            ConicalDecodedFamilyFailure::Homotopy(cause) => Some(cause.as_ref()),
            ConicalDecodedFamilyFailure::Member(cause) => Some(cause.as_ref()),
            ConicalDecodedFamilyFailure::Fill(ConicalDecodedFillFailure::Winding(cause)) => {
                Some(cause.as_ref())
            }
            ConicalDecodedFamilyFailure::Fill(ConicalDecodedFillFailure::Probe(cause)) => {
                Some(cause.as_ref())
            }
            ConicalDecodedFamilyFailure::Family(_) | ConicalDecodedFamilyFailure::Fill(_) => None,
        }
    }
}
impl ConicalDecodedFamilyError {
    /// Whether a bounded leaf observed cancellation. Use `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        match &self.cause {
            ConicalDecodedFamilyFailure::Member(cause) => cancelled_member(cause),
            ConicalDecodedFamilyFailure::Family(cause) => {
                matches!(
                    cause,
                    ConicalReferenceFailure::Arithmetic(BoundedArithmeticFailure::Cancelled)
                )
            }
            ConicalDecodedFamilyFailure::Homotopy(_) => false,
            ConicalDecodedFamilyFailure::Fill(cause) => cause.cancelled(),
        }
    }
}
fn cancelled_member(cause: &ConicalCapNormalError) -> bool {
    match cause {
        ConicalCapNormalError::Spatial(error) => error.is_cancelled(),
        ConicalCapNormalError::Volume(error) => match &error.cause {
            ConicalCapVolumeFailure::Arithmetic(BoundedArithmeticFailure::Cancelled) => true,
            ConicalCapVolumeFailure::Trigonometric(CylindricalProfileFailure::Arithmetic(
                cause,
            )) => {
                matches!(cause.as_ref(), BoundedArithmeticFailure::Cancelled)
            }
            _ => false,
        },
    }
}
impl ConicalReferenceEvidence {
    /// Whether the issuance checked filled disjointness, including all periodic images.
    /// Example: `assert!(proof.has_separated_decoded_fill());`.
    pub fn has_separated_decoded_fill(&self) -> bool {
        self.decoded
            .as_ref()
            .is_some_and(|family| family.separated_fill)
    }

    /// Decoded local-chart receipts in the certified contour-owner order. Use `proof.decoded_caps()`.
    pub fn decoded_caps(&self) -> Option<&[ConicalDecodedCapEvidence]> {
        self.decoded
            .as_ref()
            .map(|family| family.members.as_slice())
    }
    /// Strict world-space separation of decoded solids, if the atlas has multiple members.
    /// Read the outward lower bound with `proof.decoded_solid_separation()`.
    pub fn decoded_solid_separation(&self) -> Option<&DyadicBound> {
        self.decoded.as_ref()?.solid_gap.as_ref()
    }

    /// Check the same immutable join receipts and complete member order without copying poles.
    /// Example: `assert!(proof.shares_decoded_join_issuance(joins.iter().copied()));`.
    pub fn shares_decoded_join_issuance<'a>(
        &self,
        joins: impl IntoIterator<Item = &'a ConicalJoinsEvidence>,
    ) -> bool {
        let Some(family) = &self.decoded else {
            return false;
        };
        let mut joins = joins.into_iter();
        family.joins.iter().all(|old| {
            joins
                .next()
                .and_then(|join| join.cap_charts.as_ref())
                .is_some_and(|new| Arc::ptr_eq(old, new))
        }) && joins.next().is_none()
    }
}

/// Measure all members under one finite grant and certify that their decoded solids stay disjoint.
/// The caller binds complete filled-cap winding relations and regular closed decoded member solids.
/// Example: `certify_conical_decoded_family(&joins, &within, &periodic, epsilon, options, || false)?`.
pub fn certify_conical_decoded_family(
    joins: &[&ConicalJoinsEvidence],
    clearance: &CertifiedCylindricalClearance,
    periodic: &super::super::cylindrical_clearance::CertifiedCylindricalPeriodicClearance,
    spatial_limit: f64,
    options: ConeClearanceOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalReferenceEvidence, ConicalDecodedFamilyError> {
    certify_family(
        joins,
        clearance,
        periodic,
        false,
        spatial_limit,
        options,
        &mut cancelled,
    )
}

/// Certify both the periodic decoded fill and its full-depth reference from the same cap sources.
/// Example: `certify_conical_decoded_family_with_fill(&joins, &within, &periodic, epsilon, options, || false)?`.
pub fn certify_conical_decoded_family_with_fill(
    joins: &[&ConicalJoinsEvidence],
    clearance: &CertifiedCylindricalClearance,
    periodic: &Periodic,
    spatial_limit: f64,
    options: ConeClearanceOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalReferenceEvidence, ConicalDecodedFamilyError> {
    certify_family(
        joins,
        clearance,
        periodic,
        true,
        spatial_limit,
        options,
        &mut cancelled,
    )
}

fn certify_family(
    joins: &[&ConicalJoinsEvidence],
    clearance: &CertifiedCylindricalClearance,
    periodic: &Periodic,
    check_fill: bool,
    spatial_limit: f64,
    options: ConeClearanceOptions,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<ConicalReferenceEvidence, ConicalDecodedFamilyError> {
    let proofs = bind(joins, clearance, periodic, options)?;
    let fill_spent = if check_fill {
        fill::certify(joins, periodic, loan(options, 0), cancelled).map_err(fill_refusal)?
    } else {
        0
    };
    let (members, spent) = measure(&proofs, spatial_limit, options, fill_spent, cancelled)?;
    let (solid_gap, used) = separate(&proofs[0], &members, loan(options, spent), cancelled)
        .map_err(|(cause, used)| failed(cause, spent + used))?;
    let joins = joins
        .iter()
        .filter_map(|join| join.cap_charts.clone())
        .collect();
    let family = DecodedFamily {
        members,
        solid_gap,
        joins,
        separated_fill: check_fill,
    };
    summarize(family, spatial_limit, options, spent + used)
}
fn fill_refusal(
    (cause, operations): (ConicalDecodedFillFailure, u32),
) -> ConicalDecodedFamilyError {
    let member = match &cause {
        ConicalDecodedFillFailure::Overlap { source, .. } => Some(*source),
        _ => None,
    };
    ConicalDecodedFamilyError {
        member,
        operations,
        cause: ConicalDecodedFamilyFailure::Fill(cause),
    }
}
type Periodic = super::super::cylindrical_clearance::CertifiedCylindricalPeriodicClearance;
fn bind<'a>(
    joins: &[&'a ConicalJoinsEvidence],
    clearance: &CertifiedCylindricalClearance,
    periodic: &Periodic,
    options: ConeClearanceOptions,
) -> Result<Vec<ConicalCapHomotopyEvidence<'a>>, ConicalDecodedFamilyError> {
    if options.max_operations > 16_000_000 {
        return Err(failed(
            binding_failure("bounded decoded family arithmetic grant"),
            0,
        ));
    }
    certify_conical_cap_family_homotopy(joins, clearance, periodic).map_err(|cause| {
        ConicalDecodedFamilyError {
            member: cause.member,
            operations: 0,
            cause: ConicalDecodedFamilyFailure::Homotopy(Box::new(cause)),
        }
    })
}
fn loan(options: ConeClearanceOptions, spent: u32) -> ConeClearanceOptions {
    ConeClearanceOptions {
        max_operations: (options.max_operations - spent).min(1_000_000),
        ..options
    }
}
fn measure(
    proofs: &[ConicalCapHomotopyEvidence<'_>],
    limit: f64,
    options: ConeClearanceOptions,
    mut spent: u32,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<(Vec<ConicalDecodedCapEvidence>, u32), ConicalDecodedFamilyError> {
    let mut members = Vec::with_capacity(proofs.len());
    for (index, proof) in proofs.iter().enumerate() {
        let member = proof
            .certify_decoded_reference(limit, loan(options, spent), &mut *cancelled)
            .map_err(|cause| ConicalDecodedFamilyError {
                member: Some(index),
                operations: spent + cause.operations(),
                cause: ConicalDecodedFamilyFailure::Member(Box::new(cause)),
            })?;
        spent += member.operations();
        members.push(member);
    }
    Ok((members, spent))
}
fn separate(
    proof: &ConicalCapHomotopyEvidence<'_>,
    members: &[ConicalDecodedCapEvidence],
    options: ConeClearanceOptions,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<(Option<DyadicBound>, u32), (ConicalReferenceFailure, u32)> {
    if members.len() == 1 {
        return Ok((None, 0));
    }
    let mut arithmetic = Arithmetic::new(sector_api::policy(options), cancelled);
    let result = separation(proof, members, &mut arithmetic);
    result
        .map(|gap| (Some(gap), arithmetic.operations()))
        .map_err(|cause| (cause, arithmetic.operations()))
}
fn separation(
    proof: &ConicalCapHomotopyEvidence<'_>,
    members: &[ConicalDecodedCapEvidence],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<DyadicBound, ConicalReferenceFailure> {
    let [within, periodic] = proof.separation_lower_bounds();
    let radius = members
        .iter()
        .map(|member| &member.normal().radius_bounds()[0])
        .min()
        .ok_or_else(|| binding_failure("nonempty decoded radius inventory"))?;
    world_separation(
        within.min(periodic),
        radius,
        maximum_storage(members)?,
        arithmetic,
    )
}
fn world_separation(
    gap: &DyadicBound,
    radius: &DyadicBound,
    storage: &DyadicBound,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<DyadicBound, ConicalReferenceFailure> {
    // On the periodic quotient the closest points of disjoint fills lie on
    // their boundaries. Orthogonal meridian coordinates preserve slant
    // differences, and the shortest angular chord is at least radius * angle / 2.
    let radius = captured_storage(radius, arithmetic)?;
    let scale = arithmetic.fraction(&radius, 1, 2)?;
    let scale = Interval {
        lower: scale.lower.min(arithmetic.scalar(1.)?.lower),
        upper: scale.upper,
    };
    let gap = captured_storage(gap, arithmetic)?;
    let world = arithmetic.multiply(&scale, &gap)?;
    // Straight boundary interpolation preserves each shell's winding outside
    // its storage-error neighborhood, so it bounds the decoded solid as well.
    let storage = captured_storage(storage, arithmetic)?;
    let twice = arithmetic.fraction(&storage, 2, 1)?;
    let lower = arithmetic.subtract(&world, &twice)?.lower;
    strict_gap(lower, arithmetic)
}

fn strict_gap(
    lower: num_bigint::BigInt,
    arithmetic: &Arithmetic<'_>,
) -> Result<DyadicBound, ConicalReferenceFailure> {
    let gap = DyadicBound::new(lower, -i32::from(arithmetic.fractional_bits()));
    if gap.significand <= 0.into() {
        return Err(ConicalReferenceFailure::Bound {
            field: "positive decoded cone solid separation",
            actual: upper(&gap)?,
            limit: 0.,
        });
    }
    Ok(gap)
}
fn maximum_storage(
    members: &[ConicalDecodedCapEvidence],
) -> Result<&DyadicBound, ConicalReferenceFailure> {
    members
        .iter()
        .map(ConicalDecodedCapEvidence::ruled_side_error)
        .max()
        .ok_or_else(|| binding_failure("nonempty decoded storage inventory"))
}
fn summarize(
    family: DecodedFamily,
    limit: f64,
    options: ConeClearanceOptions,
    operations: u32,
) -> Result<ConicalReferenceEvidence, ConicalDecodedFamilyError> {
    let members = &family.members;
    let bound = |points: Option<&DyadicBound>| {
        points
            .ok_or_else(|| binding_failure("nonempty decoded reference inventory"))
            .and_then(upper)
            .map_err(|cause| failed(cause, operations))
    };
    let developed = bound(
        members
            .iter()
            .map(ConicalDecodedCapEvidence::developed_error)
            .max(),
    )?;
    let filled = bound(
        members
            .iter()
            .map(|member| member.normal().filled_error())
            .max(),
    )?;
    let storage = bound(Some(
        maximum_storage(members).map_err(|cause| failed(cause, operations))?,
    ))?;
    Ok(ConicalReferenceEvidence {
        input: Vec::new(),
        lengths: Vec::new(),
        components: Vec::new(),
        spatial_limit: limit.to_bits(),
        options,
        developed,
        filled,
        storage,
        operations,
        decoded: Some(Box::new(family)),
    })
}
fn upper(value: &DyadicBound) -> Result<f64, ConicalReferenceFailure> {
    value
        .binary64_bounds()
        .map(|bounds| bounds[1])
        .ok_or_else(|| binding_failure("finite decoded reference enclosure"))
}
fn failed(cause: ConicalReferenceFailure, operations: u32) -> ConicalDecodedFamilyError {
    ConicalDecodedFamilyError {
        member: None,
        operations,
        cause: ConicalDecodedFamilyFailure::Family(cause),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_radii_leave_the_meridian_scale_at_one() {
        let mut never = || false;
        let mut arithmetic = Arithmetic::new(sector_api::policy(Default::default()), &mut never);
        let [gap, radius, storage] = [0.25, 8., 0.03125].map(|v| DyadicBound::from_f64(v).unwrap());
        let bound = world_separation(&gap, &radius, &storage, &mut arithmetic).unwrap();
        assert_eq!(bound.binary64_bounds(), Some([0.1875; 2]));
    }

    #[test]
    fn thin_radii_and_both_storage_errors_can_close_the_gap() {
        let mut never = || false;
        let mut arithmetic = Arithmetic::new(sector_api::policy(Default::default()), &mut never);
        let [gap, radius, storage] =
            [0.25, 0.5, 0.03125].map(|v| DyadicBound::from_f64(v).unwrap());
        assert!(matches!(
            world_separation(&gap, &radius, &storage, &mut arithmetic),
            Err(ConicalReferenceFailure::Bound {
                field: "positive decoded cone solid separation",
                actual: 0.,
                ..
            })
        ));
    }
}
