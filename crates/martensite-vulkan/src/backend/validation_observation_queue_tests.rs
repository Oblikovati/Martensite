//! Actual callback and queue controls preserve poison as incomplete rather than empty success.
use super::*;
use std::{ffi::CString, panic::AssertUnwindSafe, sync::Arc};
struct FakePoisonedValidationMessages;
impl FakePoisonedValidationMessages {
    fn create(seed: &[&str]) -> ValidationMessages {
        let messages = ValidationMessages::new(true);
        for text in seed {
            messages.record_callback_message((*text).to_owned());
        }
        let outcome = std::panic::catch_unwind(AssertUnwindSafe(|| {
            let _owned_guard = messages.errors.lock().unwrap();
            panic!("named validation queue poison; retained observation must remain incomplete");
        }));
        assert!(outcome.is_err());
        messages
    }
}
fn invoke_actual_validation_callback(messages: &ValidationMessages, text: &str) -> vk::Bool32 {
    let text = CString::new(text).unwrap();
    let payload = vk::DebugUtilsMessengerCallbackDataEXT::default().message(&text);
    // SAFETY: the layer passes callback data that stays valid for the duration of this call.
    unsafe {
        validation_callback(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            &payload,
            std::ptr::from_ref(messages).cast_mut().cast(),
        )
    }
}
/// The callback prefixes severity and message ID; `recorded("first VUID")`.
fn recorded(text: &str) -> String {
    format!("[ERROR] unnamed: {text}")
}
#[test]
fn enabled_readable_empty_queue_is_the_only_clean_empty_state() {
    let messages = ValidationMessages::new(true);
    assert_eq!(messages.errors(), Ok(Vec::new()));
    assert!(messages.is_observable());
}
#[test]
fn disabled_observation_is_typed_incomplete_even_with_no_messages() {
    let messages = ValidationMessages::new(false);
    let failure = messages.errors().unwrap_err();
    assert_eq!(failure.cause(), ObservationCause::Disabled);
    assert!(!failure.was_enabled());
    assert!(!messages.is_observable());
    assert!(failure.retained_messages().is_empty());
}
#[test]
fn actual_callback_preserves_readable_messages_in_original_order() {
    let messages = ValidationMessages::new(true);
    assert_eq!(
        invoke_actual_validation_callback(&messages, "first VUID"),
        vk::FALSE
    );
    assert_eq!(
        invoke_actual_validation_callback(&messages, "second VUID"),
        vk::FALSE
    );
    assert_eq!(
        messages.errors().unwrap(),
        [recorded("first VUID"), recorded("second VUID")]
    );
    assert_eq!(
        messages.errors().unwrap(),
        [recorded("first VUID"), recorded("second VUID")]
    );
}
#[test]
fn poisoned_empty_and_nonempty_queues_never_report_clean_observation() {
    for seed in [&[][..], &["retained original"]] {
        let messages = FakePoisonedValidationMessages::create(seed);
        let failure = messages.errors().unwrap_err();
        assert_eq!(failure.cause(), ObservationCause::Poisoned);
        assert_eq!(failure.retained_messages(), seed);
        assert!(!messages.is_observable());
        assert!(messages.errors.is_poisoned());
    }
}
#[test]
fn actual_callback_after_poison_retains_new_messages_without_clearing_failure() {
    let messages = FakePoisonedValidationMessages::create(&["before poison"]);
    assert_eq!(
        invoke_actual_validation_callback(&messages, "after poison"),
        vk::FALSE
    );
    let failure = messages.errors().unwrap_err();
    assert_eq!(failure.cause(), ObservationCause::Poisoned);
    assert_eq!(
        failure.retained_messages(),
        ["before poison".to_owned(), recorded("after poison")]
    );
    assert_eq!(messages.errors().unwrap_err(), failure);
    assert!(messages.errors.is_poisoned());
}
#[test]
fn retained_arc_observer_preserves_observation_after_owner_retirement() {
    let owner = Arc::new(ValidationMessages::new(true));
    let observer = owner.clone();
    invoke_actual_validation_callback(&owner, "before retirement");
    let pointer = Arc::as_ptr(&owner);
    drop(owner);
    assert_eq!(Arc::strong_count(&observer), 1);
    assert_eq!(Arc::as_ptr(&observer), pointer);
    assert_eq!(observer.errors().unwrap(), [recorded("before retirement")]);
}
#[test]
fn ordered_thread_callbacks_do_not_drop_messages_or_depend_on_reader_thread() {
    let messages = Arc::new(ValidationMessages::new(true));
    let (sent, received) = std::sync::mpsc::channel();
    let first = messages.clone();
    let first_writer = std::thread::spawn(move || {
        invoke_actual_validation_callback(&first, "thread one");
        sent.send(()).unwrap();
    });
    let second = messages.clone();
    let second_writer = std::thread::spawn(move || {
        received.recv().unwrap();
        invoke_actual_validation_callback(&second, "thread two");
    });
    first_writer.join().unwrap();
    second_writer.join().unwrap();
    assert_eq!(
        messages.errors().unwrap(),
        [recorded("thread one"), recorded("thread two")]
    );
}
