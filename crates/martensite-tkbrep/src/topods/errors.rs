//! The exceptions TopoDS raises, as [`Failure`] values.
//!
//! `TopoDS_FrozenShape`, `TopoDS_LockedShape` and `TopoDS_UnCompatibleShapes` are
//! `DEFINE_STANDARD_EXCEPTION` classes derived from Standard_DomainError; the failure
//! keeps the most derived name.
use martensite_tkernel::standard::Failure;

/// `TopoDS_FrozenShape`: the shape is not free and cannot be modified;
/// `frozen_shape("TopoDS_Builder::Add: parent is not free")`.
pub fn frozen_shape(message: &str) -> Failure {
    Failure::with_message("TopoDS_FrozenShape", message)
        .with_exception_parent("Standard_DomainError")
}

/// `TopoDS_LockedShape`: the shape is locked against modifications. TopoDS itself
/// never raises it; BRep_Builder does for a locked TShape; `locked_shape("locked")`.
pub fn locked_shape(message: &str) -> Failure {
    Failure::with_message("TopoDS_LockedShape", message)
        .with_exception_parent("Standard_DomainError")
}

/// `TopoDS_UnCompatibleShapes`: a component type that the parent type cannot hold;
/// `uncompatible_shapes("TopoDS_Builder::Add: VERTEX into WIRE")`.
pub fn uncompatible_shapes(message: &str) -> Failure {
    Failure::with_message("TopoDS_UnCompatibleShapes", message)
        .with_exception_parent("Standard_DomainError")
}

/// Builds a `FrozenShape` failure with its explicit stack and domain-error base.
/// `frozen_shape_with_stack("shape rejected", "trace").stack_string() == "trace"`.
pub fn frozen_shape_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("TopoDS_FrozenShape", message, stack)
        .with_exception_parent("Standard_DomainError")
}

/// Builds a `LockedShape` failure with its explicit stack and domain-error base.
/// `locked_shape_with_stack("shape rejected", "trace").stack_string() == "trace"`.
pub fn locked_shape_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("TopoDS_LockedShape", message, stack)
        .with_exception_parent("Standard_DomainError")
}

/// Builds a `UnCompatibleShapes` failure with its explicit stack and domain-error base.
/// `uncompatible_shapes_with_stack("shape rejected", "trace").stack_string() == "trace"`.
pub fn uncompatible_shapes_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("TopoDS_UnCompatibleShapes", message, stack)
        .with_exception_parent("Standard_DomainError")
}

/// `Standard_NullObject` for a member that dereferences the TShape of a null shape (a
/// null handle dereference natively); `null_shape("TopoDS_Shape::ShapeType")`.
pub fn null_shape(member: &str) -> Failure {
    Failure::with_type(
        "Standard_NullObject",
        &format!("{member}: the shape is null; expected a shape with a TShape"),
        "",
    )
}

/// `Standard_TypeMismatch` of the `TopoDS` casts;
/// `type_mismatch("TopoDS::Vertex", "EDGE")`.
pub fn type_mismatch(cast: &str, found: &str) -> Failure {
    Failure::with_type(
        "Standard_TypeMismatch",
        &format!("{cast}: shape type {found}; expected the cast type or a null shape"),
        "",
    )
}

#[cfg(test)]
mod tests;
