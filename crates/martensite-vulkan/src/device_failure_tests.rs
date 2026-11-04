//! Exact cause assertions are independent of the diagnostic display string.
use super::*;
use crate::RenderError;

#[test]
fn exact_backend_codes_preserve_distinct_failure_classes() {
    for (code, class) in [
        (-4, RenderDeviceFailureClass::DeviceLost),
        (-1, RenderDeviceFailureClass::OutOfMemory),
        (-2, RenderDeviceFailureClass::OutOfMemory),
        (-13, RenderDeviceFailureClass::Backend),
    ] {
        let failure = RenderDeviceFailure::backend("submit frame", code);
        assert_eq!(failure.operation(), "submit frame");
        assert_eq!(failure.class(), class);
        assert_eq!(failure.result_code(), Some(code));
        assert_eq!(failure.state(), None);
    }
}
#[test]
fn admission_refusal_retains_the_observed_state_and_no_backend_code() {
    for state in [
        RenderDeviceState::Lost,
        RenderDeviceState::Closing,
        RenderDeviceState::RetirementIncomplete,
        RenderDeviceState::Closed,
    ] {
        let error = RenderError::from(RenderDeviceFailure::admission("begin frame", state));
        let failure = error.device_failure().unwrap();
        assert_eq!(failure.state(), Some(state));
        assert_eq!(failure.class(), RenderDeviceFailureClass::Admission);
        assert_eq!(failure.result_code(), None);
        assert!(error.to_string().contains("begin frame"));
        assert!(std::error::Error::source(&error).is_some());
    }
}
#[test]
fn backend_cause_keeps_legacy_text_and_cleanup_companion() {
    let primary = RenderError::device_backend("wait fence", -1, "legacy primary".into());
    let cleanup = RenderError::device_backend("wait idle", -4, "legacy cleanup".into());
    let error = primary.with_observed_failure(cleanup.clone());
    assert_eq!(error.to_string(), "legacy primary");
    assert_eq!(error.device_failure().unwrap().result_code(), Some(-1));
    assert_eq!(error.observed_failure(), Some(&cleanup));
    assert_eq!(cleanup.device_failure().unwrap().result_code(), Some(-4));
}
#[test]
fn public_device_causes_and_states_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {
        // The generic bounds make both traits a compile-time requirement.
        let _: std::marker::PhantomData<T> = Default::default();
    }
    assert_send_sync::<RenderDeviceFailure>();
    assert_send_sync::<RenderDeviceFailureClass>();
    assert_send_sync::<RenderDeviceState>();
}
#[test]
fn owner_reservation_failure_retains_requested_slots_and_actual_standard_cause() {
    let cause = Vec::<u8>::new().try_reserve(usize::MAX).unwrap_err();
    let error = RenderError::from(RenderDeviceFailure::host_reservation(
        "reserve owners",
        usize::MAX,
        cause.clone(),
    ));
    let failure = error.device_failure().unwrap();
    assert_eq!(failure.class(), RenderDeviceFailureClass::HostReservation);
    assert_eq!(failure.reserved_owner_slots(), Some(usize::MAX));
    assert_eq!(failure.host_reservation_cause(), Some(&cause));
    assert_eq!(failure.result_code(), None);
    assert!(
        std::error::Error::source(failure)
            .unwrap()
            .is::<std::collections::TryReserveError>()
    );
    assert!(error.to_string().contains(&usize::MAX.to_string()));
}
