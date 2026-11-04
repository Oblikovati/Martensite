//! Only the source-consumed ranked state rows; real owners and child entities supply types.
use super::*;
use martensite_tkv3d::{
    select_mgr::SelectMgrViewerSelectorResultMembers as _,
    select3d::{Select3DSensitiveEntityMembers as _, Select3DSensitiveWire},
    std_select::{StdSelectBRepOwner, StdSelectBRepOwnerMembers as _},
};

/// Owned scalar snapshot releases selector guards before callbacks; `let copy = hit;`.
#[derive(Clone, Copy)]
pub(super) struct FrustumCriterionText {
    pub depth: f64,
    pub distance: f64,
    pub point: [f64; 3],
    pub normal: [f32; 3],
}

/// Retain actual operand identities for source names; `frustum_bindings(session, &["b", "bz"])?`.
pub(super) fn frustum_bindings(
    session: &ViewerSession,
    names: &[&str],
) -> Result<FrustumBindings, Failure> {
    names
        .iter()
        .map(|name| Ok(((*name).to_owned(), frustum_operand(session, name)?)))
        .collect()
}

fn frustum_criterion(session: &ViewerSession, rank: i32) -> Result<FrustumCriterionText, Failure> {
    let selector = session.context().main_selector();
    let hit = selector.picked_data(rank)?;
    Ok(FrustumCriterionText {
        depth: hit.depth,
        distance: hit.min_dist,
        point: [hit.point.x(), hit.point.y(), hit.point.z()],
        normal: [hit.normal.x, hit.normal.y, hit.normal.z],
    })
}

/// Reuse the released C-locale six-digit formatter; `frustum_xyz_text([0.; 3])`.
pub(super) fn frustum_xyz_text(point: [f64; 3]) -> String {
    let words = point.map(|value| format_general(value, 6));
    words.join(" ")
}

/// Render the native scalar/conditional-normal row fields; `frustum_criterion_text(hit)`.
pub(super) fn frustum_criterion_text(hit: FrustumCriterionText) -> String {
    let mut text = format!(
        " Depth: {} Distance: {} Point: {}",
        format_general(hit.depth, 6),
        format_general(hit.distance, 6),
        frustum_xyz_text(hit.point),
    );
    if hit.normal.iter().map(|value| value * value).sum::<f32>() > f32::EPSILON {
        text.push_str(&format!(
            " Normal: {}",
            frustum_xyz_text(hit.normal.map(f64::from))
        ));
    }
    text
}

/// Join one real ranked entity to its actual named owner; `frustum_entity_row(session, bindings, 1)?`.
pub(super) fn frustum_entity_row(
    session: &ViewerSession,
    bindings: &FrustumBindings,
    rank: i32,
) -> Result<String, Failure> {
    // PickedData's borrowed guard ends before virtual entity/owner/type access.
    let hit = frustum_criterion(session, rank)?;
    let selector = session.context().main_selector();
    let entity = selector.picked_entity(rank)?;
    let object = entity
        .owner_id()
        .selectable()
        .cast::<dyn AisInteractiveObject>();
    let name = frustum_bound_name(&object, bindings)?;
    let kind = frustum_runtime_name(entity.dynamic_type(), "ranked entity")?;
    Ok(format!(
        "  {name:<20}{} ({kind})",
        frustum_criterion_text(hit)
    ))
}

/// Preserve real shape/child row offsets for the wire comparison; `frustum_wire_rows(session, bindings)?`.
pub(super) fn frustum_wire_rows(
    session: &ViewerSession,
    bindings: &FrustumBindings,
) -> Result<Vec<String>, Failure> {
    let selector = session.context().main_selector();
    let mut rows = vec!["Detected entities:".to_owned()];
    for rank in 1..=selector.nb_picked() {
        rows.push(frustum_entity_row(session, bindings, rank)?);
        let entity = selector.picked_entity(rank)?;
        rows.extend(frustum_brep_shape_row(&entity.owner_id())?);
        if let Some(wire) = entity.cast::<Select3DSensitiveWire>().get() {
            rows.push(frustum_wire_child_row(wire)?);
        }
    }
    Ok(rows)
}

/// A missing actual descriptor is a refusal, never an expected type name; `frustum_runtime_name(None, "entity")?`.
pub(super) fn frustum_runtime_name(
    actual: Option<&'static martensite_tkernel::standard::RuntimeType>,
    operand: &str,
) -> Result<String, Failure> {
    let actual = actual.ok_or_else(|| {
        program_error(&format!(
            "{operand} runtime type=None; expected an actual registered pointee descriptor",
        ))
    })?;
    Ok(actual.name().to_owned())
}

fn frustum_brep_shape_row(
    owner: &Handle<dyn martensite_tkv3d::select_mgr::SelectMgrEntityOwnerSource>,
) -> Result<Option<String>, Failure> {
    let owner = owner.cast::<StdSelectBRepOwner>();
    let Some(owner) = owner.get() else {
        return Ok(None);
    };
    let shape = owner.shape_ref().clone_shape();
    let kind = frustum_runtime_name(shape.tshape().dynamic_type(), "ranked shape")?;
    Ok(Some(format!(
        "                       Detected Shape: {kind}"
    )))
}

fn frustum_wire_child_row(wire: &Select3DSensitiveWire) -> Result<String, Failure> {
    let child = wire.get_last_detected();
    let child = child.get().ok_or_else(|| {
        program_error("ranked wire child=null; expected its actual last detected sensitive entity")
    })?;
    Ok(format!(
        "                       Detected Child: {}",
        child.dynamic_type().name()
    ))
}

/// Consume the source Point tokens without expected-coordinate fallback; `frustum_point_from_row("Point: 1 2 3")?`.
pub(super) fn frustum_point_from_row(row: &str) -> Result<[f64; 3], Failure> {
    let words: Vec<&str> = row.split_whitespace().collect();
    let at = words
        .iter()
        .position(|word| *word == "Point:")
        .ok_or_else(|| program_error(&format!("ranked row {row:?}; expected Point: x y z")))?;
    let coordinates = words.get(at + 1..at + 4).ok_or_else(|| {
        program_error(&format!(
            "ranked row {row:?}; expected three Point coordinates"
        ))
    })?;
    let mut point = [0.; 3];
    for (target, value) in point.iter_mut().zip(coordinates) {
        *target = value.parse::<f64>().map_err(|_| {
            program_error(&format!("ranked coordinate {value:?}; expected a %g real"))
        })?;
    }
    Ok(point)
}
