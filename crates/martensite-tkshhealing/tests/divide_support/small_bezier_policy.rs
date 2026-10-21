//! Conversion policies retain the specialized small-curve recovery behavior.
use martensite_tkbrep::{
    brep::BRepTool,
    brep_tools::ReShape,
    topexp,
    topods::{self, Builder, Edge, Face, Vertex, Wire},
};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg3d::{
    geom::{GeomBSplineCurve, GeomBezierCurve, GeomCurve},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tkshhealing::shape_upgrade::{
    FaceDivider, FixSmallCurves, ShapeDivider, ShapeUpgradeFixSmallBezierCurves,
    ShapeUpgradeFixSmallCurves, ShapeUpgradeShapeConvertToBezier, ShapeUpgradeWireDivide,
    WireDivider,
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};

fn selected_wire_policy() -> Handle<ShapeUpgradeWireDivide> {
    let converter = ShapeUpgradeShapeConvertToBezier::empty();
    converter.set_3d_conversion(true);
    let face = ShapeDivider::split_face_tool(&converter);
    face.cast::<dyn FaceDivider>().wire_divide_tool()
}

#[test]
fn selected_conversion_policy_installs_specialized_small_bezier_recovery() {
    let small = selected_wire_policy().fix_small_curve_tool();
    assert!(!small.cast::<ShapeUpgradeFixSmallBezierCurves>().is_null());
}

fn configured_small_policy() -> Handle<ShapeUpgradeFixSmallCurves> {
    let wire = selected_wire_policy();
    let small = wire.fix_small_curve_tool();
    let edge = recovery_line(Point3::default(), Point3::new(1., 0., 0.));
    small.init(&edge, &Face::default());
    small.set_split_curve_3d_tool(&wire.cast::<dyn WireDivider>().split_curve_3d_tool());
    small
}

fn approximate_selected_subrange() -> Handle<dyn GeomCurve> {
    let small = configured_small_policy().cast::<dyn FixSmallCurves>();
    let mut curves = (Handle::default(), Handle::default(), Handle::default());
    let mut range = (0.25, 0.75);
    let changed = small
        .approx(
            &mut curves.0,
            &mut curves.1,
            &mut curves.2,
            &mut range.0,
            &mut range.1,
        )
        .unwrap();
    assert!(changed);
    assert_eq!(range, (0.25, 0.75));
    assert!(curves.1.is_null() && curves.2.is_null());
    curves.0
}

#[test]
fn selected_small_policy_approximates_straight_subrange_as_bezier() {
    let spatial = approximate_selected_subrange();
    assert!(!spatial.cast::<GeomBezierCurve>().is_null());
    for (parameter, expected) in [(0., 0.25), (0.5, 0.5), (1., 0.75)] {
        let distance = spatial
            .value(parameter)
            .unwrap()
            .distance(&Point3::new(expected, 0., 0.));
        assert!(distance < 1e-7, "straight approximation error {distance}");
    }
}

fn short_segment_spline() -> Handle<dyn GeomCurve> {
    let points = [
        Point3::default(),
        Point3::new(7.5e-8, 0., 0.),
        Point3::new(1.5e-7, 0., 0.),
        Point3::new(1., 0., 0.),
    ];
    let poles = Array1::from_shared(&points, 1);
    let knots = Array1::from_shared(&[0., 0.25, 0.5, 1.], 1);
    let multiplicities = Array1::from_shared(&[2, 1, 1, 2], 1);
    Handle::new(GeomBSplineCurve::new(&poles, &knots, &multiplicities, 1, false).unwrap()).cast()
}

fn recovery_wire() -> (Wire, Vertex) {
    let edge = BRepBuilderAPIMakeEdge::from_curve(&short_segment_spline())
        .unwrap()
        .edge()
        .unwrap();
    let first = topexp::vertices(&edge, false).0;
    let edges = [
        edge,
        recovery_line(Point3::new(1., 0., 0.), Point3::new(0., 1., 0.)),
        recovery_line(Point3::new(0., 1., 0.), Point3::default()),
    ];
    (recovery_wire_from_edges(edges), first)
}

fn recovery_wire_from_edges(edges: [Edge; 3]) -> Wire {
    let mut wire = Wire::default();
    Builder.make_wire(&mut wire);
    for edge in edges {
        Builder
            .add(&mut wire.clone().into_shape(), &edge.into_shape())
            .unwrap();
    }
    wire
}

fn recovery_line(first: Point3, last: Point3) -> Edge {
    BRepBuilderAPIMakeEdge::from_points(first, last)
        .unwrap()
        .edge()
        .unwrap()
}

#[test]
fn public_conversion_recovers_two_short_spans_without_moving_the_first_vertex() {
    let (wire, first) = recovery_wire();
    let face = BRepBuilderAPIMakeFace::from_plane_wire(Plane3::default(), &wire, true)
        .unwrap()
        .face()
        .unwrap();
    let converter = ShapeUpgradeShapeConvertToBezier::new(face.into_shape());
    converter.set_3d_conversion(true);
    assert!(converter.perform(true).unwrap());
    let applied = converter
        .context()
        .apply(&first.into_shape(), ShapeEnum::Shape)
        .unwrap();
    assert_eq!(
        BRepTool::pnt(&topods::vertex(&applied).unwrap()).unwrap(),
        Point3::default()
    );
}
