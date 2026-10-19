use super::{checked_input::*, fakes::FailingBufferedInput};
use martensite_tkernel::{standard::Failure, storage::state};
use std::{io::Cursor, sync::Arc};

#[test]
fn line_capacity_preserves_the_next_byte_for_cleared_resume() {
    let mut bytes = Cursor::new(b"ABC>tail".as_slice());
    let mut source = PositionedInput(&mut bytes);
    let mut input = InputContext::new(&mut source);
    let mut line = [0xee; 3];
    input.getline(&mut line, b'>').unwrap();
    assert_eq!(
        (line, input.gcount(), input.state()),
        ([b'A', b'B', 0], 2, state::FAIL)
    );
    assert_eq!(input.position().unwrap(), None);
    input.clear().unwrap();
    assert_eq!(input.position().unwrap(), Some(2));
    let mut byte = 0;
    input.get(&mut byte).unwrap();
    assert_eq!(byte, b'C');
    assert_eq!(input.position().unwrap(), Some(3));
}

#[test]
fn line_delimiter_is_counted_and_not_stored_even_at_capacity() {
    let mut bytes = Cursor::new(b"AB>tail".as_slice());
    let mut source = PositionedInput(&mut bytes);
    let mut input = InputContext::new(&mut source);
    let mut line = [0xee; 3];
    input.getline(&mut line, b'>').unwrap();
    assert_eq!(
        (line, input.gcount(), input.state()),
        ([b'A', b'B', 0], 3, 0)
    );
    assert_eq!(input.position().unwrap(), Some(3));
}

#[test]
fn empty_buffer_can_consume_an_immediate_delimiter_without_writing() {
    let mut bytes = b">tail".as_slice();
    let mut source = BufferedInput(&mut bytes);
    let mut input = InputContext::new(&mut source);
    input.getline(&mut [], b'>').unwrap();
    assert_eq!((input.gcount(), input.state()), (1, 0));
    assert_eq!(input.position().unwrap(), None);
    drop(input);
    assert_eq!(bytes, b"tail");
}

#[test]
fn caller_owned_slice_is_borrowed_without_write_seek_or_static_requirements() {
    let owned = vec![b'X', b'>', b'Y'];
    let mut bytes = owned.as_slice();
    let mut source = BufferedInput(&mut bytes);
    let mut input = InputContext::new(&mut source);
    let mut output = [0xee; 8];
    let consumer: &mut dyn UnformattedInput = &mut input;
    consumer.getline(&mut output, b'>').unwrap();
    assert_eq!(&output[..2], b"X\0");
    assert_eq!((consumer.gcount(), consumer.state()), (2, 0));
    drop(input);
    assert_eq!(bytes, b"Y");
    assert_eq!(owned, b"X>Y");
}

#[test]
fn typed_device_error_identity_survives_without_stringification() {
    let mut reader = FailingBufferedInput::new(b"AB>");
    reader.failure_at = Some(1);
    let mut source = PositionedInput(&mut reader);
    let mut input = InputContext::new(&mut source);
    input.set_exception_mask(state::BAD).unwrap();
    let mut output = [0xee; 8];
    let fault = input.getline(&mut output, b'>').unwrap_err();
    let InputCause::Device(cause) = fault.cause else {
        panic!("expected typed device cause");
    };
    assert!(Arc::ptr_eq(&cause, input.last_device_error().unwrap()));
    let typed = cause.get_ref().unwrap().downcast_ref::<Failure>().unwrap();
    assert_eq!(typed.exception_type(), "InputQuotaExceeded");
    assert_eq!(typed.stack_string(), "owned stack");
    assert_eq!(
        (fault.operation, fault.state, fault.gcount, fault.extracted),
        (InputOperation::Line, state::BAD, 1, 1)
    );
    assert_eq!(output[0], b'A');
    assert_eq!(output[1], 0xee);
}

#[test]
fn independent_contexts_and_error_values_have_explicit_thread_traits() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<InputFailure>();
    fn assert_send<T: Send>() {}
    assert_send::<InputContext<'_, ConcreteBorrow>>();
    struct ConcreteBorrow(Cursor<Vec<u8>>);
    impl InputBuffer for ConcreteBorrow {
        fn buffer(&mut self) -> &mut dyn std::io::BufRead {
            &mut self.0
        }
    }
    let mut one = b"A".as_slice();
    let mut two = b"B".as_slice();
    let mut source_one = BufferedInput(&mut one);
    let mut source_two = BufferedInput(&mut two);
    let mut first = InputContext::new(&mut source_one);
    let mut second = InputContext::new(&mut source_two);
    first.get(&mut 0).unwrap();
    assert_eq!(second.state(), 0);
    let mut byte = 0;
    second.get(&mut byte).unwrap();
    assert_eq!(byte, b'B');
}
