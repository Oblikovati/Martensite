//! Declared exchange exception bases remain observable without native libraries.
use martensite_tkxsbase::{interface, transfer};

#[test]
fn interface_failures_preserve_declared_parent_and_message() {
    for failure in [
        interface::check_failure("entity 7"),
        interface::interface_mismatch("entity 7"),
    ] {
        assert_eq!(
            failure.explicit_exception_parent(),
            Some("InterfaceInterfaceError")
        );
        assert!(failure.is_kind("InterfaceInterfaceError"));
        assert!(failure.is_kind("Standard_Failure"));
        assert!(!failure.is_kind("TransferTransferFailure"));
        assert_eq!(failure.message(), "entity 7");
    }
}

#[test]
fn transfer_failures_preserve_their_distinct_declared_bases() {
    let failure = transfer::transfer_failure("binder 11");
    let dead_loop = transfer::transfer_dead_loop("process 13");
    assert_eq!(
        failure.explicit_exception_parent(),
        Some("InterfaceInterfaceError")
    );
    assert_eq!(
        dead_loop.explicit_exception_parent(),
        Some("TransferTransferFailure")
    );
    assert!(failure.is_kind("InterfaceInterfaceError"));
    assert!(dead_loop.is_kind("TransferTransferFailure"));
    assert!(!failure.is_kind("TransferTransferDeadLoop"));
    assert_eq!(failure.message(), "binder 11");
    assert_eq!(dead_loop.message(), "process 13");
}
