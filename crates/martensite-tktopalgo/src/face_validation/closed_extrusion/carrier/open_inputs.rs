//! Input extraction uses the same declared cap and exact world-vector gate as face validation.
use super::*;
use crate::face_validation::closed_extrusion::open_support_inputs::{
    OpenSupportInputFailure, OpenSupportInputRole,
};
use martensite_tkg3d::geom::StoredOpenExtrusionInput;

pub(in crate::face_validation::closed_extrusion) fn capture<F: FnMut() -> bool>(
    strip: &Strip,
    boundary: &Boundary,
    budget: &mut Budget<'_, F>,
) -> Result<(StoredOpenExtrusionInput, usize, usize), OpenSupportInputFailure> {
    let mut carrier = reference(strip, boundary).map_err(OpenSupportInputFailure::Face)?;
    carrier.direction = direction::select(&carrier, 3).map_err(OpenSupportInputFailure::Face)?;
    relations::direction(&carrier, budget).map_err(OpenSupportInputFailure::Face)?;
    let input = admitted(&carrier)?;
    Ok((input, carrier.edge.owner, carrier.index))
}

fn admitted(carrier: &Carrier<'_>) -> Result<StoredOpenExtrusionInput, OpenSupportInputFailure> {
    carrier
        .curve
        .clone()
        .into_open_extrusion_input(
            carrier.direction,
            [carrier.boundary.bounds[2], carrier.boundary.bounds[3]],
        )
        .map_err(|cause| OpenSupportInputFailure::Spline {
            role: OpenSupportInputRole::BoundaryCarrier,
            owner: carrier.edge.owner,
            cause,
        })
}
