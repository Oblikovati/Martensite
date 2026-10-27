//! Explicit refusal when a backend cannot consume a live buffer-kind field.
use super::Graphic3dBufferType;
use std::fmt;

/// The reason reference-aware capture was refused; `CaptureBufferKindCause::UnsupportedReference`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureBufferKindCause {
    /// The backend implements only immutable capture arguments.
    UnsupportedReference,
}
/// A capture refusal with operation, actual kind and cause; inspect `error.requested_kind()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CaptureBufferKindError {
    requested_kind: Graphic3dBufferType,
    cause: CaptureBufferKindCause,
}
impl CaptureBufferKindError {
    /// Refuses without sampling into the immutable entry; `CaptureBufferKindError::unsupported(kind.read())`.
    pub fn unsupported(requested_kind: Graphic3dBufferType) -> Self {
        Self {
            requested_kind,
            cause: CaptureBufferKindCause::UnsupportedReference,
        }
    }
    /// Identifies the refused operation; `assert_eq!(error.operation(), "buffer_dump_referenced");`.
    pub fn operation(&self) -> &'static str {
        // A single operation owns this error rather than a caller-supplied label.
        "buffer_dump_referenced"
    }
    /// Returns the kind observed at refusal; `let requested = error.requested_kind();`.
    pub fn requested_kind(&self) -> Graphic3dBufferType {
        // This is diagnostic evidence, not a frozen capture argument.
        self.requested_kind
    }
    /// Returns the typed failure cause; `assert_eq!(error.cause(), CaptureBufferKindCause::UnsupportedReference);`.
    pub fn cause(&self) -> CaptureBufferKindCause {
        // Backends must explicitly opt into the reference-aware operation.
        self.cause
    }
}
impl fmt::Display for CaptureBufferKindError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Refusal must retain both the actual request and the required implementation.
        write!(
            output,
            "{} requested_kind={:?} cause={:?}; expected a reference-capable capture backend",
            self.operation(),
            self.requested_kind,
            self.cause
        )
    }
}
impl std::error::Error for CaptureBufferKindError {}
