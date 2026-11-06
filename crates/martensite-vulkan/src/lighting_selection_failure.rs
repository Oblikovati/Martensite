//! A lighting selection change invalidates an earlier capture or an in-flight frame.
use crate::{RenderError, RenderLightingPipeline};
use std::fmt;
/// An obsolete frame's lighting selection; `error.lighting_selection_changed()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderLightingSelectionChanged {
    /// Lighting used when the frame started.
    pub started: RenderLightingPipeline,
    /// Lighting selected when publication or capture was checked.
    pub observed: RenderLightingPipeline,
}
impl fmt::Display for RenderLightingSelectionChanged {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            output,
            "lighting selection changed from {:?} to {:?}; expected one unchanged selection revision",
            self.started, self.observed
        )
    }
}
impl std::error::Error for RenderLightingSelectionChanged {}
impl RenderError {
    /// Inspect an obsolete lighting frame; `error.lighting_selection_changed()`.
    pub fn lighting_selection_changed(&self) -> Option<&RenderLightingSelectionChanged> {
        match &self.1 {
            Some(super::RenderFailureCause::LightingSelection(change)) => Some(change),
            _ => self
                .primary_failure()
                .and_then(Self::lighting_selection_changed)
                .or_else(|| {
                    self.observed_failure()
                        .and_then(Self::lighting_selection_changed)
                }),
        }
    }
    pub(crate) fn lighting_changed(
        started: RenderLightingPipeline,
        observed: RenderLightingPipeline,
    ) -> Self {
        let change = RenderLightingSelectionChanged { started, observed };
        Self(
            change.to_string(),
            Some(super::RenderFailureCause::LightingSelection(change)),
        )
    }
}
