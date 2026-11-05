//! Source-bound cylinder driver checks with independent axis and label expectations.
use super::naming_function_values as values;
use super::{
    naming_cylinder_driver::{NamingCylinderDriver, load_cylinder_names},
    naming_function_graph::{add_function, add_object},
    naming_transform_arguments,
};
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkcaf::tnaming::{TNaming, TNamingBuilder, TNamingNamedShape, TNamingTool};
use martensite_tkernel::{standard::Handle, tcollection::ExtendedString};
use martensite_tkg3d::{
    geom::{GeomCylindricalSurface, GeomElementarySurface},
    top_abs::ShapeEnum,
};
use martensite_tklcaf::{
    guid_from_text,
    tdf::{TdfAttributeState, TdfLabel, TdfTagSource},
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriver, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::{
    gp::{Direction3, Line3, Point3, Transform3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};

fn axis_shape(infinite: bool, reversed: bool) -> topods::Shape {
    let line = Line3::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    );
    let edge = BRepBuilderAPIMakeEdge::from_lin_params(line, 2.0, 5.0)
        .unwrap()
        .edge()
        .unwrap();
    edge.set_infinite(infinite).unwrap();
    let shape = if reversed {
        edge.reversed()
    } else {
        edge.into_shape()
    };
    shape
        .located(
            &Location::new(Transform3::translation([3.0, 4.0, 5.0])),
            false,
        )
        .unwrap()
}

fn cylinder_fixture(axis: &topods::Shape) -> (Handle<TDocStdDocument>, Handle<TFunctionFunction>) {
    let document = TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap();
    let source = cylinder_axis_object(&document, axis);
    let object = add_object(&document).unwrap();
    let function = add_function(
        &object,
        &guid_from_text("12e94545-6dbc-11d4-b9c8-0060b0ee281b"),
    )
    .unwrap();
    naming_transform_arguments::set_object_argument(&function, 3, &source).unwrap();
    set_cylinder_radius(&function, 8.0);
    values::real_argument(&function, 2)
        .unwrap()
        .set_value(11.0)
        .unwrap();
    (document, function)
}

fn cylinder_axis_object(
    document: &Handle<TDocStdDocument>,
    axis: &topods::Shape,
) -> Handle<martensite_tklcaf::tdata_std::TDataStdUAttribute> {
    let source = add_object(document).unwrap();
    let seed = add_function(
        &source,
        &guid_from_text("00000000-0000-0000-0000-000000000000"),
    )
    .unwrap();
    TNamingBuilder::new(&values::result_label(&seed).unwrap())
        .unwrap()
        .generated(axis)
        .unwrap();
    source
}

fn set_cylinder_radius(function: &Handle<TFunctionFunction>, radius: f64) {
    values::real_argument(function, 1)
        .unwrap()
        .set_value(radius)
        .unwrap();
}

fn assert_face_tags(label: &TdfLabel, last: i32) {
    for tag in 1..=last {
        assert_eq!(
            named_at(&label.find_child(tag, false).unwrap())
                .shape_type()
                .unwrap(),
            ShapeEnum::Face
        );
    }
}

fn run_cylinder(function: &Handle<TFunctionFunction>) -> (i32, Handle<TFunctionLogbook>) {
    let driver = NamingCylinderDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label()).unwrap();
    log.clear().unwrap();
    (driver.execute(&mut log).unwrap(), log)
}

fn named_at(label: &TdfLabel) -> topods::Shape {
    let mut named: Handle<TNamingNamedShape> = Handle::default();
    assert!(
        label
            .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
            .unwrap()
    );
    TNamingTool::get_shape(&named).unwrap()
}

fn cylinder_surface(function: &Handle<TFunctionFunction>) -> Handle<GeomCylindricalSurface> {
    let label = values::result_label(function)
        .unwrap()
        .find_child(3, false)
        .unwrap();
    BRepTool::surface(&topods::face(&named_at(&label)).unwrap())
        .unwrap()
        .cast()
}

#[test]
fn cylinder_finite_axis_uses_first_intrinsic_vertex_even_when_reversed() {
    for reversed in [false, true] {
        let (_document, function) = cylinder_fixture(&axis_shape(false, reversed));
        let (status, log) = run_cylinder(&function);
        assert_eq!((status, function.get_failure()), (0, 0));
        let surface = cylinder_surface(&function);
        assert_eq!(surface.radius(), 8.0);
        assert_eq!(surface.position().location(), Point3::new(3.0, 4.0, 7.0));
        assert_eq!(
            surface.position().direction(),
            Direction3::new(0.0, 0.0, 1.0).unwrap()
        );
        assert!(
            log.get_valid()
                .contains(&values::result_label(&function).unwrap())
        );
    }
}

#[test]
fn cylinder_infinite_flag_uses_located_underlying_line_origin() {
    let (_document, function) = cylinder_fixture(&axis_shape(true, false));
    assert_eq!(run_cylinder(&function).0, 0);
    assert_eq!(
        cylinder_surface(&function).position().location(),
        Point3::new(3.0, 4.0, 5.0)
    );
}

#[test]
fn cylinder_fixed_face_tags_preserve_tag_source_and_recompute_location() {
    let (_document, function) = cylinder_fixture(&axis_shape(false, false));
    let result = values::result_label(&function).unwrap();
    TdfTagSource::set(&result).unwrap().set_value(37).unwrap();
    assert_eq!(run_cylinder(&function).0, 0);
    let location = Location::new(Transform3::translation([10.0, 20.0, 30.0]));
    TNaming::displace(&result, &location, true).unwrap();
    set_cylinder_radius(&function, 4.0);
    assert_eq!(run_cylinder(&function).0, 0);
    assert_eq!(named_at(&result).location(), &location);
    assert_eq!(TdfTagSource::set(&result).unwrap().get().unwrap().get(), 37);
    assert_face_tags(&result, 3);
    assert!(result.find_child(4, false).unwrap().is_null());
    assert_eq!(cylinder_surface(&function).radius(), 4.0);
}

#[test]
fn cylinder_wrong_shape_retains_previous_result_without_validating_log() {
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(1.0, 2.0, 3.0))
        .unwrap()
        .vertex()
        .unwrap();
    let (_document, function) = cylinder_fixture(&vertex.into_shape());
    let result = values::result_label(&function).unwrap();
    let previous = axis_shape(false, false);
    TNamingBuilder::new(&result)
        .unwrap()
        .generated(&previous)
        .unwrap();
    let (status, log) = run_cylinder(&function);
    assert_eq!((status, function.get_failure()), (-1, 13));
    assert!(log.is_empty());
    assert!(named_at(&result).is_equal(&previous));
}

#[test]
fn cylinder_partial_sweep_names_all_five_faces_without_allocating_tags() {
    let document = TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap();
    let label = document.main().unwrap();
    let mut maker = BRepPrimAPIMakeCylinder::with_angle(8.0, 11.0, 1.0).unwrap();
    load_cylinder_names(&label, &mut maker).unwrap();
    assert_face_tags(&label, 5);
    assert_eq!(named_at(&label).shape_type().unwrap(), ShapeEnum::Solid);
}

#[test]
fn cylinder_no_function_preserves_virtual_defaults_and_empty_log() {
    let document = TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap();
    let label = document.main().unwrap();
    let driver = NamingCylinderDriver::default();
    driver.init(&label);
    let mut log = TFunctionLogbook::set(&label).unwrap();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
    assert!(log.is_empty());
}

#[test]
fn cylinder_wire_uses_first_explored_edge() {
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeWire;
    let first = topods::edge(&axis_shape(false, false)).unwrap();
    let second = BRepBuilderAPIMakeEdge::from_points(
        Point3::new(3.0, 4.0, 10.0),
        Point3::new(8.0, 4.0, 10.0),
    )
    .unwrap()
    .edge()
    .unwrap();
    let wire = BRepBuilderAPIMakeWire::from_two_edges(&first, &second)
        .unwrap()
        .wire()
        .unwrap();
    let (_document, function) = cylinder_fixture(&wire.into_shape());
    assert_eq!(run_cylinder(&function).0, 0);
    assert_eq!(
        cylinder_surface(&function).position().location(),
        Point3::new(3.0, 4.0, 7.0)
    );
    assert_eq!(
        cylinder_surface(&function).position().direction(),
        Direction3::new(0.0, 0.0, 1.0).unwrap()
    );
}

#[test]
fn cylinder_nonlinear_edge_reports_wrong_axis_without_result() {
    use martensite_tkmath::gp::{Circle3, xoy};
    let circle = Circle3::new(xoy(), 5.0).unwrap();
    let edge = BRepBuilderAPIMakeEdge::from_circ(circle)
        .unwrap()
        .edge()
        .unwrap();
    let (_document, function) = cylinder_fixture(&edge.into_shape());
    let (status, log) = run_cylinder(&function);
    assert_eq!((status, function.get_failure()), (-1, 13));
    assert!(log.is_empty());
    assert!(values::function_result(&function).unwrap().is_null());
}

#[test]
fn cylinder_empty_axis_returns_wrong_axis_without_creating_result() {
    let (_document, function) = cylinder_fixture(&axis_shape(false, false));
    let named = naming_transform_arguments::argument_value(&function, 3).unwrap();
    named.get().unwrap().clear().unwrap();
    let (status, log) = run_cylinder(&function);
    assert_eq!((status, function.get_failure()), (-1, 13));
    assert!(log.is_empty());
    assert!(values::function_result(&function).unwrap().is_null());
}

#[test]
fn cylinder_undefined_native_null_axis_has_explicit_rust_failure_boundary() {
    let (_document, function) = cylinder_fixture(&axis_shape(false, false));
    let named = naming_transform_arguments::argument_value(&function, 3).unwrap();
    assert!(
        named
            .label()
            .forget_attribute_id(&TNamingNamedShape::get_id())
            .unwrap()
    );
    let driver = NamingCylinderDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label()).unwrap();
    let failure = driver.execute(&mut log).unwrap_err();
    assert_eq!(
        failure.to_string(),
        "Standard_Failure: null cylinder axis; expected named shape"
    );
    assert!(log.is_empty());
    assert!(values::function_result(&function).unwrap().is_null());
}
