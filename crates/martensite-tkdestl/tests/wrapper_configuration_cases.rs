//! The wrapper configuration tests on the real STL node and provider.
//!
//! One bind per test, the node's three settings, its priority and the two units.
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{
    DeConfigurationContext, DeConfigurationNode, DeProvider, DeWrapper, new_configuration_node,
};
use martensite_tkdestl::de_stl::{DeStlConfigurationNode, DeStlProvider};
use martensite_tkernel::{
    message::ProgressRange, ncollection::List, standard::Handle, tcollection::AsciiString,
};

/// The `InitialConfiguration` lines of this one node.
const INITIAL_CONFIGURATION: &str = "\
global.priority.STL :\t Martensite
global.general.length.unit :\t 1
global.general.system.unit :\t 1
provider.STL.Martensite.read.merge.angle :\t 90
provider.STL.Martensite.read.brep :\t 0
provider.STL.Martensite.write.ascii :\t 1
";
/// The `TwoFormatsConfiguration` lines left by this node: a filtered format keeps its
/// priority and loses its provider keys.
const FILTERED_CONFIGURATION: &str = "\
global.priority.STL :\t Martensite
global.general.length.unit :\t 1
global.general.system.unit :\t 1
";

fn new_wrapper() -> (DeWrapper, Handle<DeStlConfigurationNode>) {
    let wrapper = DeWrapper::new();
    let node = new_configuration_node(DeStlConfigurationNode::new());
    assert!(wrapper.bind(&node.cast::<dyn DeConfigurationNode>()));
    (wrapper, node)
}

fn saved(wrapper: &DeWrapper) -> AsciiString {
    wrapper.save(true, &List::default(), &List::default())
}

/// The `SameConfiguration`: both load, with the same keys and values.
fn same_configuration(first: &AsciiString, second: &AsciiString) -> bool {
    let first_context = DeConfigurationContext::new();
    let second_context = DeConfigurationContext::new();
    if !first_context.load_str(first).unwrap() || !second_context.load_str(second).unwrap() {
        return false;
    }
    let first_map = first_context.get_internal_map();
    let second_map = second_context.get_internal_map();
    first_map.extent() == second_map.extent()
        && first_map
            .items()
            .all(|(key, value)| second_map.seek(key) == Some(value))
}

/// The `ChangeProviderValues`: provider values replaced, global lines kept.
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
    let (wrapper, _node) = new_wrapper();
    let changed = change_provider_values(&saved(&wrapper), value);
    assert!(wrapper.load(&changed, true).unwrap());
    assert!(same_configuration(&changed, &saved(&wrapper)));
}

#[test]
fn stl_a1_change_to_one() {
    assert_changed_values_save_stably("1");
}

#[test]
fn stl_a2_change_to_zero() {
    assert_changed_values_save_stably("0");
}

#[test]
fn stl_a3_initial_configuration() {
    let (wrapper, _node) = new_wrapper();
    assert!(same_configuration(
        &saved(&wrapper),
        &AsciiString::new(INITIAL_CONFIGURATION)
    ));
}

#[test]
fn stl_a4_filtered_format_keeps_only_its_priority() {
    let (wrapper, _node) = new_wrapper();
    let mut formats = List::default();
    formats.append(AsciiString::new("STEP"));
    formats.append(AsciiString::new("IGES"));
    assert!(same_configuration(
        &wrapper.save(true, &formats, &List::default()),
        &AsciiString::new(FILTERED_CONFIGURATION)
    ));
}

#[test]
fn stl_a5_empty_load_keeps_defaults() {
    let (wrapper, _node) = new_wrapper();
    let old = saved(&wrapper);
    assert!(wrapper.load(&AsciiString::new(""), true).unwrap());
    assert!(same_configuration(&saved(&wrapper), &old));
}

/// What the recorded rows could not reach with a settings-only node: values loaded through the
/// wrapper reach the provider it builds, which transfers with them.
#[test]
fn loaded_settings_reach_the_real_provider() {
    let (wrapper, node) = new_wrapper();
    let settings =
        "provider.STL.Martensite.write.ascii : 0\nprovider.STL.Martensite.read.merge.angle : 45\n";
    assert!(wrapper.load(&AsciiString::new(settings), true).unwrap());
    let parameters = node.get().unwrap().internal_parameters.snapshot();
    assert!(!parameters.write_ascii);
    assert_eq!(parameters.read_merge_angle, 45.0);
    let provider = node.get().unwrap().build_provider();
    let provider_node = provider.get().unwrap().get_node();
    assert_eq!(
        provider_node.object_address(),
        node.clone()
            .cast::<dyn DeConfigurationNode>()
            .object_address()
    );
    let direct = DeStlProvider::with_node(&node.cast());
    let mut empty = Default::default();
    let done = direct.write_shape_streams(&mut empty, &Shape::default(), &ProgressRange::default());
    assert_eq!(done, Ok(false));
}
