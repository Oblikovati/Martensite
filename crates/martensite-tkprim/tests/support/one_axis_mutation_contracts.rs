//! Cache and retry semantics retain real builder mutations, including failed callbacks.
use super::{ObservedCylinder, face_contracts::topology_count};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{Edge, Face, Shape, Wire},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim::BRepPrimOneAxis;
use std::f64::consts::PI;

type FaceBuild = fn(&mut ObservedCylinder) -> Result<Face, Failure>;
type WireBuild = fn(&mut ObservedCylinder) -> Result<Wire, Failure>;
fn first_child(shape: &Shape, kind: ShapeEnum) -> Shape {
    let explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    assert!(explorer.more());
    explorer.current().clone()
}
fn cylinder_with_sides() -> ObservedCylinder {
    let mut cylinder = ObservedCylinder::new(2.0, false);
    cylinder.set_angle(PI).unwrap();
    cylinder
}
fn assert_locked(cylinder: &mut ObservedCylinder) {
    let error = cylinder.set_angle(1.0).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
}
fn assert_domain_failure(result: Result<Face, Failure>) {
    let failure = result.unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
}
fn assert_bound_pcurve(edge: &Edge, face: &Face) {
    let curve = BRepTool::curve_on_face(edge, face).unwrap().pcurve;
    let point = curve.value(0.25).unwrap();
    assert_eq!((point.x(), point.y()), (1.0, 0.25));
}
fn retry_mutating_callback(cylinder: &mut ObservedCylinder, build: FaceBuild, edge: &Edge) {
    let abandoned = cylinder.bound_faces.borrow()[0].clone();
    assert_bound_pcurve(edge, &abandoned);
    assert_locked(cylinder);
    cylinder.fail_after_pcurve.set(false);
    let completed = build(cylinder).unwrap();
    assert!(!completed.is_same(&abandoned));
    let old_wire = first_child(&abandoned, ShapeEnum::Wire);
    assert!(old_wire.is_same(&first_child(&completed, ShapeEnum::Wire)));
    assert_bound_pcurve(edge, &abandoned);
    assert_bound_pcurve(edge, &completed);
    let calls = cylinder.virtual_calls.borrow().clone();
    assert!(build(cylinder).unwrap().is_same(&completed));
    assert_eq!(*cylinder.virtual_calls.borrow(), calls);
}
#[test]
fn side_pcurve_mutation_survives_callback_failure_and_completed_wire_retry() {
    for (build, start) in [
        (ObservedCylinder::start_face as FaceBuild, true),
        (ObservedCylinder::end_face as FaceBuild, false),
    ] {
        let mut cylinder = cylinder_with_sides();
        cylinder.fail_after_pcurve.set(true);
        let error = build(&mut cylinder).unwrap_err();
        assert_eq!(error.exception_type(), "Standard_DomainError");
        let edge = if start {
            cylinder.start_edge()
        } else {
            cylinder.end_edge()
        }
        .unwrap();
        retry_mutating_callback(&mut cylinder, build, &edge);
    }
}
#[test]
fn lateral_pcurve_failure_after_completed_wire_retains_real_partial_binding() {
    let mut cylinder = cylinder_with_sides();
    let wire = cylinder.lateral_wire().unwrap();
    cylinder
        .fail_meridian_read
        .set(Some(cylinder.meridian_reads.get() + 1));
    assert_domain_failure(cylinder.lateral_face());
    let abandoned = cylinder.lateral_faces.borrow()[0].clone();
    assert!(first_child(&abandoned, ShapeEnum::Wire).is_same(&wire));
    let top = cylinder.top_edge().unwrap();
    let curve = BRepTool::curve_on_face(&top, &abandoned).unwrap().pcurve;
    assert_eq!(curve.value(0.25).unwrap().y(), 2.0);
    assert_locked(&mut cylinder);
    cylinder.fail_meridian_read.set(None);
    let completed = cylinder.lateral_face().unwrap();
    assert!(!completed.is_same(&abandoned));
    assert!(first_child(&completed, ShapeEnum::Wire).is_same(&wire));
}
fn post_wire_cap_failure(build: FaceBuild, wire_build: WireBuild) {
    let mut cylinder = cylinder_with_sides();
    let wire = wire_build(&mut cylinder).unwrap();
    cylinder
        .fail_meridian_read
        .set(Some(cylinder.meridian_reads.get() + 2));
    assert_domain_failure(build(&mut cylinder));
    assert_locked(&mut cylinder);
    assert!(wire_build(&mut cylinder).unwrap().is_same(&wire));
    cylinder.fail_meridian_read.set(None);
    let completed = build(&mut cylinder).unwrap();
    assert!(first_child(&completed, ShapeEnum::Wire).is_same(&wire));
    assert!(build(&mut cylinder).unwrap().is_same(&completed));
}
#[test]
fn cap_radius_failure_after_wire_keeps_locked_cached_topology_for_retry() {
    post_wire_cap_failure(ObservedCylinder::top_face, ObservedCylinder::top_wire);
    post_wire_cap_failure(ObservedCylinder::bottom_face, ObservedCylinder::bottom_wire);
}
fn remove_first_child(shape: &Shape, kind: ShapeEnum) {
    let child = first_child(shape, kind);
    let mut shared = shape.clone();
    shared.set_free(true).unwrap();
    BRepBuilder::default().remove(&mut shared, &child).unwrap();
}
#[test]
fn cached_face_preserves_external_wire_removal_without_rebuilding() {
    let mut cylinder = cylinder_with_sides();
    let face = cylinder.lateral_face().unwrap();
    remove_first_child(&face, ShapeEnum::Wire);
    let calls = cylinder.virtual_calls.borrow().clone();
    let cached = cylinder.lateral_face().unwrap();
    assert!(cached.is_same(&face));
    assert_eq!(topology_count(&cached, ShapeEnum::Wire), 0);
    assert_eq!(*cylinder.virtual_calls.borrow(), calls);
    assert_locked(&mut cylinder);
}
#[test]
fn cached_shell_preserves_external_face_removal_and_closed_flag() {
    let mut cylinder = cylinder_with_sides();
    let shell = cylinder.shell().unwrap();
    remove_first_child(&shell, ShapeEnum::Face);
    shell.set_closed(false).unwrap();
    let calls = cylinder.virtual_calls.borrow().clone();
    let cached = cylinder.shell().unwrap();
    assert!(cached.is_same(&shell));
    assert_eq!(topology_count(&cached, ShapeEnum::Face), 4);
    assert!(!cached.closed().unwrap());
    assert_eq!(*cylinder.virtual_calls.borrow(), calls);
    assert_locked(&mut cylinder);
}
