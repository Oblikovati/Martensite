//! Independent polynomial rounded-square strips, assembled from actual geometry/topology builders.
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepCurveOnClosedSurface, BRepCurveOnSurface, BRepTEdge},
    topods::{Edge, Face, Vertex, Wire},
};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::{
    geom::{GeomBSplineCurve, GeomLine, GeomSurfaceOfLinearExtrusion},
    top_abs::Orientation,
};
use martensite_tkmath::gp::{Direction2, Direction3, Point2, Point3};
pub(super) struct StripFixture {
    pub face: Face,
    pub edges: [Edge; 3],
    pub vertices: [Vertex; 2],
}
pub(super) const ROUND_NET: [[f64; 2]; 9] = [
    [1., 0.],
    [1., 1.],
    [0., 1.],
    [-1., 1.],
    [-1., 0.],
    [-1., -1.],
    [0., -1.],
    [1., -1.],
    [1., 0.],
];

fn spline(level: f64, delta: f64, xy: [[f64; 2]; 9]) -> Handle<GeomBSplineCurve> {
    let coordinates = xy.map(|p| Point3::new(p[0] + delta, p[1], level));
    Handle::new(
        GeomBSplineCurve::new(
            &Array1::from_shared(&coordinates, 1),
            &Array1::from_shared(&[0., 1., 2., 3., 4.], 1),
            &Array1::from_shared(&[3, 2, 2, 2, 3], 1),
            2,
            false,
        )
        .unwrap(),
    )
}
fn vertex(level: f64, tolerance: f64) -> Vertex {
    let mut result = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut result, Point3::new(1., 0., level), tolerance)
        .unwrap();
    result
}
fn cap(
    face: &Face,
    level: f64,
    vertex: &Vertex,
    tolerance: f64,
    delta: f64,
    xy: [[f64; 2]; 9],
) -> Edge {
    let builder = BRepBuilder::default();
    let mut result = Edge::default();
    builder
        .make_edge_with_curve(&mut result, &spline(level, delta, xy).cast(), tolerance)
        .unwrap();
    let pcurve = Handle::new(Geom2dLine::from_point(
        Point2::new(0., level),
        Direction2::X,
    ));
    builder
        .update_edge_pcurve_on_face(&result, &pcurve.cast(), face, tolerance)
        .unwrap();
    builder.range(&result, 0., 4., false).unwrap();
    builder
        .add(&mut result, &vertex.oriented(Orientation::Forward))
        .unwrap();
    builder
        .add(&mut result, &vertex.oriented(Orientation::Reversed))
        .unwrap();
    set_cache(&result, [[0., level], [4., level]], None);
    result
}
fn seam(face: &Face, vertices: &[Vertex; 2], tolerance: f64) -> Edge {
    let builder = BRepBuilder::default();
    let mut result = Edge::default();
    let curve = Handle::new(GeomLine::from_point(
        Point3::new(1., 0., -1.),
        Direction3::Z,
    ));
    builder
        .make_edge_with_curve(&mut result, &curve.cast(), tolerance)
        .unwrap();
    let a = Handle::new(Geom2dLine::from_point(Point2::new(0., -1.), Direction2::Y));
    let b = Handle::new(Geom2dLine::from_point(Point2::new(4., -1.), Direction2::Y));
    builder
        .update_edge_pcurves_on_face(&result, (&a.cast(), &b.cast()), face, tolerance)
        .unwrap();
    builder.range(&result, 0., 3., false).unwrap();
    builder
        .add(&mut result, &vertices[0].oriented(Orientation::Forward))
        .unwrap();
    builder
        .add(&mut result, &vertices[1].oriented(Orientation::Reversed))
        .unwrap();
    set_cache(&result, [[0., -1.], [0., 2.]], Some([[4., -1.], [4., 2.]]));
    result
}
pub(super) fn strip(tolerance: f64, top_delta: f64) -> StripFixture {
    strip_from_net(tolerance, top_delta, ROUND_NET)
}
pub(super) fn strip_from_net(tolerance: f64, top_delta: f64, xy: [[f64; 2]; 9]) -> StripFixture {
    let builder = BRepBuilder::default();
    let mut face = Face::default();
    let support = Handle::new(
        GeomSurfaceOfLinearExtrusion::new(&spline(0., 0., xy).cast(), Direction3::Z).unwrap(),
    );
    builder
        .make_face_on_surface(&mut face, &support.cast(), tolerance)
        .unwrap();
    let vertices = [vertex(-1., tolerance), vertex(2., tolerance)];
    let edges = [
        seam(&face, &vertices, tolerance),
        cap(&face, 2., &vertices[1], tolerance, top_delta, xy),
        cap(&face, -1., &vertices[0], tolerance, 0., xy),
    ];
    let mut wire = Wire::default();
    builder.make_wire(&mut wire);
    for edge in [
        edges[0].oriented(Orientation::Forward),
        edges[1].oriented(Orientation::Forward),
        edges[0].oriented(Orientation::Reversed),
        edges[2].oriented(Orientation::Reversed),
    ] {
        builder.add(&mut wire, &edge).unwrap();
    }
    builder.add(&mut face, &wire).unwrap();
    StripFixture {
        face,
        edges,
        vertices,
    }
}

pub(super) fn set_cache(edge: &Edge, first: [[f64; 2]; 2], second: Option<[[f64; 2]; 2]>) {
    let owner = edge.tshape().cast::<BRepTEdge>();
    for representation in owner.get().unwrap().curves() {
        let ordinary = representation.cast::<BRepCurveOnSurface>();
        if let Some(curve) = ordinary.get() {
            curve.set_uv_points(
                Point2::new(first[0][0], first[0][1]),
                Point2::new(first[1][0], first[1][1]),
            );
        }
        if let Some(points) = second {
            let seam = representation.cast::<BRepCurveOnClosedSurface>();
            if let Some(curve) = seam.get() {
                curve.set_uv_points(
                    Point2::new(first[0][0], first[0][1]),
                    Point2::new(first[1][0], first[1][1]),
                );
                curve.set_uv_points2(
                    Point2::new(points[0][0], points[0][1]),
                    Point2::new(points[1][0], points[1][1]),
                );
            }
        }
    }
}
