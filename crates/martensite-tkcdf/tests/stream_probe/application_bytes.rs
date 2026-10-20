//! CDF's formatting catch preserves bytes, while its original I/O cause stays separately owned.
use super::super::support::{FakeDocument, new_document};
use super::{FakeProbeApplication, FakeProbeInput};
use martensite_tkcdf::{
    cdf::CdfApplicationServices,
    pcdm::{PcdmInputStream, PcdmReaderStatus, PcdmStreamFailure},
};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle},
    storage::state,
};
use std::sync::atomic::Ordering;

struct RawFailureCase {
    message: &'static [u8],
    stack: &'static [u8],
    wrapped: &'static [u8],
}
const CASES: [RawFailureCase; 3] = [
    RawFailureCase {
        message: b"probe sentinel",
        stack: b"",
        wrapped: b"Standard_DomainError: probe sentinel\n",
    },
    RawFailureCase {
        message: b"device\xffcause",
        stack: b"stack\xfetail",
        wrapped: b"Standard_DomainError: device\xffcausestack\xfetail\n",
    },
    RawFailureCase {
        message: b"",
        stack: b"stack\xfetail",
        wrapped: b"Standard_DomainErrorstack\xfetail\n",
    },
];
impl RawFailureCase {
    fn failure(&self) -> Failure {
        Failure::with_type_bytes("Standard_DomainError", self.message, self.stack)
    }
}
struct ReadObservation {
    result: Result<(), PcdmStreamFailure>,
    state: u8,
    status: PcdmReaderStatus,
}
fn read_document(
    app: &FakeProbeApplication,
    input: &mut PcdmInputStream<'_>,
) -> Result<(), PcdmStreamFailure> {
    let (mut document, original) = new_document();
    let outcome = app.read_input_stream(
        input,
        &mut document,
        &Handle::default(),
        &ProgressRange::default(),
    );
    assert!(document.cast::<FakeDocument>() == original);
    assert_eq!(app.callbacks.load(Ordering::SeqCst), 0);
    outcome
}
fn capture_read(
    app: &FakeProbeApplication,
    input: &mut PcdmInputStream<'_>,
    original: &Failure,
) -> ReadObservation {
    let result = read_document(app, input);
    assert_eq!(
        input
            .last_error()
            .unwrap()
            .get_ref()
            .unwrap()
            .downcast_ref::<Failure>(),
        Some(original)
    );
    if let Err(error) = &result {
        assert_eq!(error.operation(), "CDF_Application::Read(stream)");
        if error.kernel_failure().is_some() {
            assert!(std::ptr::eq(error.cause(), input.last_error().unwrap()));
        }
    }
    ReadObservation {
        result,
        state: input.state(),
        status: app.get_retrieve_status(),
    }
}
fn observe(case: &RawFailureCase, position: usize, mask: u8) -> ReadObservation {
    let app = FakeProbeApplication::default();
    app.base
        .set_retrievable_status(PcdmReaderStatus::PermissionDenied);
    let mut device = FakeProbeInput::unknown(12);
    device.fail_at = Some(position);
    device.kernel_payload = Some(case.failure());
    let outcome = {
        let mut input = PcdmInputStream::with_exception_mask(&mut device, mask);
        capture_read(&app, &mut input, &case.failure())
    };
    assert_eq!((device.consumed, device.calls), (position, position + 1));
    outcome
}
fn assert_wrapped(outcome: ReadObservation, case: &RawFailureCase, position: usize) {
    let failure = outcome.result.unwrap_err();
    assert_eq!(failure.consumed(), position);
    assert_eq!(failure.state(), state::BAD);
    assert_eq!(
        (outcome.state, outcome.status),
        (state::BAD, PcdmReaderStatus::FormatFailure)
    );
    let wrapped = failure.kernel_failure().unwrap();
    assert_eq!(wrapped.exception_type(), "Standard_Failure");
    assert_eq!(wrapped.what(), case.wrapped);
    assert!(wrapped.stack_bytes().is_empty());
    assert_eq!(
        failure.cause().get_ref().unwrap().downcast_ref::<Failure>(),
        Some(&case.failure())
    );
}

#[test]
fn caught_kernel_exception_formatting_preserves_raw_message_and_stack_bytes() {
    for case in &CASES {
        for position in [0, 3, 6] {
            for mask in [state::BAD, state::BAD | state::FAIL] {
                assert_wrapped(observe(case, position, mask), case, position);
            }
        }
    }
}
#[test]
fn unthrown_raw_kernel_errors_keep_default_mask_behavior() {
    for case in &CASES {
        for position in [0, 3, 6] {
            let outcome = observe(case, position, 0);
            assert!(outcome.result.is_ok());
            assert_eq!(outcome.state, state::BAD | state::FAIL);
            assert_eq!(outcome.status, PcdmReaderStatus::FormatFailure);
        }
    }
}
#[test]
fn failbit_only_keeps_the_generated_exception_and_existing_application_status() {
    for case in &CASES {
        for position in [0, 3, 6] {
            let outcome = observe(case, position, state::FAIL);
            assert_eq!(outcome.state, state::BAD | state::FAIL);
            assert_eq!(outcome.status, PcdmReaderStatus::PermissionDenied);
            assert!(outcome.result.unwrap_err().kernel_failure().is_none());
        }
    }
}
