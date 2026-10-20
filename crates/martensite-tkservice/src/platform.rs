//! The typed error of platform interfaces whose backend is not implemented yet.
//!
//! Every such interface exists on every platform; where its backend is missing it raises
//! `Standard_NotImplemented` naming the interface and the backend, and its source carries a
//! `TODO(platform)` comment for the port on that platform.
use martensite_tkernel::standard::Failure;

/// The exception type of a platform interface without an implemented backend.
pub const PLATFORM_UNSUPPORTED_TYPE: &str = "Standard_NotImplemented";

/// `Standard_NotImplemented` for `interface` without its `backend`;
/// `unsupported("Cocoa_Window", "AppKit (macOS)")`.
pub(crate) fn unsupported(interface: &str, backend: &str) -> Failure {
    let message = format!(
        "{interface}: the {backend} backend is not implemented on this platform; expected a platform with that backend implemented"
    );
    Failure::with_type(PLATFORM_UNSUPPORTED_TYPE, &message, "")
}

/// Whether `failure` is the error of a platform interface without an implemented backend;
/// `is_platform_unsupported(&failure)`.
pub fn is_platform_unsupported(failure: &Failure) -> bool {
    failure.exception_type() == PLATFORM_UNSUPPORTED_TYPE
}
