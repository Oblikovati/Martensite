//! Keep existing catalog flags separate from renderer resource ownership.
use super::{PipelineVariant, RenderCappingPass, vk};
/// Preserve the existing ordinary catalog clipping bit; `ordinary_clipping(index)` never reads resource state.
pub(super) fn ordinary_clipping(index: usize) -> bool {
    if index < 20 {
        return index >= 10;
    }
    index & 2 != 0
}
/// Preserve the existing cap catalog mapping; `capping_variant(index)` has no new rendering policy.
pub(super) fn capping_variant(index: usize) -> RenderCappingPass {
    match index {
        0..20 => RenderCappingPass::None,
        20..24 => RenderCappingPass::MaskStart,
        _ => RenderCappingPass::Surface,
    }
}
impl PipelineVariant {
    /// The device's depth attachment for this variant; `variant.depth_format(runtime.depth.formats)`.
    pub fn depth_format(
        self,
        formats: super::super::depth_equivalence::DepthFormats,
    ) -> vk::Format {
        formats.of(self.stencil)
    }
}

impl PipelineVariant {
    /// Keep face assembly separate from polygon raster mode; `kind.polygon_raster_mode()`.
    pub(in crate::backend) fn polygon_raster_mode(self) -> vk::PolygonMode {
        if self.polygon_points {
            return vk::PolygonMode::POINT;
        }
        if self.polygon_edges {
            return vk::PolygonMode::LINE;
        }
        vk::PolygonMode::FILL
    }
}
