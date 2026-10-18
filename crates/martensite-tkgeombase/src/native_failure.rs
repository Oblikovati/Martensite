//! retained exception values shared by the TKGeomBase ports that raise them from
//! container accesses (FEmTool, AppParCurves).
use martensite_tkernel::ncollection::ArrayError;
use martensite_tkernel::standard::Failure;

/// A retained exception of class `kind` with `message`, without a stack trace;
/// `failure("domain failure", "Assembly::AddMatrix")`.
pub(crate) fn failure(kind: &'static str, message: &str) -> Failure {
    Failure::with_type(kind, message, "")
}

/// The exception an NCollection or math container raises for `error`: bad bounds
/// are range failure (the allocating constructors), a bad index
/// index-range failure and a size mismatch dimension mismatch;
/// `array_failure(error, "Curve::Knots")`.
pub(crate) fn array_failure(error: ArrayError, context: &str) -> Failure {
    let kind: &'static str = match error {
        ArrayError::Bounds { .. } => "Standard_RangeError",
        ArrayError::Index { .. } => "Standard_OutOfRange",
        _ => "Standard_DimensionMismatch",
    };
    failure(kind, &format!("{context}: {error}"))
}

/// The exception of a retained `Array1(Lower, Upper)` allocation: bounds
/// with `Upper < Lower - 1` wrap the size to a huge value whose allocation fails
/// (natively a null buffer, then a crash on the first write); the port raises
/// OutOfMemory (LinearCriteria, Variational);
/// `allocation_failure(error, "Variational")`.
pub(crate) fn allocation_failure(error: ArrayError, context: &str) -> Failure {
    match error {
        ArrayError::Bounds { .. } => {
            failure("Standard_OutOfMemory", &format!("{context}: {error}"))
        }
        other => array_failure(other, context),
    }
}
