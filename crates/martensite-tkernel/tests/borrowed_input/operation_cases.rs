use super::{fakes::FailingBufferedInput, checked_input::*};
use martensite_tkernel::storage::state;
use std::io::Cursor;

#[test]
fn block_read_short_eof_preserves_prefix_and_sets_both_flags() {
    let mut bytes = Cursor::new(b"ABC".as_slice());
    let mut source = PositionedInput(&mut bytes);
    let mut input = InputContext::new(&mut source);
    let mut output = [0xee; 5];
    input.read(&mut output).unwrap();
    assert_eq!(output, [b'A', b'B', b'C', 0xee, 0xee]);
    assert_eq!(input.gcount(), 3);
    assert_eq!(input.state(), state::EOF | state::FAIL);
    input.clear().unwrap();
    assert_eq!(input.position().unwrap(), Some(3));
}

#[test]
fn block_device_failure_publishes_zero_count_but_keeps_partial_output() {
    let mut reader = FailingBufferedInput::new(b"ABCDE");
    reader.failure_at = Some(2);
    let mut source = PositionedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    input.set_exception_mask(state::FAIL).unwrap();
    let mut output = [0xee; 4];
    input.read(&mut output).unwrap();
    assert_eq!(output, [b'A', b'B', 0xee, 0xee]);
    assert_eq!((input.gcount(), input.state()), (0, state::BAD));
    input.clear().unwrap();
    assert_eq!(input.position().unwrap(), Some(2));
}

#[test]
fn position_is_actual_nonzero_origin_and_does_not_change_last_count() {
    let mut bytes = Cursor::new(b"prefixAB".as_slice());
    bytes.set_position(6);
    let mut source = PositionedInput(&mut bytes);
    let mut input = InputContext::new(&mut source);
    let mut byte = 0;
    input.get(&mut byte).unwrap();
    assert_eq!(byte, b'A');
    assert_eq!(input.position().unwrap(), Some(7));
    assert_eq!(input.gcount(), 1);
    input.read(&mut []).unwrap();
    assert_eq!(input.gcount(), 0);
    assert_eq!(input.position().unwrap(), Some(7));
}

#[test]
fn position_failure_sets_bad_without_fabricating_seek_or_eof() {
    let mut reader = FailingBufferedInput::new(b"A");
    reader.fail_position = true;
    let mut source = PositionedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    assert_eq!(input.position().unwrap(), None);
    assert_eq!(input.state(), state::BAD);
    assert!(input.last_device_error().is_some());
}
