//! Original no-vertex edge/sphere distance case from bugs/moddata_1/bug22303.
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCurve, GeomLine};
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape},
    brep_extrema::BRepExtremaDistShapeShape,
};

fn original_infinite_line() -> Handle<dyn GeomCurve> {
    let axis = Axis1::new(
        Point3::new(0., 20., 0.),
        Direction3::new(1., 0., 0.).unwrap(),
    );
    Handle::new(GeomLine::new(axis)).into_cast()
}

fn original_sphere_distance() -> f64 {
    let line = original_infinite_line();
    let edge = BRepBuilderAPIMakeEdge::from_curve(&line)
        .unwrap()
        .edge()
        .unwrap();
    let mut sphere = BRepPrimAPIMakeSphere::new(10.0).unwrap();
    let distance =
        BRepExtremaDistShapeShape::with_shapes(&edge, sphere.shape().unwrap(), Default::default())
            .unwrap();
    assert!(distance.is_done());
    distance.value().unwrap()
}

#[test]
fn infinite_edge_to_sphere_keeps_original_asymmetric_distance_limit() {
    crate::isolated_primitive_case::run_primitive_case(
        "infinite_edge_sphere_distance::infinite_edge_to_sphere_keeps_original_asymmetric_distance_limit",
        infinite_edge_to_sphere_keeps_original_asymmetric_distance_limit_body,
    );
}

fn infinite_edge_to_sphere_keeps_original_asymmetric_distance_limit_body() {
    let measured = original_sphere_distance();
    // The original only rejects distances above 10+1e-3; do not replace it with an absolute check.
    assert!(
        measured - 10. <= 1e-3,
        "distance {measured}: expected original upper difference <=1e-3"
    );
}
