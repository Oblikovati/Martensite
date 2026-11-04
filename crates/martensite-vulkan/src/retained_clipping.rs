//! GlStructure::Render clipping policy: world anchors,
//! local overrides, whole-structure bounds, and immutable fragment-plane chains.
use crate::{RenderClipping, RenderError, error::rejected};
use martensite_tkernel::{ncollection::Vec4, standard::Handle};
use martensite_tkservice::graphic3d::*;

type ClipPlanes = Vec<Handle<Graphic3dClipPlane>>;

pub(crate) fn structure_is_clipped(
    global: &Handle<Graphic3dSequenceOfHClipPlane>,
    structure: &Graphic3dCStructureFields,
) -> Result<bool, RenderError> {
    if !overrides_global(structure) && anchor_is_clipped(global, structure)? {
        return Ok(true);
    }
    if !structure.trsf_pers.is_null()
        || !structure.bnd_box.is_valid()
        || !structure.bnd_box_clip_check
    {
        return Ok(false);
    }
    Ok(effective_planes(global, structure)?
        .iter()
        .any(|plane| plane.probe_box(&structure.bnd_box) == Graphic3dClipState::Out))
}
fn anchor_is_clipped(
    global: &Handle<Graphic3dSequenceOfHClipPlane>,
    structure: &Graphic3dCStructureFields,
) -> Result<bool, RenderError> {
    let Some(persistence) = structure
        .trsf_pers
        .get()
        .filter(|pers| pers.is_zoom_or_rotate())
    else {
        return Ok(false);
    };
    let anchor = persistence
        .anchor_point()
        .map_err(|error| rejected(error, "a world-space persistence anchor"))?;
    let point = Vec4::new(anchor.x(), anchor.y(), anchor.z(), 1.);
    Ok(enabled_planes(global)?
        .iter()
        .any(|plane| plane.probe_point(&point) == Graphic3dClipState::Out))
}
/// `local_origin` is the Z layer's origin the planes are taken relative to, if any.
pub(crate) fn structure_clipping(
    global: &Handle<Graphic3dSequenceOfHClipPlane>,
    structure: &Graphic3dCStructureFields,
    model: [f32; 16],
    local_origin: Option<[f64; 3]>,
) -> Result<Option<RenderClipping>, RenderError> {
    let chains = structure_planes(global, structure)?
        .into_iter()
        .map(|plane| snapshot_chain_around(plane, local_origin))
        .collect::<Result<Vec<_>, _>>()?;
    if chains.is_empty() {
        return Ok(None);
    }
    RenderClipping::new(chains, model).map(Some)
}
fn overrides_global(structure: &Graphic3dCStructureFields) -> bool {
    structure
        .clip_planes
        .get()
        .is_some_and(|planes| planes.to_override_global())
}
fn effective_planes(
    global: &Handle<Graphic3dSequenceOfHClipPlane>,
    structure: &Graphic3dCStructureFields,
) -> Result<ClipPlanes, RenderError> {
    // Native disables all global fragment planes for persistent structures, even
    // when zoom/rotate anchor classification kept the complete structure visible.
    let mut planes = if overrides_global(structure) || !structure.trsf_pers.is_null() {
        Vec::new()
    } else {
        enabled_planes(global)?
    };
    planes.extend(enabled_planes(&structure.clip_planes)?);
    Ok(planes)
}
fn enabled_planes(
    sequence: &Handle<Graphic3dSequenceOfHClipPlane>,
) -> Result<ClipPlanes, RenderError> {
    let Some(sequence) = sequence.get() else {
        return Ok(Vec::new());
    };
    let mut planes = Vec::new();
    for plane in sequence.items() {
        let head = plane
            .get()
            .ok_or_else(|| rejected("null clip plane", "a non-null sequence member"))?;
        if head.is_on() {
            planes.push(plane);
        }
    }
    Ok(planes)
}
pub(crate) fn snapshot_chain(
    cursor: Handle<Graphic3dClipPlane>,
) -> Result<Vec<[f32; 4]>, RenderError> {
    snapshot_chain_around(cursor, None)
}

/// The chain's equations; with a layer origin, each plane's W is recomputed from its location
/// relative to the origin in double before the float cast, as GlShaderManager's
/// pushClippingState does (`LocalClippingPlaneW`).
pub(crate) fn snapshot_chain_around(
    mut cursor: Handle<Graphic3dClipPlane>,
    origin: Option<[f64; 3]>,
) -> Result<Vec<[f32; 4]>, RenderError> {
    let mut equations = Vec::new();
    while let Some(plane) = cursor.get() {
        let mut equation = plane.get_equation().data();
        if let Some(origin) = origin {
            equation[3] = local_plane_w(plane, equation, origin);
        }
        equations.push(equation.map(|value| value as f32));
        cursor = plane.chain_next_plane();
    }
    Ok(equations)
}

/// `-(n . (location - origin))` of the plane's own location; `local_plane_w(&p, eq, [0.; 3])`.
pub(crate) fn local_plane_w(
    plane: &Graphic3dClipPlane,
    equation: [f64; 4],
    origin: [f64; 3],
) -> f64 {
    let location = plane.to_plane().location();
    let local = [
        location.x() - origin[0],
        location.y() - origin[1],
        location.z() - origin[2],
    ];
    -(equation[0] * local[0] + equation[1] * local[1] + equation[2] * local[2])
}

pub(crate) fn structure_planes(
    global: &Handle<Graphic3dSequenceOfHClipPlane>,
    structure: &Graphic3dCStructureFields,
) -> Result<ClipPlanes, RenderError> {
    Ok(effective_planes(global, structure)?
        .into_iter()
        .filter(|plane| {
            !structure.trsf_pers.is_null()
                || !structure.bnd_box.is_valid()
                || plane.probe_box(&structure.bnd_box) != Graphic3dClipState::In
        })
        .collect())
}

#[cfg(test)]
#[path = "retained_clipping_tests.rs"]
mod tests;
