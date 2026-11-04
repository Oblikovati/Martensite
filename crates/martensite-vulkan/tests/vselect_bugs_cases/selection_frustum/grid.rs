//! Real point grids and the source's deliberately prefix-only selected-name comparisons.
use super::*;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// Retain each real source point under its numeric name; `display_frustum_grid(session)?`.
pub(super) fn display_frustum_grid(
    session: &mut ViewerSession,
) -> Result<FrustumBindings, Failure> {
    let mut bindings = Vec::new();
    for z in [10., 0.] {
        for x in 0..7 {
            for y in 0..7 {
                let name = bindings.len().to_string();
                let object = point_object([f64::from(x), f64::from(y), z])?;
                session.display_object(&name, object.clone())?;
                bindings.push((name, object));
            }
        }
    }
    Ok(bindings)
}

/// Preserve the top/perspective/fit/left/top sequence; `frustum_region_camera(session)?`.
pub(super) fn frustum_region_camera(session: &mut ViewerSession) -> Result<(), Failure> {
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    session.set_projection(Graphic3dCameraProjection::Perspective)?;
    session.fit()?;
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_LEFT)?;
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)
}

/// Fit the already-perspective front-point grid from above; `frustum_front_camera(session)?`.
pub(super) fn frustum_front_camera(session: &mut ViewerSession) -> Result<(), Failure> {
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    session.fit()?;
    Ok(())
}

/// Read global owners in the context cursor order; `frustum_selected_names(session, bindings)?`.
pub(super) fn frustum_selected_names(
    session: &ViewerSession,
    bindings: &FrustumBindings,
) -> Result<Vec<String>, Failure> {
    let context = session.context();
    let mut names = Vec::new();
    context.init_selected();
    while context.more_selected() {
        let object = context.selected_interactive()?;
        if context.selected_owner()? == object.global_sel_owner() {
            names.push(frustum_bound_name(&object, bindings)?);
        }
        context.next_selected();
    }
    Ok(names)
}

/// Cut names before source lexical sorting, ignoring the tail; `frustum_prefix_matches(names, &["24"])`.
pub(super) fn frustum_prefix_matches(actual: &[String], expected: &[&str]) -> bool {
    // The case cuts the state rows before sorting; trailing rows are untested.
    let mut prefix: Vec<&str> = actual
        .iter()
        .take(expected.len())
        .map(String::as_str)
        .collect();
    prefix.resize(expected.len(), "");
    let mut reference = expected.to_vec();
    prefix.sort_unstable();
    reference.sort_unstable();
    prefix == reference
}
