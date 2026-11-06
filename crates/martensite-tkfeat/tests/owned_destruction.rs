//! Implicit Rust field destruction releases actual topology graphs and point arrays.
mod support;
use martensite_tkbrep::topods::TShape;
use martensite_tkernel::{ncollection::Sequence, standard::WeakHandle};
use martensite_tkfeat::{brep_feat::BRepFeatBuilder, loc_ope::CSIntersector};
use martensite_tkmath::gp::{Direction3, Line3, Point3};
use support::make_box;

fn populated_intersector() -> (
    CSIntersector,
    WeakHandle<dyn TShape>,
    WeakHandle<dyn TShape>,
) {
    let shape = make_box((0., 0., 0.), (10., 10., 10.));
    let root = shape.tshape().downgrade();
    let mut intersector = CSIntersector::with_shape(&shape);
    let mut lines = Sequence::default();
    lines.append(Line3::new(Point3::new(-5., 5., 5.), Direction3::X));
    intersector.perform_lines(&lines).unwrap();
    assert_eq!(intersector.nb_points(1).unwrap(), 2);
    let point_face = intersector.point(1, 1).unwrap().face().tshape().downgrade();
    (intersector, root, point_face)
}

#[test]
fn intersector_destruction_releases_shape_and_populated_point_face_owners() {
    let (intersector, root, point_face) = populated_intersector();
    assert!(!root.upgrade().is_null());
    assert!(!point_face.upgrade().is_null());
    drop(intersector);
    assert!(root.upgrade().is_null());
    assert!(point_face.upgrade().is_null());
}

#[test]
fn cloned_intersector_keeps_resources_until_both_owners_are_destroyed() {
    let (intersector, root, point_face) = populated_intersector();
    let cloned = intersector.clone();
    drop(intersector);
    assert_eq!(cloned.nb_points(1).unwrap(), 2);
    assert!(!root.upgrade().is_null());
    assert!(!point_face.upgrade().is_null());
    drop(cloned);
    assert!(root.upgrade().is_null());
    assert!(point_face.upgrade().is_null());
}

fn populated_builder() -> (
    BRepFeatBuilder,
    WeakHandle<dyn TShape>,
    WeakHandle<dyn TShape>,
) {
    let basis = make_box((0., 0., 0.), (10., 10., 10.));
    let tool = make_box((5., 0., 0.), (10., 10., 10.));
    let basis_owner = basis.tshape().downgrade();
    let tool_owner = tool.tshape().downgrade();
    let mut builder = BRepFeatBuilder::new();
    builder.init_with_tool(&basis, &tool);
    builder.keep_part(&basis).unwrap();
    (builder, basis_owner, tool_owner)
}

#[test]
fn default_builder_destruction_releases_argument_tool_and_kept_shape_owners() {
    let (builder, basis, tool) = populated_builder();
    assert!(!basis.upgrade().is_null());
    assert!(!tool.upgrade().is_null());
    assert_eq!(builder.arguments().len(), 1);
    drop(builder);
    assert!(basis.upgrade().is_null());
    assert!(tool.upgrade().is_null());
}
