//! The admitted cylinder patch is an analytic rotation of one radial/axial section.
use super::{admission::Admitted, failure::*, frame};
use martensite_tkbrep::topods::{Builder, Edge, Face, Shape, Vertex, Wire};
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeRevol;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex,
};

pub(super) struct Section {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub face: Face,
    pub inner_depth: usize,
}

pub(super) fn construct(admitted: &Admitted) -> Checked<(Section, BRepPrimAPIMakeRevol, Shape)> {
    let section = radial_section(admitted).map_err(|error| native("radial section", error))?;
    let support = admitted.rectangle.support;
    let [x, y, z] = support.axis;
    let direction = Direction3::new(x, y, z).map_err(|error| native("axis", error))?;
    let [x, y, z] = support.origin;
    let axis = Axis1::new(Point3::new(x, y, z), direction);
    let mut maker = BRepPrimAPIMakeRevol::new(&section.face, axis, admitted.angle, false)
        .map_err(|error| native("revolution", error))?;
    let solid = maker
        .shape()
        .map_err(|error| native("revolution result", error))?
        .clone();
    Ok((section, maker, solid))
}

fn radial_section(admitted: &Admitted) -> Result<Section, martensite_tkernel::standard::Failure> {
    let inner_depth = usize::from(admitted.radii[1] < admitted.radii[0]);
    let vertices = section_vertices(admitted, inner_depth)?;
    let edges = (0..4)
        .map(|i| {
            BRepBuilderAPIMakeEdge::from_vertices(&vertices[i], &vertices[(i + 1) % 4])?.edge()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let face = section_face(&edges)?;
    Ok(Section {
        vertices,
        edges,
        face,
        inner_depth,
    })
}
fn section_face(edges: &[Edge]) -> Result<Face, martensite_tkernel::standard::Failure> {
    let mut wire = Wire::default();
    Builder.make_wire(&mut wire);
    for edge in edges {
        Builder.add(&mut wire, edge)?;
    }
    wire.set_closed(true)?;
    BRepBuilderAPIMakeFace::from_wire(&wire, true)?.face()
}

fn section_vertices(
    admitted: &Admitted,
    inner_depth: usize,
) -> Result<Vec<Vertex>, martensite_tkernel::standard::Failure> {
    let inner = admitted.radii[inner_depth];
    let outer = admitted.radii[1 - inner_depth];
    let [low, high] = admitted.rectangle.z;
    [(inner, low), (outer, low), (outer, high), (inner, high)]
        .into_iter()
        .map(|(radius, z)| {
            let [x, y, z] = frame::point(&admitted.rectangle.support, admitted.start, radius, z);
            BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex()
        })
        .collect()
}
