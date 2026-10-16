//! The package's `DEFINE_STANDARD_EXCEPTION` classes as typed `Failure` constructors.
use martensite_tkernel::standard::Failure;

/// The not-square exception, derived from `dimension failure`: the declared base
/// that lets `Failure::is_kind` answer a catch by base kind.
const NOT_SQUARE_PARENT: &str = "Standard_DimensionError";

/// `non-square-matrix failure` (a `dimension failure`), raised for non-square input; the
/// one-argument constructor, with the stack capture;
/// `not_square("Matrix::invert() - matrix is not square")`.
pub fn not_square(message: &str) -> Failure {
    Failure::with_message("math_NotSquare", message).with_exception_parent(NOT_SQUARE_PARENT)
}

/// The `non-square-matrix failure(theMessage, theStackTrace)` constructor: an explicit stack, no capture;
/// `not_square_with_stack("not square", "trace")`.
pub fn not_square_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("math_NotSquare", message, stack).with_exception_parent(NOT_SQUARE_PARENT)
}

/// `singular-matrix failure` (a `failure`), raised when a factorization has no pivot;
/// `singular_matrix("")`.
pub fn singular_matrix(message: &str) -> Failure {
    Failure::with_type("math_SingularMatrix", message, "")
}
