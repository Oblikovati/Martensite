//! Plugin destruction unregisters from the current global registry, as the original
//! DePluginHolder destructor does. A separate executable isolates that global state.
mod support;
use martensite_tkde::de::{DeConfigurationNode, DePluginHolder, DeWrapper, new_configuration_node};
use martensite_tkernel::{standard::Handle, tcollection::AsciiString};
use support::probe_node::PluginVendorA;

fn find_plugin(wrapper: &Handle<DeWrapper>) -> Handle<dyn DeConfigurationNode> {
    let mut node = Handle::default();
    wrapper.find(&AsciiString::new("PLG"), &AsciiString::new("A"), &mut node);
    node
}

fn fresh_global() -> Handle<DeWrapper> {
    let wrapper = Handle::new(DeWrapper::new());
    DeWrapper::set_global_wrapper(&wrapper);
    wrapper
}

fn normal_drop_releases_the_registered_node() {
    let wrapper = fresh_global();
    let holder = DePluginHolder::<PluginVendorA>::new().unwrap();
    let node = find_plugin(&wrapper);
    assert!(!node.is_null());
    let weak = node.downgrade();
    drop(node);
    assert!(!weak.upgrade().is_null());
    drop(holder);
    assert!(find_plugin(&wrapper).is_null());
    assert!(weak.upgrade().is_null());
}

fn duplicate_holder_drop_removes_the_first_registration() {
    let wrapper = fresh_global();
    let first = DePluginHolder::<PluginVendorA>::new().unwrap();
    let original = find_plugin(&wrapper);
    let second = DePluginHolder::<PluginVendorA>::new().unwrap();
    assert_eq!(find_plugin(&wrapper), original);
    drop(second);
    assert!(find_plugin(&wrapper).is_null());
    assert!(!original.is_null());
    drop(first);
    assert!(find_plugin(&wrapper).is_null());
}

fn replacement_registry_is_the_destructor_target() {
    let original = fresh_global();
    let holder = DePluginHolder::<PluginVendorA>::new().unwrap();
    let case_node = find_plugin(&original);
    let replacement = fresh_global();
    let replacement_node = new_configuration_node(PluginVendorA::default());
    assert!(replacement.bind(&replacement_node.cast()));
    drop(holder);
    assert!(find_plugin(&replacement).is_null());
    assert_eq!(find_plugin(&original), case_node);
    assert_ne!(
        case_node,
        replacement_node.cast::<dyn DeConfigurationNode>()
    );
}

#[test]
fn plugin_holder_destruction_preserves_registry_and_owner_contracts() {
    let previous = DeWrapper::global_wrapper();
    normal_drop_releases_the_registered_node();
    duplicate_holder_drop_removes_the_first_registration();
    replacement_registry_is_the_destructor_target();
    DeWrapper::set_global_wrapper(&previous);
}
