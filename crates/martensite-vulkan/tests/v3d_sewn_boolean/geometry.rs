use super::*;
use martensite_tkbrep::topods;
use martensite_tkernel::{message::ProgressRange, precision};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire, BRepBuilderAPISewing,
};

const FACE_POINTS: [[[f64; 3]; 3]; 4] = [
    [[0., 1., 0.], [1., 0., 0.], [0., 0., 0.]],
    [[0., 0., 2.], [1., 0., 0.], [0., 0., 0.]],
    [[0., 0., 2.], [0., 1., 0.], [1., 0., 0.]],
    [[0., 0., 2.], [0., 0., 0.], [0., 1., 0.]],
];

fn triangle_face(points: [[f64; 3]; 3]) -> Result<Shape, Failure> {
    let vertex =
        |[x, y, z]: [f64; 3]| BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?.vertex();
    let vertices = [vertex(points[0])?, vertex(points[1])?, vertex(points[2])?];
    let edge = |a, b| BRepBuilderAPIMakeEdge::from_vertices(&vertices[a], &vertices[b])?.edge();
    let edges = [edge(0, 1)?, edge(1, 2)?, edge(2, 0)?];
    let wire = BRepBuilderAPIMakeWire::from_three_edges(&edges[0], &edges[1], &edges[2])?.wire()?;
    let mut face = BRepBuilderAPIMakeFace::from_wire(&wire, false)?;
    Ok(face.shape()?.clone())
}

fn source_sewing() -> BRepBuilderAPISewing {
    let mut sewing = BRepBuilderAPISewing::new(1e-6, true, true, true, false);
    sewing.set_same_parameter_mode(true);
    sewing.set_floating_edges_mode(false);
    sewing.set_face_mode(true);
    sewing.set_min_tolerance(precision::confusion());
    sewing.set_max_tolerance(precision::infinite());
    sewing
}

pub(super) fn source_faces() -> Result<[Shape; 4], Failure> {
    let [first, second, third, fourth] = FACE_POINTS.map(triangle_face);
    Ok([first?, second?, third?, fourth?])
}

pub(super) fn sew_faces(faces: &[Shape; 4]) -> Result<Shape, Failure> {
    let mut sewing = source_sewing();
    for face in faces {
        sewing.add(face)?;
    }
    sewing.perform(&ProgressRange::default())?;
    let shell = topods::shell(sewing.sewed_shape())?;
    super::shell_solid::solid_from_shell(shell)
}

#[cfg(test)]
pub(super) fn sewn_tetrahedron() -> Result<Shape, Failure> {
    let faces = source_faces()?;
    sew_faces(&faces)
}
