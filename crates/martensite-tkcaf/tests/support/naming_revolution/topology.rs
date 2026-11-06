//! Native atomic-body dangle classification and open-basis endpoints.
use martensite_tkbrep::{
    topexp::{vertices, wire_vertices},
    topods::{self, Shape, ShapeIterator, Vertex},
    toptools::MapOfShape,
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire},
    brep_check::{BRepCheckShell, BRepCheckStatus, BRepCheckWire},
};
fn atomic_shapes(shape: &Shape, shapes: &mut MapOfShape) -> Result<(), Failure> {
    if !matches!(
        shape.shape_type()?,
        ShapeEnum::Compound | ShapeEnum::CompSolid
    ) {
        shapes.add(shape.clone());
        return Ok(());
    }
    for child in ShapeIterator::new(shape, true, true) {
        atomic_shapes(&child, shapes)?;
    }
    Ok(())
}
fn atomic_dangle(shape: &Shape) -> Result<bool, Failure> {
    match shape.shape_type()? {
        ShapeEnum::Solid => Ok(false),
        ShapeEnum::Shell => {
            Ok(BRepCheckShell::new(&topods::shell(shape)?)?.closed(false)?
                != BRepCheckStatus::NoError)
        }
        ShapeEnum::Face | ShapeEnum::Wire | ShapeEnum::Edge | ShapeEnum::Vertex => Ok(true),
        _ => Ok(false),
    }
}
/// Flatten compound/compsolid atoms before native classification; `has_dangle(&shape)?`.
pub(super) fn has_dangle(shape: &Shape) -> Result<bool, Failure> {
    let mut shapes = MapOfShape::default();
    atomic_shapes(shape, &mut shapes)?;
    for atom in shapes.iter() {
        if atomic_dangle(atom)? {
            return Ok(true);
        }
    }
    Ok(false)
}
fn wire_endpoints(shape: &Shape) -> Result<Option<[Vertex; 2]>, Failure> {
    let wire = topods::wire(shape)?;
    if BRepCheckWire::new(&wire)?.closed(false)? == BRepCheckStatus::NoError {
        return Ok(None);
    }
    let (first, last) = wire_vertices(&wire);
    Ok(Some([first, last]))
}
/// Preserve the closed-circle wire check for an edge basis; `open_endpoints(&basis)?`.
pub(super) fn open_endpoints(shape: &Shape) -> Result<Option<[Vertex; 2]>, Failure> {
    if shape.shape_type()? == ShapeEnum::Wire {
        return wire_endpoints(shape);
    }
    if shape.shape_type()? != ShapeEnum::Edge {
        return Ok(None);
    }
    let edge = topods::edge(shape)?;
    let mut maker = BRepBuilderAPIMakeWire::new();
    maker.add_edge(&edge)?;
    if !maker.is_done() {
        return Ok(None);
    }
    if BRepCheckWire::new(&maker.wire()?)?.closed(false)? == BRepCheckStatus::NoError {
        return Ok(None);
    }
    let (first, last) = vertices(&edge, false);
    Ok(Some([first, last]))
}
