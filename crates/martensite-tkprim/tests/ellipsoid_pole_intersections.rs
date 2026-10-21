//! Literal sphere/ellipsoid pole regressions from bugs/moddata_2/bug22809_{1,2,3,4}.
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape, Vertex},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    geom::{GeomCurve, GeomLine},
    geom_adaptor::GeomAdaptorCurve,
};
use martensite_tkmath::gp::{Axis1, Direction3, GeneralTransform3, Matrix3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIGTransform, BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert,
    },
    brep_int_curve_surface::BRepIntCurveSurfaceInter,
};

fn original_pole_sphere() -> Shape {
    BRepPrimAPIMakeSphere::new(100.)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn original_nurbs_sphere() -> Shape {
    let mut conversion = BRepBuilderAPINurbsConvert::new();
    conversion.perform(&original_pole_sphere(), false).unwrap();
    conversion.shape().unwrap().clone()
}

fn original_scaled_sphere() -> Shape {
    let mut transform = GeneralTransform3::default();
    transform.set_vectorial_part(Matrix3::new([[2., 0., 0.], [0., 1., 0.], [0., 0., 1.]]));
    let mut operation = BRepBuilderAPIGTransform::new(transform);
    operation.perform(&original_pole_sphere(), false).unwrap();
    assert!(operation.is_done());
    operation.shape().unwrap().clone()
}

fn original_pole_line(x: f64, y: f64) -> GeomAdaptorCurve {
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::new(Axis1::new(
        Point3::new(x, y, -100.),
        Direction3::Z,
    )))
    .into_cast();
    GeomAdaptorCurve::with_curve(&line).unwrap()
}

fn original_point_vertex(compound: &mut Compound, point: Point3) {
    let builder = BRepBuilder::default();
    let mut vertex = Vertex::default();
    builder.make_vertex_at(&mut vertex, point, 0.).unwrap();
    builder.add(compound, &vertex).unwrap();
}

fn original_intersection_points(shape: &Shape, x: f64, y: f64) -> Vec<Point3> {
    let adaptor = original_pole_line(x, y);
    let mut compound = Compound::default();
    BRepBuilder::default().make_compound(&mut compound);
    let mut intersection = BRepIntCurveSurfaceInter::new();
    intersection.init_curve(shape, &adaptor, 1e-6).unwrap();
    let mut points = Vec::new();
    while intersection.more() {
        let point = intersection.pnt().unwrap();
        original_point_vertex(&mut compound, point);
        points.push(point);
        intersection.next().unwrap();
    }
    points
}

fn assert_original_coordinate(measured: f64, expected: f64) {
    assert!(
        (measured - expected).abs() <= 0.001 * expected.abs(),
        "coordinate {measured}: expected {expected} at original relative tolerance 0.001"
    );
}

fn assert_original_pole(point: Point3, x: f64, y: f64, z: f64) {
    assert_original_coordinate(point.x(), x);
    assert_original_coordinate(point.y(), y);
    assert_original_coordinate(point.z(), z);
}

fn assert_original_either_pole(point: Point3) {
    assert_original_coordinate(point.x(), 0.);
    assert_original_coordinate(point.y(), 0.);
    if (point.z() + 100.).abs() > 0.1 {
        assert_original_coordinate(point.z(), 100.);
    }
}

#[test]
fn nurbs_sphere_exact_axis_retains_original_first_and_forty_first_points() {
    crate::isolated_primitive_case::run_primitive_case(
        "ellipsoid_pole_intersections::nurbs_sphere_exact_axis_retains_original_first_and_forty_first_points",
        nurbs_sphere_exact_axis_retains_original_first_and_forty_first_points_body,
    );
}

fn nurbs_sphere_exact_axis_retains_original_first_and_forty_first_points_body() {
    let points = original_intersection_points(&original_nurbs_sphere(), 0., 0.);
    assert_original_either_pole(points[0]);
    assert_original_either_pole(points[40]);
}

#[test]
fn scaled_sphere_exact_axis_retains_original_first_and_twenty_third_points() {
    crate::isolated_primitive_case::run_primitive_case(
        "ellipsoid_pole_intersections::scaled_sphere_exact_axis_retains_original_first_and_twenty_third_points",
        scaled_sphere_exact_axis_retains_original_first_and_twenty_third_points_body,
    );
}

fn scaled_sphere_exact_axis_retains_original_first_and_twenty_third_points_body() {
    let points = original_intersection_points(&original_scaled_sphere(), 0., 0.);
    assert_original_either_pole(points[0]);
    assert_original_either_pole(points[22]);
}

#[test]
fn nurbs_sphere_offset_axis_retains_original_ordered_poles() {
    crate::isolated_primitive_case::run_primitive_case(
        "ellipsoid_pole_intersections::nurbs_sphere_offset_axis_retains_original_ordered_poles",
        nurbs_sphere_offset_axis_retains_original_ordered_poles_body,
    );
}

fn nurbs_sphere_offset_axis_retains_original_ordered_poles_body() {
    let points = original_intersection_points(&original_nurbs_sphere(), -1e-6, 1e-10);
    assert_original_pole(points[0], -1e-6, 1e-10, -100.);
    assert_original_pole(points[1], -1e-6, 1e-10, 100.);
}

#[test]
fn scaled_sphere_offset_axis_retains_original_ordered_poles() {
    crate::isolated_primitive_case::run_primitive_case(
        "ellipsoid_pole_intersections::scaled_sphere_offset_axis_retains_original_ordered_poles",
        scaled_sphere_offset_axis_retains_original_ordered_poles_body,
    );
}

fn scaled_sphere_offset_axis_retains_original_ordered_poles_body() {
    let points = original_intersection_points(&original_scaled_sphere(), -1e-6, 1e-10);
    assert_original_pole(points[0], -1e-6, 1e-10, -100.);
    assert_original_pole(points[1], -1e-6, 1e-10, 100.);
}
