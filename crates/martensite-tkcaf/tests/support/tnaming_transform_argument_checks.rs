//! Independent geometric and graph checks for real transformation prerequisites.
use super::{
    document, modeled_box, named_shape, naming_function_graph::add_function,
    naming_transform_arguments::*,
};
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingNamedShape};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::TDataStdTreeNode,
    tdf::{TdfAttributeState, TdfLabel},
};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeWire};

fn named_on(
    label: &TdfLabel,
    shape: &martensite_tkbrep::topods::Shape,
) -> Handle<TNamingNamedShape> {
    let builder = TNamingBuilder::new(label).unwrap();
    builder.generated(shape).unwrap();
    builder.named_shape()
}

fn append_nonfunction(label: &TdfLabel) {
    let extra = label.find_child(100, true).unwrap();
    TDataStdTreeNode::set(label)
        .unwrap()
        .append(&TDataStdTreeNode::set(&extra).unwrap())
        .unwrap();
}

#[test]
fn previous_function_skips_real_non_function_sibling_nodes() {
    let (_document, object, first) = modeled_box();
    assert!(previous_function(&first).unwrap().is_null());
    append_nonfunction(&object.label());
    let second = add_function(
        &object,
        &guid_from_text("12e94556-6dbc-11d4-b9c8-0060b0ee281b"),
    )
    .unwrap();
    assert_eq!(previous_function(&second).unwrap().label(), first.label());
    set_object_argument(&second, 2, &object).unwrap();
    assert_eq!(object_argument(&second, 2).unwrap().label(), object.label());
    assert!(object_argument(&second, 3).unwrap().is_null());
    set_object_argument(&second, 3, &Handle::default()).unwrap();
    assert!(object_argument(&second, 3).unwrap().is_null());
    assert!(argument_value(&second, 3).unwrap().is_null());
}

fn linear_edge() -> martensite_tkbrep::topods::Edge {
    BRepBuilderAPIMakeEdge::from_points(Point3::new(5.0, 6.0, 7.0), Point3::new(5.0, 6.0, 17.0))
        .unwrap()
        .edge()
        .unwrap()
}

#[test]
fn linear_edge_axis_retains_geometry_origin_and_direction() {
    let document = document();
    let label = document.main().unwrap();
    let edge = linear_edge();
    let named = named_on(
        &label.find_child(1, true).unwrap(),
        &edge.clone().into_shape(),
    );
    let axis = axis_from_named(&named).unwrap().unwrap();
    assert_eq!(axis.location(), Point3::new(5.0, 6.0, 7.0));
    assert_eq!(axis.direction().z(), 1.0);
    assert_wire_axis(&label, &edge, axis);
    assert!(plane_from_named(&named).unwrap().is_none());
    assert!(axis_from_named(&Handle::default()).unwrap().is_none());
}

fn assert_wire_axis(
    label: &TdfLabel,
    edge: &martensite_tkbrep::topods::Edge,
    axis: martensite_tkmath::gp::Axis1,
) {
    let mut wire = BRepBuilderAPIMakeWire::new();
    wire.add_edge(edge).unwrap();
    let named_wire = named_on(
        &label.find_child(2, true).unwrap(),
        &wire.wire().unwrap().into_shape(),
    );
    assert_eq!(axis_from_named(&named_wire).unwrap().unwrap(), axis);
}

#[test]
fn named_box_face_supplies_the_native_mirror_plane() {
    let (_document, _object, function) = modeled_box();
    let label = super::naming_box_driver::result_label(&function)
        .unwrap()
        .find_child(3, false)
        .unwrap();
    let shape = named_shape(&label);
    let mut named: Handle<TNamingNamedShape> = Handle::default();
    label
        .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
        .unwrap();
    let plane = plane_from_named(&named).unwrap().unwrap();
    assert_eq!(plane.distance(Point3::new(190.0, 0.0, 0.0)), 0.0);
    assert_eq!(plane.distance(Point3::new(191.0, 0.0, 0.0)), 1.0);
    assert!(axis_from_named(&named).unwrap().is_none());
    assert!(!shape.is_null());
    assert!(plane_from_named(&Handle::default()).unwrap().is_none());
}

#[test]
fn empty_named_shape_has_neither_line_axis_nor_mirror_plane() {
    let empty = Handle::new(TNamingNamedShape::new());
    assert!(axis_from_named(&empty).unwrap().is_none());
    assert!(plane_from_named(&empty).unwrap().is_none());
}
