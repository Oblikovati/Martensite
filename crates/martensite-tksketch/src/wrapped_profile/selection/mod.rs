//! Qualify every selected region against one global authored planar arrangement.
mod admission;
mod error;
mod material;
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
pub(super) fn certify(
    requests: &[Request],
    options: MaterialOptions,
) -> Result<QualifiedSelection, SelectionFailure> {
    certify_with_cancel(requests, options, || false)
}
pub(super) fn certify_with_cancel(
    requests: &[Request],
    options: MaterialOptions,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<QualifiedSelection, SelectionFailure> {
    if cancelled() {
        return Err(SelectionFailure::Material(Box::new(
            MaterialFailure::Cancelled,
        )));
    }
    let prepared = prepare(requests)?;
    let material = material::certify_with_cancel(requests, &prepared, options, &mut cancelled)
        .map_err(|cause| SelectionFailure::Material(Box::new(cause)))?;
    Ok(QualifiedSelection { prepared, material })
}

pub(super) fn prepare(requests: &[Request]) -> Result<PreparedSelection, SelectionFailure> {
    let planar = qualify(requests)?;
    let prepared = requests
        .iter()
        .map(prepare_region)
        .collect::<Result<Vec<_>, _>>()?;
    let boundary_clearance = planar_clearance::certify_many(requests, &prepared)
        .map_err(|cause| SelectionFailure::Clearance(Box::new(cause)))?;
    Ok(PreparedSelection {
        planar,
        prepared,
        boundary_clearance,
    })
}
fn prepare_region(request: &Request) -> Result<Request, SelectionFailure> {
    subdivision::prepare(request).map_err(|cause| SelectionFailure::Planning {
        region: request.region,
        cause: Box::new(cause),
    })
}

pub(super) fn qualify(requests: &[Request]) -> Result<PlanarSelectionEvidence, SelectionFailure> {
    let tolerance = admission::admit(requests)?;
    let captured = planar::capture(requests, tolerance)
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
    })
}
