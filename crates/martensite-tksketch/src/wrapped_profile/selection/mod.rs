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
use martensite_tkbo::profile_extrusion::RegionCertificate;
use std::collections::BTreeMap;

pub(super) struct PlanarSelectionEvidence {
    pub members: BTreeMap<SurfaceRegionId, planar::PlanarEvidence>,
    pub certificate: RegionCertificate,
    pub proof_work: martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
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
    certify_with_work(
        requests,
        options,
        martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault,
        cancelled,
    )
}
pub(super) fn certify_with_work(
    requests: &[Request],
    options: MaterialOptions,
    work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<QualifiedSelection, SelectionFailure> {
    let prepared = prepare_with_work(requests, work, &mut cancelled)?;
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
    prepare_with_work(
        requests,
        martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault,
        cancelled,
    )
}
fn prepare_with_work(
    requests: &[Request],
    work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<PreparedSelection, SelectionFailure> {
    if cancelled() {
        return Err(SelectionFailure::Cancelled);
    }
    let planar = qualify_with_work(requests, work)?;
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
    qualify_with_work(
        requests,
        martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault,
    )
}
fn qualify_with_work(
    requests: &[Request],
    work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
) -> Result<PlanarSelectionEvidence, SelectionFailure> {
    let tolerance = admission::admit(requests)?;
    let captured = planar::capture_with_work(requests, tolerance, work)
        .map_err(|cause| SelectionFailure::Capture(Box::new(cause)))?;
    let mut members = BTreeMap::new();
    for request in requests {
        let evidence =
            planar::select(request, &captured).map_err(|cause| SelectionFailure::Region {
                region: request.region,
                cause: Box::new(cause),
            })?;
        members.insert(request.region, evidence);
    }
    Ok(PlanarSelectionEvidence {
        members,
        certificate: captured.certificate(),
        proof_work: captured.proof_work(),
    })
}
