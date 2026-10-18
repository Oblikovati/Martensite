//! Undefined-axis failures retain their declared domain-error base.
use super::{undefined_axis, undefined_axis_with_stack};

#[test]
fn undefined_axis_is_a_domain_error_and_preserves_its_message() {
    let failure = undefined_axis("principal moments have no symmetry axis");
    assert_eq!(failure.exception_type(), "GProp_UndefinedAxis");
    assert_eq!(failure.message(), "principal moments have no symmetry axis");
    assert!(failure.is_kind("GProp_UndefinedAxis"));
    assert!(failure.is_kind("Standard_DomainError"));
    assert!(failure.is_kind("Standard_Failure"));
    assert!(!failure.is_kind("Standard_OutOfRange"));
}

#[test]
fn explicit_axis_stack_keeps_the_declared_base_through_clone() {
    let failure = undefined_axis_with_stack("axis unavailable", "owned trace").clone();
    assert_eq!(failure.message(), "axis unavailable");
    assert_eq!(failure.stack_string(), "owned trace");
    assert!(failure.is_kind("Standard_DomainError"));
    assert!(!failure.is_kind("Standard_RangeError"));
}
