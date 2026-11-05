//! Independent native Boolean driver contracts, using closed-form overlapping boxes.
use super::{
    execute_box, modeled_box,
    naming_boolean::{NamingBooleanDriver, execute_boolean},
    naming_box_driver::{function_result, result_label},
    naming_function_graph::{add_function, add_object},
    naming_transform::make_translation,
    naming_transform_arguments::set_object_argument,
    set_box_dimensions,
    transform_checks::current,
};
use martensite_tkbrep::{topexp::Explorer, topods::Shape, toptools::MapOfShape};
use martensite_tkcaf::tnaming::{TNamingEvolution, TNamingIterator};
use martensite_tkernel::standard::{Handle, Transient};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::TDataStdUAttribute,
    tdf::TdfAttributeState,
    tfunction::{TFunctionDriver, TFunctionFunction, TFunctionLogbook},
};
use martensite_tktopalgo::{brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp};
fn operation_guid(suffix: &str) -> martensite_tkernel::standard::Guid {
    guid_from_text(&format!("12e945{suffix}-6dbc-11d4-b9c8-0060b0ee281b"))
}
fn overlapping_boxes(
    offset: f64,
) -> (
    Handle<martensite_tklcaf::tdoc_std::TDocStdDocument>,
    Handle<TDataStdUAttribute>,
    Handle<TDataStdUAttribute>,
    Shape,
) {
    let (document, object, box_function) = modeled_box();
    let source = current(&box_function);
    let tool = add_object(&document).unwrap();
    let tool_box = add_function(&tool, &operation_guid("43")).unwrap();
    set_box_dimensions(&tool_box, 190.0);
    execute_box(&tool_box);
    make_translation(&tool, [offset, 0.0, 0.0]).unwrap();
    (document, object, tool, source)
}
fn boolean_function(
    object: &Handle<TDataStdUAttribute>,
    tool: &Handle<TDataStdUAttribute>,
    suffix: &str,
) -> Handle<TFunctionFunction> {
    let function = add_function(object, &operation_guid(suffix)).unwrap();
    set_object_argument(&function, 1, tool).unwrap();
    function
}
fn distinct_count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut found = MapOfShape::default();
    for item in Explorer::new(shape, kind, ShapeEnum::Shape).unwrap() {
        found.add(item);
    }
    found.len()
}
fn assert_root_history(function: &Handle<TFunctionFunction>, source: &Shape) {
    let label = result_label(function).unwrap();
    let iterator = TNamingIterator::from_label(&label).unwrap();
    assert_eq!(iterator.evolution().unwrap(), TNamingEvolution::Modify);
    assert!(iterator.old_shape().unwrap().is_same(source));
    assert!(iterator.new_shape().unwrap().is_same(&current(function)));
    assert!(
        TFunctionLogbook::set(&function.label())
            .unwrap()
            .get_valid()
            .contains(&label)
    );
}
fn assert_changed_face(old: &Shape, new: &Shape) {
    assert_eq!(old.shape_type().unwrap(), ShapeEnum::Face);
    assert_eq!(new.shape_type().unwrap(), ShapeEnum::Face);
    assert!(!old.is_same(new));
}
fn volume(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false).unwrap();
    properties.mass()
}
fn assert_modified_faces(function: &Handle<TFunctionFunction>) {
    let label = result_label(function)
        .unwrap()
        .find_child(1, false)
        .unwrap();
    let mut names = TNamingIterator::from_label(&label).unwrap();
    assert!(names.more());
    while names.more() {
        assert_eq!(names.evolution().unwrap(), TNamingEvolution::Modify);
        assert_changed_face(&names.old_shape().unwrap(), &names.new_shape().unwrap());
        names.next().unwrap();
    }
}
#[test]
fn cut_fuse_common_match_closed_form_volumes_and_keep_actual_history() {
    for (suffix, width) in [("48", 100.0), ("49", 290.0), ("4a", 90.0)] {
        let (_document, object, tool, source) = overlapping_boxes(100.0);
        let function = boolean_function(&object, &tool, suffix);
        assert_eq!(execute_boolean(&function).unwrap(), 0);
        let shape = current(&function);
        assert!(
            BRepCheckAnalyzer::new(&shape, true, false, false)
                .unwrap()
                .is_valid()
                .unwrap()
        );
        let expected = width * 290.0 * 390.0;
        assert!((volume(&shape) - expected).abs() < expected * 1e-12);
        assert_root_history(&function, &source);
        assert_modified_faces(&function);
        assert!(!function_result(&function).unwrap().is_empty());
    }
}
#[test]
fn section_records_real_face_generated_edges_and_reuses_result_tags() {
    let (_document, object, tool, source) = overlapping_boxes(100.0);
    let function = boolean_function(&object, &tool, "4b");
    assert_eq!(execute_boolean(&function).unwrap(), 0);
    let shape = current(&function);
    assert_eq!(distinct_count(&shape, ShapeEnum::Face), 0);
    assert!(distinct_count(&shape, ShapeEnum::Edge) > 0);
    assert_root_history(&function, &source);
    assert_eq!(execute_boolean(&function).unwrap(), 0);
    let result = result_label(&function).unwrap();
    assert!(!result.find_child(1, false).unwrap().is_null());
    assert!(result.find_child(2, false).unwrap().is_null());
}
#[test]
fn boolean_reports_missing_tool_unsupported_driver_and_empty_cut() {
    let (_document, object, _box_function) = modeled_box();
    let missing = add_function(&object, &operation_guid("48")).unwrap();
    assert_eq!(execute_boolean(&missing).unwrap(), -1);
    assert_eq!(missing.get_failure(), 14);
    let (_document, object, tool, _source) = overlapping_boxes(100.0);
    let unsupported = boolean_function(&object, &tool, "ff");
    assert_eq!(execute_boolean(&unsupported).unwrap(), -1);
    assert_eq!(unsupported.get_failure(), 15);
    let (_document, object, tool, _source) = overlapping_boxes(0.0);
    let cut = boolean_function(&object, &tool, "48");
    assert_eq!(execute_boolean(&cut).unwrap(), -1);
    assert_eq!(cut.get_failure(), 16);
}
#[test]
fn default_boolean_driver_preserves_native_noops_and_missing_predecessor() {
    let driver = NamingBooleanDriver::default();
    assert_eq!(
        driver.dynamic_type().name(),
        "DNamingBooleanOperationDriver"
    );
    let document = super::document();
    driver.init(&document.main().unwrap());
    let mut log = Handle::default();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
    let object = add_object(&document).unwrap();
    let function = add_function(&object, &operation_guid("48")).unwrap();
    assert_eq!(execute_boolean(&function).unwrap(), -1);
    assert_eq!(function.get_failure(), 0);
}
