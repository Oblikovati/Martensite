// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::{
    brep::{BRepBuilder, BRepTEdge, BRepTVertex},
    topods::{self, Builder, Shape},
};
use martensite_tkernel::standard::{Handle, SharedReadFailure};
use martensite_tkg3d::geom::GeomLine;
use martensite_tkmath::{
    gp::{Transform3, Vector3},
    toploc::Location,
};

fn fixture() -> (Edge, [Vertex; 2]) {
    let builder = BRepBuilder::default();
    let mut edge = Edge::default();
    let mut vertices = [Vertex::default(), Vertex::default()];
    let carrier = Line3::new(Point3::new(5., 2., 0.), Direction3::Z);
    let curve = Handle::new(GeomLine::from_lin(carrier)).into_upcast();
    builder
        .make_edge_with_curve(&mut edge, &curve, 1e-6)
        .unwrap();
    builder.range(&edge, 2., 4., false).unwrap();
    for (i, (vertex, t)) in vertices.iter_mut().zip([2., 4.]).enumerate() {
        builder
            .make_vertex_at(vertex, Point3::new(5., 2., t), 1e-7)
            .unwrap();
        let orientation = [Orientation::Forward, Orientation::Reversed][i];
        Builder
            .add(&mut edge, &vertex.oriented(orientation))
            .unwrap();
    }
    (edge, vertices)
}

fn located<T>(
    shape: &Shape,
    offset: &Location,
    cast: impl FnOnce(&Shape) -> Result<T, martensite_tkernel::standard::Failure>,
) -> T {
    cast(&shape.moved(offset, false).unwrap()).unwrap()
}

#[test]
fn capture_preserves_the_actual_reversed_located_trim_and_endpoint_names() {
    let (edge, [a, b]) = fixture();
    let offset = Location::new(Transform3::translation_vector(Vector3::new(3., 0., 10.)));
    let edge = located(&edge.reversed(), &offset, |s| topods::edge(s));
    let a = located(&a, &offset, |s| topods::vertex(s));
    let b = located(&b, &offset, |s| topods::vertex(s));
    let observation = capture_line_edge(&edge, [&a, &b], || false).unwrap();
    assert_eq!(observation.vertices, [1, 0]);
    let proof = observation.geometry;
    assert_eq!(proof.orientation, Orientation::Reversed);
    assert_eq!(proof.range, [2., 4.]);
    assert_eq!(
        proof.points,
        [Point3::new(8., 2., 14.), Point3::new(8., 2., 12.)]
    );
    assert_eq!(proof.edge_tolerance, 1e-6);
    assert_eq!(proof.vertex_tolerances, [1e-7; 2]);
}

#[test]
fn capture_refuses_the_actual_writer_lying_vertex_and_extra_spatial_record() {
    let (edge, [a, b]) = fixture();
    let stored = edge.tshape().downcast::<BRepTEdge>();
    let stored = stored.get().unwrap();
    let writer = stored.fields.write();
    assert_eq!(
        capture_line_edge(&edge, [&a, &b], || false),
        Err(LineEdgeCaptureError::Read(
            LineEdgeReadCell::EdgeRepresentations,
            SharedReadFailure::Busy
        )),
    );
    drop(writer);
    let source_point = a.tshape().downcast::<BRepTVertex>();
    let source_point = source_point.get().unwrap();
    source_point.set_pnt(Point3::new(6., 2., 2.));
    assert!(matches!(
        capture_line_edge(&edge, [&a, &b], || false),
        Err(LineEdgeCaptureError::Endpoint { index: 0, .. })
    ));
    assert_eq!(
        capture_line_edge(&edge, [&a, &a], || false),
        Err(LineEdgeCaptureError::VertexBinding)
    );
    stored.change_curves(|curves| curves.push(curves[0].clone()));
    assert_eq!(
        capture_line_edge(&edge, [&a, &b], || false),
        Err(LineEdgeCaptureError::Representation)
    );
}

#[test]
fn capture_detaches_from_subsequent_native_edit_and_honors_cancel() {
    let (edge, [a, b]) = fixture();
    let initial = capture_line_edge(&edge, [&a, &b], || false).unwrap();
    let vertex = a.tshape().downcast::<BRepTVertex>();
    vertex.get().unwrap().set_pnt(Point3::default());
    assert_eq!(initial.geometry.points[0], Point3::new(5., 2., 2.));
    assert_eq!(
        capture_line_edge(&edge, [&a, &b], || true),
        Err(LineEdgeCaptureError::Cancelled)
    );
}
