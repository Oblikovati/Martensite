//! Exact circle/mkedge/wire/mkplane and signed tangent-circle profile construction.
#![allow(dead_code)] // Each source cohort exercises its original construction path.
use martensite_tkbrep::topods::{Edge, Shape, Wire};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve};
use martensite_tkg3d::geom::{GeomCircle, GeomCurve};
use martensite_tkgeomalgo::geom_api;
use martensite_tkmath::gp::{Axis2, Axis2d, Axis3, Direction2, Direction3, Plane3, Point2, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
use std::f64::consts::PI;

fn edge_wire(edge: &Edge) -> Wire {
    let mut builder = BRepBuilderAPIMakeWire::new();
    builder.add_edge(edge).unwrap();
    builder.wire().unwrap()
}

/// Preserve circle, mkedge, wire, mkplane defaults; `circle_face([0.;3],[0.,0.,1.],1.,false)`.
pub fn circle_face(origin: [f64; 3], normal: [f64; 3], radius: f64, only_plane: bool) -> Shape {
    let frame = Axis2::new(
        Point3::new(origin[0], origin[1], origin[2]),
        Direction3::new(normal[0], normal[1], normal[2]).unwrap(),
    )
    .unwrap();
    let curve: Handle<dyn GeomCurve> =
        Handle::new(GeomCircle::new(frame, radius).unwrap()).into_cast();
    let edge = BRepBuilderAPIMakeEdge::from_curve(&curve)
        .unwrap()
        .edge()
        .unwrap();
    BRepBuilderAPIMakeFace::from_wire(&edge_wire(&edge), only_plane)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

/// Preserve signed tangent-circle C instruction; `tangent_arc([0.;2],[1.,0.],10.,180.)`.
pub fn tangent_arc(
    first: [f64; 2],
    direction: [f64; 2],
    signed_radius: f64,
    degrees: f64,
) -> (Handle<dyn Geom2dCurve>, f64) {
    let radius = signed_radius.abs();
    // Negative radius reverses the moving tangent before center and X-axis construction.
    let sign = if signed_radius < 0. { -1. } else { 1. };
    let (dx, dy) = (sign * direction[0], sign * direction[1]);
    let axis = Axis2d::new(
        Point2::new(first[0] - radius * dy, first[1] + radius * dx),
        Direction2::new(dy, -dx).unwrap(),
    );
    let angle = degrees * (PI / 180.);
    let sense = (signed_radius >= 0.) == (angle >= 0.);
    let curve = Handle::new(Geom2dCircle::from_axis(axis, radius, sense).unwrap()).into_cast();
    (curve, angle.abs())
}

fn profile_plane(origin: [f64; 3], normal: [f64; 3], x_axis: [f64; 3]) -> Plane3 {
    Plane3::from_position(
        Axis3::new(
            Point3::new(origin[0], origin[1], origin[2]),
            Direction3::new(normal[0], normal[1], normal[2]).unwrap(),
            Direction3::new(x_axis[0], x_axis[1], x_axis[2]).unwrap(),
        )
        .unwrap(),
    )
}

/// Preserve P/O/F/C360, including signed radius and explicit plane; `tangent_profile_face([0.;3],[0.,0.,1.],[1.,0.,0.],[0.;2],60.)`.
pub fn tangent_profile_face(
    origin: [f64; 3],
    normal: [f64; 3],
    x_axis: [f64; 3],
    first: [f64; 2],
    signed_radius: f64,
) -> Shape {
    let plane = profile_plane(origin, normal, x_axis);
    let (curve, angle) = tangent_arc(first, [1., 0.], signed_radius, 360.);
    // Source adds a closing line only when the endpoint gap exceeds Confusion.
    let (end, _) = curve.get().unwrap().d1(angle).unwrap();
    assert!(end.distance(Point2::new(first[0], first[1])) <= precision::confusion());
    let curve3d = geom_api::to_3d(&curve, &plane).unwrap();
    let edge = BRepBuilderAPIMakeEdge::from_curve_params(&curve3d, 0., angle)
        .unwrap()
        .edge()
        .unwrap();
    BRepBuilderAPIMakeFace::from_plane_wire(plane, &edge_wire(&edge), true)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}
