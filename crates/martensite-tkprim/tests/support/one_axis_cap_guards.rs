//! Actual pole/apex primitives reject caps; the strict axis threshold is observable.
use super::face_contracts::topology_count;
use martensite_tkbrep::{brep::BRepTool, topods::Face};
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim::{
    BRepPrimCone, BRepPrimCylinder, BRepPrimOneAxis, BRepPrimSphere,
};
use std::f64::consts::PI;

fn domain_face(result: Result<Face, Failure>) {
    let failure = result.unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
}
fn assert_absent_cap(primitive: &mut impl BRepPrimOneAxis, top: bool) {
    let face = if top {
        primitive.top_face()
    } else {
        primitive.bottom_face()
    };
    domain_face(face);
    let wire = if top {
        primitive.top_wire()
    } else {
        primitive.bottom_wire()
    };
    assert_eq!(wire.unwrap_err().exception_type(), "Standard_DomainError");
}
fn assert_cone_apex(top: bool) {
    let (first, last) = if top { (2.0, 0.0) } else { (0.0, 2.0) };
    let mut cone = BRepPrimCone::from_radii(first, last, 3.0).unwrap();
    let parameter = if top { cone.vmax() } else { cone.vmin() };
    assert!(cone.meridian_on_axis(parameter).unwrap());
    assert_eq!(
        (cone.has_top().unwrap(), cone.has_bottom().unwrap()),
        (!top, top)
    );
    assert_absent_cap(&mut cone, top);
    cone.set_angle(PI).unwrap();
    assert_apex_rim(&mut cone, top);
}
fn assert_apex_rim(cone: &mut impl BRepPrimOneAxis, top: bool) {
    let apex = if top {
        cone.top_edge()
    } else {
        cone.bottom_edge()
    }
    .unwrap();
    assert!(BRepTool::degenerated(&apex).unwrap());
    let lateral = cone.lateral_face().unwrap();
    assert_eq!(BRepTool::range_on_face(&apex, &lateral).unwrap(), (0.0, PI));
    assert_eq!(topology_count(&cone.shell().unwrap(), ShapeEnum::Face), 4);
}
#[test]
fn both_supported_cone_apices_reject_caps_before_locks_and_keep_degenerate_rims() {
    assert_cone_apex(true);
    assert_cone_apex(false);
}
#[test]
fn supported_sphere_poles_reject_both_cap_faces_and_wires_before_parameter_locks() {
    let mut sphere = BRepPrimSphere::new(2.0).unwrap();
    assert_absent_cap(&mut sphere, true);
    assert_absent_cap(&mut sphere, false);
    sphere.set_angle(PI).unwrap();
    assert_eq!(topology_count(&sphere.shell().unwrap(), ShapeEnum::Face), 3);
}
fn assert_cylinder_axis_threshold(radius: f64, on_axis: bool) {
    let mut cylinder = BRepPrimCylinder::from_radius_height(radius, 2.0);
    assert_eq!(cylinder.meridian_on_axis(cylinder.vmin()).unwrap(), on_axis);
    assert_eq!(cylinder.meridian_on_axis(cylinder.vmax()).unwrap(), on_axis);
    assert_eq!(cylinder.has_top().unwrap(), !on_axis);
    assert_eq!(cylinder.has_bottom().unwrap(), !on_axis);
    if on_axis {
        assert_absent_cap(&mut cylinder, true);
        assert_absent_cap(&mut cylinder, false);
        cylinder.set_angle(PI).unwrap();
        return;
    }
    assert!(!cylinder.top_face().unwrap().is_null());
    assert!(!cylinder.bottom_face().unwrap().is_null());
    assert!(!BRepTool::degenerated(&cylinder.top_edge().unwrap()).unwrap());
    assert!(!BRepTool::degenerated(&cylinder.bottom_edge().unwrap()).unwrap());
}
#[test]
fn strict_axis_distance_threshold_suppresses_caps_only_below_confusion() {
    let limit = precision::confusion();
    for (radius, on_axis) in [
        (0.0, true),
        (limit / 2.0, true),
        (limit, false),
        (2.0 * limit, false),
    ] {
        assert_cylinder_axis_threshold(radius, on_axis);
    }
}
