//! No user code or numerical certificate executes during simultaneous read leases.
use super::{
    analytic::{AnalyticFaceInput, CaptureMode},
    cells::Lease,
    copy::CopyContext,
    inventory::Inventory,
    types::{FaceCaptureBehavior, FaceCaptureIncomplete, FaceCaptureLimits, FaceInputSnapshot},
};
use crate::topods::Face;

/// Copy finite admitted face input without validating geometry; `capture_face_input(&face, FaceCaptureLimits::bounded(), ||false)`.
/// A successful capture supplies immutable input only; it does not validate face geometry.
pub fn capture_face_input(
    face: &Face,
    limits: FaceCaptureLimits,
    cancelled: impl FnMut() -> bool,
) -> Result<FaceInputSnapshot, FaceCaptureIncomplete> {
    capture_face_input_with_behavior(
        face,
        limits,
        FaceCaptureBehavior::StoredRepresentationsV1,
        cancelled,
    )
}

/// Select a stored-input domain explicitly; `capture_face_input_with_behavior(&face, limits, FaceCaptureBehavior::StoredSurfaceQueriesV2, || false)`.
pub fn capture_face_input_with_behavior(
    face: &Face,
    limits: FaceCaptureLimits,
    behavior: FaceCaptureBehavior,
    mut cancelled: impl FnMut() -> bool,
) -> Result<FaceInputSnapshot, FaceCaptureIncomplete> {
    if cancelled() {
        return Err(FaceCaptureIncomplete::Cancelled);
    }
    let result = captured(face, limits, behavior.into()).map(|capture| capture.common);
    if cancelled() {
        return Err(FaceCaptureIncomplete::Cancelled);
    }
    result
}

/// Compatibility entry point for extrusion-backed callers;
/// `capture_closed_extrusion_face(&face, limits, || false)`.
pub fn capture_closed_extrusion_face(
    face: &Face,
    limits: FaceCaptureLimits,
    cancelled: impl FnMut() -> bool,
) -> Result<FaceInputSnapshot, FaceCaptureIncomplete> {
    capture_face_input(face, limits, cancelled)
}
fn captured(
    face: &Face,
    limits: FaceCaptureLimits,
    behavior: CaptureMode,
) -> Result<AnalyticFaceInput, FaceCaptureIncomplete> {
    let inventory = Inventory::gather(face, limits, behavior)?;
    let cells = inventory.cells(limits)?;
    let leases = cells
        .iter()
        .map(|(owner, cell)| cell.acquire(*owner).map(|lease| (*owner, lease)))
        .collect::<Result<Vec<_>, _>>()?;
    inventory.revalidate(&leases, limits)?;
    copy_snapshot(face, &inventory, &leases, limits)
}
fn copy_snapshot(
    face: &Face,
    inventory: &Inventory,
    leases: &[(usize, Lease<'_>)],
    limits: FaceCaptureLimits,
) -> Result<AnalyticFaceInput, FaceCaptureIncomplete> {
    let mut copier = CopyContext::new(inventory, limits.entries);
    let root = copier.occurrence(face).map_err(|error| error.at_owner(0))?;
    let mut ordered: Vec<_> = leases.iter().collect();
    ordered.sort_by_key(|(owner, lease)| (*owner, super::cells::lease_order(lease)));
    let mut cells = Vec::new();
    let mut carriers = Vec::new();
    for (owner, lease) in ordered {
        let measured = copier
            .analytic_cell(lease)
            .map_err(|error| error.at_owner(*owner))?;
        if let Some(carrier) = measured {
            carriers.push((*owner, carrier));
        } else {
            let cell = copier.cell(lease).map_err(|error| error.at_owner(*owner))?;
            cells.push((*owner, cell));
        }
    }
    Ok(AnalyticFaceInput {
        common: FaceInputSnapshot { root, cells },
        carriers,
    })
}

pub(crate) fn capture_cylindrical_input(
    face: &Face,
    limits: FaceCaptureLimits,
    cancelled: impl FnMut() -> bool,
) -> Result<AnalyticFaceInput, FaceCaptureIncomplete> {
    capture_analytic(face, limits, CaptureMode::CylinderWall, cancelled)
}

pub(crate) fn capture_spherical_input(
    face: &Face,
    limits: FaceCaptureLimits,
    cancelled: impl FnMut() -> bool,
) -> Result<AnalyticFaceInput, FaceCaptureIncomplete> {
    capture_analytic(face, limits, CaptureMode::SphereBand, cancelled)
}

fn capture_analytic(
    face: &Face,
    limits: FaceCaptureLimits,
    mode: CaptureMode,
    mut cancelled: impl FnMut() -> bool,
) -> Result<AnalyticFaceInput, FaceCaptureIncomplete> {
    if cancelled() {
        return Err(FaceCaptureIncomplete::Cancelled);
    }
    let result = captured(face, limits, mode);
    if cancelled() {
        return Err(FaceCaptureIncomplete::Cancelled);
    }
    result
}
