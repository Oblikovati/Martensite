//! Explicit behavior versions keep earlier rasterization selectable for replay.
/// Select the provoking convention without changing the legacy default.
/// Example: `RenderRasterPolicy::LastProvokingVertexV2` requires device admission.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderRasterPolicy {
    /// Preserve the existing Vulkan first-vertex behavior and its replay images.
    #[default]
    LegacyFirstV1,
    /// Require last-vertex rasterization on every pipeline of this runtime.
    LastProvokingVertexV2,
}
impl RenderRasterPolicy {
    /// Record the selected behavior version; `RenderRasterPolicy::LegacyFirstV1.behavior_version()`.
    pub fn behavior_version(self) -> u32 {
        match self {
            Self::LegacyFirstV1 => 1,
            Self::LastProvokingVertexV2 => 2,
        }
    }
    /// Identify the required capability; `policy.requires_last_vertex()` guards admission.
    pub(crate) fn requires_last_vertex(self) -> bool {
        let required = Self::LastProvokingVertexV2;
        self == required
    }
}
