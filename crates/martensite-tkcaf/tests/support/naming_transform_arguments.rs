//! Real DNaming argument/axis helpers required by the transformation driver.
use super::naming_function_graph::object_value;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
use martensite_tkcaf::tnaming::TNamingNamedShape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::{geom::GeomLine, top_abs::ShapeEnum};
use martensite_tkgeombase::geom_lib::IsPlanarSurface;
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::{TDataStdTreeNode, TDataStdUAttribute},
    tdf::{TdfAttributeState, TdfReference},
    tfunction::TFunctionFunction,
};
use martensite_tkmath::gp::{Axis1, Plane3};

/// Walk actual previous tree siblings, skipping non-functions; `previous_function(&f)?`.
pub fn previous_function(
    function: &Handle<TFunctionFunction>,
) -> Result<Handle<TFunctionFunction>, Failure> {
    let mut node = function_node(function)?;
    let mut previous = Handle::default();
    while !node.is_null() {
        if !node.has_previous() {
            return Ok(previous);
        }
        node = node.previous();
        node.find_attribute_as(&TFunctionFunction::get_id(), &mut previous)?;
        if !previous.is_null() {
            break;
        }
    }
    Ok(previous)
}

fn function_node(
    function: &Handle<TFunctionFunction>,
) -> Result<Handle<TDataStdTreeNode>, Failure> {
    let mut node = Handle::default();
    if !function.is_null() {
        function.find_attribute_as(&TDataStdTreeNode::get_default_tree_id(), &mut node)?;
    }
    Ok(node)
}

/// Preserve DNaming's object reference layout; `set_object_argument(&f, 2, &object)?`.
pub fn set_object_argument(
    function: &Handle<TFunctionFunction>,
    position: i32,
    object: &Handle<TDataStdUAttribute>,
) -> Result<(), Failure> {
    if object.is_null() {
        return Ok(());
    }
    let label = function
        .label()
        .find_child(1, true)?
        .find_child(position, true)?;
    TdfReference::set(&label, &object.label())?;
    Ok(())
}

/// Read only an object carrying the native geometry GUID; `object_argument(&f, 2)?`.
pub fn object_argument(
    function: &Handle<TFunctionFunction>,
    position: i32,
) -> Result<Handle<TDataStdUAttribute>, Failure> {
    let label = function
        .label()
        .find_child(1, true)?
        .find_child(position, true)?;
    let mut reference: Handle<TdfReference> = Handle::default();
    let mut object = Handle::default();
    if label.find_attribute_as(&TdfReference::get_id(), &mut reference)? {
        reference.get().unwrap().get().find_attribute_as(
            &guid_from_text("6c6915ab-775f-4475-859e-befd74d26a23"),
            &mut object,
        )?;
    }
    Ok(object)
}

/// Resolve the actual selected object's naming value; `argument_value(&f, 2)?`.
pub fn argument_value(
    function: &Handle<TFunctionFunction>,
    position: i32,
) -> Result<Handle<TNamingNamedShape>, Failure> {
    object_value(&object_argument(function, position)?)
}

/// DNaming::ComputeAxis uses a line edge, or the first edge of a wire; `axis_from_named(&named)?`.
pub fn axis_from_named(named: &Handle<TNamingNamedShape>) -> Result<Option<Axis1>, Failure> {
    if named.is_null() || named.is_empty() {
        return Ok(None);
    }
    let mut shape = named.get().unwrap().get()?;
    if shape.is_null() {
        return Ok(None);
    }
    if shape.shape_type()? == ShapeEnum::Wire {
        shape = Explorer::new(&shape, ShapeEnum::Edge, ShapeEnum::Shape)?
            .current()
            .clone();
    }
    if shape.shape_type()? != ShapeEnum::Edge {
        return Ok(None);
    }
    let (curve, _, _) = BRepTool::curve(&topods::edge(&shape)?)?;
    let line: Handle<GeomLine> = curve.cast();
    Ok(line.get().map(GeomLine::position))
}

/// The native mirror branch accepts only a planar face, with its default 1e-7 planarity test.
/// Example: `plane_from_named(&named)?`.
pub fn plane_from_named(named: &Handle<TNamingNamedShape>) -> Result<Option<Plane3>, Failure> {
    if named.is_null() || named.is_empty() {
        return Ok(None);
    }
    let shape = named.get().unwrap().get()?;
    if shape.is_null() || shape.shape_type()? != ShapeEnum::Face {
        return Ok(None);
    }
    let surface = BRepTool::surface(&topods::face(&shape)?)?;
    let planar = IsPlanarSurface::new(&surface, 1e-7)?;
    if !planar.is_planar() {
        return Ok(None);
    }
    Ok(Some(planar.plan()?))
}
