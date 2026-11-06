//! A single admission seam keeps unsupported requests out of device creation.
use crate::{
    RenderDriverIdentity, RenderError, RenderRasterCapabilityFailure, RenderRasterPolicy,
    RenderRasterPolicyFailure,
};
/// Own one capability observation; `RasterPolicyCapabilities { extension_present: true, last_vertex_feature: true }`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RasterPolicyCapabilities {
    pub extension_present: bool,
    pub last_vertex_feature: bool,
}
/// Query the selected device at the admission seam; use `create_after_raster_admission`.
pub(crate) trait RasterPolicyCapabilitySource {
    /// Query required support; `source.capabilities()?` returns an owned observation.
    fn capabilities(&self) -> Result<RasterPolicyCapabilities, RenderError>;
    /// Own selected-device provenance on refusal; `source.identity()` keeps no native borrow.
    fn identity(&self) -> RenderDriverIdentity;
}
/// Admit before invoking device I/O; `create_after_raster_admission(policy, source, create)`.
pub(crate) fn create_after_raster_admission<T>(
    requested: RenderRasterPolicy,
    source: &impl RasterPolicyCapabilitySource,
    create: impl FnOnce(RenderRasterPolicy) -> Result<T, RenderError>,
) -> Result<T, RenderError> {
    let admitted = admit_raster_policy(requested, source)?;
    create(admitted)
}
fn admit_raster_policy(
    requested: RenderRasterPolicy,
    source: &impl RasterPolicyCapabilitySource,
) -> Result<RenderRasterPolicy, RenderError> {
    if !requested.requires_last_vertex() {
        return Ok(requested);
    }
    let capabilities = source.capabilities()?;
    if let Some(reason) = capabilities.missing_requirement() {
        return Err(RenderRasterPolicyFailure {
            requested_policy: requested,
            device: source.identity(),
            reason,
        }
        .into());
    }
    Ok(requested)
}
impl RasterPolicyCapabilities {
    fn missing_requirement(self) -> Option<RenderRasterCapabilityFailure> {
        if !self.extension_present {
            return Some(RenderRasterCapabilityFailure::MissingExtension);
        }
        if !self.last_vertex_feature {
            return Some(RenderRasterCapabilityFailure::MissingLastVertexFeature);
        }
        None
    }
}
#[cfg(test)]
#[path = "raster_policy_admission_tests.rs"]
mod tests;
