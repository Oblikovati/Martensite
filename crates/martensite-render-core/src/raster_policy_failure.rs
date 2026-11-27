//! Capability refusals own the selected device identity and requested behavior.
use crate::{RenderDriverIdentity, RenderRasterPolicy};
use std::fmt;
/// Distinguish the exact unavailable requirement; `failure.reason` needs no text parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderRasterCapabilityFailure {
    /// The selected device does not advertise VK_EXT_provoking_vertex.
    MissingExtension,
    /// The extension exists but provokingVertexLast is false.
    MissingLastVertexFeature,
}
/// A policy-admission refusal raised before device creation.
/// Example: `error.raster_policy_failure().unwrap().requested_policy`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderRasterPolicyFailure {
    /// The requested behavior that could not be admitted.
    pub requested_policy: RenderRasterPolicy,
    /// Owned provenance of the physical device selected for this request.
    pub device: RenderDriverIdentity,
    /// The missing extension or feature; no fallback policy is selected.
    pub reason: RenderRasterCapabilityFailure,
}
impl fmt::Display for RenderRasterPolicyFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            output,
            "raster policy {:?} refused for device {:?}: {:?}; expected VK_EXT_provoking_vertex and provokingVertexLast=true before device creation",
            self.requested_policy, self.device, self.reason
        )
    }
}
impl std::error::Error for RenderRasterPolicyFailure {}
