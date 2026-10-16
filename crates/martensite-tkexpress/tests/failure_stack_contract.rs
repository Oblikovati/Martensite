//! This isolated executable is the only writer of its process-wide trace setting.
use martensite_tkernel::standard::Failure;
use martensite_tkexpress::{expr::errors, expr_intrp::errors::syntax_error};

type MessageFactory = fn(&str) -> Failure;
struct TraceSetting(i32);
impl TraceSetting {
    fn disabled() -> Self {
        let previous = Failure::default_stack_trace_length();
        Failure::set_default_stack_trace_length(0);
        Self(previous)
    }
}
impl Drop for TraceSetting {
    fn drop(&mut self) {
        Failure::set_default_stack_trace_length(self.0);
    }
}
fn message_factories() -> [(&'static str, MessageFactory); 12] {
    [
        ("Expr_ExprFailure", errors::expr_failure),
        ("Expr_NotEvaluable", errors::not_evaluable),
        ("Expr_NotAssigned", errors::not_assigned),
        ("Expr_InvalidAssignment", errors::invalid_assignment),
        ("Expr_InvalidFunction", errors::invalid_function),
        ("Expr_InvalidOperand", errors::invalid_operand),
        ("Standard_OutOfRange", errors::out_of_range),
        ("Standard_DimensionMismatch", errors::dimension_mismatch),
        ("Standard_NoSuchObject", errors::no_such_object),
        ("Standard_NoMoreObject", errors::no_more_object),
        ("Standard_NullObject", errors::null_object),
        ("ExprIntrp_SyntaxError", syntax_error),
    ]
}
fn missing_captured_stacks() -> Vec<&'static str> {
    Failure::set_default_stack_trace_length(8);
    assert!(!Failure::new("control").stack_string().is_empty());
    message_factories()
        .into_iter()
        .filter_map(|(name, factory)| {
            let failure = factory("message");
            assert_eq!(failure.exception_type(), name);
            assert_eq!(failure.message(), "message");
            failure.stack_string().is_empty().then_some(name)
        })
        .collect()
}
fn assert_explicit_stacks_do_not_capture() {
    for constructor in [
        errors::not_evaluable_with_stack,
        errors::not_assigned_with_stack,
        errors::invalid_assignment_with_stack,
        errors::invalid_function_with_stack,
        errors::invalid_operand_with_stack,
    ] {
        assert_eq!(
            constructor("message", "explicit trace").stack_string(),
            "explicit trace"
        );
        assert!(constructor("message", "").stack_string().is_empty());
    }
    for (name, _) in message_factories() {
        assert_explicit_base_stack(name);
    }
}
fn assert_explicit_base_stack(name: &'static str) {
    let explicit = Failure::with_type(name, "message", "explicit trace");
    assert_eq!(explicit.stack_string(), "explicit trace");
    assert!(
        Failure::with_type(name, "message", "")
            .stack_string()
            .is_empty()
    );
}
#[test]
fn every_message_constructor_honors_capture_and_explicit_stack_constructors_do_not() {
    let previous = Failure::default_stack_trace_length();
    let setting = TraceSetting::disabled();
    for (_, factory) in message_factories() {
        assert!(factory("message").stack_string().is_empty());
    }
    let missing = missing_captured_stacks();
    assert_explicit_stacks_do_not_capture();
    assert_eq!(
        missing,
        Vec::<&str>::new(),
        "message constructors missing configured stack capture"
    );
    drop(setting);
    assert_eq!(Failure::default_stack_trace_length(), previous);
}
