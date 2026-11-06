//! Actual DNaming first-function/attachment context and sweep-axis helpers.
use super::{naming_box_driver::function_result, naming_transform_arguments::object_argument};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Shape},
};
use martensite_tkcaf::tnaming::TNamingNamedShape;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::{
    geom::{GeomPlane, GeomRectangularTrimmedSurface},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::{TDataStdTreeNode, TDataStdUAttribute},
    tdf::{TdfAttributeState, TdfReference},
    tfunction::TFunctionFunction,
};
use martensite_tkmath::gp::Axis1;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape},
    brep_check::{BRepCheckAnalyzer, BRepCheckShell, BRepCheckStatus, BRepCheckWire},
    brep_gprop::BRepGProp,
    brep_lib::BRepLibFindSurface,
};
/// Find the first actual function sibling, skipping non-functions; `first_function(&object)?`.
pub(super) fn first_function(
    object: &Handle<TDataStdUAttribute>,
) -> Result<Handle<TFunctionFunction>, Failure> {
    let mut function: Handle<TFunctionFunction> = Handle::default();
    let mut node = first_node(object)?;
    while !node.is_null() {
        node.find_attribute_as(&TFunctionFunction::get_id(), &mut function)?;
        if !function.is_null() {
            break;
        }
        node = node.next();
    }
    Ok(function)
}
fn first_node(object: &Handle<TDataStdUAttribute>) -> Result<Handle<TDataStdTreeNode>, Failure> {
    let mut node: Handle<TDataStdTreeNode> = Handle::default();
    if object.is_null() {
        return Ok(node);
    }
    object.find_attribute_as(&TDataStdTreeNode::get_default_tree_id(), &mut node)?;
    if node.is_null() || !node.has_first() {
        return Ok(Handle::default());
    }
    Ok(node.first())
}
fn has_attachment_guid(function: &Handle<TFunctionFunction>, external: bool) -> bool {
    let guid = function.get_driver_guid();
    let internal = guid_from_text("12e94554-6dbc-11d4-b9c8-0060b0ee281b");
    let extended = guid_from_text("12e94555-6dbc-11d4-b9c8-0060b0ee281b");
    guid == internal || (external && guid == extended)
}
fn attachment_reference(
    object: &Handle<TDataStdUAttribute>,
    external: bool,
) -> Result<Handle<TdfReference>, Failure> {
    let function = first_function(object)?;
    let mut reference = Handle::default();
    if function.is_null() {
        return Ok(reference);
    }
    if !has_attachment_guid(&function, external) {
        return Ok(reference);
    }
    function
        .label()
        .find_child(1, true)?
        .find_child(1, true)?
        .find_attribute_as(&TdfReference::get_id(), &mut reference)?;
    Ok(reference)
}
/// Both attachment GUIDs count when their argument carries a reference; `is_attachment(&object)?`.
pub(super) fn is_attachment(object: &Handle<TDataStdUAttribute>) -> Result<bool, Failure> {
    Ok(!attachment_reference(object, true)?.is_null())
}
/// Native GetAttachmentsContext only resolves the internal attachment GUID; `attachment_context(&object)?`.
pub(super) fn attachment_context(
    object: &Handle<TDataStdUAttribute>,
) -> Result<Handle<TNamingNamedShape>, Failure> {
    let reference = attachment_reference(object, false)?;
    if reference.is_null() {
        return Ok(Handle::default());
    }
    let mut function: Handle<TFunctionFunction> = Handle::default();
    if !reference
        .get()
        .unwrap()
        .get()
        .find_attribute_as(&TFunctionFunction::get_id(), &mut function)?
    {
        return Ok(Handle::default());
    }
    function_result(&function)
}
fn face_plane(shape: &Shape) -> Result<Handle<GeomPlane>, Failure> {
    if shape.shape_type()? != ShapeEnum::Face {
        return Ok(Handle::default());
    }
    let mut surface = BRepTool::surface(&topods::face(shape)?)?;
    if surface
        .dynamic_type()
        .is_some_and(|kind| kind.name() == "Geom_RectangularTrimmedSurface")
    {
        surface = surface
            .cast::<GeomRectangularTrimmedSurface>()
            .basis_surface();
    }
    Ok(surface.cast())
}
fn sweep_plane(shape: &Shape) -> Result<Handle<GeomPlane>, Failure> {
    let plane = face_plane(shape)?;
    if !plane.is_null() {
        return Ok(plane);
    }
    let finder = BRepLibFindSurface::new(shape, 0.0, true, false)?;
    if !finder.found() {
        return Ok(Handle::default());
    }
    Ok(finder.surface().cast())
}
/// Preserve ignored ComputeSweepDir(false): caller keeps default Z axis; `sweep_axis(&shape)?`.
pub(super) fn sweep_axis(shape: &Shape) -> Result<Axis1, Failure> {
    let plane = sweep_plane(shape)?;
    if plane.is_null() {
        return Ok(Axis1::default());
    }
    let plane = plane.pln();
    let mut axis = plane.axis();
    if !plane.direct()? {
        axis.reverse();
    }
    if shape.orientation() == Orientation::Reversed {
        axis.reverse();
    }
    Ok(axis)
}

/// Convert a closed planar wire or retain a face; `planar_basis(&shape)?`.
pub(super) fn planar_basis(shape: &Shape) -> Result<Option<Shape>, Failure> {
    if shape.shape_type()? == ShapeEnum::Face {
        return Ok(Some(shape.clone()));
    }
    if shape.shape_type()? != ShapeEnum::Wire {
        return Ok(None);
    }
    let wire = topods::wire(shape)?;
    if BRepCheckWire::new(&wire)?.closed(true)? != BRepCheckStatus::NoError {
        return Ok(None);
    }
    let maker = BRepBuilderAPIMakeFace::from_wire(&wire, true)?;
    if !maker.is_done() {
        return Ok(None);
    }
    Ok(Some((*maker.face()?).clone()))
}
/// Solid or closed shell requires positive native volume; `volumetric(&shape)?`.
pub(super) fn volumetric(shape: &Shape) -> Result<bool, Failure> {
    if shape.shape_type()? == ShapeEnum::Solid {
        return Ok(true);
    }
    if shape.shape_type()? != ShapeEnum::Shell {
        return Ok(false);
    }
    Ok(BRepCheckShell::new(&topods::shell(shape)?)?.closed(false)? == BRepCheckStatus::NoError)
}
/// Preserve analyzer and closed-volume acceptance; `valid_result(&shape)?`.
pub(super) fn valid_result(shape: &Shape) -> Result<bool, Failure> {
    if !BRepCheckAnalyzer::new(shape, true, false, false)?.is_valid_shape(shape)? {
        return Ok(false);
    }
    if !volumetric(shape)? {
        return Ok(true);
    }
    let mut properties = GProps::default();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false)?;
    Ok(!matches!(
        properties.mass().partial_cmp(&precision::confusion()),
        Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
    ))
}

/// Preserve ordinary basis values and internal attachment contexts; `basis_context(&function,&basis)?`.
pub(super) fn basis_context(
    function: &Handle<TFunctionFunction>,
    basis: &Shape,
) -> Result<Option<Shape>, Failure> {
    let object = object_argument(function, 1)?;
    if !is_attachment(&object)? {
        return Ok(Some(basis.clone()));
    }
    let context = attachment_context(&object)?;
    if context.is_null() || context.is_empty() {
        function.set_failure(14)?;
        return Ok(None);
    }
    Ok(Some(context.get().unwrap().get()?))
}
