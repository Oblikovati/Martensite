//! Owned payload controls for language-managed cleanup; explicit Destroy stays callable.
use martensite_tkernel::{
    standard::{Handle, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceEntityIterator, InterfaceFileParameter, InterfaceMsg, InterfaceParamType,
};

#[test]
fn final_iterator_owner_releases_entities_after_cursor_detachment() {
    let entity = Handle::new(StandardTransient::default());
    let weak = entity.downgrade();
    let mut iterator = InterfaceEntityIterator::new();
    iterator.add_item(&entity);
    let remaining = iterator.clone();
    drop(entity);
    iterator.destroy();
    assert!(!iterator.more());
    assert!(remaining.more());
    assert!(!weak.upgrade().is_null());
    drop(iterator);
    assert!(!weak.upgrade().is_null());
    drop(remaining);
    assert!(weak.upgrade().is_null());
}

#[test]
fn escaped_iterator_list_retains_payload_until_its_final_owner() {
    let entity = Handle::new(StandardTransient::default());
    let weak = entity.downgrade();
    let mut iterator = InterfaceEntityIterator::new();
    iterator.add_item(&entity);
    let list = iterator.content_handle().unwrap();
    drop(entity);
    drop(iterator);
    assert_eq!(list.read().len(), 1);
    assert!(!weak.upgrade().is_null());
    drop(list);
    assert!(weak.upgrade().is_null());
}

#[test]
fn formatted_message_outlives_input_and_destroy_does_not_mutate_its_copy() {
    let key = String::from("owned integer %d");
    let mut message = InterfaceMsg::with_integer(&key, 37).unwrap();
    let retained = message.clone();
    drop(key);
    assert_eq!(message.value().unwrap(), "owned integer 37");
    message.destroy();
    assert_eq!(message.value().unwrap(), "owned integer %d");
    assert_eq!(retained.value().unwrap(), "owned integer 37");
    drop(message);
    assert_eq!(retained.value().unwrap(), "owned integer 37");
}

#[test]
fn replacing_formatted_message_preserves_an_independent_owned_copy() {
    let mut message = InterfaceMsg::with_text("first %s", "payload").unwrap();
    let retained = message.clone();
    message = InterfaceMsg::with_integer("second %d", 19).unwrap();
    assert_eq!(message.value().unwrap(), "second 19");
    assert_eq!(retained.value().unwrap(), "first payload");
    drop(message);
    assert_eq!(retained.value().unwrap(), "first payload");
}

#[test]
fn populated_parameter_destroy_is_a_no_op_and_clear_preserves_metadata() {
    let mut parameter = InterfaceFileParameter::new();
    parameter.init_text(b"#17", InterfaceParamType::ParamIdent);
    parameter.set_entity_number(17);
    parameter.destroy();
    assert!(parameter.has_value());
    assert_eq!(parameter.c_value(), b"#17");
    assert_eq!(parameter.param_type(), InterfaceParamType::ParamIdent);
    assert_eq!(parameter.entity_number(), 17);
    parameter.clear();
    assert!(!parameter.has_value());
    assert_eq!(parameter.param_type(), InterfaceParamType::ParamIdent);
    assert_eq!(parameter.entity_number(), 17);
}

#[test]
fn parameter_text_is_owned_and_replacement_does_not_change_its_copy() {
    let text = AsciiString::new("3.25");
    let mut parameter = InterfaceFileParameter::new();
    parameter.init(&text, InterfaceParamType::ParamReal);
    parameter.set_entity_number(6);
    let retained = parameter.clone();
    drop(text);
    parameter.init_text(b"#8", InterfaceParamType::ParamIdent);
    assert_eq!(parameter.entity_number(), 0);
    assert_eq!(parameter.c_value(), b"#8");
    drop(parameter);
    assert_eq!(retained.c_value(), b"3.25");
    assert_eq!(retained.param_type(), InterfaceParamType::ParamReal);
    assert_eq!(retained.entity_number(), 6);
}
