//! Isolate process-wide stack capture settings from concurrent toolkit tests.
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_extrema::{uncompatible_shape, uncompatible_shape_with_stack};

#[test]
fn message_constructor_captures_but_explicit_constructor_keeps_supplied_stack() {
    let previous = Failure::default_stack_trace_length();
    Failure::set_default_stack_trace_length(8);
    let captured = uncompatible_shape("support IsVertex");
    let explicit = uncompatible_shape_with_stack("support IsVertex", "explicit caller");
    Failure::set_default_stack_trace_length(previous);
    assert!(!captured.stack_string().is_empty());
    assert_eq!(explicit.stack_string(), "explicit caller");
    assert!(captured.is_kind("Standard_DomainError"));
    assert!(explicit.is_kind("Standard_DomainError"));
}
