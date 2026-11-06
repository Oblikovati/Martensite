//! Adaptive screen sampling of path tracing (GlView::runPathtrace and runPathtraceOut with
//! AdaptiveScreenSampling): the per-view tile sampler, the tile maps and seeds of
//! each frame, and the visual-error image read back after the frame it was accumulated in.
use super::tile_sampler::{RaytraceTileOffset, RaytraceTileSampler};
use crate::RenderError;
use martensite_tkmath::math::BullardGenerator;
use martensite_tkservice::graphic3d::{
    Graphic3dCView, Graphic3dCViewMembers as _, Graphic3dRenderingParams,
};
use std::sync::{Arc, Mutex};

/// The adaptive-sampling settings of a path-traced frame.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PathtraceAdaptive {
    /// ADAPTIVE_SAMPLING_ATOMIC: one pass over the redirected offset tiles with float atomics.
    pub atomic: bool,
    /// `ShowSamplingTiles`: the output shows the sample count (uDebugAdaptive).
    pub debug: bool,
    /// `RayTracingTileSize` and `NbRayTracingTiles`.
    pub tile_size: i32,
    pub nb_tiles: i32,
    /// The maps and seeds of this frame; `None` until the view's accumulation fills them.
    pub passes: Option<AdaptivePasses>,
}

/// What the GPU half of one adaptive frame uploads and runs.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AdaptivePasses {
    /// The window: the output image holds 3x2 texels per pixel.
    pub window: [i32; 2],
    /// NbTilesX and NbTilesY: the visual-error and tile-samples images.
    pub tiles: [i32; 2],
    /// The tile-samples image (UploadSamples), one value per tile, row 0 at window y 0.
    pub tile_samples: Vec<i32>,
    /// The tile-offsets image (UploadOffsets) and its size in tiles.
    pub offsets: Vec<RaytraceTileOffset>,
    pub offset_tiles: [i32; 2],
    /// NbOffsetTilesMax: the tile-offsets image covers either map.
    pub offset_tiles_max: [i32; 2],
    /// The render passes' viewport: the window, or the offset tiles of the atomic variant.
    pub viewport: [i32; 2],
    /// `myRNG.NextInt() >> 2` of each pass (MaxTileSamples passes without atomics).
    pub seeds: Vec<i32>,
    /// `uTileSize` and `uVarianceScaleFactor`.
    pub tile_size: i32,
    pub variance_scale: f32,
    /// Receives the frame's visual-error image once the frame completes.
    pub feedback: VisualErrorFeedback,
}

/// The visual-error image of the last completed adaptive frame (GrabVarianceMap's read-back),
/// shared by the view's accumulation and the frame that fills it.
#[derive(Clone, Debug, Default)]
pub(crate) struct VisualErrorFeedback(Arc<Mutex<Option<Vec<i32>>>>);
impl PartialEq for VisualErrorFeedback {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl VisualErrorFeedback {
    /// Store the raw errors of a completed frame; `feedback.publish(errors)`.
    pub fn publish(&self, errors: Vec<i32>) {
        if let Ok(mut slot) = self.0.lock() {
            *slot = Some(errors);
        }
    }
    fn take(&self) -> Option<Vec<i32>> {
        self.0.lock().ok()?.take()
    }
}

/// The adaptive settings of `params` when adaptive sampling is on. The view's flags were
/// admitted against the device first (`admit_adaptive_sampling`), as initRaytraceResources
/// clears the unsupported ones. Example: `pathtrace_adaptive(&view.rendering_params())`.
pub(crate) fn pathtrace_adaptive(params: &Graphic3dRenderingParams) -> Option<PathtraceAdaptive> {
    let ray_tracing = &params.ray_tracing;
    ray_tracing
        .adaptive_screen_sampling
        .then_some(PathtraceAdaptive {
            atomic: ray_tracing.adaptive_screen_sampling_atomic,
            debug: ray_tracing.show_sampling_tiles,
            tile_size: ray_tracing.ray_tracing_tile_size,
            nb_tiles: ray_tracing.nb_ray_tracing_tiles,
            passes: None,
        })
}

/// Clear the adaptive-sampling flags the device cannot honour (initRaytraceResources):
/// adaptive sampling without `adaptive` support, its atomic variant without `atomic` support
/// (float image atomics, GL_NV_shader_atomic_float natively). Returns whether a flag was
/// cleared. Example: `admit_adaptive_sampling(&mut params, (true, false))`.
pub(crate) fn admit_adaptive_sampling(
    params: &mut Graphic3dRenderingParams,
    (adaptive, atomic): (bool, bool),
) -> bool {
    let ray_tracing = &mut params.ray_tracing;
    if !ray_tracing.adaptive_screen_sampling {
        return false;
    }
    if !adaptive {
        ray_tracing.adaptive_screen_sampling = false;
        return true;
    }
    let cleared = ray_tracing.adaptive_screen_sampling_atomic && !atomic;
    ray_tracing.adaptive_screen_sampling_atomic &= atomic;
    cleared
}

/// Clear the view's adaptive-sampling flags the device cannot honour, as native
/// initRaytraceResources clears myRenderParams' (the atomic variant without float image
/// atomics falls back to multi-pass sampling); without a device the flags stay.
/// Example: `admit_view_adaptive_sampling(view, renderer_limits)`.
pub(crate) fn admit_view_adaptive_sampling(
    view: &dyn Graphic3dCView,
    limits: Option<crate::RenderDeviceLimits>,
) {
    let Some(limits) = limits else {
        return;
    };
    let support = (
        limits.has_ray_tracing_adaptive_sampling(),
        limits.has_ray_tracing_adaptive_sampling_atomic(),
    );
    let mut params = view.rendering_params();
    if admit_adaptive_sampling(&mut params, support) {
        view.change_rendering_params(&mut |params| {
            admit_adaptive_sampling(params, support);
        });
    }
}
/// The tile sampler of one view (GlView::myTileSampler) and the feedback of its frames.
#[derive(Default)]
pub(crate) struct AdaptiveSampling {
    sampler: RaytraceTileSampler,
    feedback: VisualErrorFeedback,
}

impl AdaptiveSampling {
    /// Fill the passes of accumulated frame `frames` of a `window` view, drawing one seed per
    /// pass from `random`. Frame 0 resets the sampler and samples every tile once; later frames
    /// first rebuild the error distribution from the previous frame, whose uploaded maps are
    /// adaptive from frame 1 on (runPathtraceOut's `UploadSamples(.., myAccumFrames != 0)`).
    /// Example: `sampling.prepare(&mut adaptive, ([256, 256], 3), &mut random)?`.
    pub fn prepare(
        &mut self,
        adaptive: &mut PathtraceAdaptive,
        (window, frames): ([i32; 2], i32),
        random: &mut BullardGenerator,
    ) -> Result<(), RenderError> {
        let sampler = &mut self.sampler;
        sampler.set_size(adaptive.tile_size, adaptive.nb_tiles, window);
        let redistributed = frames > 1;
        if frames == 0 {
            sampler.reset();
        } else if let Some(errors) = self.feedback.take() {
            sampler.grab_variance_map(&errors)?;
        }
        let offsets = sampler
            .update_offsets(redistributed)
            .ok_or_else(|| crate::error::rejected(window[0], "a sized adaptive tile sampler"))?
            .to_vec();
        let passes = match adaptive.atomic {
            true => 1,
            false => sampler.max_tile_samples(),
        };
        let viewport = match adaptive.atomic {
            true => sampler.offset_tiles_viewport(redistributed),
            false => window,
        };
        adaptive.passes = Some(AdaptivePasses {
            window,
            tiles: [sampler.nb_tiles_x(), sampler.nb_tiles_y()],
            tile_samples: sampler.tile_samples().to_vec(),
            offsets,
            offset_tiles: sampler.nb_offset_tiles(redistributed),
            offset_tiles_max: sampler.nb_offset_tiles_max(),
            viewport,
            seeds: (0..passes)
                .map(|_| (random.next_int() >> 2) as i32)
                .collect(),
            tile_size: sampler.tile_size()[0],
            variance_scale: sampler.variance_scale_factor(),
            feedback: self.feedback.clone(),
        });
        Ok(())
    }
}

#[cfg(test)]
#[path = "adaptive_tests.rs"]
mod tests;
