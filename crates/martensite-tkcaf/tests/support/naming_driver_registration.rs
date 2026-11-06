//! Original RegisterNamingDrivers setup: all nine registrations use actual native fixtures.
use super::{
    naming_boolean::NamingBooleanDriver, naming_box_driver::NamingBoxDriver,
    naming_cylinder_driver::NamingCylinderDriver, naming_fillet::NamingFilletDriver,
    naming_line3d::NamingLine3dDriver, naming_point_driver::NamingPointDriver,
    naming_prism_driver::NamingPrismDriver, naming_revolution::NamingRevolutionDriver,
    naming_selection_driver::NamingSelectionDriver, naming_sphere_driver::NamingSphereDriver,
    naming_transform::NamingTransformDriver,
};
use martensite_tkernel::standard::Handle;
use martensite_tklcaf::{
    guid_from_text,
    tfunction::{TFunctionDriver, TFunctionDriverTable},
};
type Entry = (&'static str, Handle<dyn TFunctionDriver>);
/// Instantiate a supported real driver without replacing its operation; `native_driver("43")`.
pub(super) fn native_driver(suffix: &str) -> Option<Handle<dyn TFunctionDriver>> {
    Some(match suffix {
        "43" => Handle::new(NamingBoxDriver::default()).cast(),
        "44" => Handle::new(NamingSphereDriver::default()).cast(),
        "45" => Handle::new(NamingCylinderDriver::default()).cast(),
        "48" | "49" | "4a" => Handle::new(NamingBooleanDriver::default()).cast(),
        "50" => Handle::new(NamingPrismDriver::default()).cast(),
        "51" | "52" => Handle::new(NamingRevolutionDriver::default()).cast(),
        "53" => Handle::new(NamingFilletDriver::default()).cast(),
        "54" | "55" => Handle::new(NamingSelectionDriver::default()).cast(),
        "56" | "57" | "58" | "59" => Handle::new(NamingTransformDriver::default()).cast(),
        "5c" | "5d" => Handle::new(NamingPointDriver::default()).cast(),
        "5e" => Handle::new(NamingLine3dDriver::default()).cast(),
        _ => return None,
    })
}
fn driver_entries() -> [Entry; 9] {
    ["43", "44", "48", "49", "56", "57", "58", "59", "53"]
        .map(|suffix| (suffix, native_driver(suffix).unwrap()))
}
/// Original setup preserves ignored duplicate-binding return values; `register_naming_drivers()`.
pub(super) fn register_naming_drivers() {
    register_naming_drivers_into(&TFunctionDriverTable::get());
}
/// Isolate actual command driver state as in a separate command-host process; `register_naming_drivers_into(&table)`.
pub(super) fn register_naming_drivers_into(table: &Handle<TFunctionDriverTable>) {
    for (suffix, driver) in driver_entries() {
        let guid = guid_from_text(&format!("12e945{suffix}-6dbc-11d4-b9c8-0060b0ee281b"));
        table.add_driver(&guid, &driver, 0);
    }
}
#[test]
fn actual_registration_retains_each_native_driver_type() {
    register_naming_drivers();
    let table = TFunctionDriverTable::get();
    for (suffix, expected) in driver_entries() {
        let mut driver = Handle::default();
        let guid = guid_from_text(&format!("12e945{suffix}-6dbc-11d4-b9c8-0060b0ee281b"));
        assert!(table.find_driver(&guid, &mut driver, 0));
        assert_eq!(
            driver.dynamic_type().unwrap().name(),
            expected.dynamic_type().unwrap().name()
        );
    }
}

#[test]
fn unsupported_native_driver_suffix_has_no_instance() {
    assert!(native_driver("ff").is_none());
    assert!(native_driver("").is_none());
}
