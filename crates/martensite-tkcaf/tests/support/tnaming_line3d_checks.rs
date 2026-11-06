//! Independent exact polyline topology, fixed naming tags and recompute/failure expectations.
use super::naming_function_values as values;
use super::{
    naming_draw_driver_registration::register_case_drivers,
    naming_draw_modeling::{compute, function, reals},
    naming_function_graph::add_object,
    naming_line3d::NamingLine3dDriver,
    naming_transform_arguments::set_object_argument,
    transform_checks::current,
};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{self, Shape},
};
use martensite_tkcaf::tnaming::{
    TNaming, TNamingEvolution, TNamingIterator, TNamingNamedShape, TNamingTool,
};
use martensite_tkernel::standard::{Handle, Transient};
use martensite_tklcaf::{
    tdf::TdfAttributeState,
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriver, TFunctionDriverTable, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::{gp::Transform3, toploc::Location};
struct LineFixture {
    document: Handle<TDocStdDocument>,
    table: Handle<TFunctionDriverTable>,
    line: Handle<TFunctionFunction>,
    points: Vec<Handle<TFunctionFunction>>,
}
fn line_points(
    document: &Handle<TDocStdDocument>,
    table: &Handle<TFunctionDriverTable>,
    line: &Handle<TFunctionFunction>,
    coordinates: &[[f64; 3]],
) -> Vec<Handle<TFunctionFunction>> {
    let mut points = Vec::new();
    for (index, coordinates) in coordinates.iter().enumerate() {
        let point_object = add_object(document).unwrap();
        let point = function(&point_object, "5c", "Point_Function");
        reals(&point, *coordinates);
        compute(table, &point);
        set_object_argument(line, index as i32 + 2, &point_object).unwrap();
        points.push(point);
    }
    points
}
fn line_fixture(coordinates: &[[f64; 3]], closed: i32) -> LineFixture {
    let document = super::document();
    let table = Handle::new(TFunctionDriverTable::new());
    register_case_drivers(&table, &["5c", "5d", "5e"]);
    let object = add_object(&document).unwrap();
    let line = function(&object, "5e", "Line3D_Function");
    values::integer_argument(&line, 1)
        .unwrap()
        .set_value(closed)
        .unwrap();
    values::integer_argument(&line, 2)
        .unwrap()
        .set_value(coordinates.len() as i32)
        .unwrap();
    let points = line_points(&document, &table, &line, coordinates);
    LineFixture {
        document,
        table,
        line,
        points,
    }
}
fn execute(fixture: &LineFixture) -> (i32, Handle<TFunctionLogbook>) {
    let driver = NamingLine3dDriver::default();
    driver.init(&fixture.line.label());
    let mut log = TFunctionLogbook::set(&fixture.line.label()).unwrap();
    log.clear().unwrap();
    (driver.execute(&mut log).unwrap(), log)
}
fn tag_shape(function: &Handle<TFunctionFunction>, tag: i32) -> Shape {
    let label = values::result_label(function)
        .unwrap()
        .find_child(tag, false)
        .unwrap();
    let mut named: Handle<TNamingNamedShape> = Handle::default();
    if !label
        .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
        .unwrap()
    {
        return Shape::default();
    }
    TNamingTool::get_shape(&named).unwrap()
}
fn point_coordinates(shape: &Shape) -> [f64; 3] {
    let point = BRepTool::pnt(&topods::vertex(shape).unwrap()).unwrap();
    [point.x(), point.y(), point.z()]
}
fn assert_edge(function: &Handle<TFunctionFunction>, tag: i32, start: [f64; 3], end: [f64; 3]) {
    let edge = topods::edge(&tag_shape(function, tag)).unwrap();
    let (first, last) = topexp::vertices(&edge, false);
    assert_eq!(point_coordinates(&first), start);
    assert_eq!(point_coordinates(&last), end);
}
#[test]
fn open_line_has_exact_ordered_segments_and_open_endpoint_tags() {
    let fixture = line_fixture(&[[0.0, 0.0, 0.0], [3.0, 0.0, 0.0], [3.0, 4.0, 0.0]], 0);
    let (status, log) = execute(&fixture);
    assert_eq!(status, 0);
    assert_edge(&fixture.line, 1, [0.0, 0.0, 0.0], [3.0, 0.0, 0.0]);
    assert_edge(&fixture.line, 2, [3.0, 0.0, 0.0], [3.0, 4.0, 0.0]);
    assert!(tag_shape(&fixture.line, 3).is_null());
    assert_eq!(
        point_coordinates(&tag_shape(&fixture.line, 4)),
        [0.0, 0.0, 0.0]
    );
    assert_eq!(
        point_coordinates(&tag_shape(&fixture.line, 5)),
        [3.0, 4.0, 0.0]
    );
    assert!(
        log.get_valid()
            .contains(&values::result_label(&fixture.line).unwrap())
    );
    assert_eq!(fixture.line.get_failure(), 0);
}
#[test]
fn closed_line_nonzero_type_names_closure_and_both_endpoints_then_clears_on_open_recompute() {
    let fixture = line_fixture(&[[0.0, 0.0, 0.0], [3.0, 0.0, 0.0], [3.0, 4.0, 0.0]], -7);
    assert_eq!(execute(&fixture).0, 0);
    assert_edge(&fixture.line, 3, [3.0, 4.0, 0.0], [0.0, 0.0, 0.0]);
    assert_eq!(
        point_coordinates(&tag_shape(&fixture.line, 4)),
        [0.0, 0.0, 0.0]
    );
    assert_eq!(
        point_coordinates(&tag_shape(&fixture.line, 5)),
        [3.0, 4.0, 0.0]
    );
    let closure = tag_shape(&fixture.line, 3);
    values::integer_argument(&fixture.line, 1)
        .unwrap()
        .set_value(0)
        .unwrap();
    assert_eq!(execute(&fixture).0, 0);
    assert!(tag_shape(&fixture.line, 3).is_null());
    assert!(!closure.is_null());
}
fn displace_line(function: &Handle<TFunctionFunction>) {
    TNaming::displace(
        &values::result_label(function).unwrap(),
        &Location::new(Transform3::translation([10.0, 20.0, 30.0])),
        true,
    )
    .unwrap();
}
#[test]
fn line_recompute_updates_reference_geometry_restores_location_and_keeps_generated_root() {
    let fixture = line_fixture(&[[0.0, 0.0, 0.0], [3.0, 0.0, 0.0]], 0);
    assert_eq!(execute(&fixture).0, 0);
    displace_line(&fixture.line);
    reals(&fixture.points[1], [8.0, 0.0, 0.0]);
    compute(&fixture.table, &fixture.points[1]);
    assert_eq!(execute(&fixture).0, 0);
    assert_edge(&fixture.line, 1, [10.0, 20.0, 30.0], [18.0, 20.0, 30.0]);
    assert_eq!(
        point_coordinates(&current(&fixture.points[1])),
        [8.0, 0.0, 0.0]
    );
    assert_eq!(
        TNamingIterator::from_label(&values::result_label(&fixture.line).unwrap())
            .unwrap()
            .evolution()
            .unwrap(),
        TNamingEvolution::Primitive
    );
}
#[test]
fn line_rejects_short_length_with_fourteen_and_identical_pair_with_eleven() {
    for length in [0, 1] {
        let fixture = line_fixture(&[], 0);
        values::integer_argument(&fixture.line, 2)
            .unwrap()
            .set_value(length)
            .unwrap();
        assert_eq!(execute(&fixture).0, -1);
        assert_eq!(fixture.line.get_failure(), 14);
    }
    let fixture = line_fixture(&[[0.0; 3], [0.0; 3]], 0);
    assert_eq!(execute(&fixture).0, -1);
    assert_eq!(fixture.line.get_failure(), 11);
}
#[test]
fn line_missing_named_attribute_is_explicit_undefined_native_boundary_with_notdone_code() {
    let fixture = line_fixture(&[], 0);
    values::integer_argument(&fixture.line, 2)
        .unwrap()
        .set_value(2)
        .unwrap();
    let driver = NamingLine3dDriver::default();
    driver.init(&fixture.line.label());
    let mut log = TFunctionLogbook::set(&fixture.line.label()).unwrap();
    assert!(driver.execute(&mut log).is_err());
    assert_eq!(fixture.line.get_failure(), 9999);
}
#[test]
fn line_driver_defaults_noops_missing_function_and_actual_factory_match_native() {
    let fixture = line_fixture(&[], 0);
    let driver = NamingLine3dDriver::default();
    driver.init(&fixture.document.main().unwrap());
    let mut log = TFunctionLogbook::set(&fixture.line.label()).unwrap();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
    assert_eq!(driver.dynamic_type().name(), "DNamingLine3DDriver");
    let actual = super::naming_driver_registration::native_driver("5e").unwrap();
    assert_eq!(actual.dynamic_type().unwrap().name(), "DNamingLine3DDriver");
}

#[test]
fn skipped_initial_degenerate_segment_fails_naming_without_partial_child_labels() {
    let fixture = line_fixture(&[[0.0; 3], [0.0; 3], [3.0, 0.0, 0.0]], 0);
    let (status, log) = execute(&fixture);
    assert_eq!(status, -1);
    assert_eq!(fixture.line.get_failure(), 15);
    let result = values::result_label(&fixture.line).unwrap();
    assert!(!current(&fixture.line).is_null());
    assert!(
        !martensite_tklcaf::tdf::TdfChildIterator::new(&result, false)
            .unwrap()
            .more()
    );
    assert!(log.is_empty());
}
#[test]
fn empty_named_point_reference_reports_fourteen_without_publishing_result() {
    let fixture = line_fixture(&[[0.0; 3], [3.0, 0.0, 0.0]], 0);
    let point_result = values::result_label(&fixture.points[0]).unwrap();
    martensite_tkcaf::tnaming::TNamingBuilder::new(&point_result).unwrap();
    assert_eq!(execute(&fixture).0, -1);
    assert_eq!(fixture.line.get_failure(), 14);
    assert!(current(&fixture.line).is_null());
}
