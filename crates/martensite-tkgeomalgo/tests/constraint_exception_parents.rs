//! The solver exceptions retain their declared domain-error parent and explicit stack.
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::{gcc_ent, geom2d_gcc};

fn check_exception(failure: &Failure, name: &str, stack: &str) {
    assert_eq!(failure.exception_type(), name);
    assert_eq!(failure.message(), "invalid qualifier 7");
    assert_eq!(failure.stack_string(), stack);
    assert!(failure.is_kind(name));
    assert!(failure.is_kind("Standard_DomainError"));
    assert!(failure.is_kind("Standard_Failure"));
    assert!(!failure.is_kind("Standard_OutOfRange"));
}

#[test]
fn bad_qualifier_message_and_explicit_stack_preserve_domain_error_parent() {
    let failure = gcc_ent::bad_qualifier("invalid qualifier 7");
    assert!(failure.is_kind("Standard_DomainError"));
    let failure = gcc_ent::bad_qualifier_with_stack("invalid qualifier 7", "stack A");
    check_exception(&failure, "GccEnt_BadQualifier", "stack A");
}

#[test]
fn parallel_message_and_explicit_stack_preserve_domain_error_parent() {
    let failure = geom2d_gcc::is_parallel("invalid qualifier 7");
    assert!(failure.is_kind("Standard_DomainError"));
    let failure = geom2d_gcc::is_parallel_with_stack("invalid qualifier 7", "stack B");
    check_exception(&failure, "Geom2dGcc_IsParallel", "stack B");
}
