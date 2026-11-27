//! Capability refusals retain the requested replay version and selected implementation.
use crate::{RenderDriverIdentity, RenderLineClippingBehavior};
use std::fmt;
/// Identify the exact missing line-clipping requirement; `failure.reason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderLineClippingRequirement {
    /// The device does not support shaderClipDistance.
    ShaderClipDistance,
    /// Four independent viewport clip distances are unavailable.
    FourClipDistances,
    /// A registered line or later geometry stage needs a verified clipping adapter.
    RegisteredLineProgram,
}
/// A refusal rather than an implicit change of replay behavior.
/// Example: `error.line_clipping_failure().unwrap().reason`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderLineClippingFailure {
    /// The immutable behavior requested by this renderer.
    pub behavior: RenderLineClippingBehavior,
    /// Owned selected-device provenance.
    pub device: RenderDriverIdentity,
    /// The unsupported feature or stage contract.
    pub reason: RenderLineClippingRequirement,
    /// The selected physical device's actual feature observation.
    pub shader_clip_distance: bool,
    /// The selected device's maximum clip-distance count.
    pub max_clip_distances: u32,
    /// The selected device's combined clip/cull-distance limit.
    pub max_combined_clip_and_cull_distances: u32,
}
impl fmt::Display for RenderLineClippingFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            output,
            "line clipping {:?} refused for {:?}: {:?}, shaderClipDistance={}, maxClipDistances={}, maxCombinedClipAndCullDistances={}; expected shaderClipDistance=true, both limits>=4 and an adapted line vertex stage",
            self.behavior,
            self.device,
            self.reason,
            self.shader_clip_distance,
            self.max_clip_distances,
            self.max_combined_clip_and_cull_distances
        )
    }
}
impl std::error::Error for RenderLineClippingFailure {}

const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<RenderLineClippingFailure>();
    send_sync::<RenderLineClippingRequirement>();
};
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clipping_refusals_retain_owned_typed_requirements() {
        let failure = RenderLineClippingFailure {
            behavior: RenderLineClippingBehavior::PlatformNativeV2,
            device: crate::RenderDriverIdentity {
                vendor_id: 0,
                device_id: 0,
                driver_id: 13,
                driver_version: 0,
                driver_name: "named missing-capability device".into(),
                driver_info: "CPU fixture".into(),
                device_name: "fixture".into(),
            },
            reason: RenderLineClippingRequirement::ShaderClipDistance,
            shader_clip_distance: false,
            max_clip_distances: 8,
            max_combined_clip_and_cull_distances: 8,
        };
        let error: crate::RenderError = failure.clone().into();
        assert_eq!(error.line_clipping_failure(), Some(&failure));
        assert!(error.to_string().contains("ShaderClipDistance"));
    }
}
