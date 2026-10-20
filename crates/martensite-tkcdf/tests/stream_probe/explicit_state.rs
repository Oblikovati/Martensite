use super::fake_input::{FakeProbeInput, ProbeSentinel};
use martensite_tkcdf::pcdm::{PcdmInputStream, PcdmReadWriterBase, PcdmStreamFailure};
use martensite_tkernel::{standard::Handle, storage::state};
use std::error::Error;

#[test]
fn explicit_state_matches_native_short_eof_and_seven_byte_limit() {
    for length in 0..=12 {
        let mut device = FakeProbeInput::unknown(length);
        let mut stream = PcdmInputStream::new(&mut device);
        let mut storage = Handle::default();
        assert!(
            PcdmReadWriterBase::file_format_input(&mut stream, &mut storage)
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            stream.state(),
            if length < 7 {
                state::EOF | state::FAIL
            } else {
                0
            }
        );
        assert!(stream.last_error().is_none());
        assert!(storage.is_null());
    }
}

#[test]
fn default_device_failure_retains_typed_cause_and_stops_future_probes() {
    let mut device = FakeProbeInput::unknown(12);
    device.fail_at = Some(3);
    let mut stream = PcdmInputStream::new(&mut device);
    let mut storage = Handle::default();
    PcdmReadWriterBase::file_format_input(&mut stream, &mut storage).unwrap();
    assert_eq!(stream.state(), state::BAD | state::FAIL);
    assert_eq!(
        stream
            .last_error()
            .unwrap()
            .get_ref()
            .unwrap()
            .downcast_ref::<ProbeSentinel>()
            .unwrap()
            .marker,
        734
    );
    PcdmReadWriterBase::file_format_input(&mut stream, &mut storage).unwrap();
    drop(stream);
    assert_eq!((device.consumed, device.calls), (3, 4));
}

#[test]
fn badbit_exception_retains_device_error_source_and_kernel_distinction() {
    for kernel in [false, true] {
        for fail_at in 0..7 {
            let mut device = FakeProbeInput::unknown(12);
            device.fail_at = Some(fail_at);
            device.kernel_failure = kernel;
            let mut stream = PcdmInputStream::with_exception_mask(&mut device, state::BAD);
            let failure =
                PcdmReadWriterBase::file_format_input(&mut stream, &mut Handle::default())
                    .unwrap_err();
            assert_eq!(
                (failure.consumed(), failure.state(), stream.state()),
                (fail_at, state::BAD, state::BAD)
            );
            assert_eq!(failure.kernel_failure().is_some(), kernel);
            assert!(std::ptr::eq(
                failure
                    .source()
                    .unwrap()
                    .downcast_ref::<std::io::Error>()
                    .unwrap(),
                failure.cause()
            ));
            assert!(failure.to_string().contains(&format!("{fail_at} of 7")));
        }
    }
}

#[test]
fn eof_and_fail_exception_masks_report_generated_iostream_errors() {
    for mask in [state::EOF, state::FAIL, state::BAD | state::FAIL] {
        let mut device = FakeProbeInput::unknown(3);
        let mut stream = PcdmInputStream::with_exception_mask(&mut device, mask);
        let failure =
            PcdmReadWriterBase::file_format_input(&mut stream, &mut Handle::default()).unwrap_err();
        assert_eq!(failure.cause().kind(), std::io::ErrorKind::UnexpectedEof);
        assert_eq!(failure.state(), state::EOF | state::FAIL);
        assert_eq!(failure.consumed(), 3);
        assert!(failure.kernel_failure().is_none());
    }
}

#[test]
fn thrown_error_is_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<PcdmStreamFailure>();
}

#[test]
fn failbit_exception_uses_generated_error_but_retains_original_device_cause() {
    let mut device = FakeProbeInput::unknown(12);
    device.fail_at = Some(3);
    device.kernel_failure = true;
    let mut stream = PcdmInputStream::with_exception_mask(&mut device, state::FAIL);
    let failure =
        PcdmReadWriterBase::file_format_input(&mut stream, &mut Handle::default()).unwrap_err();
    assert_eq!(failure.state(), state::BAD | state::FAIL);
    assert!(failure.kernel_failure().is_none());
    assert!(failure.cause().to_string().contains("basic_ios::clear"));
    assert!(
        stream
            .last_error()
            .unwrap()
            .get_ref()
            .unwrap()
            .is::<martensite_tkernel::standard::Failure>()
    );
}
