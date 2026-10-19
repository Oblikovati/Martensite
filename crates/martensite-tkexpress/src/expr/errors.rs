//! Typed expression failures for invalid operations and unresolved values.
use martensite_tkernel::standard::Failure;

/// expression failure; `expr_failure("Expr: unexpected state")`.
pub fn expr_failure(message: &str) -> Failure {
    Failure::with_message("Expr_ExprFailure", message)
}
/// Expression-specific failures share one family for category queries.
const EXPR_FAILURE_PARENT: &str = "Expr_ExprFailure";

/// Capture the stack while retaining the expression failure family.
fn derived(exception_type: &'static str, message: &str) -> Failure {
    Failure::with_message(exception_type, message).with_exception_parent(EXPR_FAILURE_PARENT)
}

/// Retain an explicit stack without capturing another, preserving the failure family.
fn derived_with_stack(exception_type: &'static str, message: &str, stack: &str) -> Failure {
    Failure::with_type(exception_type, message, stack).with_exception_parent(EXPR_FAILURE_PARENT)
}

/// an unresolved-value failure; `not_evaluable("ExprNamedUnknown::Evaluate: x not in vars")`.
pub fn not_evaluable(message: &str) -> Failure {
    derived("Expr_NotEvaluable", message)
}
/// an unresolved-value failure with an explicit stack; `not_evaluable_with_stack("m", "trace")`.
pub fn not_evaluable_with_stack(message: &str, stack: &str) -> Failure {
    derived_with_stack("Expr_NotEvaluable", message, stack)
}
/// an unassigned-variable failure; `not_assigned("ExprNamedUnknown::AssignedExpression: x")`.
pub fn not_assigned(message: &str) -> Failure {
    derived("Expr_NotAssigned", message)
}
/// an unassigned-variable failure with an explicit stack; `not_assigned_with_stack("m", "trace")`.
pub fn not_assigned_with_stack(message: &str, stack: &str) -> Failure {
    derived_with_stack("Expr_NotAssigned", message, stack)
}
/// an invalid-assignment failure; `invalid_assignment("ExprNamedUnknown::Assign: x+1 contains x")`.
pub fn invalid_assignment(message: &str) -> Failure {
    derived("Expr_InvalidAssignment", message)
}
/// an invalid-assignment failure with an explicit stack;
/// `invalid_assignment_with_stack("m", "trace")`.
pub fn invalid_assignment_with_stack(message: &str, stack: &str) -> Failure {
    derived_with_stack("Expr_InvalidAssignment", message, stack)
}
/// an invalid-function failure; `invalid_function("ExprUnaryFunction: 2 variables; expected 1")`.
pub fn invalid_function(message: &str) -> Failure {
    derived("Expr_InvalidFunction", message)
}
/// an invalid-function failure with an explicit stack;
/// `invalid_function_with_stack("m", "trace")`.
pub fn invalid_function_with_stack(message: &str, stack: &str) -> Failure {
    derived_with_stack("Expr_InvalidFunction", message, stack)
}
/// an invalid-operand failure; `invalid_operand("SetOperand: the operand contains the expression")`.
pub fn invalid_operand(message: &str) -> Failure {
    derived("Expr_InvalidOperand", message)
}
/// an invalid-operand failure with an explicit stack; `invalid_operand_with_stack("m", "trace")`.
pub fn invalid_operand_with_stack(message: &str, stack: &str) -> Failure {
    derived_with_stack("Expr_InvalidOperand", message, stack)
}
/// an out-of-range failure; `out_of_range("SubExpression: I = 3; expected 1")`.
pub fn out_of_range(message: &str) -> Failure {
    Failure::out_of_range(message)
}
/// a dimension-mismatch failure; `dimension_mismatch("RemoveOperand: 2 operands; expected > 2")`.
pub fn dimension_mismatch(message: &str) -> Failure {
    Failure::with_message("Standard_DimensionMismatch", message)
}
/// a missing-object failure; `no_such_object("ExprRelationIterator::Value: no more relation")`.
pub fn no_such_object(message: &str) -> Failure {
    Failure::with_message("Standard_NoSuchObject", message)
}
/// an exhausted-iterator failure; `no_more_object("ExprUnknownIterator::Next: past the end")`.
pub fn no_more_object(message: &str) -> Failure {
    Failure::with_message("Standard_NoMoreObject", message)
}
/// Report a call through a null handle; `null_object("ExprSum::Evaluate: null operand 2")`.
pub fn null_object(message: &str) -> Failure {
    Failure::with_message("Standard_NullObject", message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_constructor_preserves_its_failure_category() {
        let cases: [(Failure, &str); 11] = [
            (expr_failure("a"), "Expr_ExprFailure"),
            (not_evaluable("a"), "Expr_NotEvaluable"),
            (not_assigned("a"), "Expr_NotAssigned"),
            (invalid_assignment("a"), "Expr_InvalidAssignment"),
            (invalid_function("a"), "Expr_InvalidFunction"),
            (invalid_operand("a"), "Expr_InvalidOperand"),
            (out_of_range("a"), "Standard_OutOfRange"),
            (dimension_mismatch("a"), "Standard_DimensionMismatch"),
            (no_such_object("a"), "Standard_NoSuchObject"),
            (no_more_object("a"), "Standard_NoMoreObject"),
            (null_object("a"), "Standard_NullObject"),
        ];
        for (failure, name) in cases {
            assert_eq!(failure.exception_type(), name);
            assert_eq!(failure.message(), "a");
        }
    }

    /// Both constructor forms retain expression-family ancestry without domain-error ancestry.
    #[test]
    fn derived_exceptions_are_caught_by_expr_failure() {
        let cases: [(Failure, Failure, &str); 5] = [
            (
                not_evaluable("m"),
                not_evaluable_with_stack("m", "trace"),
                "Expr_NotEvaluable",
            ),
            (
                not_assigned("m"),
                not_assigned_with_stack("m", "trace"),
                "Expr_NotAssigned",
            ),
            (
                invalid_assignment("m"),
                invalid_assignment_with_stack("m", "trace"),
                "Expr_InvalidAssignment",
            ),
            (
                invalid_function("m"),
                invalid_function_with_stack("m", "trace"),
                "Expr_InvalidFunction",
            ),
            (
                invalid_operand("m"),
                invalid_operand_with_stack("m", "trace"),
                "Expr_InvalidOperand",
            ),
        ];
        for (one, two, name) in cases {
            for failure in [&one, &two] {
                assert_eq!(failure.exception_type(), name);
                assert!(failure.is_kind(name));
                assert!(failure.is_kind("Expr_ExprFailure"));
                assert!(failure.is_kind("Standard_Failure"));
                assert!(!failure.is_kind("Standard_DomainError"));
                assert_eq!(failure.message(), "m");
            }
            assert_eq!(two.stack_string(), "trace");
        }
        assert!(!expr_failure("m").is_kind("Expr_NotEvaluable"));
    }
}
