use super::{fakes::FailingBufferedInput, prototype::*};
use martensite_tkernel::storage::state;

#[test]
fn get_eof_throws_after_publishing_eof_and_fail_together() {
    for mask in [state::EOF, state::FAIL, state::BAD | state::FAIL] {
        let mut bytes = b"".as_slice();
        let mut source = BufferedInput(&mut bytes);
        let mut input = InputContext::new(&mut source);
        input.set_exception_mask(mask).unwrap();
        let mut output = 0xee;
        let fault = input.get(&mut output).unwrap_err();
        assert_eq!(
            (output, fault.state, fault.gcount),
            (0xee, state::EOF | state::FAIL, 0)
        );
        assert!(matches!(fault.cause, InputCause::State));
    }
}

#[test]
fn line_eof_with_existing_characters_does_not_add_fail() {
    let mut bytes = b"AB".as_slice();
    let mut source = BufferedInput(&mut bytes);
    let mut input = InputContext::new(&mut source);
    input.set_exception_mask(state::FAIL).unwrap();
    let mut output = [0xee; 8];
    input.getline(&mut output, b'>').unwrap();
    assert_eq!((input.state(), input.gcount()), (state::EOF, 2));
    assert_eq!(&output[..3], b"AB\0");
    let fault = input.position().unwrap_err();
    assert_eq!(fault.state, state::EOF | state::FAIL);
}

#[test]
fn mask_changes_throw_current_state_and_preserve_the_new_mask() {
    let mut bytes = b"".as_slice();
    let mut source = BufferedInput(&mut bytes);
    let mut input = InputContext::new(&mut source);
    input.get(&mut 0).unwrap();
    let fault = input.set_exception_mask(state::EOF).unwrap_err();
    assert_eq!(fault.operation, InputOperation::Mask);
    assert_eq!(input.exception_mask(), state::EOF);
    input.clear().unwrap();
    assert_eq!(input.state(), 0);
}

#[test]
fn line_default_and_fail_masks_preserve_count_publication_boundary() {
    for (at, mask, count, flags, throws) in [
        (0, 0, 0, 5, false),
        (1, 0, 1, 1, false),
        (0, 4, 0, 5, true),
        (1, 4, 1, 1, false),
    ] {
        let mut reader = FailingBufferedInput::new(b"ABC>");
        reader.failure_at = Some(at);
        let mut source = PositionedInput(&mut reader);
        let mut input = InputContext::new(&mut source);
        input.set_exception_mask(mask).unwrap();
        let mut output = [0xee; 8];
        assert_eq!(input.getline(&mut output, b'>').is_err(), throws);
        assert_eq!((input.gcount(), input.state()), (count, flags));
        assert_eq!(output[at], 0);
        assert!(input.last_device_error().is_some());
    }
}
