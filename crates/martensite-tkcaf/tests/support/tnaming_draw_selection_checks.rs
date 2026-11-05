//! Independent owned checks of actual native command-host selection command behavior.
use super::{
    modeled_box,
    naming_draw_selection::{self as commands, setup},
    naming_driver_registration::register_naming_drivers_into,
};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    guid_from_text,
    tdf::TdfAttributeState,
    tfunction::{TFunctionDriverTable, TFunctionFunction, TFunctionLogbook},
};
fn register_actual_drivers() -> Handle<TFunctionDriverTable> {
    let table = Handle::new(TFunctionDriverTable::new());
    register_naming_drivers_into(&table);
    setup::register_selection_driver(
        &table,
        &guid_from_text("12e94554-6dbc-11d4-b9c8-0060b0ee281b"),
    );
    table
}
#[test]
fn single_and_multiple_selection_visit_every_box_incidence_and_lower_kind() {
    let table = register_actual_drivers();
    let (document, object, _box_function) = modeled_box();
    let single = commands::test_selection(&document, &object, true, false).unwrap();
    assert_eq!(single.attempted, 85);
    assert!(single.failed_shapes.is_empty());
    assert!(single.diagnostics.is_empty(), "{:?}", single.diagnostics);
    let multiple = commands::test_selection(&document, &object, false, true).unwrap();
    assert_eq!(multiple.attempted, 5);
    assert!(multiple.failed_shapes.is_empty());
    let diagnostics = &multiple.diagnostics;
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    assert!(
        commands::solve_flat_from(&table, &single.first)
            .unwrap()
            .is_empty()
    );
}
#[test]
fn compute_function_without_registered_guid_returns_native_one() {
    let (document, _object, _box_function) = modeled_box();
    let function = TFunctionFunction::set_with_driver(
        &document.main().unwrap(),
        &guid_from_text("599cbadd-8d0b-4a3c-a7f7-d7a26d51480a"),
    )
    .unwrap();
    let table = Handle::new(TFunctionDriverTable::new());
    let mut log = TFunctionLogbook::set(&function.label()).unwrap();
    let valid_count = log.get_valid().len();
    log.set_touched(&function.label()).unwrap();
    assert_eq!(
        setup::compute_function(&table, &function, &mut log).unwrap(),
        1
    );
    assert_eq!(log.get_valid().len(), valid_count);
    assert!(log.is_modified(&function.label(), false).unwrap());
}

#[test]
fn source_diagnostic_rendering_preserves_modes_and_label_suffixes() {
    use commands::diagnostics::rendered_diagnostic as report;
    let warning = " Warning => Type migration to Compound";
    assert_eq!(
        report(warning, "0:1:223", false),
        format!("{warning} at 0:1:223")
    );
    assert_eq!(
        report("failure", "0:1:2", false),
        "Failed units: failure at 0:1:2"
    );
    assert_eq!(
        report(warning, "0:1:223", true),
        format!("Failed units: {warning}")
    );
}

#[test]
fn solve_from_missing_function_retains_and_emits_case_null_diagnostic() {
    let (document, _object, _box) = modeled_box();
    let first = super::naming_function_graph::add_object(&document).unwrap();
    let table = Handle::new(TFunctionDriverTable::new());
    let reports = commands::solve_flat_from(&table, &first).unwrap();
    assert_eq!(reports, ["DNamingSolveFlatFrom:: Null function is found!"]);
}

#[test]
fn typed_selection_failure_preserves_what_bytes_without_type_or_stack() {
    use martensite_tkernel::standard::Failure;
    let failure = Failure::new("typed fixture");
    let report = commands::diagnostics::rendered_selection_failure(&failure);
    assert_eq!(
        report,
        b"%%%INFO:Error: ::TestSingleSelection failed :typed fixture\n"
    );
}
