//! WNT: the platform-neutral part of the Windows package.
//!
//! The Space Mouse decoder, the orientation enumeration and the class-definition exception
//! are platform-neutral and implemented. `WNT_Window` exists on every platform as an interface
//! whose Win32 backend is not implemented yet (its constructors raise `Standard_NotImplemented`).
mod hid_space_mouse;
mod window;

use martensite_tkernel::standard::Failure;

pub use window::{WntWindow, WntWindowFields, WntWindowPtr};

pub use hid_space_mouse::{
    WNT_HID_VENDOR_ID_3DCONNEXION, WNT_HID_VENDOR_ID_LOGITECH, WntHidSpaceMouse,
};

/// Portrait or landscape orientation (`WNT_OrientationType`).
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WntOrientationType {
    /// `WNT_OT_PORTRAIT`.
    Portrait = 0,
    /// `WNT_OT_LANDSCAPE`.
    Landscape = 1,
}

/// `DEFINE_STANDARD_EXCEPTION(WNT_ClassDefinitionError, Standard_ConstructionError)`: the
/// declared base that lets `Failure::is_kind` answer a catch by base kind.
const CLASS_DEFINITION_ERROR_PARENT: &str = "Standard_ConstructionError";

/// `WNT_ClassDefinitionError(theMessage)` (a `Standard_ConstructionError`), with the native
/// stack capture; `class_definition_error("why")`.
pub fn class_definition_error(message: &str) -> Failure {
    Failure::with_message("WNT_ClassDefinitionError", message)
        .with_exception_parent(CLASS_DEFINITION_ERROR_PARENT)
}
/// `WNT_ClassDefinitionError(theMessage, theStackTrace)`: an explicit stack, no capture;
/// `class_definition_error_with_stack("why", "trace")`.
pub fn class_definition_error_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("WNT_ClassDefinitionError", message, stack)
        .with_exception_parent(CLASS_DEFINITION_ERROR_PARENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A catch of the `Standard_ConstructionError` kind catches WNT_ClassDefinitionError,
    /// a catch of `Standard_OutOfRange` does not.
    #[test]
    fn class_definition_error_is_caught_by_its_construction_error_base() {
        let stacked = class_definition_error_with_stack("why", "trace");
        for failure in [class_definition_error("why"), stacked.clone()] {
            assert_eq!(failure.exception_type(), "WNT_ClassDefinitionError");
            assert_eq!(failure.message(), "why");
            assert!(failure.is_kind("Standard_ConstructionError"));
            assert!(failure.is_kind("Standard_DomainError"));
            assert!(failure.is_kind("Standard_Failure"));
            assert!(!failure.is_kind("Standard_OutOfRange"));
        }
        assert_eq!(stacked.stack_string(), "trace");
    }

    #[test]
    fn win32_window_constructors_raise_the_unsupported_platform_error() {
        use crate::platform::is_platform_unsupported;
        let created = WntWindow::new("Viewer", 0, 0, 640, 480);
        assert!(created.is_err_and(|failure| is_platform_unsupported(&failure)));
        let wrapped = WntWindow::from_handle(1);
        assert!(wrapped.is_err_and(|failure| is_platform_unsupported(&failure)));
    }
}
