//! Typed failures for invalid geometry, undefined values and undefined derivatives.
use martensite_tkernel::standard::Failure;

/// Geometry evaluation failures belong to the domain-error family.
const UNDEFINED_PARENT: &str = "Standard_DomainError";

/// Build an undefined-derivative failure with stack capture;
/// `undefined_derivative("Geom2dLine::EvalDN: N = 0; expected >= 1")`.
pub fn undefined_derivative(message: &str) -> Failure {
    Failure::with_message("Geom2d_UndefinedDerivative", message)
        .with_exception_parent(UNDEFINED_PARENT)
}
/// Build an undefined-derivative failure with an explicit stack,
/// no capture; `undefined_derivative_with_stack("EvalDN", "trace")`.
pub fn undefined_derivative_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("Geom2d_UndefinedDerivative", message, stack)
        .with_exception_parent(UNDEFINED_PARENT)
}
/// Build an undefined-value failure with stack capture;
/// `undefined_value("Geom2dOffsetCurve::EvalD0: ...")`.
pub fn undefined_value(message: &str) -> Failure {
    Failure::with_message("Geom2d_UndefinedValue", message).with_exception_parent(UNDEFINED_PARENT)
}
/// Build an undefined-value failure with an explicit stack, no
/// capture; `undefined_value_with_stack("EvalD0", "trace")`.
pub fn undefined_value_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("Geom2d_UndefinedValue", message, stack)
        .with_exception_parent(UNDEFINED_PARENT)
}
/// `a construction-error failure`; `construction("Geom2dCircle: Radius = -1; expected >= 0")`.
pub fn construction(message: &str) -> Failure {
    Failure::with_type("Standard_ConstructionError", message, "")
}
/// `a range-error failure`; `range("IsCN: N = -1; expected >= 0")`.
pub fn range(message: &str) -> Failure {
    Failure::with_type("Standard_RangeError", message, "")
}
/// `an out-of-range failure`; `out_of_range("Pole: Index = 9; expected in [1, 4]")`.
pub fn out_of_range(message: &str) -> Failure {
    Failure::out_of_range(message)
}
/// `a domain-error failure`; `domain("Segment: U2 = 0 < U1 = 1; expected U1 <= U2")`.
pub fn domain(message: &str) -> Failure {
    Failure::with_type("Standard_DomainError", message, "")
}
/// `a no-such-object failure`; `no_such_object("Period: the curve is not periodic")`.
pub fn no_such_object(message: &str) -> Failure {
    Failure::with_type("Standard_NoSuchObject", message, "")
}
/// `a dimension-error failure`; `dimension("Poles: length 3; expected 4")`.
pub fn dimension(message: &str) -> Failure {
    Failure::with_type("Standard_DimensionError", message, "")
}
/// `a not-implemented failure` (a a program-error failure internally);
/// `not_implemented("Geom2dEvalSineWaveCurve::Reverse")`.
pub fn not_implemented(message: &str) -> Failure {
    Failure::with_type("Standard_NotImplemented", message, "")
}
/// `a program-error failure`; `program("SetEvalRepresentation: null Representation")`.
pub fn program(message: &str) -> Failure {
    Failure::with_type("Standard_ProgramError", message, "")
}
/// `a type-mismatch failure`; `type_mismatch("Adaptor2dOffsetCurve::IntervalContinuity")`.
pub fn type_mismatch(message: &str) -> Failure {
    Failure::with_type("Standard_TypeMismatch", message, "")
}
/// `a null-object failure`; `null_object("Geom2dAdaptorCurve::Load: null curve")`.
pub fn null_object(message: &str) -> Failure {
    Failure::with_type("Standard_NullObject", message, "")
}
/// Convert an `NCollection` bound failure into the defined an out-of-range failure;
/// `array.value(i).map_err(array_bounds)?`.
pub fn array_bounds(error: impl std::fmt::Display) -> Failure {
    Failure::out_of_range(&error.to_string())
}
