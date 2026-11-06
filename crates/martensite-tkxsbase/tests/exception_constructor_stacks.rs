//! Message overloads capture configured stacks; explicit stack overloads never capture.
use martensite_tkernel::standard::Failure;
use martensite_tkxsbase::{interface, transfer};

type MessageFactory = fn(&str) -> Failure;
type ByteFactory = fn(&[u8]) -> Failure;
type StackFactory = fn(&str, &str) -> Failure;
type ByteStackFactory = fn(&[u8], &[u8]) -> Failure;

struct ConstructorCase {
    message: MessageFactory,
    bytes: ByteFactory,
    stack: StackFactory,
    stack_bytes: ByteStackFactory,
    leaf: &'static str,
    parent: Option<&'static str>,
}

struct StackLengthRestore(i32);

impl Drop for StackLengthRestore {
    fn drop(&mut self) {
        Failure::set_default_stack_trace_length(self.0);
    }
}

const CONSTRUCTORS: [ConstructorCase; 5] = [
    ConstructorCase {
        message: interface::interface_error,
        bytes: interface::interface_error_bytes,
        stack: interface::interface_error_with_stack,
        stack_bytes: interface::interface_error_with_stack_bytes,
        leaf: "InterfaceInterfaceError",
        parent: None,
    },
    ConstructorCase {
        message: interface::check_failure,
        bytes: interface::check_failure_bytes,
        stack: interface::check_failure_with_stack,
        stack_bytes: interface::check_failure_with_stack_bytes,
        leaf: "InterfaceCheckFailure",
        parent: Some("InterfaceInterfaceError"),
    },
    ConstructorCase {
        message: interface::interface_mismatch,
        bytes: interface::interface_mismatch_bytes,
        stack: interface::interface_mismatch_with_stack,
        stack_bytes: interface::interface_mismatch_with_stack_bytes,
        leaf: "InterfaceInterfaceMismatch",
        parent: Some("InterfaceInterfaceError"),
    },
    ConstructorCase {
        message: transfer::transfer_failure,
        bytes: transfer::transfer_failure_bytes,
        stack: transfer::transfer_failure_with_stack,
        stack_bytes: transfer::transfer_failure_with_stack_bytes,
        leaf: "TransferTransferFailure",
        parent: Some("InterfaceInterfaceError"),
    },
    ConstructorCase {
        message: transfer::transfer_dead_loop,
        bytes: transfer::transfer_dead_loop_bytes,
        stack: transfer::transfer_dead_loop_with_stack,
        stack_bytes: transfer::transfer_dead_loop_with_stack_bytes,
        leaf: "TransferTransferDeadLoop",
        parent: Some("TransferTransferFailure"),
    },
];

fn assert_stack_capture(length: i32) {
    Failure::set_default_stack_trace_length(length);
    let control = Failure::with_message("CaptureControl", "message");
    assert_eq!(control.stack_bytes().is_empty(), length <= 0);
    for case in &CONSTRUCTORS {
        assert_captured_constructor(case, (case.message)("message\0suffix"), b"message", length);
        assert_captured_constructor(case, (case.bytes)(&[255, 17, 0, 19]), &[255, 17], length);
        assert_explicit_constructors(case);
    }
}

fn assert_captured_constructor(
    case: &ConstructorCase,
    failure: Failure,
    message: &[u8],
    length: i32,
) {
    assert_eq!(failure.message_bytes(), message);
    assert_eq!(
        failure.stack_bytes().is_empty(),
        length <= 0,
        "{} must follow message-constructor capture setting {length}",
        failure.exception_type()
    );
    assert_identity(case, &failure);
}

fn assert_identity(case: &ConstructorCase, failure: &Failure) {
    assert_eq!(failure.exception_type(), case.leaf);
    assert_eq!(failure.explicit_exception_parent(), case.parent);
    assert!(failure.is_kind(case.leaf) && failure.is_kind("Standard_Failure"));
    if let Some(parent) = case.parent {
        assert!(failure.is_kind(parent));
    }
}

fn assert_explicit_constructors(case: &ConstructorCase) {
    let text = (case.stack)("message\0suffix", "trace\0suffix");
    assert_eq!(text.what(), b"message");
    assert_eq!(text.stack_bytes(), b"trace");
    assert_identity(case, &text);
    let raw = (case.stack_bytes)(&[255, 17, 0, 19], &[254, 29, 0, 31]);
    assert_eq!(raw.what(), &[255, 17]);
    assert_eq!(raw.stack_bytes(), &[254, 29]);
    assert_identity(case, &raw);
    assert!((case.stack)("message", "").stack_bytes().is_empty());
    assert_clone_isolation(raw);
}

fn assert_clone_isolation(mut failure: Failure) {
    let saved = failure.clone();
    failure.set_message("changed message");
    failure.set_stack_string("changed stack");
    assert_eq!(saved.what(), &[255, 17]);
    assert_eq!(saved.stack_bytes(), &[254, 29]);
    assert_eq!(
        failure.explicit_exception_parent(),
        saved.explicit_exception_parent()
    );
}

#[test]
fn message_only_overloads_capture_stacks_and_restore_the_setting() {
    let previous = Failure::default_stack_trace_length();
    let restore = StackLengthRestore(previous);
    for length in [-1, 0, 4] {
        assert_stack_capture(length);
    }
    drop(restore);
    assert_eq!(Failure::default_stack_trace_length(), previous);
}
