//! Whole-registry resource cycles through the wrapper, on shape-healing test nodes: an empty load
//! keeps everything, and a saved resource with every provider value changed reaches the nodes and
//! saves back unchanged. The values below are the ones the recorded replay printed.
mod support;
use support::reference_round_trip::{
    ChangedValues, changed_values, empty_load_after_values, empty_load_at_defaults,
};
use support::resource_compare::{context, same_configuration};

/// 11 nodes of 53 keys, 9 format priorities and 2 units.
const REGISTRY_KEYS: usize = 594;
const DEFAULT_ORDER: &str = "VRML:Martensite+,;IGES:Martensite+,;OBJ:Martensite+,;GLTF:Martensite+,;PLY:Martensite+,;XCAF:Martensite+,;STEP:Martensite+,ACME+,ZETA+,;STL:Martensite+,;BREP:Martensite+,;";

#[test]
fn an_empty_load_keeps_the_default_registry_byte_for_byte() {
    let case = empty_load_at_defaults();
    assert!(case.result);
    assert!(case.same);
    assert!(case.bytes);
    assert_eq!(case.order, DEFAULT_ORDER);
}

#[test]
fn an_empty_load_keeps_loaded_values_units_and_priorities() {
    let case = empty_load_after_values();
    assert!(case.result);
    assert!(case.same);
    assert_eq!(case.value, 3);
    assert_eq!(case.tolerance, 0.5);
    assert_eq!(case.unit, 10.0);
    assert!(
        case.order.contains("STEP:ACME+,Martensite+,ZETA-,;"),
        "{}",
        case.order
    );
}

/// Every provider key of the saved resource must hold the value, and the globals must not change.
fn assert_every_provider_value(case: &ChangedValues, value: &str) {
    let saved = context(&case.saved);
    assert_eq!(saved.get_internal_map().extent(), REGISTRY_KEYS);
    for (key, found) in saved.get_internal_map().items() {
        let name = String::from_utf8_lossy(key.as_bytes()).into_owned();
        let text = String::from_utf8_lossy(found.as_bytes()).into_owned();
        match name.starts_with("provider.") {
            true => assert_eq!(text, value, "{name}"),
            false if name.starts_with("global.priority.STEP") => {
                assert_eq!(text, "Martensite ACME ZETA")
            }
            false if name.starts_with("global.priority.") => {
                assert_eq!(text, "Martensite", "{name}")
            }
            false => assert_eq!(text, "1", "{name}"),
        }
    }
}

fn assert_changed_values_round_trip(value: &str, number: i32) {
    let case = changed_values(value);
    assert_eq!(case.keys, REGISTRY_KEYS);
    assert!(case.result);
    assert!(same_configuration(&case.changed, &case.saved));
    assert!(case.same);
    assert_eq!(case.order, DEFAULT_ORDER);
    assert_eq!(case.value, number);
    assert_eq!(case.tolerance, f64::from(number));
    assert_every_provider_value(&case, value);
    let tolerance = format!("provider.STEP.ACME.healing.tolerance3d :\t {value}\n");
    let node_text = String::from_utf8_lossy(case.node_text.as_bytes()).into_owned();
    assert!(node_text.contains(&tolerance), "{node_text}");
}

#[test]
fn every_provider_value_changed_to_one_saves_back_unchanged() {
    assert_changed_values_round_trip("1", 1);
}

#[test]
fn every_provider_value_changed_to_zero_saves_back_unchanged() {
    assert_changed_values_round_trip("0", 0);
}

/// The comparison above has to be able to fail: a load that stops at the wrapper leaves the nodes
/// at their defaults, so the saved resource must differ from the changed one.
#[test]
fn a_load_that_does_not_reach_the_nodes_does_not_match() {
    assert!(!changed_values("1").shallow_same);
    assert!(!changed_values("0").shallow_same);
}
