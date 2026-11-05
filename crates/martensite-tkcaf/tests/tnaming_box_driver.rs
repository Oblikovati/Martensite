//! Genuine box-driver prerequisite for the still-unported naming transformation pipeline.
#[path = "support/naming_box_driver.rs"]
mod naming_box_driver;
#[path = "support/naming_function_values.rs"]
mod naming_function_values;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
use martensite_tkcaf::tnaming::{TNaming, TNamingNamedShape, TNamingTool};
use martensite_tkernel::{standard::Handle, tcollection::ExtendedString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::{
    guid_from_text,
    tdf::{TdfAttributeState, TdfLabel},
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriver, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::{
    gp::{Transform3, Vector3},
    toploc::Location,
};
use naming_box_driver::{NamingBoxDriver, function_result, real_argument, result_label};

const ORIGIN_FACE_PLANES: [(usize, f64); 6] = [
    (2, 0.0),
    (2, 390.0),
    (0, 190.0),
    (1, 290.0),
    (0, 0.0),
    (1, 0.0),
];

fn new_box_function() -> (Handle<TDocStdDocument>, Handle<TFunctionFunction>) {
    let document = TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap();
    let label = document.main().unwrap().find_child(1, true).unwrap();
    let guid = guid_from_text("12e94543-6dbc-11d4-b9c8-0060b0ee281b");
    let function = TFunctionFunction::set_with_driver(&label, &guid).unwrap();
    (document, function)
}

fn set_dimensions(function: &Handle<TFunctionFunction>, sizes: [f64; 3]) {
    for (index, size) in sizes.into_iter().enumerate() {
        real_argument(function, index as i32 + 1)
            .unwrap()
            .set_value(size)
            .unwrap();
    }
}

fn execute(function: &Handle<TFunctionFunction>) -> Handle<TFunctionLogbook> {
    let driver = NamingBoxDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label()).unwrap();
    log.clear().unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), 0);
    assert_eq!(function.get_failure(), 0);
    log
}

fn result_shape(label: &TdfLabel) -> topods::Shape {
    let mut named: Handle<TNamingNamedShape> = Handle::default();
    assert!(
        label
            .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
            .unwrap()
    );
    TNamingTool::get_shape(&named).unwrap()
}

fn exact_vertex_bounds(shape: &topods::Shape) -> [f64; 6] {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for vertex in Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap() {
        let p = BRepTool::pnt(&topods::vertex(&vertex).unwrap()).unwrap();
        for (axis, coordinate) in [p.x(), p.y(), p.z()].into_iter().enumerate() {
            min[axis] = min[axis].min(coordinate);
            max[axis] = max[axis].max(coordinate);
        }
    }
    [min[0], min[1], min[2], max[0], max[1], max[2]]
}

#[test]
fn driver_builds_e2_dimensions_with_native_face_labels_and_validity() {
    let (_document, function) = new_box_function();
    set_dimensions(&function, [190.0, 290.0, 390.0]);
    function.set_failure(9999).unwrap();
    let log = execute(&function);
    let result = result_label(&function).unwrap();
    assert_eq!(
        result_shape(&result).shape_type().unwrap(),
        ShapeEnum::Solid
    );
    assert_native_face_names(&result, &log, &ORIGIN_FACE_PLANES);
    assert!(log.get_valid().contains(&result));
}

fn assert_native_face_names(
    result: &TdfLabel,
    log: &Handle<TFunctionLogbook>,
    planes: &[(usize, f64)],
) {
    for (index, (axis, coordinate)) in planes.iter().copied().enumerate() {
        let label = result.find_child(index as i32 + 1, false).unwrap();
        let shape = result_shape(&label);
        assert_eq!(shape.shape_type().unwrap(), ShapeEnum::Face);
        let bounds = exact_vertex_bounds(&shape);
        assert_eq!((bounds[axis], bounds[axis + 3]), (coordinate, coordinate));
        assert!(log.get_valid().contains(&label));
    }
}

#[test]
fn recompute_preserves_prior_location_and_changes_live_dimensions() {
    let (_document, function) = new_box_function();
    set_dimensions(&function, [190.0, 290.0, 390.0]);
    execute(&function);
    displace_box_result(&function);
    real_argument(&function, 1)
        .unwrap()
        .set_value(290.0)
        .unwrap();
    let log = execute(&function);
    assert_shifted_face_names(&function, &log);
    let shape = TNamingTool::get_shape(&function_result(&function).unwrap()).unwrap();
    assert_eq!(
        exact_vertex_bounds(&shape),
        [150.0, 40.0, 90.0, 440.0, 330.0, 480.0]
    );
}

#[test]
fn argument_access_creates_zero_and_preserves_existing_value() {
    let (_document, function) = new_box_function();
    let first = real_argument(&function, 1).unwrap();
    assert_eq!(first.get().unwrap().get(), 0.0);
    first.set_value(190.0).unwrap();
    assert_eq!(
        real_argument(&function, 1).unwrap().get().unwrap().get(),
        190.0
    );
    assert!(function_result(&function).unwrap().is_null());
}

#[test]
fn driver_without_function_returns_minus_one_and_validate_is_noop() {
    let document = TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap();
    let label = document.main().unwrap();
    let driver = NamingBoxDriver::default();
    driver.init(&label);
    let mut log = TFunctionLogbook::set(&label).unwrap();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert!(log.is_empty());
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
    assert!(log.is_empty());
}

fn displace_box_result(function: &Handle<TFunctionFunction>) {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(150.0, 40.0, 90.0));
    let result = result_label(function).unwrap();
    TNaming::displace(&result, &Location::new(transform), true).unwrap();
}

fn assert_shifted_face_names(function: &Handle<TFunctionFunction>, log: &Handle<TFunctionLogbook>) {
    let result = result_label(function).unwrap();
    let planes = [
        (2, 90.0),
        (2, 480.0),
        (0, 440.0),
        (1, 330.0),
        (0, 150.0),
        (1, 40.0),
    ];
    assert_native_face_names(&result, log, &planes);
}
