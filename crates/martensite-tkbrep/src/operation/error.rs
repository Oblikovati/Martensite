// SPDX-License-Identifier: GPL-2.0-only
//! Typed operation error contract (#6).
use crate::topods::Shape;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

/// Identifies the modelling operation that produced an error or result (#6).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OperationKind {
    /// Linear extrusion (`BRepPrimAPI_MakePrism`).
    Prism,
    /// Boolean operation (Fuse, Cut, Common, Section, General Fuse).
    Boolean,
    /// 3D or 2D fillet blend.
    Fillet,
    /// Chamfer blend.
    Chamfer,
    /// Offset surface or solid.
    Offset,
    /// Thick solid / shell operation.
    Shell,
    /// Multi-section / guided loft.
    Loft,
    /// Deep topological/geometric copy.
    Copy,
    /// Named extension operation.
    Custom(&'static str),
}

impl fmt::Display for OperationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prism => f.write_str("Prism"),
            Self::Boolean => f.write_str("Boolean"),
            Self::Fillet => f.write_str("Fillet"),
            Self::Chamfer => f.write_str("Chamfer"),
            Self::Offset => f.write_str("Offset"),
            Self::Shell => f.write_str("Shell"),
            Self::Loft => f.write_str("Loft"),
            Self::Copy => f.write_str("Copy"),
            Self::Custom(name) => f.write_str(name),
        }
    }
}

/// High-level failure classification so callers never parse strings or flags (#6).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FailureClass {
    /// Caller supplied invalid, null, or degenerate input.
    BadInput,
    /// Operation refused an unsupported configuration at classification.
    NamedDecline,
    /// Internal kernel failure during construction or stitching.
    KernelFailure,
    /// Operation completed partially or could not certify completeness/validity.
    IncompleteCheck,
    /// Caller or progress indicator cancelled the operation.
    Cancelled,
}

impl fmt::Display for FailureClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadInput => f.write_str("BadInput"),
            Self::NamedDecline => f.write_str("NamedDecline"),
            Self::KernelFailure => f.write_str("KernelFailure"),
            Self::IncompleteCheck => f.write_str("IncompleteCheck"),
            Self::Cancelled => f.write_str("Cancelled"),
        }
    }
}

/// Typed, `Send + Sync` operation failure carrying operation, failure class,
/// affected entities, and an inspectable underlying cause (#6).
///
/// ```
/// use martensite_tkbrep::operation::{FailureClass, OperationError, OperationKind};
/// let err = OperationError::bad_input(OperationKind::Prism, "zero extrusion vector");
/// assert_eq!(err.failure_class(), FailureClass::BadInput);
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct OperationError {
    operation: OperationKind,
    failure_class: FailureClass,
    affected_entities: Vec<Shape>,
    message: String,
    cause: Option<Arc<dyn Error + Send + Sync>>,
}

impl OperationError {
    /// Creates an error with the given operation, class, and diagnostic message.
    pub fn new(
        operation: OperationKind,
        failure_class: FailureClass,
        message: impl Into<String>,
    ) -> Self {
        Self {
            operation,
            failure_class,
            affected_entities: Vec::new(),
            message: message.into(),
            cause: None,
        }
    }

    /// Convenience constructor for [`FailureClass::BadInput`].
    pub fn bad_input(operation: OperationKind, message: impl Into<String>) -> Self {
        Self::new(operation, FailureClass::BadInput, message)
    }

    /// Convenience constructor for [`FailureClass::NamedDecline`].
    pub fn named_decline(operation: OperationKind, message: impl Into<String>) -> Self {
        Self::new(operation, FailureClass::NamedDecline, message)
    }

    /// Convenience constructor for [`FailureClass::KernelFailure`].
    pub fn kernel_failure(operation: OperationKind, message: impl Into<String>) -> Self {
        Self::new(operation, FailureClass::KernelFailure, message)
    }

    /// Convenience constructor for [`FailureClass::IncompleteCheck`].
    pub fn incomplete_check(operation: OperationKind, message: impl Into<String>) -> Self {
        Self::new(operation, FailureClass::IncompleteCheck, message)
    }

    /// Convenience constructor for [`FailureClass::Cancelled`].
    pub fn cancelled(operation: OperationKind, message: impl Into<String>) -> Self {
        Self::new(operation, FailureClass::Cancelled, message)
    }

    /// Attaches an affected topology entity to this error.
    pub fn with_entity(mut self, entity: Shape) -> Self {
        if !entity.is_null() {
            self.affected_entities.push(entity);
        }
        self
    }

    /// Attaches multiple affected topology entities to this error.
    pub fn with_entities(mut self, entities: impl IntoIterator<Item = Shape>) -> Self {
        for entity in entities {
            if !entity.is_null() {
                self.affected_entities.push(entity);
            }
        }
        self
    }

    /// Preserves an underlying `Send + Sync` error as `Error::source()`.
    pub fn with_cause<E: Error + Send + Sync + 'static>(mut self, cause: E) -> Self {
        self.cause = Some(Arc::new(cause));
        self
    }

    /// Operation that failed.
    pub fn operation(&self) -> OperationKind {
        self.operation
    }

    /// Structured failure class.
    pub fn failure_class(&self) -> FailureClass {
        self.failure_class
    }

    /// Entities associated with the failure, in deterministic discovery order.
    pub fn affected_entities(&self) -> &[Shape] {
        &self.affected_entities
    }

    /// Human-readable diagnostic summary.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Underlying cause, when present.
    pub fn cause(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
        self.cause.as_deref()
    }
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} failed ({}): {}",
            self.operation, self.failure_class, self.message
        )
    }
}

impl Error for OperationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause
            .as_deref()
            .map(|err| err as &(dyn Error + 'static))
    }
}
