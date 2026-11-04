//! Independent expected coverage states cannot pass the successful-empty predicate.
use super::*;
fn require_validation_send_sync<T: Send + Sync>() {
    // Public failures and their causes may outlive the rendering thread that recorded them.
    let _: std::marker::PhantomData<T> = Default::default();
}
#[test]
fn owned_validation_failures_and_causes_are_send_sync() {
    require_validation_send_sync::<RenderValidationObservationFailure>();
    require_validation_send_sync::<RenderValidationObservationCause>();
    require_validation_send_sync::<Result<Vec<String>, RenderValidationObservationFailure>>();
}
#[test]
fn incomplete_observation_retains_typed_cause_operation_and_original_messages() {
    let failure = RenderValidationObservationFailure::new(
        RenderValidationObservationCause::Poisoned,
        true,
        vec!["first".into(), "second".into()],
    );
    assert_eq!(failure.operation(), "read validation observations");
    assert_eq!(failure.cause(), RenderValidationObservationCause::Poisoned);
    assert!(failure.was_enabled());
    assert_eq!(failure.retained_messages(), ["first", "second"]);
    assert_eq!(failure.source().unwrap().to_string(), "Poisoned");
    assert!(failure.to_string().contains("retained_messages=2"));
    assert!(
        failure
            .to_string()
            .contains("expected enabled, available, unpoisoned")
    );
}
#[test]
fn unavailable_backend_cannot_be_mistaken_for_empty_validation() {
    let failure = RenderValidationObservationFailure::unavailable();
    let result: Result<Vec<String>, _> = Err(failure.clone());
    assert!(!matches!(&result, Ok(messages) if messages.is_empty()));
    assert!(matches!(result, Err(ref actual) if actual == &failure));
    assert_eq!(
        failure.cause(),
        RenderValidationObservationCause::Unavailable
    );
    assert!(!failure.was_enabled());
    assert!(failure.retained_messages().is_empty());
}

struct FakeValidationInjectedBackend;
impl crate::RenderBackend for FakeValidationInjectedBackend {
    fn render(
        &mut self,
        _: &crate::RenderFrame,
    ) -> Result<crate::FrameCapture, crate::RenderError> {
        // This injected device has no Vulkan observer; validation must not depend on a render call.
        Err(crate::error::rejected(
            "injected validation fixture",
            "no rendering requested",
        ))
    }
}
#[test]
fn actual_injected_backend_forwarder_returns_unavailable_without_rendering() {
    let backend =
        crate::driver_backend::DriverBackend::Injected(Box::new(FakeValidationInjectedBackend));
    let result = backend.validation_errors();
    assert!(!matches!(&result, Ok(messages) if messages.is_empty()));
    assert_eq!(
        result.unwrap_err().cause(),
        RenderValidationObservationCause::Unavailable
    );
}
