//! The exception classes of the Graphic3d package, as [`Failure`] values.
//!
//! Each `DEFINE_STANDARD_EXCEPTION(Graphic3d_*Error, Standard_OutOfRange)` becomes two
//! constructors: the one-argument form captures the stack like the native constructor, the
//! `*_with_stack` form ports the `(theMessage, theStackTrace)` constructor. The failure keeps
//! the most derived class name, which `Failure::exception_type` reports, and the declared base,
//! so `Failure::is_kind` answers a catch by base kind.
use martensite_tkernel::standard::Failure;

/// The base every Graphic3d exception class declares in `DEFINE_STANDARD_EXCEPTION`.
const GRAPHIC3D_EXCEPTION_PARENT: &str = "Standard_OutOfRange";

macro_rules! define_graphic3d_exceptions {
    ($($constructor:ident, $stacked:ident => $name:literal;)*) => {
        $(
            #[doc = concat!("`", $name, "(theMessage)`, with the native stack capture; `",
                stringify!($constructor), "(\"bad value 12\")`.")]
            pub fn $constructor(message: &str) -> Failure {
                Failure::with_message($name, message).with_exception_parent(GRAPHIC3D_EXCEPTION_PARENT)
            }
            #[doc = concat!("`", $name, "(theMessage, theStackTrace)`: an explicit stack, no capture; `",
                stringify!($stacked), "(\"bad value 12\", \"trace\")`.")]
            pub fn $stacked(message: &str, stack: &str) -> Failure {
                Failure::with_type($name, message, stack).with_exception_parent(GRAPHIC3D_EXCEPTION_PARENT)
            }
        )*
    };
}

define_graphic3d_exceptions! {
    group_definition_error, group_definition_error_with_stack => "Graphic3d_GroupDefinitionError";
    material_definition_error, material_definition_error_with_stack => "Graphic3d_MaterialDefinitionError";
    priority_definition_error, priority_definition_error_with_stack => "Graphic3d_PriorityDefinitionError";
    structure_definition_error, structure_definition_error_with_stack => "Graphic3d_StructureDefinitionError";
}

#[cfg(test)]
mod tests {
    use super::*;

    type ExceptionCase = (fn(&str) -> Failure, fn(&str, &str) -> Failure, &'static str);
    const CASES: [ExceptionCase; 4] = [
        (
            group_definition_error,
            group_definition_error_with_stack,
            "Graphic3d_GroupDefinitionError",
        ),
        (
            material_definition_error,
            material_definition_error_with_stack,
            "Graphic3d_MaterialDefinitionError",
        ),
        (
            priority_definition_error,
            priority_definition_error_with_stack,
            "Graphic3d_PriorityDefinitionError",
        ),
        (
            structure_definition_error,
            structure_definition_error_with_stack,
            "Graphic3d_StructureDefinitionError",
        ),
    ];

    /// A catch of the `Standard_OutOfRange` kind and every base above it catches each class;
    /// unrelated bases do not.
    #[test]
    fn graphic3d_exceptions_are_caught_by_their_declared_bases() {
        for (constructor, stacked, name) in CASES {
            for failure in [constructor("why"), stacked("why", "trace")] {
                assert_eq!(failure.exception_type(), name);
                assert_eq!(failure.message(), "why");
                for base in [
                    "Standard_OutOfRange",
                    "Standard_RangeError",
                    "Standard_DomainError",
                    "Standard_Failure",
                ] {
                    assert!(failure.is_kind(base), "{name} should derive from {base}");
                }
                assert!(!failure.is_kind("Standard_ProgramError"));
                assert!(!failure.is_kind("Standard_ConstructionError"));
            }
            assert_eq!(stacked("why", "trace").stack_string(), "trace");
        }
    }
}
