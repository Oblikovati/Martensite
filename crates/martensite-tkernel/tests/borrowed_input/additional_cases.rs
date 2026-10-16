use super::{checked_input::*, fakes::FailingBufferedInput};
use martensite_tkernel::storage::state;
use std::io::Cursor;

#[test]
fn get_device_masks_distinguish_original_and_generated_causes() {
    for mask in [0, state::BAD, state::FAIL] {
        let mut reader = FailingBufferedInput::new(b"A");
        reader.failure_at = Some(0);
        let mut source = PositionedInput(&mut reader);
        let mut input = InputContext::new(&mut source);
        input.set_exception_mask(mask).unwrap();
        let mut byte = 0xee;
        let result = input.get(&mut byte);
        assert_eq!(result.is_err(), mask != 0);
        assert_eq!(
            input.state(),
            if mask == state::BAD {
                state::BAD
            } else {
                state::BAD | state::FAIL
            }
        );
        if let Err(fault) = result {
            assert_eq!(
                matches!(fault.cause, InputCause::Device(_)),
                mask == state::BAD
            );
        }
        assert_eq!((byte, input.gcount()), (0xee, 0));
        assert!(input.last_device_error().is_some());
    }
}

#[test]
fn block_failures_preserve_the_original_cause_without_a_published_count() {
    let mut reader = FailingBufferedInput::new(b"ABC");
    reader.failure_at = Some(2);
    let mut source = PositionedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    input.set_exception_mask(state::BAD).unwrap();
    let mut output = [0xee; 4];
    let fault = input.read(&mut output).unwrap_err();
    assert!(matches!(fault.cause, InputCause::Device(_)));
    assert_eq!(
        (fault.operation, fault.state, fault.gcount, fault.extracted),
        (InputOperation::Read, state::BAD, 0, 2)
    );
    assert_eq!(output, [b'A', b'B', 0xee, 0xee]);
}

#[test]
fn position_failure_preserves_count_and_the_original_typed_error() {
    let mut reader = FailingBufferedInput::new(b"A");
    reader.fail_position = true;
    let mut source = PositionedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    input.get(&mut 0).unwrap();
    input.set_exception_mask(state::BAD).unwrap();
    let fault = input.position().unwrap_err();
    assert_eq!(
        (fault.operation, fault.gcount, fault.state),
        (InputOperation::Position, 1, state::BAD)
    );
    assert!(matches!(fault.cause, InputCause::Device(_)));
}

#[test]
fn tag_sized_reads_stop_before_the_caller_body() {
    let mut reader = Cursor::new(b"<?header?><document format='X'>BODY".as_slice());
    let mut source = PositionedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    let mut output = [0xee; 64];
    input.getline(&mut output, b'>').unwrap();
    assert_eq!(&output[..10], b"<?header?\0");
    assert_eq!(input.position().unwrap(), Some(10));
    input.getline(&mut output, b'>').unwrap();
    assert_eq!(&output[..21], b"<document format='X'\0");
    assert_eq!(input.position().unwrap(), Some(31));
    let mut body = [0; 4];
    input.read(&mut body).unwrap();
    assert_eq!(&body, b"BODY");
}

#[test]
fn large_line_capacity_keeps_lookahead_in_the_callers_buffer() {
    let bytes = vec![b' '; 20_481];
    let mut reader = Cursor::new(bytes.as_slice());
    let mut source = PositionedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    let mut output = vec![0xee; 20_480];
    input.getline(&mut output, b'>').unwrap();
    assert_eq!((input.state(), input.gcount()), (state::FAIL, 20_479));
    input.clear().unwrap();
    assert_eq!(input.position().unwrap(), Some(20_479));
    assert_eq!(output[20_479], 0);
    drop(input);
    assert_eq!(reader.position(), 20_479);
}

#[test]
fn generated_state_fault_exposes_context_without_losing_error_type() {
    use std::error::Error;
    let mut bytes = b"".as_slice();
    let mut source = BufferedInput(&mut bytes);
    let mut input = InputContext::new(&mut source);
    input.set_exception_mask(state::FAIL).unwrap();
    let fault = input.get(&mut 0).unwrap_err();
    assert_eq!(fault.mask, state::FAIL);
    assert!(fault.source().is_none());
    assert_eq!(
        fault.to_string(),
        "input Get: state 6, mask 4, count 0, extracted 0; expected state bits outside mask"
    );
}
