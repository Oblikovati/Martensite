//! Concrete surface-search accessor failures use the native payload and capture policy.
#[path = "intpatch_accessor_contract/boundary.rs"]
mod boundary;
#[path = "intpatch_accessor_contract/fixtures.rs"]
mod fixtures;
#[path = "intpatch_accessor_contract/interior.rs"]
mod interior;
#[path = "intpatch_accessor_contract/payload.rs"]
mod payload;
use martensite_tkernel::standard::Failure;
use std::process::Command;
const CAPTURE_ENV: &str = "MARTENSITE_INTPATCH_ACCESSOR_STACK_LENGTH";

fn checked_failure<T>(result: Result<T, Failure>, kind: &str, message: &str, capture: bool) {
    let failure = result.err().expect("expected a concrete accessor failure");
    assert_eq!(failure.exception_type(), kind);
    assert_eq!(failure.message_bytes(), message.as_bytes());
    assert_eq!(!failure.stack_bytes().is_empty(), capture);
}

fn range_failure<T>(result: Result<T, Failure>, index: i32, capture: bool) {
    let message = if index < 0 {
        "NCollection_Sequence::Value: negative index"
    } else {
        "NCollection_Sequence::Value"
    };
    checked_failure(result, "Standard_OutOfRange", message, capture);
}

fn execute_child(length: i32) {
    Failure::set_default_stack_trace_length(length);
    boundary::check(length > 0);
    interior::check(length > 0);
}

fn isolated_child(test_name: &str, length: i32) {
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", test_name, "--nocapture", "--test-threads=1"])
        .env(CAPTURE_ENV, length.to_string())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "child length={length}, stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("1 passed; 0 failed"),
        "child length={length} must execute the exact accessor test"
    );
}

#[test]
fn concrete_accessors_keep_payloads_and_capture_in_isolated_processes() {
    if let Ok(length) = std::env::var(CAPTURE_ENV) {
        execute_child(length.parse().unwrap());
        return;
    }
    let thread = std::thread::current();
    let test_name = thread.name().expect("named test thread");
    for length in [-2, 0, 4] {
        isolated_child(test_name, length);
    }
}
