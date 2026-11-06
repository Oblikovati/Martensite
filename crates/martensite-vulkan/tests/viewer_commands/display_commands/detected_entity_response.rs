//! A selector response reads owned pick values and resolves names through actual object identity.
use martensite_tkernel::{
    ncollection::Vec3,
    standard::{Failure, Handle},
    storage::format_general,
};
use martensite_tkmath::gp::Point3;
use martensite_tkv3d::{
    ais::AisInteractiveObject,
    select_mgr::{
        SelectMgrEntityOwnerSource, SelectMgrViewerSelectorResultMembers,
        SelectMgrViewerSelectorSource,
    },
    select3d::{
        Select3DSensitiveEntity, Select3DSensitiveEntityMembers, Select3DSensitivePrimitiveArray,
        Select3DSensitiveTriangulation, Select3DSensitiveWire,
    },
    std_select::{StdSelectBRepOwnerMembers, StdSelectBRepOwnerSource},
};

struct RankedPick {
    entity: Handle<dyn Select3DSensitiveEntity>,
    depth: f64,
    min_dist: f64,
    point: Point3,
    normal: Vec3<f32>,
}

/// Read the complete detected-entity response without another pick or redraw.
/// Example: `detected_entity_response(selector.get(), &|object| names.get(object).cloned().unwrap_or_default())?`.
pub(crate) fn detected_entity_response(
    selector: Option<&dyn SelectMgrViewerSelectorSource>,
    name: &impl Fn(&Handle<dyn AisInteractiveObject>) -> String,
) -> Result<String, Failure> {
    let mut response = String::from("Detected entities:\n");
    let Some(selector) = selector else {
        return Ok(response);
    };
    for rank in 1..=selector.nb_picked() {
        response.push_str(&entity_row(&ranked_pick(selector, rank)?, name));
    }
    Ok(response)
}

fn ranked_pick(
    selector: &dyn SelectMgrViewerSelectorSource,
    rank: i32,
) -> Result<RankedPick, Failure> {
    let (depth, min_dist, point, normal) = {
        let data = selector.picked_data(rank)?;
        (data.depth, data.min_dist, data.point, data.normal)
    };
    let entity = (*selector.picked_entity(rank)?).clone();
    Ok(RankedPick {
        entity,
        depth,
        min_dist,
        point,
        normal,
    })
}

fn entity_row(
    pick: &RankedPick,
    name: &impl Fn(&Handle<dyn AisInteractiveObject>) -> String,
) -> String {
    let Some(entity) = pick.entity.get() else {
        return String::new();
    };
    let owner = entity.owner_id();
    let object = owner_object(&owner);
    let mut row = format!(
        "  {:<20}{} ({})\n",
        name(&object),
        pick_metrics(pick),
        entity.dynamic_type().name()
    );
    row.push_str(&shape_row(&owner));
    row.push_str(&subtype_row(&pick.entity));
    row
}

fn owner_object(
    owner: &Handle<dyn SelectMgrEntityOwnerSource>,
) -> Handle<dyn AisInteractiveObject> {
    owner
        .get()
        .map(|value| value.selectable().cast())
        .unwrap_or_default()
}

fn pick_metrics(pick: &RankedPick) -> String {
    let g = |value| format_general(value, 6);
    let mut text = format!(
        " Depth: {} Distance: {} Point: {} {} {}",
        g(pick.depth),
        g(pick.min_dist),
        g(pick.point.x()),
        g(pick.point.y()),
        g(pick.point.z())
    );
    text.push_str(&normal_metrics(pick.normal));
    text
}

fn normal_metrics(normal: Vec3<f32>) -> String {
    let g = |value| format_general(value, 6);
    let mut text = String::new();
    let square = normal.x() * normal.x() + normal.y() * normal.y() + normal.z() * normal.z();
    if square > f32::EPSILON {
        text.push_str(&format!(
            " Normal: {} {} {}",
            g(f64::from(normal.x())),
            g(f64::from(normal.y())),
            g(f64::from(normal.z()))
        ));
    }
    text
}

fn shape_row(owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> String {
    if let Some(owner) = owner.cast::<dyn StdSelectBRepOwnerSource>().get()
        && let Some(shape) = owner.shape_ref().clone_shape().tshape().get()
    {
        return format!(
            "                       Detected Shape: {}\n",
            shape.dynamic_type().name()
        );
    }
    String::new()
}

fn subtype_row(entity: &Handle<dyn Select3DSensitiveEntity>) -> String {
    if let Some(wire) = entity.cast::<Select3DSensitiveWire>().get() {
        let child = wire.get_last_detected();
        let name = child.get().map_or("", |value| value.dynamic_type().name());
        return format!("                       Detected Child: {name}\n");
    }
    if let Some(array) = entity.cast::<Select3DSensitivePrimitiveArray>().get() {
        return format!(
            "                       Detected Element: {}\n",
            array.last_detected_element()
        );
    }
    if let Some(triangle) = entity.cast::<Select3DSensitiveTriangulation>().get() {
        return format!(
            "                       Detected Triangle: {}\n",
            triangle.last_detected_triangle_index()
        );
    }
    String::new()
}

#[cfg(test)]
#[path = "detected_entity_response_controls.rs"]
mod controls;
