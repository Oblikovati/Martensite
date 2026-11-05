//! Independent exact point coordinates, reference failures, location and naming expectations.
use super::naming_function_values as values;
use super::{
    naming_draw_modeling::{function, reals},
    naming_function_graph::add_object,
    naming_point_driver::NamingPointDriver,
    naming_transform_arguments::set_object_argument,
    transform_checks::current,
};
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkcaf::tnaming::{TNaming, TNamingEvolution, TNamingIterator};
use martensite_tkernel::standard::{Handle, Transient};
use martensite_tklcaf::{
    tdata_std::TDataStdUAttribute,
    tdf::{TdfAttributeState, TdfChildIterator},
    tdoc_std::TDocStdDocument,
    tfunction::{TFunctionDriver, TFunctionFunction, TFunctionLogbook},
};
use martensite_tkmath::{gp::Transform3, toploc::Location};
fn point_function(
    document: &Handle<TDocStdDocument>,
    suffix: &str,
    offsets: [f64; 3],
) -> (Handle<TDataStdUAttribute>, Handle<TFunctionFunction>) {
    let object = add_object(document).unwrap();
    let function = function(&object, suffix, "Point_Function");
    reals(&function, offsets);
    (object, function)
}
fn execute(function: &Handle<TFunctionFunction>) -> (i32, Handle<TFunctionLogbook>) {
    let driver = NamingPointDriver::default();
    driver.init(&function.label());
    let mut log = TFunctionLogbook::set(&function.label()).unwrap();
    log.clear().unwrap();
    (driver.execute(&mut log).unwrap(), log)
}
fn coordinates(function: &Handle<TFunctionFunction>) -> [f64; 3] {
    let point = BRepTool::pnt(&topods::vertex(&current(function)).unwrap()).unwrap();
    [point.x(), point.y(), point.z()]
}
#[test]
fn cartesian_point_has_exact_coordinates_generated_history_and_valid_root_only() {
    let document = super::document();
    let (_, function) = point_function(&document, "5c", [-3.5, 2.0, 9.0]);
    let (status, log) = execute(&function);
    assert_eq!(status, 0);
    assert_eq!(coordinates(&function), [-3.5, 2.0, 9.0]);
    let result = values::result_label(&function).unwrap();
    assert_eq!(
        TNamingIterator::from_label(&result)
            .unwrap()
            .evolution()
            .unwrap(),
        TNamingEvolution::Primitive
    );
    assert!(!TdfChildIterator::new(&result, false).unwrap().more());
    assert!(log.get_valid().contains(&result));
    assert_eq!(function.get_failure(), 0);
}
#[test]
fn relative_point_uses_real_reference_coordinates_and_recomputed_offsets() {
    let document = super::document();
    let (base, source) = point_function(&document, "5c", [10.0, 20.0, 30.0]);
    assert_eq!(execute(&source).0, 0);
    let (_, relative) = point_function(&document, "5d", [1.0, -2.0, 3.0]);
    set_object_argument(&relative, 4, &base).unwrap();
    assert_eq!(execute(&relative).0, 0);
    assert_eq!(coordinates(&relative), [11.0, 18.0, 33.0]);
    reals(&source, [-4.0, 8.0, 12.0]);
    assert_eq!(execute(&source).0, 0);
    assert_eq!(execute(&relative).0, 0);
    assert_eq!(coordinates(&relative), [-3.0, 6.0, 15.0]);
}
#[test]
fn point_recompute_restores_previous_result_location_without_moving_source() {
    let document = super::document();
    let (_, function) = point_function(&document, "5c", [3.0, 4.0, 5.0]);
    assert_eq!(execute(&function).0, 0);
    let original = current(&function);
    TNaming::displace(
        &values::result_label(&function).unwrap(),
        &Location::new(Transform3::translation([10.0, -5.0, 2.0])),
        true,
    )
    .unwrap();
    reals(&function, [1.0, 2.0, 3.0]);
    assert_eq!(execute(&function).0, 0);
    assert_eq!(coordinates(&function), [11.0, -3.0, 5.0]);
    assert!(original.location().is_identity());
}
#[test]
fn relative_point_missing_or_empty_reference_reports_wrong_argument_fourteen() {
    let document = super::document();
    let (_, relative) = point_function(&document, "5d", [1.0, 2.0, 3.0]);
    let (status, log) = execute(&relative);
    assert_eq!(status, -1);
    assert_eq!(relative.get_failure(), 14);
    assert!(log.is_empty());
    let empty = add_object(&document).unwrap();
    let _seed = function(&empty, "5c", "Point_Function");
    set_object_argument(&relative, 4, &empty).unwrap();
    assert_eq!(execute(&relative).0, -1);
    assert_eq!(relative.get_failure(), 14);
}
#[test]
fn relative_point_wrong_shape_propagates_cast_failure_without_replacing_failure_code() {
    let (document, box_object, _box) = super::modeled_box();
    let (_, relative) = point_function(&document, "5d", [0.0; 3]);
    relative.set_failure(91).unwrap();
    set_object_argument(&relative, 4, &box_object).unwrap();
    let driver = NamingPointDriver::default();
    driver.init(&relative.label());
    let mut log = TFunctionLogbook::set(&relative.label()).unwrap();
    let prior_valid = log.get_valid().extent();
    assert!(driver.execute(&mut log).is_err());
    assert_eq!(relative.get_failure(), 91);
    assert_eq!(log.get_valid().extent(), prior_valid);
}
#[test]
fn point_driver_defaults_noops_and_missing_function_match_native_contract() {
    let document = super::document();
    let driver = NamingPointDriver::default();
    driver.init(&document.main().unwrap());
    let mut log = TFunctionLogbook::set(&document.main().unwrap()).unwrap();
    assert!(driver.must_execute(&log).unwrap());
    driver.validate(&log).unwrap();
    assert_eq!(driver.execute(&mut log).unwrap(), -1);
    let object = add_object(&document).unwrap();
    let function = function(&object, "5c", "Point_Function");
    assert_eq!(execute(&function).0, 0);
    assert_eq!(coordinates(&function), [0.0; 3]);
    assert_eq!(driver.dynamic_type().name(), "DNamingPointDriver");
}

#[test]
fn point_registry_factories_use_real_class_for_both_native_modes() {
    for suffix in ["5c", "5d"] {
        let driver = super::naming_driver_registration::native_driver(suffix).unwrap();
        assert_eq!(driver.dynamic_type().unwrap().name(), "DNamingPointDriver");
    }
}
