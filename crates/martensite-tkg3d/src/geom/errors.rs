//! retained exception types raised by Geom, as [`Failure`] values.
//!
//! `GeomUndefinedDerivative` and `GeomUndefinedValue` derive from
//! StandardDomainError natively; the failure keeps the most derived name and
//! carries that declared base. The
//! Standard exceptions are shared with Geom2d.
use martensite_tkernel::standard::Failure;
pub use martensite_tkg2d::geom2d::errors::{
    array_bounds, construction, dimension, domain, no_such_object, null_object, out_of_range,
    program, range,
};

/// Undefined geometry failures retain domain ancestry for catch-by-category.
const UNDEFINED_PARENT: &str = "Standard_DomainError";

/// `GeomUndefinedDerivative`, the one-argument constructor with the retained stack capture;
/// `undefined_derivative("GeomLine::EvalDN: N = 0; expected >= 1")`.
pub fn undefined_derivative(message: &str) -> Failure {
    Failure::with_message("Geom_UndefinedDerivative", message)
        .with_exception_parent(UNDEFINED_PARENT)
}
/// The `GeomUndefinedDerivative(theMessage, theStackTrace)` constructor: an explicit stack,
/// no capture; `undefined_derivative_with_stack("EvalDN", "trace")`.
pub fn undefined_derivative_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("Geom_UndefinedDerivative", message, stack)
        .with_exception_parent(UNDEFINED_PARENT)
}
/// `GeomUndefinedValue`, the one-argument constructor with the retained stack capture;
/// `undefined_value("GeomOffsetCurve::EvalD0: ...")`.
pub fn undefined_value(message: &str) -> Failure {
    Failure::with_message("Geom_UndefinedValue", message).with_exception_parent(UNDEFINED_PARENT)
}
/// The `GeomUndefinedValue(theMessage, theStackTrace)` constructor: an explicit stack, no
/// capture; `undefined_value_with_stack("EvalD0", "trace")`.
pub fn undefined_value_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("Geom_UndefinedValue", message, stack)
        .with_exception_parent(UNDEFINED_PARENT)
}
/// `StandardNotImplemented`; `not_implemented("GeomSurfaceOfRevolution::...")`.
pub fn not_implemented(message: &str) -> Failure {
    Failure::with_type("Standard_NotImplemented", message, "")
}
