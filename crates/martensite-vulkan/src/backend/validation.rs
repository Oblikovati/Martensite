//! Validation callback storage outlives the messenger and never prints from driver threads.
use crate::{
    RenderValidationObservationCause as ObservationCause,
    RenderValidationObservationFailure as ObservationFailure,
};
use ash::{ext::debug_utils, vk};
use std::{
    ffi::{CStr, c_void},
    sync::Mutex,
};
pub(super) struct ValidationMessages {
    enabled: bool,
    errors: Mutex<Vec<String>>,
}
impl ValidationMessages {
    /// Retain explicit observation coverage; `ValidationMessages::new(options.validation)` precedes messenger creation.
    pub(super) fn new(enabled: bool) -> Self {
        // A failed messenger constructor never publishes this storage as a live renderer.
        Self {
            enabled,
            errors: Mutex::new(Vec::new()),
        }
    }
    /// Whether the validation layer observes this device; `messages.is_enabled()`.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    /// Read complete observations only; `messages.errors()?` refuses disabled or poisoned evidence.
    pub fn errors(&self) -> Result<Vec<String>, ObservationFailure> {
        match self.errors.lock() {
            Ok(messages) if self.enabled => Ok(messages.clone()),
            Ok(messages) => Err(ObservationFailure::new(
                ObservationCause::Disabled,
                false,
                messages.clone(),
            )),
            Err(poisoned) => Err(ObservationFailure::new(
                ObservationCause::Poisoned,
                self.enabled,
                poisoned.into_inner().clone(),
            )),
        }
    }
    /// Check witness readability; `messages.is_observable()` requires enabled monitoring and an unpoisoned queue.
    #[cfg(test)]
    pub(super) fn is_observable(&self) -> bool {
        // The public typed read remains authoritative; this helper cannot make disabled evidence clean.
        self.enabled && !self.errors.is_poisoned()
    }
    fn record_callback_message(&self, message: String) {
        // Recover the actual queue but deliberately retain sticky poison for every later reader.
        let mut messages = self
            .errors
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        messages.push(message);
    }
    pub fn create_info(&self) -> vk::DebugUtilsMessengerCreateInfoEXT<'_> {
        vk::DebugUtilsMessengerCreateInfoEXT::default()
            //  warnings are failures too; INFO/VERBOSE stay off.
            .message_severity(recorded_severities())
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(validation_callback))
            .user_data(std::ptr::from_ref(self).cast_mut().cast())
    }
}
/// Severities that fail a validated run; `recorded_severities().contains(WARNING)`.
fn recorded_severities() -> vk::DebugUtilsMessageSeverityFlagsEXT {
    vk::DebugUtilsMessageSeverityFlagsEXT::ERROR | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
}
unsafe extern "system" fn validation_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _: vk::DebugUtilsMessageTypeFlagsEXT,
    message: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    context: *mut c_void,
) -> vk::Bool32 {
    // The Vulkan callback's valid-pointer contract is kept at this one thin FFI boundary.
    // SAFETY: the layer passes callback data that stays valid for the duration of this call.
    unsafe { record_validation_callback(severity, message, context) };
    vk::FALSE
}
unsafe fn record_validation_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    context: *mut c_void,
) {
    // Vulkan owns the callback duration; both pointers remain valid until the messenger is destroyed.
    // SAFETY: the pointer comes from a live Vulkan structure of this scope, and Vulkan's structure rules make it valid to read here.
    let (Some(message), Some(storage)) = (unsafe { message.as_ref() }, unsafe {
        context.cast::<ValidationMessages>().as_ref()
    }) else {
        return;
    };
    if !message.p_message.is_null() {
        // SAFETY: the pointer names a live NUL-terminated buffer that outlives this view.
        let text = unsafe { CStr::from_ptr(message.p_message) }.to_string_lossy();
        // SAFETY: the layer's strings are NUL-terminated and stay valid during this callback.
        let id = unsafe { optional_text(message.p_message_id_name) };
        storage.record_callback_message(format!("[{severity:?}] {id}: {text}"));
    }
}
/// Read a nullable layer-owned string; `optional_text(data.p_message_id_name)`.
unsafe fn optional_text(text: *const std::ffi::c_char) -> String {
    if text.is_null() {
        return String::from("unnamed");
    }
    // Non-null callback strings stay valid for the callback duration.
    // SAFETY: the pointer names a live NUL-terminated buffer that outlives this view.
    unsafe { CStr::from_ptr(text) }
        .to_string_lossy()
        .into_owned()
}
pub(super) struct ValidationMessenger {
    loader: debug_utils::Instance,
    handle: vk::DebugUtilsMessengerEXT,
}
impl ValidationMessenger {
    pub fn new(
        entry: &ash::Entry,
        instance: &ash::Instance,
        messages: &ValidationMessages,
    ) -> Result<Self, vk::Result> {
        let loader = debug_utils::Instance::new(entry, instance);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { loader.create_debug_utils_messenger(&messages.create_info(), None) }?;
        Ok(Self { loader, handle })
    }
}
impl Drop for ValidationMessenger {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe { self.loader.destroy_debug_utils_messenger(self.handle, None) };
    }
}
#[cfg(test)]
mod ownership_tests {
    use super::*;
    #[test]
    fn callback_storage_is_send_sync_and_keeps_one_stable_arc_payload() {
        fn require_send_sync<T: Send + Sync>() {
            // Callback storage crosses driver threads while its operation-owned mutex guards writes.
            let _: std::marker::PhantomData<T> = Default::default();
        }
        require_send_sync::<ValidationMessages>();
        let messages = std::sync::Arc::new(ValidationMessages::new(true));
        let pointer = std::sync::Arc::as_ptr(&messages);
        assert_eq!(
            messages.create_info().p_user_data.cast_const(),
            pointer.cast()
        );
        let observer = messages.clone();
        assert_eq!(std::sync::Arc::as_ptr(&observer), pointer);
        assert_eq!(std::sync::Arc::strong_count(&messages), 2);
        drop(observer);
        assert_eq!(std::sync::Arc::strong_count(&messages), 1);
        report_validation_owner_layout(&messages);
    }
    #[test]
    fn warnings_and_errors_are_recorded_but_info_and_verbose_are_not() {
        let messages = ValidationMessages::new(true);
        let severity = messages.create_info().message_severity;
        assert_eq!(severity, recorded_severities());
        assert!(severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING));
        assert!(severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR));
        assert!(!severity.intersects(
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
        ));
    }
    #[test]
    fn recorded_warning_names_severity_and_message_id() {
        let messages = ValidationMessages::new(true);
        let data = vk::DebugUtilsMessengerCallbackDataEXT::default()
            .message_id_name(c"WARNING-unit")
            .message(c"unit warning");
        let context = std::ptr::from_ref(&messages).cast_mut().cast();
        let severity = vk::DebugUtilsMessageSeverityFlagsEXT::WARNING;
        // SAFETY: the layer passes callback data that stays valid for the duration of this call.
        unsafe { record_validation_callback(severity, &data, context) };
        assert_eq!(
            messages.errors().unwrap(),
            ["[WARNING] WARNING-unit: unit warning"]
        );
    }
}

#[cfg(test)]
fn report_validation_owner_layout(messages: &std::sync::Arc<ValidationMessages>) {
    println!(
        "{{\"event\":\"validation-owner-layout\",\"payload_bytes\":{},\"arc_handle_bytes\":{},\"box_handle_bytes\":{},\"allocator_overhead_measured\":false}}",
        std::mem::size_of::<ValidationMessages>(),
        std::mem::size_of_val(messages),
        std::mem::size_of::<Box<ValidationMessages>>()
    );
}

#[cfg(test)]
#[path = "validation_observation_queue_tests.rs"]
mod observation_tests;
