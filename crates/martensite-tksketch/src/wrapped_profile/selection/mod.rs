//! Qualify every selected region against one global authored planar arrangement.
mod admission;
mod error;
pub(super) mod material;
pub(super) use material::{MaterialFailure, MaterialOptions, PeriodicMaterialEvidence};
#[cfg(test)]
mod material_tests;
#[cfg(test)]
mod tests;
use super::*;
pub(super) use error::SelectionFailure;
use martensite_tkbo::profile_extrusion::{
    PlanarIntegralEvidence, PlanarProofVersion, PlanarProofWorkLimit, PlanarProofWorkReceipt,
    RegionCertificate,
};
use std::collections::BTreeMap;

pub(super) struct PlanarSelectionEvidence {
    pub members: BTreeMap<SurfaceRegionId, planar::PlanarEvidence>,
    pub certificate: RegionCertificate,
    pub proof_work: PlanarProofWorkReceipt,
}

impl PlanarSelectionEvidence {
    // A graph-wide integral receipt names the complete family, even when stored on a member.
    pub(super) fn integral_mismatch(&self, requests: &[Request]) -> Option<SurfaceRegionId> {
        let (&region, first) = self.members.first_key_value()?;
        let issued = first.integral_program_evidence();
        for (&id, member) in &self.members {
            if !same_issuance(issued, member.integral_program_evidence()) {
                return Some(id);
            }
        }
        issued
            .is_some_and(|proof| !planar::matches_integral_profile(proof, requests))
            .then_some(region)
    }
}

fn same_issuance(
    issued: Option<&PlanarIntegralEvidence>,
    member: Option<&PlanarIntegralEvidence>,
) -> bool {
    match (issued, member) {
        (Some(proof), Some(other)) => proof.shares_issuance(other),
        (None, None) => true,
        _ => false,
    }
}

pub(super) struct PreparedSelection {
    pub planar: PlanarSelectionEvidence,
    pub prepared: Vec<Request>,
    pub boundary_clearance: planar_clearance::OriginalClearance,
}
pub(super) struct QualifiedSelection {
    pub prepared: PreparedSelection,
    pub material: PeriodicMaterialEvidence,
}
#[cfg(test)]
pub(super) fn certify(
    requests: &[Request],
    options: MaterialOptions,
) -> Result<QualifiedSelection, SelectionFailure> {
    certify_with_cancel(requests, options, || false)
}
#[cfg(test)]
pub(super) fn certify_with_cancel(
    requests: &[Request],
    options: MaterialOptions,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<QualifiedSelection, SelectionFailure> {
    certify_with_proof(
        requests,
        options,
        PlanarProofWorkLimit::CurrentDefault,
        PlanarProofVersion::CarrierV2,
        cancelled,
    )
}
pub(super) fn certify_with_proof(
    requests: &[Request],
    options: MaterialOptions,
    work: PlanarProofWorkLimit,
    version: PlanarProofVersion,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<QualifiedSelection, SelectionFailure> {
    let prepared = prepare_with_proof(requests, work, version, &mut cancelled)?;
    let material = material::certify_with_cancel(requests, &prepared, options, &mut cancelled)
        .map_err(|cause| SelectionFailure::Material(Box::new(cause)))?;
    Ok(QualifiedSelection { prepared, material })
}

#[cfg(test)]
pub(super) fn prepare(requests: &[Request]) -> Result<PreparedSelection, SelectionFailure> {
    prepare_with_cancel(requests, || false)
}
#[cfg(test)]
pub(super) fn prepare_with_cancel(
    requests: &[Request],
    cancelled: impl FnMut() -> bool + Send,
) -> Result<PreparedSelection, SelectionFailure> {
    prepare_with_proof(
        requests,
        PlanarProofWorkLimit::CurrentDefault,
        PlanarProofVersion::CarrierV2,
        cancelled,
    )
}
fn prepare_with_proof(
    requests: &[Request],
    work: PlanarProofWorkLimit,
    version: PlanarProofVersion,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<PreparedSelection, SelectionFailure> {
    if cancelled() {
        return Err(SelectionFailure::Cancelled);
    }
    let planar = qualify_with_proof(requests, work, version, &mut cancelled)?;
    let prepared = requests
        .iter()
        .map(|request| prepare_region(request, &mut cancelled))
        .collect::<Result<Vec<_>, _>>()?;
    let boundary_clearance = prepare_boundary(requests, &prepared, &mut cancelled)?;
    Ok(PreparedSelection {
        planar,
        prepared,
        boundary_clearance,
    })
}
fn prepare_boundary(
    originals: &[Request],
    prepared: &[Request],
    cancelled: &mut CancelPoll<'_>,
) -> Result<planar_clearance::OriginalClearance, SelectionFailure> {
    planar_clearance::certify_many_with_cancel(originals, prepared, cancelled)
        .map_err(|cause| SelectionFailure::Clearance(Box::new(cause)))
}

fn prepare_region(
    request: &Request,
    cancelled: &mut CancelPoll<'_>,
) -> Result<Request, SelectionFailure> {
    subdivision::prepare_with_cancel(request, cancelled).map_err(|cause| {
        SelectionFailure::Planning {
            region: request.region,
            cause: Box::new(cause),
        }
    })
}

#[cfg(test)]
pub(super) fn qualify(requests: &[Request]) -> Result<PlanarSelectionEvidence, SelectionFailure> {
    qualify_with_proof(
        requests,
        PlanarProofWorkLimit::CurrentDefault,
        PlanarProofVersion::CarrierV2,
        &mut || false,
    )
}
fn qualify_with_proof(
    requests: &[Request],
    work: PlanarProofWorkLimit,
    version: PlanarProofVersion,
    cancelled: &mut CancelPoll<'_>,
) -> Result<PlanarSelectionEvidence, SelectionFailure> {
    let tolerance = admission::admit(requests)?;
    let captured =
        planar::capture_with_proof_and_cancel(requests, tolerance, work, version, cancelled)
            .map_err(|cause| SelectionFailure::Capture(Box::new(cause)))?;
    let members = planar::select_members(requests, &captured).map_err(|(region, cause)| {
        SelectionFailure::Region {
            region,
            cause: Box::new(cause),
        }
    })?;
    Ok(PlanarSelectionEvidence {
        members,
        certificate: captured.certificate(),
        proof_work: captured.proof_work(),
    })
}
