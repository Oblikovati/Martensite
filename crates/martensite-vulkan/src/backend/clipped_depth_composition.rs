//! V4 owns pre-raster clipping only for its selected family under the public fragment policy.
use crate::{
    PrimitiveDraw, RenderWindowDepthBehavior as Depth, RenderWorldClippingBehavior as Clip,
};
fn owns_clip_stage(depth: Depth, requested: Clip) -> bool {
    requested == Clip::FragmentV1 && super::clipped_depth_device::selected(depth)
}
pub(super) fn device_clipping(depth: Depth, requested: Clip) -> Clip {
    if owns_clip_stage(depth, requested) {
        return Clip::RasterClipDistanceV2;
    }
    requested
}
pub(super) fn draw_clipping(depth: Depth, requested: Clip, draw: &PrimitiveDraw) -> Clip {
    if matches!(
        requested,
        Clip::RasterFixedFunctionV4 | Clip::RasterFixedFunctionClipSpaceV5
    ) {
        // Every other draw keeps the discard that its programmable native route uses.
        let served = super::world_clip_device::fixed_function_triangles(draw)
            && (requested == Clip::RasterFixedFunctionV4
                || super::factorized_line_upload::clip_space_clipping(draw).is_some());
        return if served { requested } else { Clip::FragmentV1 };
    }
    if owns_clip_stage(depth, requested) && super::clipped_depth_upload::applies(draw) {
        return Clip::RasterClipDistanceV2;
    }
    requested
}
#[cfg(any(test, feature = "command-state-diagnostics"))]
pub(super) fn pipeline_modes(
    requested: Clip,
    kind: super::pipeline_variants::PipelineVariant,
) -> [Clip; 2] {
    let owned = kind.window_depth.is_some_and(|p| p.clipped_fan()) && kind.world_clip_v2;
    let effective = if requested == Clip::FragmentV1 && owned {
        Clip::RasterClipDistanceV2
    } else {
        requested
    };
    [requested, effective]
}
impl super::runtime::VulkanRuntime {
    pub(super) fn effective_world_clipping(
        &self,
        draw: &crate::PrimitiveDraw,
    ) -> crate::RenderWorldClippingBehavior {
        draw_clipping(self.window_depth_behavior, self.world_clipping, draw)
    }
    pub(super) fn world_clip_device_behavior(&self) -> crate::RenderWorldClippingBehavior {
        device_clipping(self.window_depth_behavior, self.world_clipping)
    }
}
#[cfg(test)]
#[path = "clipped_depth_composition_tests.rs"]
mod tests;
