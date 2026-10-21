//! `BRepExtrema_SupportType`, the kind of sub-shape that carries a solution point, and
//! the `BRepExtrema_UnCompatibleShape` exception.
use martensite_tkernel::standard::Failure;

/// `BRepExtrema_SupportType`: a solution point is a vertex, lies on an edge or lies
/// inside a face; `assert_eq!(BRepExtremaSupportType::IsInFace as i32, 2)`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum BRepExtremaSupportType {
    /// `BRepExtrema_IsVertex`.
    #[default]
    IsVertex = 0,
    /// `BRepExtrema_IsOnEdge`.
    IsOnEdge = 1,
    /// `BRepExtrema_IsInFace`.
    IsInFace = 2,
}

/// `BRepExtrema_UnCompatibleShape` (a `Standard_DomainError`): a parameter accessor
/// asked for a solution whose support has another kind;
/// `uncompatible_shape("ParOnEdgeS1 is impossible without EDGE (support IsVertex)")`.
pub fn uncompatible_shape(message: &str) -> Failure {
    Failure::with_message("BRepExtrema_UnCompatibleShape", message)
        .with_exception_parent("Standard_DomainError")
}

/// Construct an incompatible-support failure with an explicit stack and domain base.
/// `uncompatible_shape_with_stack("expected EDGE", "caller trace").stack_string() == "caller trace"`.
pub fn uncompatible_shape_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("BRepExtrema_UnCompatibleShape", message, stack)
        .with_exception_parent("Standard_DomainError")
}
