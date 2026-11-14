//! Account for every retained curve representation on both owning faces.
use super::*;
use martensite_tkbrep::{
    brep::{BRepCurveRepresentation, BRepTool, access::edge_tshape},
    topods,
};
use martensite_tkernel::standard::Handle;

pub(super) fn check(registry: &Registry<'_>, incidences: &edges::EdgeFaces) -> Checked<()> {
    for (owner, faces) in incidences {
        edge(*owner, faces, registry)?;
    }
    Ok(())
}
fn edge(owner: BindingKey, faces: &[BoundUse], registry: &Registry<'_>) -> Checked<()> {
    let representations = inventory(owner, registry.shape(owner)?)?;
    let mut space_curves = 0;
    let mut pcurves = Vec::new();
    for handle in &representations {
        let rep = handle
            .get()
            .ok_or_else(|| mismatch(owner, "null representation", vec![]))?;
        capture(owner, rep, faces, registry, &mut space_curves, &mut pcurves)?;
    }
    count(owner, "edge space curves", space_curves, 1)?;
    members::compare(owner, &pcurves, faces)
}
fn inventory(
    owner: BindingKey,
    shape: &Shape,
) -> Checked<Vec<Handle<dyn BRepCurveRepresentation>>> {
    let edge = native(owner, topods::edge(shape))?;
    let stored = native(owner, edge_tshape(&edge, "wrapped coverage"))?;
    let representations = stored.curves();
    count(
        owner,
        "edge curve representations",
        representations.len(),
        3,
    )?;
    Ok(representations)
}

fn capture(
    owner: BindingKey,
    rep: &dyn BRepCurveRepresentation,
    faces: &[BoundUse],
    registry: &Registry<'_>,
    space_curves: &mut usize,
    pcurves: &mut Vec<BoundUse>,
) -> Checked<()> {
    if rep.is_curve3d() && rep.location().is_identity() {
        *space_curves += 1;
        return Ok(());
    }
    let matched = matching_faces(owner, rep, faces, registry)?;
    if !rep.is_curve_on_surface() || rep.is_curve_on_closed_surface() || matched.len() != 1 {
        let keys = matched.iter().map(|u| u.key).collect();
        return Err(mismatch(owner, rep.dynamic_type().name(), keys));
    }
    pcurves.extend(matched);
    Ok(())
}
fn matching_faces(
    owner: BindingKey,
    rep: &dyn BRepCurveRepresentation,
    faces: &[BoundUse],
    registry: &Registry<'_>,
) -> Checked<Vec<BoundUse>> {
    let mut matched = Vec::new();
    for face in faces {
        let shape = native(owner, topods::face(registry.shape(face.key)?))?;
        let (surface, location) = native(owner, BRepTool::surface_with_location(&shape))?;
        if rep.is_curve_on_surface_at(&surface, &location) {
            matched.push(*face);
        }
    }
    Ok(matched)
}
fn mismatch(owner: BindingKey, actual: &str, matched_faces: Vec<BindingKey>) -> CoverageFailure {
    fail(
        owner,
        CoverageCause::Representation {
            actual: actual.into(),
            matched_faces,
        },
    )
}
