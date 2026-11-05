//! The statuses of a transfer binder and the exceptions of the Transfer package (as
//! [`Failure`] values with their native names).
use martensite_tkernel::standard::Failure;

/// `TransferStatusResult`: whether a binder has a result, and whether it was used.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TransferStatusResult {
    /// `TransferStatusVoid`.
    #[default]
    StatusVoid = 0,
    /// `TransferStatusDefined`.
    StatusDefined = 1,
    /// `TransferStatusUsed`.
    StatusUsed = 2,
}

/// `TransferStatusExec`: the execution status of a transfer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TransferStatusExec {
    /// `TransferStatusInitial`.
    #[default]
    StatusInitial = 0,
    /// `TransferStatusRun`.
    StatusRun = 1,
    /// `TransferStatusDone`.
    StatusDone = 2,
    /// `TransferStatusError`.
    StatusError = 3,
    /// `TransferStatusLoop`.
    StatusLoop = 4,
}

/// `TransferTransferFailure` (an `InterfaceInterfaceError`);
/// `transfer_failure("Binder: SetResult, Result is Already Set")`.
pub fn transfer_failure(message: &str) -> Failure {
    transfer_failure_bytes(message.as_bytes())
}

/// `TransferTransferDeadLoop` (a `TransferTransferFailure`);
/// `transfer_dead_loop("TransferProcess: Transfer in Loop")`.
pub fn transfer_dead_loop(message: &str) -> Failure {
    transfer_dead_loop_bytes(message.as_bytes())
}

/// The byte-message constructor of `TransferTransferFailure`, including configured stack capture.
/// `transfer_failure_bytes(&[255, 17]).what() == [255, 17]`.
pub fn transfer_failure_bytes(message: &[u8]) -> Failure {
    Failure::with_message_bytes("TransferTransferFailure", message)
        .with_exception_parent("InterfaceInterfaceError")
}

/// Constructs `TransferTransferFailure` with an explicit stack, bypassing stack capture.
/// `transfer_failure_with_stack("input", "trace").stack_string() == "trace"`.
pub fn transfer_failure_with_stack(message: &str, stack: &str) -> Failure {
    transfer_failure_with_stack_bytes(message.as_bytes(), stack.as_bytes())
}

/// Preserves arbitrary message and stack bytes through the first NUL of each.
/// `transfer_failure_with_stack_bytes(&[255], &[254]).stack_bytes() == [254]`.
pub fn transfer_failure_with_stack_bytes(message: &[u8], stack: &[u8]) -> Failure {
    Failure::with_type_bytes("TransferTransferFailure", message, stack)
        .with_exception_parent("InterfaceInterfaceError")
}

/// The byte-message constructor of `TransferTransferDeadLoop`, including configured stack capture.
/// `transfer_dead_loop_bytes(&[255, 17]).what() == [255, 17]`.
pub fn transfer_dead_loop_bytes(message: &[u8]) -> Failure {
    Failure::with_message_bytes("TransferTransferDeadLoop", message)
        .with_exception_parent("TransferTransferFailure")
}

/// Constructs `TransferTransferDeadLoop` with an explicit stack, bypassing stack capture.
/// `transfer_dead_loop_with_stack("input", "trace").stack_string() == "trace"`.
pub fn transfer_dead_loop_with_stack(message: &str, stack: &str) -> Failure {
    transfer_dead_loop_with_stack_bytes(message.as_bytes(), stack.as_bytes())
}

/// Preserves arbitrary message and stack bytes through the first NUL of each.
/// `transfer_dead_loop_with_stack_bytes(&[255], &[254]).stack_bytes() == [254]`.
pub fn transfer_dead_loop_with_stack_bytes(message: &[u8], stack: &[u8]) -> Failure {
    Failure::with_type_bytes("TransferTransferDeadLoop", message, stack)
        .with_exception_parent("TransferTransferFailure")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminants_and_names() {
        assert_eq!(TransferStatusResult::StatusUsed as i32, 2);
        assert_eq!(TransferStatusExec::StatusLoop as i32, 4);
        assert_eq!(
            transfer_failure("x").exception_type(),
            "TransferTransferFailure"
        );
        assert_eq!(
            transfer_dead_loop("x").exception_type(),
            "TransferTransferDeadLoop"
        );
    }

    /// Every enumerator keeps its native TransferStatus* order and value.
    #[test]
    fn every_enumerator_keeps_native_value() {
        use TransferStatusExec as X;
        let exec = [
            X::StatusInitial,
            X::StatusRun,
            X::StatusDone,
            X::StatusError,
            X::StatusLoop,
        ];
        assert!(exec.iter().enumerate().all(|(i, v)| *v as i32 == i as i32));
        use TransferStatusResult as R;
        let result = [R::StatusVoid, R::StatusDefined, R::StatusUsed];
        assert!(
            result
                .iter()
                .enumerate()
                .all(|(i, v)| *v as i32 == i as i32)
        );
    }
}
