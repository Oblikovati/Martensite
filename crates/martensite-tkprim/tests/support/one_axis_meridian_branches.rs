//! Actual primitive meridians exercise finite/infinite, pole, seam and equal-height branches.
use super::face_contracts::topology_count;
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{Edge, Face, Shape},
};
use martensite_tkernel::{
    precision,
    standard::real::{real_first, real_last},
};
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkprim::brep_prim::{
    BRepPrimCylinder, BRepPrimOneAxis, BRepPrimSphere, BRepPrimTorus,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use std::f64::consts::{PI, TAU};

fn one_infinite_cylinder(upper: bool, angle: f64) -> BRepPrimCylinder {
    let mut cylinder = BRepPrimCylinder::from_radius_height(1.0, 2.0);
    cylinder.set_angle(angle).unwrap();
    if upper {
        cylinder.set_vmax(real_last()).unwrap();
    } else {
        cylinder.set_vmin(real_first()).unwrap();
    }
    cylinder
}
fn assert_side_faces(primitive: &mut impl BRepPrimOneAxis, start: Orientation, end: Orientation) {
    let first = primitive.start_face().unwrap();
    let last = primitive.end_face().unwrap();
    assert_eq!(first.orientation(), start);
    assert_eq!(last.orientation(), end);
    assert!(primitive.start_face().unwrap().is_same(&first));
    assert!(primitive.end_face().unwrap().is_same(&last));
}
fn check_one_infinite_cylinder(upper: bool, angle: f64) {
    let mut cylinder = one_infinite_cylinder(upper, angle);
    let infinity = (cylinder.vmax_infinite(), cylinder.vmin_infinite());
    assert_eq!(infinity, (upper, !upper));
    let caps = (cylinder.has_top().unwrap(), cylinder.has_bottom().unwrap());
    assert_eq!(caps, (!upper, upper));
    let lateral = cylinder.lateral_face().unwrap();
    assert_eq!(topology_count(&lateral, ShapeEnum::Wire), 1);
    assert!(cylinder.lateral_face().unwrap().is_same(&lateral));
    let shell = cylinder.shell().unwrap();
    let expected = if angle == TAU { 2 } else { 4 };
    assert_eq!(topology_count(&shell, ShapeEnum::Face), expected);
    assert!(cylinder.shell().unwrap().is_same(&shell));
    let error = cylinder.set_angle(1.0).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
}
#[test]
fn exactly_one_infinite_end_keeps_only_the_opposite_cap() {
    for upper in [true, false] {
        for angle in [TAU, PI] {
            check_one_infinite_cylinder(upper, angle);
        }
    }
}
#[test]
fn half_infinite_side_faces_follow_the_finite_wire_branch_and_reject_missing_caps() {
    for upper in [true, false] {
        let mut cylinder = one_infinite_cylinder(upper, PI);
        assert_side_faces(&mut cylinder, Orientation::Forward, Orientation::Reversed);
        let start = cylinder.start_face().unwrap();
        let end = cylinder.end_face().unwrap();
        assert_eq!(topology_count(&start, ShapeEnum::Wire), 1);
        assert_eq!(topology_count(&end, ShapeEnum::Wire), 1);
        let cap = if upper {
            cylinder.top_face()
        } else {
            cylinder.bottom_face()
        };
        assert_eq!(cap.unwrap_err().exception_type(), "Standard_DomainError");
    }
}
fn pole_edge(edge: &Edge, face: &Face, latitude: f64, angle: f64) {
    assert!(BRepTool::degenerated(edge).unwrap());
    assert_eq!(BRepTool::range_on_face(edge, face).unwrap(), (0.0, angle));
    let pcurve = BRepTool::curve_on_face(edge, face).unwrap().pcurve;
    assert_eq!(pcurve.value(0.25).unwrap().y(), latitude);
}
fn check_spherical_poles(angle: f64) {
    let mut sphere = BRepPrimSphere::new(2.0).unwrap();
    sphere.set_angle(angle).unwrap();
    assert!(sphere.meridian_on_axis(sphere.vmin()).unwrap());
    assert!(sphere.meridian_on_axis(sphere.vmax()).unwrap());
    assert!(!sphere.has_top().unwrap() && !sphere.has_bottom().unwrap());
    let face = sphere.lateral_face().unwrap();
    pole_edge(&sphere.top_edge().unwrap(), &face, PI / 2.0, angle);
    pole_edge(&sphere.bottom_edge().unwrap(), &face, -PI / 2.0, angle);
    let top = sphere.axis_top_vertex().unwrap();
    let bottom = sphere.axis_bottom_vertex().unwrap();
    assert!(top.is_same(&sphere.top_end_vertex().unwrap()));
    assert!(bottom.is_same(&sphere.bottom_end_vertex().unwrap()));
    let expected = if angle == TAU { 1 } else { 3 };
    assert_eq!(
        topology_count(&sphere.shell().unwrap(), ShapeEnum::Face),
        expected
    );
}
#[test]
fn spherical_axis_endpoints_keep_degenerate_edge_ranges_and_pole_identity() {
    for angle in [TAU, PI / 2.0] {
        check_spherical_poles(angle);
    }
}
fn seam_latitudes(edge: &Edge, face: &Face, first: f64, last: f64) {
    assert!(BRepTool::is_closed_on_face(edge, face).unwrap());
    let forward = Edge::unchecked(edge.oriented(Orientation::Forward));
    let reversed = Edge::unchecked(edge.oriented(Orientation::Reversed));
    let a = edge_latitude(&forward, face);
    let b = edge_latitude(&reversed, face);
    assert!((a == first && b == last) || (a == last && b == first));
}
fn edge_latitude(edge: &Edge, face: &Face) -> f64 {
    let pcurve = BRepTool::curve_on_face(edge, face).unwrap().pcurve;
    pcurve.value(0.25).unwrap().y()
}
#[test]
fn closed_toroidal_meridian_shares_caps_and_keeps_both_lateral_seam_pcurves() {
    for angle in [TAU, PI] {
        let mut torus = BRepPrimTorus::new(3.0, 1.0).unwrap();
        torus.set_angle(angle).unwrap();
        assert!(torus.meridian_closed().unwrap());
        assert!(!torus.has_top().unwrap() && !torus.has_bottom().unwrap());
        let face = torus.lateral_face().unwrap();
        let top = torus.top_edge().unwrap();
        assert!(top.is_same(&torus.bottom_edge().unwrap()));
        seam_latitudes(&top, &face, 0.0, TAU);
        assert_eq!(
            topology_count(&torus.shell().unwrap(), ShapeEnum::Face),
            if angle == TAU { 1 } else { 3 }
        );
        assert!(torus.lateral_face().unwrap().is_same(&face));
    }
}
#[test]
fn closed_equal_height_meridian_keeps_side_orientation_and_rejects_axis_edge() {
    let mut torus = BRepPrimTorus::new(3.0, 1.0).unwrap();
    torus.set_angle(PI).unwrap();
    assert_side_faces(&mut torus, Orientation::Forward, Orientation::Reversed);
    let error = torus.axis_edge().unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
    let start = torus.start_face().unwrap();
    let end = torus.end_face().unwrap();
    assert_eq!(topology_count(&start, ShapeEnum::Wire), 1);
    assert_eq!(topology_count(&end, ShapeEnum::Wire), 1);
}
fn equal_height_torus() -> BRepPrimTorus {
    let mut torus = BRepPrimTorus::new(3.0, 1.0).unwrap();
    torus.set_vmin(0.0).unwrap();
    torus.set_vmax(PI).unwrap();
    torus.set_angle(PI / 4.0).unwrap();
    torus
}
fn offset_line(edge: &Edge, face: &Face, expected: f64) {
    let curve = BRepTool::curve_on_face(edge, face)
        .unwrap()
        .pcurve
        .cast::<Geom2dLine>();
    let line = curve.get().unwrap();
    assert!((line.location().y() - expected).abs() < 1e-14);
    assert_eq!((line.direction().x(), line.direction().y()), (1.0, 0.0));
}
#[test]
fn equal_height_open_meridian_reverses_sides_and_offsets_cap_pcurves() {
    let mut torus = equal_height_torus();
    assert!(!torus.meridian_closed().unwrap());
    let first = torus.meridian_value(torus.vmin()).unwrap();
    let last = torus.meridian_value(torus.vmax()).unwrap();
    assert!((first.y() - last.y()).abs() < precision::confusion());
    assert_side_faces(&mut torus, Orientation::Reversed, Orientation::Forward);
    let offset = 10.0 * precision::confusion();
    let start = torus.start_face().unwrap();
    let end = torus.end_face().unwrap();
    offset_line(&torus.start_top_edge().unwrap(), &start, offset);
    offset_line(&torus.start_bottom_edge().unwrap(), &start, -offset);
    offset_line(&torus.end_top_edge().unwrap(), &end, offset);
    offset_line(&torus.end_bottom_edge().unwrap(), &end, -offset);
}
#[test]
fn equal_height_axis_edge_is_degenerate_but_absent_from_side_face_wires() {
    let mut torus = equal_height_torus();
    let axis = torus.axis_edge().unwrap();
    assert!(BRepTool::degenerated(&axis).unwrap());
    let top = torus.axis_top_vertex().unwrap();
    assert!(top.is_same(&torus.axis_bottom_vertex().unwrap()));
    let face = torus.start_face().unwrap();
    let shell: Shape = torus.shell().unwrap().into();
    assert_eq!(topology_count(&face, ShapeEnum::Edge), 3);
    assert_axis_absent(&face, &axis);
    assert_axis_absent(&torus.end_face().unwrap(), &axis);
    let analyzer = BRepCheckAnalyzer::new(&shell, true, false, false).unwrap();
    assert!(!analyzer.is_valid().unwrap());
}

fn assert_axis_absent(face: &Face, axis: &Edge) {
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        assert!(!explorer.current().is_same(axis));
        explorer.next().unwrap();
    }
}
