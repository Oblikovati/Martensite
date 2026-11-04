//! The selected surface is joined through the captured root face, never through equal poles.
use super::{
    domain, layout,
    types::{FaceQueryIncomplete as Cause, SurfaceSelfQuery, binding},
};
use martensite_tkbrep::brep::face_input_capture::{
    CapturedFaceCell as Cell, CapturedLocation, FaceInputSnapshot,
};
use martensite_tkg3d::geom::StoredSplineSurfaceInput;
pub(super) fn bind(
    snapshot: &FaceInputSnapshot,
    request: SurfaceSelfQuery,
) -> Result<(usize, &StoredSplineSurfaceInput), Cause> {
    let root = snapshot.root();
    identity(&root.location, root.owner)?;
    let (surface, location) = root_support(snapshot)?;
    identity(location, root.owner)?;
    let input = selected_surface(snapshot, surface)?;
    layout::validate(input, surface)?;
    domain::validate(input, request, surface)?;
    Ok((surface, input))
}
fn selected_surface(
    snapshot: &FaceInputSnapshot,
    surface: usize,
) -> Result<&StoredSplineSurfaceInput, Cause> {
    let selected = exactly_one(
        snapshot,
        surface,
        |cell| matches!(cell, Cell::SplineSurface(_)),
        "stored tensor support",
    )?;
    let Cell::SplineSurface(input) = selected else {
        unreachable!("selected tensor variant")
    };
    Ok(input)
}
fn root_support(snapshot: &FaceInputSnapshot) -> Result<(usize, &CapturedLocation), Cause> {
    let owner = snapshot.root().owner;
    let selected = exactly_one(
        snapshot,
        owner,
        |cell| matches!(cell, Cell::Face { .. }),
        "actual root face fields",
    )?;
    let Cell::Face {
        surface, location, ..
    } = selected
    else {
        unreachable!("selected face variant")
    };
    Ok((*surface, location))
}
fn exactly_one<'a>(
    snapshot: &'a FaceInputSnapshot,
    owner: usize,
    predicate: impl Fn(&Cell) -> bool,
    relation: &'static str,
) -> Result<&'a Cell, Cause> {
    let mut matches = snapshot
        .cells()
        .iter()
        .filter(|(id, cell)| *id == owner && predicate(cell));
    let first = matches
        .next()
        .map(|(_, cell)| cell)
        .ok_or_else(|| binding(owner, relation, "missing cell"))?;
    if matches.next().is_some() {
        return Err(binding(owner, relation, "duplicate cells"));
    }
    Ok(first)
}
fn identity(location: &CapturedLocation, owner: usize) -> Result<(), Cause> {
    if !location.steps().is_empty()
        || location.effective_transform() != martensite_tkmath::gp::Transform3::default()
    {
        return Err(binding(
            owner,
            "untransformed support coefficients and root placement",
            location,
        ));
    }
    Ok(())
}
