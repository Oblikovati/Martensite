//! Shape failures preserve the declared domain-error catch contract.
use super::{
    frozen_shape, frozen_shape_with_stack, locked_shape, locked_shape_with_stack,
    uncompatible_shapes, uncompatible_shapes_with_stack,
};
use martensite_tkernel::standard::Failure;

type MessageConstructor = fn(&str) -> Failure;

#[test]
fn shape_exception_messages_keep_their_domain_error_base() {
    let constructors: [(&str, MessageConstructor); 3] = [
        ("TopoDS_FrozenShape", frozen_shape),
        ("TopoDS_LockedShape", locked_shape),
        ("TopoDS_UnCompatibleShapes", uncompatible_shapes),
    ];
    for (name, constructor) in constructors {
        let failure = constructor("shape contract rejected its input");
        assert_eq!(failure.exception_type(), name);
        assert_eq!(failure.message(), "shape contract rejected its input");
        assert!(failure.is_kind("Standard_DomainError"), "{name}");
        assert!(failure.is_kind("Standard_Failure"), "{name}");
        assert!(!failure.is_kind("Standard_OutOfRange"), "{name}");
    }
}

type StackConstructor = fn(&str, &str) -> Failure;

#[test]
fn shape_exception_explicit_stacks_keep_their_base_through_clone() {
    let constructors: [(&str, StackConstructor); 3] = [
        ("TopoDS_FrozenShape", frozen_shape_with_stack),
        ("TopoDS_LockedShape", locked_shape_with_stack),
        ("TopoDS_UnCompatibleShapes", uncompatible_shapes_with_stack),
    ];
    for (name, constructor) in constructors {
        let failure = constructor("shape rejected", "owned trace").clone();
        assert_eq!(failure.exception_type(), name);
        assert_eq!(failure.message(), "shape rejected");
        assert_eq!(failure.stack_string(), "owned trace");
        assert!(failure.is_kind("Standard_DomainError"), "{name}");
        assert!(!failure.is_kind("Standard_OutOfRange"), "{name}");
    }
}
