//! The wrapper configuration tests on the real IGES node, beside the real STEP node and, for the
//! filter, the real STL one.
//!
//! Their expected values are the fixture lines for these nodes, under the Martensite vendor.
use martensite_tkde::de::{
    DeConfigurationContext, DeConfigurationNode, DeWrapper, new_configuration_node,
};
use martensite_tkdeiges::de_iges::DeIgesConfigurationNode;
use martensite_tkdestep::de_step::DeStepConfigurationNode;
use martensite_tkdestl::de_stl::DeStlConfigurationNode;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::tcollection::AsciiString;

/// `InitialConfiguration` with both nodes bound.
const BOTH_INITIAL: &str = include_str!("fixtures/wrapper_step_iges_initial.conf");
/// `TwoFormatsConfiguration` with the STL node bound as well: filtered, it keeps only its priority.
const FILTERED: &str = include_str!("fixtures/wrapper_step_iges_filtered.conf");

fn both_wrapper() -> DeWrapper {
    let wrapper = DeWrapper::new();
    let step = new_configuration_node(DeStepConfigurationNode::default());
    assert!(wrapper.bind(&step.cast::<dyn DeConfigurationNode>()));
    let iges = new_configuration_node(DeIgesConfigurationNode::default());
    assert!(wrapper.bind(&iges.cast::<dyn DeConfigurationNode>()));
    wrapper
}

fn saved(wrapper: &DeWrapper) -> AsciiString {
    wrapper.save(true, &List::default(), &List::default())
}

fn saved_for_step_and_iges(wrapper: &DeWrapper) -> AsciiString {
    let mut formats = List::default();
    formats.append(AsciiString::new("STEP"));
    formats.append(AsciiString::new("IGES"));
    wrapper.save(true, &formats, &List::default())
}

/// the `SameConfiguration`, naming the first difference: both load, with the same keys and
/// values.
fn assert_same_configuration(actual: &AsciiString, expected: &AsciiString) {
    let (actual_context, expected_context) =
        (DeConfigurationContext::new(), DeConfigurationContext::new());
    assert!(actual_context.load_str(actual).unwrap());
    assert!(expected_context.load_str(expected).unwrap());
    let (actual_map, expected_map) = (
        actual_context.get_internal_map(),
        expected_context.get_internal_map(),
    );
    for (key, value) in expected_map.items() {
        assert_eq!(actual_map.seek(key), Some(value), "key {key:?}");
    }
    assert_eq!(actual_map.extent(), expected_map.extent());
}

/// the `ChangeProviderValues`: provider values replaced, global lines kept, others dropped.
fn change_provider_values(configuration: &AsciiString, value: &str) -> AsciiString {
    let text = String::from_utf8_lossy(configuration.as_bytes()).into_owned();
    let mut result = String::new();
    for line in text.split('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("provider.") {
            let Some(colon) = line.find(':') else {
                continue;
            };
            result += &format!("{} {value} \t\n", &line[..=colon]);
        } else if trimmed.starts_with("global.") {
            result += &format!("{line}\n");
        }
    }
    AsciiString::new(&result)
}

fn assert_changed_values_save_stably(value: &str) {
    let wrapper = both_wrapper();
    let changed = change_provider_values(&saved(&wrapper), value);
    assert!(wrapper.load(&changed, true).unwrap());
    assert_same_configuration(&changed, &saved(&wrapper));
}

#[test]
fn a1_change_to_one() {
    assert_changed_values_save_stably("1");
}

#[test]
fn a2_change_to_zero() {
    assert_changed_values_save_stably("0");
}

#[test]
fn a3_initial_configuration() {
    assert_same_configuration(&saved(&both_wrapper()), &AsciiString::new(BOTH_INITIAL));
}

#[test]
fn a4_two_formats_keep_their_keys_and_a_filtered_one_only_its_priority() {
    let wrapper = both_wrapper();
    let stl = new_configuration_node(DeStlConfigurationNode::new());
    assert!(wrapper.bind(&stl.cast::<dyn DeConfigurationNode>()));
    assert_same_configuration(
        &saved_for_step_and_iges(&wrapper),
        &AsciiString::new(FILTERED),
    );
}

#[test]
fn a5_empty_load_keeps_defaults() {
    let wrapper = both_wrapper();
    let old = saved(&wrapper);
    assert!(wrapper.load(&AsciiString::new(""), true).unwrap());
    assert_same_configuration(&saved(&wrapper), &old);
}
