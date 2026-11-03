//! No user code or numerical certificate executes during simultaneous read leases.
use super::{cells::Lease, copy::CopyContext, inventory::Inventory, types::*};
use crate::topods::Face;

/// Copy one finite admitted face input; `capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), ||false)`.
/// A successful capture supplies immutable input only; it does not validate face geometry.
pub fn capture_closed_extrusion_face(
    face: &Face,
    limits: FaceCaptureLimits,
    mut cancelled: impl FnMut() -> bool,
) -> Result<FaceInputSnapshot, FaceCaptureIncomplete> {
    if cancelled() {
        return Err(FaceCaptureIncomplete::Cancelled);
    }
    let result = captured(face, limits);
    if cancelled() {
        return Err(FaceCaptureIncomplete::Cancelled);
    }
    result
}
fn captured(
    face: &Face,
    limits: FaceCaptureLimits,
) -> Result<FaceInputSnapshot, FaceCaptureIncomplete> {
    let inventory = Inventory::gather(face, limits)?;
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
) -> Result<FaceInputSnapshot, FaceCaptureIncomplete> {
    let mut copier = CopyContext::new(inventory, limits.entries);
    let root = copier.occurrence(face).map_err(|error| error.at_owner(0))?;
    let mut ordered: Vec<_> = leases.iter().collect();
    ordered.sort_by_key(|(owner, lease)| (*owner, super::cells::lease_order(lease)));
    let cells = copied_cells(&mut copier, ordered)?;
    Ok(FaceInputSnapshot { root, cells })
}

fn copied_cells(
    copier: &mut CopyContext,
    ordered: Vec<&(usize, Lease<'_>)>,
) -> Result<Vec<(usize, CapturedFaceCell)>, FaceCaptureIncomplete> {
    ordered
        .into_iter()
        .map(|(owner, lease)| {
            copier
                .cell(lease)
                .map(|cell| (*owner, cell))
                .map_err(|error| error.at_owner(*owner))
        })
        .collect()
}
