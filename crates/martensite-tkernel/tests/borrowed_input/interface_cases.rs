use super::{checked_input::*, fakes::FailingBufferedInput};
use martensite_tkernel::storage::state;
use std::sync::Arc;

#[test]
fn erased_consumer_observes_eof_and_restores_flags_without_moving_the_source() {
    let mut bytes = b"".as_slice();
    let mut source = BufferedInput(&mut bytes);
    let mut input = InputContext::new(&mut source);
    let consumer: &mut dyn UnformattedInput = &mut input;
    assert!(consumer.good());
    consumer.set_exception_mask(state::FAIL).unwrap();
    assert_eq!(consumer.exception_mask(), state::FAIL);
    assert!(consumer.get(&mut 0).is_err());
    assert!(consumer.eof() && consumer.fail());
    consumer.clear().unwrap();
    assert!(consumer.good() && !consumer.eof() && !consumer.fail());
    assert_eq!(consumer.gcount(), 0);
    assert!(consumer.last_device_error().is_none());
    assert_eq!(consumer.position().unwrap(), None);
}

#[test]
fn erased_consumer_retains_the_same_device_cause_after_clearing_flags() {
    let mut reader = FailingBufferedInput::new(b"X");
    reader.failure_at = Some(0);
    let mut source = BufferedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    let consumer: &mut dyn UnformattedInput = &mut input;
    consumer.set_exception_mask(state::BAD).unwrap();
    let fault = consumer.get(&mut 0).unwrap_err();
    let InputCause::Device(cause) = fault.cause else { panic!("expected device cause") };
    assert!(consumer.fail() && !consumer.eof());
    assert!(Arc::ptr_eq(&cause, consumer.last_device_error().unwrap()));
    consumer.clear().unwrap();
    assert!(consumer.good());
    assert!(Arc::ptr_eq(&cause, consumer.last_device_error().unwrap()));
}
