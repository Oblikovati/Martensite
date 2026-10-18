//! Fixed-byte header, position, publication and typed-cause contracts.
use martensite_tkernel::fsd::{BinaryFile, CheckedReadError, FileHeader};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::storage::checked_input::*;
use martensite_tkernel::storage::{IoStream, state};
use std::{error::Error, io::Cursor, sync::Arc};
#[path = "borrowed_input/fakes.rs"]
mod checked_source;
#[path = "checked_binary_header/fixtures.rs"]
mod fixtures;
use checked_source::FailingBufferedInput;
use fixtures::*;

#[test]
fn complete_header_uses_actual_nonzero_origin_and_leaves_the_callers_body_unread() {
    let mut bytes = [b"prefix".as_slice(), &HEADER, b"body"].concat();
    let mut cursor = Cursor::new(bytes.as_mut_slice());
    cursor.set_position(6);
    let mut source = PositionedInput(&mut cursor);
    let mut input = InputContext::new(&mut source);
    assert_eq!(input.position().unwrap(), Some(6));
    let mut header = FileHeader::unset();
    BinaryFile::read_header_checked(&mut input, &mut header).unwrap();
    assert_eq!(header, expected_header());
    assert_eq!((input.position().unwrap(), input.gcount()), (Some(58), 4));
    assert!(input.good());
    drop(input);
    assert_eq!(cursor.position(), 58);
    assert_eq!(&cursor.get_ref()[58..], b"body");
}

#[test]
fn complete_header_needs_no_write_or_position_capability() {
    let bytes = [HEADER.as_slice(), b"tail"].concat();
    let mut borrowed = bytes.as_slice();
    let mut source = BufferedInput(&mut borrowed);
    let mut input = InputContext::new(&mut source);
    let mut header = FileHeader::default();
    BinaryFile::read_header_checked(&mut input, &mut header).unwrap();
    assert_eq!(input.position().unwrap(), None);
    assert_eq!(header, expected_header());
    drop(input);
    assert_eq!(borrowed, b"tail");
}

#[test]
fn short_header_publishes_partial_field_and_describes_the_actual_count() {
    let mut borrowed = HEADER[..2].as_ref();
    let mut source = BufferedInput(&mut borrowed);
    let mut input = InputContext::new(&mut source);
    let mut header = FileHeader::from_fields([0x5555_5555; 13]);
    let fault = BinaryFile::read_header_checked(&mut input, &mut header).unwrap_err();
    assert_short_format_failure(fault);
    assert_eq!(header, short_initial_header());
    assert_eq!(
        (input.state(), input.gcount()),
        (state::EOF | state::FAIL, 2)
    );
}

fn assert_short_format_failure(fault: CheckedReadError) {
    assert!(fault.source().unwrap().is::<Failure>());
    assert!(fault.to_string().contains("received 2 bytes; expected 4"));
    let CheckedReadError::Format { failure, cause } = fault else {
        panic!("expected format error")
    };
    assert_eq!(failure.exception_type(), "Storage_StreamTypeMismatchError");
    assert_eq!(
        failure.message(),
        "binary header field received 2 bytes; expected 4"
    );
    assert!(cause.is_none());
}

#[test]
fn eof_masks_stop_with_typed_state_failure_after_partial_field_publication() {
    for mask in [state::EOF, state::FAIL, state::EOF | state::FAIL] {
        assert_masked_short_header(mask);
    }
}
fn assert_masked_short_header(mask: u8) {
    let mut borrowed = HEADER[..2].as_ref();
    let mut source = BufferedInput(&mut borrowed);
    let mut input = InputContext::new(&mut source);
    input.set_exception_mask(mask).unwrap();
    let mut header = FileHeader::from_fields([0x5555_5555; 13]);
    let fault = BinaryFile::read_header_checked(&mut input, &mut header).unwrap_err();
    let CheckedReadError::Input(fault) = fault else {
        panic!("expected input state error")
    };
    assert!(matches!(fault.cause, InputCause::State));
    assert_eq!(
        (fault.operation, fault.state, fault.mask),
        (InputOperation::Read, 6, mask)
    );
    assert_eq!((fault.gcount, fault.extracted), (2, 2));
    assert_eq!(header, short_initial_header());
}

#[test]
fn masked_device_failure_keeps_partial_header_and_the_original_typed_cause() {
    let mut reader = FailingBufferedInput::new(&HEADER);
    reader.failure_at = Some(2);
    let mut source = PositionedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    input.set_exception_mask(state::BAD).unwrap();
    let mut header = FileHeader::from_fields([0x5555_5555; 13]);
    let error = BinaryFile::read_header_checked(&mut input, &mut header).unwrap_err();
    assert_masked_device_failure(error, &input);
    assert_eq!(header, short_initial_header());
}
fn assert_masked_device_failure(error: CheckedReadError, input: &dyn UnformattedInput) {
    assert!(error.source().unwrap().is::<InputFailure>());
    let CheckedReadError::Input(fault) = error else {
        panic!("expected device input error")
    };
    assert_eq!(
        (fault.state, fault.gcount, fault.extracted),
        (state::BAD, 0, 2)
    );
    let InputCause::Device(cause) = fault.cause else {
        panic!("expected original cause")
    };
    assert!(Arc::ptr_eq(&cause, input.last_device_error().unwrap()));
    assert_quota_failure(&cause);
}

fn assert_quota_failure(cause: &std::io::Error) {
    let failure = cause.get_ref().unwrap().downcast_ref::<Failure>().unwrap();
    assert_eq!(failure.exception_type(), "InputQuotaExceeded");
    assert_eq!(failure.message(), "quota 0, expected positive capacity");
    assert_eq!(failure.stack_string(), "owned stack");
}

#[test]
fn default_mask_format_error_owns_the_original_device_cause_after_context_drop() {
    let error = detached_device_failure();
    assert!(error.source().unwrap().is::<std::io::Error>());
    let CheckedReadError::Format { failure, cause } = error else {
        panic!("expected format error")
    };
    assert_eq!(
        failure.message(),
        "binary header field received 0 bytes; expected 4"
    );
    assert_quota_failure(cause.as_ref().unwrap());
}
fn detached_device_failure() -> CheckedReadError {
    let mut reader = FailingBufferedInput::new(&HEADER);
    reader.failure_at = Some(2);
    let mut source = BufferedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    let mut header = FileHeader::from_fields([0x5555_5555; 13]);
    let error = BinaryFile::read_header_checked(&mut input, &mut header).unwrap_err();
    let CheckedReadError::Format {
        cause: Some(ref cause),
        ..
    } = error
    else {
        panic!("expected retained cause")
    };
    assert!(Arc::ptr_eq(cause, input.last_device_error().unwrap()));
    assert_eq!((input.state(), input.gcount()), (state::BAD, 0));
    assert_eq!(header, short_initial_header());
    error
}

#[test]
fn cleared_device_history_is_not_attached_to_a_later_eof_format_failure() {
    let mut reader = FailingBufferedInput::new(b"");
    reader.fail_position = true;
    let mut source = PositionedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    assert_eq!(input.position().unwrap(), None);
    assert!(input.last_device_error().is_some());
    input.clear().unwrap();
    let mut header = FileHeader::default();
    let error = BinaryFile::read_header_checked(&mut input, &mut header).unwrap_err();
    assert!(matches!(
        error,
        CheckedReadError::Format { cause: None, .. }
    ));
    assert_eq!(input.state(), state::EOF | state::FAIL);
}

#[test]
fn checked_header_error_is_owned_and_the_existing_owned_diagnostic_is_unchanged() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CheckedReadError>();
    let mut stream = IoStream::from_bytes(vec![1, 2]);
    let mut header = FileHeader::from_fields([0x5555_5555; 13]);
    let error = BinaryFile::read_header_stream(&mut stream, &mut header).unwrap_err();
    assert_eq!(
        error.message(),
        "FSD_BinaryFile::GetInteger: expected more bytes in the file"
    );
    assert_eq!(header, short_initial_header());
}
