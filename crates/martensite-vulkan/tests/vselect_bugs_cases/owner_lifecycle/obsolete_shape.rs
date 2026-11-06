//! OCC27523 refreshes a dormant global owner after replacing its topology.
use super::*;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
use martensite_tkv3d::ais::{AisInteractiveObjectMembers as _, AisShapeParameters as _};
use martensite_tkv3d::std_select::{StdSelectBRepOwner, StdSelectBRepOwnerMembers as _};

pub(super) fn case_edge() -> Result<Shape, Failure> {
    let start = Point3::new(100., 100., 100.);
    let end = Point3::new(300., 400., 600.);
    Ok(BRepBuilderAPIMakeEdge::from_points(start, end)?
        .edge()?
        .into_shape())
}

pub(super) fn replacement_vertex() -> Result<Shape, Failure> {
    let point = Point3::new(75., 0., 0.);
    let vertex = BRepBuilderAPIMakeVertex::new(point)?.vertex()?;
    Ok(vertex.into_shape())
}

fn activate_vertex_after_global(
    context: &AisInteractiveContext,
    object: &Handle<dyn AisInteractiveObject>,
) -> Result<(), Failure> {
    context.deactivate(object)?;
    context.load(object, -1)?;
    context.activate(object, 0, false)?;
    context.deactivate_mode(object, 0)?;
    context.activate(object, 1, false)
}

pub(super) fn display_edge_for_vertex_mode(
    context: &AisInteractiveContext,
    edge: &Shape,
) -> Result<Handle<AisShape>, Failure> {
    let object = AisShape::new(edge);
    context.display(&object.cast(), false)?;
    activate_vertex_after_global(context, &object.cast())?;
    Ok(object)
}

pub(super) fn replace_with_vertex(object: &Handle<AisShape>) -> Result<Shape, Failure> {
    // The native vertex is constructed only after display and selection-mode changes.
    let vertex = replacement_vertex()?;
    object.set(&vertex);
    // AisInteractiveObject::Redisplay defaults AllModes=false and viewer update=false.
    object.redisplay(false)?;
    Ok(vertex)
}

fn owner_shape_kind(
    owner: &Handle<dyn martensite_tkv3d::select_mgr::SelectMgrEntityOwnerSource>,
) -> Result<Option<ShapeEnum>, Failure> {
    let brep = owner.cast::<StdSelectBRepOwner>();
    let Some(brep) = brep.get() else {
        return Ok(None);
    };
    if !brep.has_shape() {
        return Ok(None);
    }
    Ok(Some(brep.shape_ref().clone_shape().shape_type()?))
}

pub(super) fn last_selected_kind_matches(
    context: &AisInteractiveContext,
    expected: ShapeEnum,
) -> Result<bool, Failure> {
    let mut valid = false;
    context.init_selected();
    while context.more_selected() {
        // Preserve RegressionCommands's assignment: the last eligible owner decides, not any/all.
        if let Some(kind) = owner_shape_kind(&context.selected_owner()?)? {
            valid = kind == expected;
        }
        context.next_selected();
    }
    Ok(valid)
}

fn recompute_obsolete_owner(context: &AisInteractiveContext) -> Outcome {
    let edge = case_edge()?;
    let object = display_edge_for_vertex_mode(context, &edge)?;
    let vertex = replace_with_vertex(&object)?;
    context.add_or_remove_selected_object(&object.cast(), true)?;
    if !last_selected_kind_matches(context, vertex.shape_type()?)? {
        eprintln!("Error: shape type is invalid.");
        return Err(program_error("selected vertex-owner predicate=false; expected true").into());
    }
    Ok(())
}

pub(crate) fn obsolete_shape_owner() -> Outcome {
    let mut run = lifecycle_begin(
        "============",
        "OCC27523: Visualization - selection owner contains obsolete shape",
    )?;
    recompute_obsolete_owner(run.session().context())?;
    run.dump("bug27523.png")?;
    run.finish()
}
