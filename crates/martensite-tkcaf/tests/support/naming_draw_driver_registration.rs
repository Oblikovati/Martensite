//! Exact ordered AddDriver setup of the E3/E4/E5/F1 cases with actual driver classes.
use super::naming_driver_registration::native_driver;
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    guid_from_text,
    tfunction::{TFunctionDriver, TFunctionDriverTable},
};
const CASE_DRIVERS: [(&str, &str); 15] = [
    ("43", "DNamingBoxDriver"),
    ("45", "DNamingCylinderDriver"),
    ("44", "DNamingSphereDriver"),
    ("54", "DNamingSelectionDriver"),
    ("55", "DNamingSelectionDriver"),
    ("56", "DNamingTransformationDriver"),
    ("57", "DNamingTransformationDriver"),
    ("58", "DNamingTransformationDriver"),
    ("59", "DNamingTransformationDriver"),
    ("48", "DNamingBooleanOperationDriver"),
    ("49", "DNamingBooleanOperationDriver"),
    ("50", "DNamingPrismDriver"),
    ("51", "DNamingRevolutionDriver"),
    ("52", "DNamingRevolutionDriver"),
    ("53", "DNamingFilletDriver"),
];
/// Register every original entry in original order in an actual isolated table; `case_driver_table()`.
pub(super) fn case_driver_table() -> Handle<TFunctionDriverTable> {
    let table = Handle::new(TFunctionDriverTable::new());
    register_case_drivers(&table, &CASE_DRIVERS.map(|(suffix, _)| suffix));
    table
}
/// Apply actual AddDriver calls in source order, including ignored duplicates; `register_case_drivers(&table,&["43"])`.
pub(super) fn register_case_drivers(table: &Handle<TFunctionDriverTable>, suffixes: &[&str]) {
    for suffix in suffixes {
        let guid = guid_from_text(&format!("12e945{suffix}-6dbc-11d4-b9c8-0060b0ee281b"));
        table.add_driver(&guid, &native_driver(suffix).unwrap(), 0);
    }
}
#[test]
fn case_registry_contains_exactly_all_fifteen_case_real_driver_types() {
    let table = case_driver_table();
    for (suffix, expected) in CASE_DRIVERS {
        let mut driver: Handle<dyn TFunctionDriver> = Handle::default();
        let guid = guid_from_text(&format!("12e945{suffix}-6dbc-11d4-b9c8-0060b0ee281b"));
        assert!(table.find_driver(&guid, &mut driver, 0));
        assert_eq!(driver.dynamic_type().unwrap().name(), expected);
    }
    let mut listing = Vec::new();
    table.dump(&mut listing).unwrap();
    assert_eq!(String::from_utf8(listing).unwrap().lines().count(), 15);
}

/// Original shared F4/E7/E8 initial AddDriver order; `sweep_driver_table()`.
pub(super) fn sweep_driver_table() -> Handle<TFunctionDriverTable> {
    let table = Handle::new(TFunctionDriverTable::new());
    register_case_drivers(
        &table,
        &["43", "54", "56", "57", "58", "59", "44", "50", "51", "52"],
    );
    table
}
