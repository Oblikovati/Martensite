use super::support::driver;
use super::{baseline, fixture, support};
use martensite_tkernel::fsd::CheckedReadError;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::storage::{
    Data,
    checked_input::{BufferedInput, InputCause, InputContext, InputOperation, UnformattedInput},
    state,
};
use std::{error::Error, sync::Arc};

#[test]
fn short_header_fails_before_dataset_allocation() {
    let mut source = support::source(&baseline::empty_container()[..9]);
    let mut set = Handle::<Data>::default();
    support::with_input(&mut source, |input| {
        let error = driver()
            .read_complete_info_checked(input, &mut set)
            .unwrap_err();
        assert_short_integer(error);
        assert_eq!(
            (input.state(), input.gcount()),
            (state::EOF | state::FAIL, 2)
        );
    });
    assert!(set.is_null());
    assert_eq!(source.cursor, 9);
}

#[test]
fn short_info_allocates_without_publishing_partial_header() {
    let mut bytes = baseline::empty_container();
    bytes[59..63].copy_from_slice(&12_i32.to_ne_bytes());
    let mut source = support::source(&bytes[..65]);
    let mut set = Handle::<Data>::default();
    support::with_input(&mut source, |input| {
        assert!(
            driver()
                .read_complete_info_checked(input, &mut set)
                .is_err()
        )
    });
    assert_eq!(set.get().unwrap().number_of_objects(), 0);
    assert_eq!(source.cursor, 65);
}

#[test]
fn short_info_preserves_existing_identity_and_metadata() {
    let mut source = support::source(&baseline::empty_container()[..65]);
    let mut set = Handle::new(Data::default());
    let identity = set.as_ptr();
    set.get()
        .unwrap()
        .header_data()
        .get()
        .unwrap()
        .set_number_of_objects(99);
    support::with_input(&mut source, |input| {
        assert!(
            driver()
                .read_complete_info_checked(input, &mut set)
                .is_err()
        )
    });
    assert_eq!(set.as_ptr(), identity);
    assert_eq!(set.get().unwrap().number_of_objects(), 99);
}

#[test]
fn enabled_eof_failure_stops_at_the_same_allocation_boundary() {
    for end in [9, 65] {
        let mut source = support::source(&baseline::empty_container()[..end]);
        let mut set = Handle::<Data>::default();
        support::with_input(&mut source, |input| {
            input.set_exception_mask(state::FAIL).unwrap();
            let CheckedReadError::Input(error) = driver()
                .read_complete_info_checked(input, &mut set)
                .unwrap_err()
            else {
                panic!("expected masked input failure")
            };
            assert_eq!((error.state, error.mask, error.gcount), (6, 4, 2));
            assert_eq!(error.operation, InputOperation::Read);
        });
        assert_eq!(set.is_null(), end == 9);
        assert_eq!(source.cursor, end);
    }
}

#[test]
fn absent_position_is_explicit_after_info_before_sections() {
    let bytes = baseline::empty_container();
    let mut remaining = &bytes[7..];
    let mut set = Handle::<Data>::default();
    {
        let mut source = BufferedInput(&mut remaining);
        let mut input = InputContext::new(&mut source);
        assert_absent_position(
            driver()
                .read_complete_info_checked(&mut input, &mut set)
                .unwrap_err(),
        );
    }
    assert_eq!(remaining, &bytes[95..]);
    assert!(!set.is_null());
}

#[test]
fn unmasked_position_error_retains_its_actual_cause_after_context_drop() {
    let mut source = support::source(&baseline::empty_container());
    source.fail_position = true;
    let mut set = Handle::<Data>::default();
    let error = support::with_input(&mut source, |input| {
        let error = driver()
            .read_complete_info_checked(input, &mut set)
            .unwrap_err();
        assert_failed_position(&error, input);
        error
    });
    let payload = quota_payload(&error);
    assert_eq!(payload.exception_type(), "InputQuotaExceeded");
    assert_eq!(payload.stack_string(), "owned stack");
    assert_eq!(source.cursor, 95);
}

#[test]
fn masked_position_error_preserves_operation_mask_and_cause() {
    let mut source = support::source(&baseline::empty_container());
    source.fail_position = true;
    let mut set = Handle::<Data>::default();
    support::with_input(&mut source, |input| {
        input.set_exception_mask(state::BAD).unwrap();
        let error = driver()
            .read_complete_info_checked(input, &mut set)
            .unwrap_err();
        assert_masked_position(error, input);
    });
    assert_eq!(source.cursor, 95);
}

#[test]
fn mid_info_device_error_keeps_original_cause_and_unpublished_metadata() {
    let mut source = support::source(&baseline::empty_container());
    source.failure_at = Some(61);
    let mut set = Handle::<Data>::default();
    support::with_input(&mut source, |input| {
        let error = driver()
            .read_complete_info_checked(input, &mut set)
            .unwrap_err();
        assert_partial_scalar(error, input);
    });
    assert_eq!(set.get().unwrap().number_of_objects(), 0);
    assert_eq!(source.cursor, 61);
}

#[test]
fn root_resolution_domain_failure_remains_distinct_from_input_format() {
    let mut source = support::source(&fixture::unattached_root());
    let mut set = Handle::<Data>::default();
    support::with_input(&mut source, |input| {
        let CheckedReadError::Domain(failure) = driver()
            .read_complete_info_checked(input, &mut set)
            .unwrap_err()
        else {
            panic!("expected domain failure")
        };
        assert_eq!(failure.exception_type(), "Standard_OutOfRange");
        assert!(input.good());
        assert_eq!(input.gcount(), 4);
    });
    assert_eq!(set.get().unwrap().number_of_roots(), 1);
    assert_eq!(source.cursor, 126);
    assert_eq!(&source.bytes[126..], b"body");
}

fn assert_short_integer(error: CheckedReadError) {
    let CheckedReadError::Format { failure, cause } = error else {
        panic!("expected incomplete integer")
    };
    assert_eq!(failure.exception_type(), "Storage_StreamTypeMismatchError");
    assert_eq!(
        failure.message(),
        "binary container integer received 2 bytes; expected 4"
    );
    assert!(cause.is_none());
}

fn assert_absent_position(error: CheckedReadError) {
    assert_eq!(
        error.to_string(),
        "binary container position unavailable with state 0 and mask 0; expected actual caller offset"
    );
    assert!(error.source().is_none());
    let CheckedReadError::PositionUnavailable {
        state,
        mask,
        gcount,
        cause,
    } = error
    else {
        panic!("expected missing position")
    };
    assert_eq!((state, mask, gcount), (0, 0, 4));
    assert!(cause.is_none());
}

fn assert_failed_position(error: &CheckedReadError, input: &dyn UnformattedInput) {
    let CheckedReadError::PositionUnavailable {
        state: bits,
        mask,
        gcount,
        cause: Some(cause),
    } = error
    else {
        panic!("expected position device cause")
    };
    assert_eq!((*bits, *mask, *gcount), (state::BAD, 0, 4));
    assert!(Arc::ptr_eq(cause, input.last_device_error().unwrap()));
}

fn quota_payload(error: &CheckedReadError) -> &Failure {
    let cause = error
        .source()
        .unwrap()
        .downcast_ref::<std::io::Error>()
        .unwrap();
    cause.get_ref().unwrap().downcast_ref::<Failure>().unwrap()
}

fn assert_masked_position(error: CheckedReadError, input: &dyn UnformattedInput) {
    let CheckedReadError::Input(error) = error else {
        panic!("expected masked position failure")
    };
    assert_eq!(
        (error.operation, error.state, error.mask, error.gcount),
        (InputOperation::Position, 1, 1, 4)
    );
    let InputCause::Device(cause) = error.cause else {
        panic!("expected device cause")
    };
    assert!(Arc::ptr_eq(&cause, input.last_device_error().unwrap()));
}

fn assert_partial_scalar(error: CheckedReadError, input: &dyn UnformattedInput) {
    let CheckedReadError::Format {
        failure,
        cause: Some(cause),
    } = error
    else {
        panic!("expected incomplete scalar with original cause")
    };
    assert_eq!(failure.exception_type(), "Storage_StreamTypeMismatchError");
    assert_eq!((input.state(), input.gcount()), (state::BAD, 0));
    assert!(Arc::ptr_eq(&cause, input.last_device_error().unwrap()));
}

#[test]
fn short_comment_preserves_info_without_publishing_partial_comments() {
    for device_error in [false, true] {
        let mut source = comment_source(device_error);
        let mut set = Handle::<Data>::default();
        support::with_input(&mut source, |input| {
            let error = driver()
                .read_complete_info_checked(input, &mut set)
                .unwrap_err();
            assert_comment_failure(error, input, device_error);
        });
        assert_info_without_comments(set.get().unwrap());
        assert_eq!(source.cursor, 144);
    }
}
fn comment_source(device_error: bool) -> super::fakes::FailingBufferedInput {
    let mut bytes = fixture::metadata();
    if !device_error {
        bytes.truncate(144);
    }
    let mut source = support::source(&bytes);
    source.failure_at = device_error.then_some(144);
    source
}
fn assert_comment_failure(
    error: CheckedReadError,
    input: &dyn UnformattedInput,
    device_error: bool,
) {
    let CheckedReadError::Format { failure, cause } = error else {
        panic!("expected incomplete comment field")
    };
    assert_eq!(failure.exception_type(), "Storage_StreamReadError");
    assert_comment_state(input, device_error);
    assert_eq!(cause.is_some(), device_error);
    if let Some(cause) = cause {
        assert!(Arc::ptr_eq(&cause, input.last_device_error().unwrap()));
    }
}

fn assert_info_without_comments(set: &Data) {
    let header = set.header_data();
    let header = header.get().unwrap();
    assert_eq!(header.application_name().as_utf16(), &[65, 937]);
    assert_eq!(header.user_info().length(), 2);
    assert_eq!(header.comments().length(), 0);
}

fn assert_comment_state(input: &dyn UnformattedInput, device_error: bool) {
    let expected = if device_error {
        (state::BAD, 0)
    } else {
        (state::EOF | state::FAIL, 1)
    };
    assert_eq!((input.state(), input.gcount()), expected);
}
