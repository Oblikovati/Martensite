//! Validation coverage failures retain diagnostics without turning unavailable evidence into clean output.
use std::{error::Error, fmt};
/// Actual reason a validation observation is incomplete; match `failure.cause()` before reading retained text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RenderValidationObservationCause {
    /// The renderer was created without a validation messenger.
    Disabled,
    /// The actual validation queue mutex was poisoned; recovered text is incomplete evidence.
    Poisoned,
    /// An injected rendering backend has no Vulkan validation observer.
    Unavailable,
}
/// Owned incomplete observation; `renderer.validation_errors()?` never treats this failure as an empty list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderValidationObservationFailure {
    operation: &'static str,
    cause: RenderValidationObservationCause,
    enabled: bool,
    retained_messages: Vec<String>,
}
impl RenderValidationObservationFailure {
    /// Identify the failed read; `failure.operation()` remains independent of Vulkan execution errors.
    pub fn operation(&self) -> &'static str {
        // Observation failure never invents device loss or resource-retirement state.
        self.operation
    }
    /// Retain the typed cause; `matches!(failure.cause(), RenderValidationObservationCause::Poisoned)`.
    pub fn cause(&self) -> RenderValidationObservationCause {
        // Callers must not infer this category from diagnostic strings.
        self.cause
    }
    /// Distinguish configured monitoring; `failure.was_enabled()` does not erase queue poison.
    pub fn was_enabled(&self) -> bool {
        // Recovered poisoned text is incomplete even when monitoring was requested.
        self.enabled
    }
    /// Inspect preserved messages; `failure.retained_messages()` does not certify their completeness.
    pub fn retained_messages(&self) -> &[String] {
        // Incomplete observations keep the original order and text instead of discarding evidence.
        &self.retained_messages
    }
    /// Build the actual unavailable-backend result; `Self::unavailable()` is not a validation event.
    pub(crate) fn unavailable() -> Self {
        Self::new(
            RenderValidationObservationCause::Unavailable,
            false,
            Vec::new(),
        )
    }
    /// Bind the observed queue state; `Self::new(cause, enabled, messages)` preserves recovered evidence.
    pub(crate) fn new(
        cause: RenderValidationObservationCause,
        enabled: bool,
        retained_messages: Vec<String>,
    ) -> Self {
        Self {
            operation: "read validation observations",
            cause,
            enabled,
            retained_messages,
        }
    }
}
impl fmt::Display for RenderValidationObservationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: actual={:?}, enabled={}, retained_messages={}; expected enabled, available, unpoisoned validation observation",
            self.operation,
            self.cause,
            self.enabled,
            self.retained_messages.len()
        )
    }
}
impl Error for RenderValidationObservationFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // The known typed cause remains available without storing a non-Send mutex guard.
        Some(&self.cause)
    }
}
impl fmt::Display for RenderValidationObservationCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Category names are diagnostic provenance, never Vulkan numeric status codes.
        write!(formatter, "{self:?}")
    }
}
impl Error for RenderValidationObservationCause {}
#[cfg(test)]
#[path = "validation_observation_tests.rs"]
mod tests;
