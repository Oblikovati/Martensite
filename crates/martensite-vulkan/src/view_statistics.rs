//! Renderer-owned view state beside the shared retained view: frame statistics
//! (GlContext::FrameStats, GlView::StatisticInformation), their overlay and the
//! shader grid. The shared retained-view forwarding macro has no exclusion list, so the retained
//! state is wrapped: the wrapper's inherent statistics methods take precedence over the
//! forwarded empty defaults. FBO bindings also retain an owned observer revision so equal-size
//! swaps and ABA cannot certify an earlier target. Every other call dereferences to the output-mode wrapper
//! (flip output, draw-to-front) and through it to the shared view.
use crate::{Graphic3dRenderView, RenderFrameStats, RenderImageMemory};
use martensite_tkernel::standard::{Handle, SharedState};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::graphic3d::{
    Graphic3dCView, Graphic3dCViewMembers as _, Graphic3dFrameStats as _, Graphic3dFrameStatsDict,
};

/// The retained Graphic3d view state (with its output modes) plus the view's frame statistics.
pub(crate) struct RenderRetainedView {
    view: crate::view_output_modes::OutputModeRetainedView,
    stats: SharedState<Handle<RenderFrameStats>>,
    fbo_revision: SharedState<std::sync::Arc<()>>,
    overlay: crate::frame_stats_overlay::FrameStatsOverlay,
    shader_grid: SharedState<crate::shader_grid::RenderShaderGrid>,
    /// GlView graduated trihedron (graduated_trihedron/mod.rs).
    pub(crate) graduated_trihedron: crate::RenderGraduatedTrihedron,
    pub(crate) graduated_trihedron_shown: SharedState<bool>,
    /// GlView background image and image based lighting state (view_environment.rs).
    pub(crate) environment: SharedState<crate::view_environment::ViewEnvironmentState>,
    /// The light cameras and last rendered shadow maps (GlShadowMapArray, shadow_maps.rs).
    pub(crate) shadow_cameras: SharedState<crate::shadow_maps::ShadowCameras>,
}

impl RenderRetainedView {
    /// Wrap new retained state with fresh statistics; `RenderRetainedView::new(retained)`.
    pub fn new(view: crate::view_output_modes::OutputModeRetainedView) -> Self {
        Self {
            view,
            stats: SharedState::new(RenderFrameStats::new()),
            fbo_revision: SharedState::new(std::sync::Arc::new(())),
            overlay: Default::default(),
            shader_grid: Default::default(),
            graduated_trihedron: Default::default(),
            graduated_trihedron_shown: Default::default(),
            environment: Default::default(),
            shadow_cameras: Default::default(),
        }
    }
    /// Keep forwarded FBO bindings observable, including same-owner round trips.
    pub(crate) fn set_fbo(&self, fbo: &Handle<martensite_tkernel::standard::StandardTransient>) {
        //  held the new revision lease across final SDK-owner release.
        let retired;
        {
            let mut revision = self.fbo_revision.write();
            retired = self.view.fbo();
            self.view.set_fbo(fbo);
            *revision = std::sync::Arc::new(());
        }
        drop(retired);
    }
    #[cfg(test)]
    pub(crate) fn fbo_revision_unlocked(&self) -> bool {
        self.fbo_revision.try_write().is_some()
    }
    /// Retain the actual selected owner and its binding revision without callbacks.
    pub(crate) fn fbo_binding_stamp(
        &self,
    ) -> (
        Handle<martensite_tkernel::standard::StandardTransient>,
        std::sync::Arc<()>,
    ) {
        let revision = self.fbo_revision.read();
        (self.view.fbo(), revision.clone())
    }
    /// The displayed shader grid of the view.
    pub fn shader_grid(&self) -> &SharedState<crate::shader_grid::RenderShaderGrid> {
        &self.shader_grid
    }
    /// `StatisticInformation()`: the collected counters as text lines.
    pub fn statistic_information(&self) -> AsciiString {
        let flags = self.view.rendering_params().stats.collected_stats;
        let stats = self.stats.read().clone();
        stats
            .get()
            .map_or_else(AsciiString::default, |s| s.format_stats(flags))
    }
    /// `StatisticInformation(dict)`: the collected counters as key/value pairs.
    pub fn statistic_information_dict(&self, dict: &mut Graphic3dFrameStatsDict) {
        let flags = self.view.rendering_params().stats.collected_stats;
        if let Some(stats) = self.stats.read().clone().get() {
            stats.format_stats_dict(dict, flags);
        }
    }
}

impl std::ops::Deref for RenderRetainedView {
    type Target = crate::view_output_modes::OutputModeRetainedView;
    fn deref(&self) -> &crate::view_output_modes::OutputModeRetainedView {
        &self.view
    }
}

impl Graphic3dRenderView {
    /// `GlContext::FrameStats`: this view's statistics; `render_view.frame_stats`.
    pub fn frame_stats(&self) -> Handle<RenderFrameStats> {
        self.retained.stats.read().clone()
    }
    /// `GlContext::SetFrameStats`; `render_view.set_frame_stats(&RenderFrameStats::new)`.
    pub fn set_frame_stats(&self, stats: &Handle<RenderFrameStats>) {
        *self.retained.stats.write() = stats.clone();
    }
    /// The statistics overlay of this view, drawn when `ToShowStats` is set.
    pub(crate) fn frame_stats_overlay(&self) -> &crate::frame_stats_overlay::FrameStatsOverlay {
        &self.retained.overlay
    }
    /// Device image bytes behind the memory counters; `render_view.image_memory()`.
    pub fn image_memory(&self) -> RenderImageMemory {
        self.device.read().image_memory()
    }
    /// Bracket one rendered frame with FrameStart/FrameEnd, as GlView::Redraw.
    /// A view that cannot render (undefined or removed) starts no frame, like a failed Activate.
    pub(crate) fn with_frame_stats<T>(&self, render: impl FnOnce() -> T) -> T {
        let stats = self.frame_stats();
        let view = self.this_cview();
        let measured = stats
            .get()
            .filter(|_| self.is_defined() && !self.is_removed());
        if let Some(stats) = measured {
            stats.frame_start(&view, false);
        }
        let outcome = render();
        if let Some(stats) = measured {
            stats.frame_end(&view, false);
        }
        outcome
    }
    fn this_cview(&self) -> Handle<dyn Graphic3dCView> {
        self.this.read().upgrade().cast()
    }
}
