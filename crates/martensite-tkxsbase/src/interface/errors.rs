//! The exceptions of the Interface package, as [`Failure`] values with their native
//! names: `InterfaceInterfaceError` (a `StandardFailure`), and its subclasses
//! `InterfaceCheckFailure` and `InterfaceInterfaceMismatch`.
use martensite_tkernel::standard::Failure;

/// `InterfaceInterfaceError`; `interface_error("TypedEntity: none found")`.
pub fn interface_error(message: &str) -> Failure {
    interface_error_bytes(message.as_bytes())
}

/// `InterfaceCheckFailure`; `check_failure("Fail on entity 3")`.
pub fn check_failure(message: &str) -> Failure {
    check_failure_bytes(message.as_bytes())
}

/// `InterfaceInterfaceMismatch`; `interface_mismatch("model does not suit the protocol")`.
pub fn interface_mismatch(message: &str) -> Failure {
    interface_mismatch_bytes(message.as_bytes())
}

/// The byte-message constructor of `InterfaceInterfaceError`, including configured stack capture.
/// `interface_error_bytes(&[255, 17]).what() == [255, 17]`.
pub fn interface_error_bytes(message: &[u8]) -> Failure {
    Failure::with_message_bytes("InterfaceInterfaceError", message)
}

/// Constructs `InterfaceInterfaceError` with an explicit stack, bypassing stack capture.
/// `interface_error_with_stack("input", "trace").stack_string() == "trace"`.
pub fn interface_error_with_stack(message: &str, stack: &str) -> Failure {
    interface_error_with_stack_bytes(message.as_bytes(), stack.as_bytes())
}

/// Preserves arbitrary message and stack bytes through the first NUL of each.
/// `interface_error_with_stack_bytes(&[255], &[254]).stack_bytes() == [254]`.
pub fn interface_error_with_stack_bytes(message: &[u8], stack: &[u8]) -> Failure {
    Failure::with_type_bytes("InterfaceInterfaceError", message, stack)
}

/// The byte-message constructor of `InterfaceCheckFailure`, including configured stack capture.
/// `check_failure_bytes(&[255, 17]).what() == [255, 17]`.
pub fn check_failure_bytes(message: &[u8]) -> Failure {
    Failure::with_message_bytes("InterfaceCheckFailure", message)
        .with_exception_parent("InterfaceInterfaceError")
}

/// Constructs `InterfaceCheckFailure` with an explicit stack, bypassing stack capture.
/// `check_failure_with_stack("input", "trace").stack_string() == "trace"`.
pub fn check_failure_with_stack(message: &str, stack: &str) -> Failure {
    check_failure_with_stack_bytes(message.as_bytes(), stack.as_bytes())
}

/// Preserves arbitrary message and stack bytes through the first NUL of each.
/// `check_failure_with_stack_bytes(&[255], &[254]).stack_bytes() == [254]`.
pub fn check_failure_with_stack_bytes(message: &[u8], stack: &[u8]) -> Failure {
    Failure::with_type_bytes("InterfaceCheckFailure", message, stack)
        .with_exception_parent("InterfaceInterfaceError")
}

/// The byte-message constructor of `InterfaceInterfaceMismatch`, including configured stack capture.
/// `interface_mismatch_bytes(&[255, 17]).what() == [255, 17]`.
pub fn interface_mismatch_bytes(message: &[u8]) -> Failure {
    Failure::with_message_bytes("InterfaceInterfaceMismatch", message)
        .with_exception_parent("InterfaceInterfaceError")
}

/// Constructs `InterfaceInterfaceMismatch` with an explicit stack, bypassing stack capture.
/// `interface_mismatch_with_stack("input", "trace").stack_string() == "trace"`.
pub fn interface_mismatch_with_stack(message: &str, stack: &str) -> Failure {
    interface_mismatch_with_stack_bytes(message.as_bytes(), stack.as_bytes())
}

/// Preserves arbitrary message and stack bytes through the first NUL of each.
/// `interface_mismatch_with_stack_bytes(&[255], &[254]).stack_bytes() == [254]`.
pub fn interface_mismatch_with_stack_bytes(message: &[u8], stack: &[u8]) -> Failure {
    Failure::with_type_bytes("InterfaceInterfaceMismatch", message, stack)
        .with_exception_parent("InterfaceInterfaceError")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_names() {
        assert_eq!(
            interface_error("x").exception_type(),
            "InterfaceInterfaceError"
        );
        assert_eq!(check_failure("x").exception_type(), "InterfaceCheckFailure");
        assert_eq!(
            interface_mismatch("x").exception_type(),
            "InterfaceInterfaceMismatch"
        );
    }
}
