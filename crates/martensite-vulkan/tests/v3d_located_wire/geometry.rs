use super::*;
use martensite_tkbrep::topods::Vertex;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire,
};

pub(super) struct LocatedWireInputs {
    pub original: Shape,
    pub independent_copy: Shape,
    pub vertices: [Vertex; 3],
    pub located_copies: [Shape; 3],
    pub compound: Shape,
}

fn case_wire() -> Result<(Shape, [Vertex; 3]), Failure> {
    let vertex = |x, y| BRepBuilderAPIMakeVertex::new(Point3::new(x, y, 0.))?.vertex();
    let vertices = [vertex(0., 0.)?, vertex(10., 0.)?, vertex(10., 10.)?];
    let first = BRepBuilderAPIMakeEdge::from_vertices(&vertices[0], &vertices[1])?.edge()?;
    let second = BRepBuilderAPIMakeEdge::from_vertices(&vertices[1], &vertices[2])?.edge()?;
    let wire = BRepBuilderAPIMakeWire::from_two_edges(&first, &second)?.wire()?;
    Ok((wire.into_shape(), vertices))
}

pub(super) fn located_inputs() -> Result<LocatedWireInputs, Failure> {
    let (original, vertices) = case_wire()?;
    let mut copier = BRepBuilderAPICopy::with_shape(&original, true, false)?;
    let independent_copy = copier.shape()?.clone();
    let located_copies = [
        original.clone(),
        crate::primitive_shapes::translated(&original, [0., 0., 10.])?,
        crate::primitive_shapes::translated(&original, [0., 0., 20.])?,
    ];
    let compound = crate::primitive_shapes::compound_of(&located_copies)?;
    Ok(LocatedWireInputs {
        original,
        independent_copy,
        vertices,
        located_copies,
        compound,
    })
}
