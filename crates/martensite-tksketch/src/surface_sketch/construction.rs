//! Build one oriented analytic section and retain its native sweep images.
use super::{failure::*, sweep::Sweep};
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
    pub first_depth: usize,
}

pub(super) fn construct(admitted: &Sweep) -> Checked<(Section, BRepPrimAPIMakeRevol, Shape)> {
    let section = radial_section(admitted).map_err(|error| native("radial section", error))?;
    let [x, y, z] = admitted.axis;
    let direction = Direction3::new(x, y, z).map_err(|error| native("axis", error))?;
    let [x, y, z] = admitted.origin;
    let axis = Axis1::new(Point3::new(x, y, z), direction);
    let mut maker = BRepPrimAPIMakeRevol::new(&section.face, axis, admitted.angle, false)
        .map_err(|error| native("revolution", error))?;
    let solid = maker
        .shape()
        .map_err(|error| native("revolution result", error))?
        .clone();
    Ok((section, maker, solid))
}

fn radial_section(admitted: &Sweep) -> Result<Section, martensite_tkernel::standard::Failure> {
    let first_depth = admitted.first_depth;
    let vertices = section_vertices(admitted)?;
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
        first_depth,
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
    admitted: &Sweep,
) -> Result<Vec<Vertex>, martensite_tkernel::standard::Failure> {
    admitted
        .points
        .into_iter()
        .map(|[x, y, z]| BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex())
        .collect()
}
