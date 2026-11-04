//! A driver that clips far lines itself draws the source segments, not the CPU band's rounding.
//!
//! `line_guard_band` interpolates object positions in binary32, so each clipped end moves by up
//! to half a unit in the last place of its object coordinate. That is a tenth of a pixel or more
//! after projecting a segment that spans millions of pixels. Mesa's GL path derives the visible
//! end from the source's clip coordinates (`draw_pipe_clip.c`, `do_clip_line`); the viewport clip
//! distances of `line_clip_shader` request that same stage on lavapipe, so only an unmodified
//! source reproduces the native slope.
use super::runtime::VulkanRuntime;
use crate::{PrimitiveDraw, RenderLineClippingBehavior as Behavior};

/// The submitted segments when the selected driver stage clips them; `restored_source(rt, draw)`.
pub(super) fn restored_source(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
) -> Option<PrimitiveDraw> {
    let admitted = driver_clips_source(
        runtime.line_clipping,
        runtime.line_clip.enabled(),
        runtime.uses_raster_world_clipping(draw),
    );
    let within_width = draw
        .line_style
        .is_some_and(|style| style.raster_width() <= runtime.limits.line_width_range[1]);
    source_geometry(draw, admitted && within_width)
}

/// World-clip modules replace the viewport-clipped line stage, so they keep the guard band.
fn driver_clips_source(behavior: Behavior, viewport_stage: bool, raster_world_clip: bool) -> bool {
    behavior == Behavior::PlatformNativeV3 && viewport_stage && !raster_world_clip
}

fn source_geometry(draw: &PrimitiveDraw, admitted: bool) -> Option<PrimitiveDraw> {
    let source = draw.guard_band_source.as_ref().filter(|_| admitted)?;
    Some(draw.with_replacement_geometry(source.vertices.clone(), source.indices.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LineStyle, RenderVertex, line_guard_band::GuardBandSource};
    use std::sync::Arc;
    fn guarded() -> (PrimitiveDraw, Vec<RenderVertex>) {
        let vertex = |y: f32| RenderVertex::new([0.25, y, 0.5], [1.; 4]);
        let style = LineStyle::new(1., 0xffff, 1).unwrap();
        let source = vec![vertex(-1.0e6), vertex(1.0e6)];
        let band = vec![vertex(-256.), vertex(256.)];
        let mut draw =
            PrimitiveDraw::segments(band, vec![0, 1], crate::identity_clip(), style).unwrap();
        draw.guard_band_source = Some(Arc::new(GuardBandSource {
            vertices: source.clone(),
            indices: vec![0, 1],
        }));
        (draw, source)
    }
    #[test]
    fn only_version_three_with_the_viewport_stage_and_no_world_stage_restores() {
        assert!(driver_clips_source(Behavior::PlatformNativeV3, true, false));
        assert!(!driver_clips_source(
            Behavior::PlatformNativeV3,
            false,
            false
        ));
        assert!(!driver_clips_source(Behavior::PlatformNativeV3, true, true));
        assert!(!driver_clips_source(
            Behavior::PlatformNativeV2,
            true,
            false
        ));
        assert!(!driver_clips_source(Behavior::VulkanDriverV1, true, false));
    }
    #[test]
    fn restoration_returns_the_source_bits_and_keeps_draw_state() {
        let (draw, source) = guarded();
        let restored = source_geometry(&draw, true).unwrap();
        assert_eq!(restored.vertices, source);
        assert_eq!(restored.indices, [0, 1]);
        assert_eq!(restored.line_style, draw.line_style);
        assert_eq!(restored.clip_transform, draw.clip_transform);
        assert_eq!(restored.guard_band_source, None);
    }
    #[test]
    fn refused_or_unclipped_draws_keep_their_prepared_geometry() {
        let (mut draw, _) = guarded();
        assert_eq!(source_geometry(&draw, false), None);
        draw.guard_band_source = None;
        assert_eq!(source_geometry(&draw, true), None);
    }
}
