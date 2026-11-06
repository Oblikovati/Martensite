//! The wrapper configuration tests on the real STEP node and provider, alone and beside the real
//! STL node.
//!
//! Their expected values are the fixture lines for these nodes, under the Martensite vendor.
#[path = "support/memory_stream.rs"]
mod memory_stream;

use martensite_tkde::de::{
    DeConfigurationContext, DeConfigurationNode, DeProvider, DeProviderWriteStreamList,
    DeProviderWriteStreamNode, DeWrapper, new_configuration_node,
};
use martensite_tkdestep::de_step::{
    DeStepConfigurationNode, DeStepProvider, DeStepWriteModeStepSchema,
};
use martensite_tkdestl::de_stl::DeStlConfigurationNode;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use memory_stream::memory_ostream;

/// `InitialConfiguration` and the `TwoFormatsConfiguration` of the STEP node alone: the filter keeps
/// STEP, so the two are the same 105 lines.
const STEP_CONFIGURATION: &str = include_str!("fixtures/wrapper_step.conf");
/// `InitialConfiguration` with both nodes bound: 109 lines.
const BOTH_INITIAL: &str = include_str!("fixtures/wrapper_step_stl_initial.conf");
/// `TwoFormatsConfiguration` with both nodes bound: STL is filtered, and keeps only its priority.
const BOTH_FILTERED: &str = include_str!("fixtures/wrapper_step_stl_filtered.conf");

fn bind_step(wrapper: &DeWrapper) -> Handle<DeStepConfigurationNode> {
    let node = new_configuration_node(DeStepConfigurationNode::default());
    assert!(wrapper.bind(&node.cast::<dyn DeConfigurationNode>()));
    node
}

fn step_wrapper() -> (DeWrapper, Handle<DeStepConfigurationNode>) {
    let wrapper = DeWrapper::new();
    let node = bind_step(&wrapper);
    (wrapper, node)
}

fn both_wrapper() -> DeWrapper {
    let wrapper = DeWrapper::new();
    bind_step(&wrapper);
    let node = new_configuration_node(DeStlConfigurationNode::new());
    assert!(wrapper.bind(&node.cast::<dyn DeConfigurationNode>()));
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
        let found = actual_map.seek(key);
        assert_eq!(found, Some(value), "key {key:?}");
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

fn assert_changed_values_save_stably(wrapper: &DeWrapper, value: &str) {
    let changed = change_provider_values(&saved(wrapper), value);
    assert!(wrapper.load(&changed, true).unwrap());
    assert_same_configuration(&changed, &saved(wrapper));
}

fn assert_empty_load_keeps_defaults(wrapper: &DeWrapper) {
    let old = saved(wrapper);
    assert!(wrapper.load(&AsciiString::new(""), true).unwrap());
    assert_same_configuration(&saved(wrapper), &old);
}

/// A box written to memory by the provider.
fn written_by(provider: &DeStepProvider) -> Vec<u8> {
    let (mut stream, written) = memory_ostream();
    let mut list = DeProviderWriteStreamList::default();
    let key = AsciiString::new("box.step");
    list.append(DeProviderWriteStreamNode::new(&key, &mut stream));
    let solid = BRepPrimAPIMakeBox::from_sizes(1.0, 2.0, 3.0)
        .unwrap()
        .solid()
        .unwrap();
    let done =
        provider.write_shape_streams(&mut list, &solid.into_shape(), &ProgressRange::default());
    assert_eq!(done, Ok(true));
    written.contents()
}

mod step_node {
    use super::*;

    #[test]
    fn a1_change_to_one() {
        assert_changed_values_save_stably(&step_wrapper().0, "1");
    }

    #[test]
    fn a2_change_to_zero() {
        assert_changed_values_save_stably(&step_wrapper().0, "0");
    }

    #[test]
    fn a3_initial_configuration() {
        let expected = AsciiString::new(STEP_CONFIGURATION);
        assert_same_configuration(&saved(&step_wrapper().0), &expected);
    }

    #[test]
    fn a4_filter_keeps_the_format() {
        let expected = AsciiString::new(STEP_CONFIGURATION);
        assert_same_configuration(&saved_for_step_and_iges(&step_wrapper().0), &expected);
    }

    #[test]
    fn a5_empty_load_keeps_defaults() {
        assert_empty_load_keeps_defaults(&step_wrapper().0);
    }

    /// What the recorded rows could not reach with a settings-only node: values loaded through the
    /// wrapper reach the provider it builds, which transfers with them.
    #[test]
    fn loaded_settings_reach_the_real_provider() {
        let (wrapper, node) = step_wrapper();
        let settings = "provider.STEP.Martensite.write.schema : 5\n\
                        provider.STEP.Martensite.read.precision.val : 0.5\n";
        assert!(wrapper.load(&AsciiString::new(settings), true).unwrap());
        let parameters = node.get().unwrap().internal_parameters.snapshot();
        assert_eq!(
            parameters.write_schema,
            DeStepWriteModeStepSchema::AP242_DIS
        );
        assert_eq!(parameters.read_precision_val, 0.5);
        let provider = node.get().unwrap().build_provider();
        let provider_node = provider.get().unwrap().get_node();
        let bound = node.clone().cast::<dyn DeConfigurationNode>();
        assert_eq!(provider_node.object_address(), bound.object_address());
        let written = written_by(&DeStepProvider::with_node(&node.cast()));
        let text = String::from_utf8_lossy(&written);
        assert!(
            text.contains("AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF"),
            "{text}"
        );
    }
}

mod step_and_stl_nodes {
    use super::*;

    #[test]
    fn a1_change_to_one() {
        assert_changed_values_save_stably(&both_wrapper(), "1");
    }

    #[test]
    fn a2_change_to_zero() {
        assert_changed_values_save_stably(&both_wrapper(), "0");
    }

    #[test]
    fn a3_initial_configuration() {
        let expected = AsciiString::new(BOTH_INITIAL);
        assert_same_configuration(&saved(&both_wrapper()), &expected);
    }

    #[test]
    fn a4_filtered_format_keeps_only_its_priority() {
        let expected = AsciiString::new(BOTH_FILTERED);
        assert_same_configuration(&saved_for_step_and_iges(&both_wrapper()), &expected);
    }

    #[test]
    fn a5_empty_load_keeps_defaults() {
        assert_empty_load_keeps_defaults(&both_wrapper());
    }
}
