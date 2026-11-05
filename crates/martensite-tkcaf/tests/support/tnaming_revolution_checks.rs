//! Independent annular-cylinder and sector fixtures for the actual revolution driver.
use super::naming_function_values as values;
use super::{
    naming_box_driver::{real_argument, result_label},
    naming_function_graph::{add_function, add_object},
    naming_revolution::{NamingRevolutionDriver, execute_revolution},
    naming_transform_arguments::set_object_argument,
    transform_checks::current,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tnaming::{TNamingEvolution, TNamingIterator};
use martensite_tkernel::{
    precision,
    standard::{Handle, Transient},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::TDataStdUAttribute,
    tdf::{TdfAttributeState, TdfReference},
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriver, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon},
    brep_gprop::BRepGProp,
};
use std::f64::consts::{FRAC_PI_2, PI};
fn rectangle_wire() -> martensite_tkbrep::topods::Wire {
    BRepBuilderAPIMakePolygon::from_four_points(
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
        Point3::new(3.0, 0.0, 4.0),
        Point3::new(1.0, 0.0, 4.0),
        true,
    )
    .unwrap()
    .wire()
    .unwrap()
}
fn basis_shape(wire: bool) -> Shape {
    let rectangle = rectangle_wire();
    if wire {
        return (*rectangle).clone();
    }
    (*BRepBuilderAPIMakeFace::from_wire(&rectangle, true)
        .unwrap()
        .face()
        .unwrap())
    .clone()
}
fn imported_object(
    document: &Handle<TDocStdDocument>,
    shape: &Shape,
) -> Handle<TDataStdUAttribute> {
    let object = add_object(document).unwrap();
    let result = object.label().new_child().unwrap();
    super::support::dnaming::load_imported_shape(&result, shape);
    TdfReference::set(&object.label(), &result).unwrap();
    object
}
fn analytic_inputs(
    document: &Handle<TDocStdDocument>,
    wire: bool,
) -> [Handle<TDataStdUAttribute>; 2] {
    let basis = imported_object(document, &basis_shape(wire));
    let axis = BRepBuilderAPIMakePolygon::from_two_points(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 4.0),
    )
    .unwrap();
    let axis = imported_object(document, &(*axis.wire().unwrap()).clone());
    [basis, axis]
}
fn requested_function(
    object: &Handle<TDataStdUAttribute>,
    suffix: &str,
) -> Handle<TFunctionFunction> {
    let guid = guid_from_text(&format!("12e945{suffix}-6dbc-11d4-b9c8-0060b0ee281b"));
    add_function(object, &guid).unwrap()
}
fn revolution(
    document: &Handle<TDocStdDocument>,
    objects: &[Handle<TDataStdUAttribute>; 2],
    suffix: &str,
    angle: f64,
    reverse: i32,
) -> Handle<TFunctionFunction> {
    let object = add_object(document).unwrap();
    let function = requested_function(&object, suffix);
    set_object_argument(&function, 1, &objects[0]).unwrap();
    set_object_argument(&function, 2, &objects[1]).unwrap();
    set_parameters(&function, angle, reverse);
    function
}
fn set_parameters(function: &Handle<TFunctionFunction>, angle: f64, reverse: i32) {
    real_argument(function, 3)
        .unwrap()
        .set_value(angle)
        .unwrap();
    values::integer_argument(function, 4)
        .unwrap()
        .set_value(reverse)
        .unwrap();
}
fn properties(function: &Handle<TFunctionFunction>) -> GProps {
    let mut properties = GProps::default();
    BRepGProp::volume_properties(&current(function), &mut properties, false, false, false).unwrap();
    properties
}
fn assert_volume(function: &Handle<TFunctionFunction>, expected: f64) {
    let mass = properties(function).mass();
    assert!(
        (mass - expected).abs() < expected * 1e-12,
        "volume {mass}; expected {expected}"
    );
}
fn assert_history(function: &Handle<TFunctionFunction>, cap_kind: ShapeEnum) {
    let result = result_label(function).unwrap();
    let root = TNamingIterator::from_label(&result).unwrap();
    assert_eq!(root.evolution().unwrap(), TNamingEvolution::Primitive);
    super::prism_checks::assert_four_edge_generated_faces(&result);
    for tag in [2, 3] {
        let named = TNamingIterator::from_label(&result.find_child(tag, false).unwrap()).unwrap();
        assert_eq!(named.evolution().unwrap(), TNamingEvolution::Primitive);
        assert_eq!(named.new_shape().unwrap().shape_type().unwrap(), cap_kind);
    }
    assert!(result.find_child(4, false).unwrap().is_null());
}
#[test]
fn full_annular_cylinder_from_face_or_closed_wire_has_exact_volume_and_seams() {
    for wire in [false, true] {
        let document = super::document();
        let objects = analytic_inputs(&document, wire);
        let function = revolution(&document, &objects, "51", 0.0, 0);
        assert_eq!(execute_revolution(&function).unwrap(), 0);
        assert_volume(&function, 32.0 * PI);
        assert_history(&function, ShapeEnum::Edge);
    }
}
#[test]
fn quarter_annular_sector_respects_any_nonzero_reverse_and_closed_form_centroid() {
    for reverse in [0, -9] {
        let document = super::document();
        let objects = analytic_inputs(&document, false);
        let function = revolution(&document, &objects, "52", FRAC_PI_2, reverse);
        assert_eq!(execute_revolution(&function).unwrap(), 0);
        assert_volume(&function, 8.0 * PI);
        let center = properties(&function).centre_of_mass();
        assert!((center.x() - 13.0 / (3.0 * PI)).abs() < 1e-12);
        let sign = if reverse == 0 { 1.0 } else { -1.0 };
        assert!((center.y() - sign * 13.0 / (3.0 * PI)).abs() < 1e-12);
        assert!((center.z() - 2.0).abs() < 1e-12);
        assert_history(&function, ShapeEnum::Face);
    }
}
#[test]
fn native_angle_missing_argument_and_unknown_guid_failures_are_preserved() {
    let document = super::document();
    let objects = analytic_inputs(&document, false);
    for (suffix, angle, code) in [("52", precision::confusion(), 14), ("ff", FRAC_PI_2, 15)] {
        let function = revolution(&document, &objects, suffix, angle, 0);
        assert_eq!(execute_revolution(&function).unwrap(), -1);
        assert_eq!(function.get_failure(), code);
    }
    let empty = add_object(&document).unwrap();
    let function = revolution(&document, &[empty, objects[1].clone()], "51", 0.0, 0);
    assert_eq!(execute_revolution(&function).unwrap(), -1);
    assert_eq!(function.get_failure(), 14);
}
#[test]
fn default_revolution_driver_has_native_rtti_and_noop_defaults() {
    let document = super::document();
    let driver = NamingRevolutionDriver::default();
    driver.init(&document.main().unwrap());
    assert_eq!(driver.dynamic_type().name(), "DNamingRevolutionDriver");
    let mut log = TFunctionLogbook::set(&document.main().unwrap()).unwrap();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
    assert!(log.is_empty());
}
fn displace_revolution(
    function: &Handle<TFunctionFunction>,
) -> martensite_tkmath::toploc::Location {
    let location =
        martensite_tkmath::toploc::Location::new(martensite_tkmath::gp::Transform3::translation([
            10.0, 20.0, 30.0,
        ]));
    martensite_tkcaf::tnaming::TNaming::displace(&result_label(function).unwrap(), &location, true)
        .unwrap();
    location
}
#[test]
fn recompute_preserves_displaced_result_without_moving_imported_basis() {
    let document = super::document();
    let objects = analytic_inputs(&document, false);
    let function = revolution(&document, &objects, "52", FRAC_PI_2, 0);
    assert_eq!(execute_revolution(&function).unwrap(), 0);
    let location = displace_revolution(&function);
    set_parameters(&function, PI, 0);
    assert_eq!(execute_revolution(&function).unwrap(), 0);
    assert_eq!(current(&function).location(), &location);
    assert_volume(&function, 16.0 * PI);
    let basis = super::naming_function_graph::object_value(&objects[0]).unwrap();
    assert!(basis.get().unwrap().get().unwrap().location().is_identity());
    assert_history(&function, ShapeEnum::Face);
}
#[test]
fn nonline_or_empty_axis_is_rejected_before_algorithm_execution() {
    let document = super::document();
    let objects = analytic_inputs(&document, false);
    let empty = add_object(&document).unwrap();
    for axis in [objects[0].clone(), empty] {
        let function = revolution(&document, &[objects[0].clone(), axis], "51", 0.0, 0);
        assert_eq!(execute_revolution(&function).unwrap(), -1);
        assert_eq!(function.get_failure(), 14);
    }
}
