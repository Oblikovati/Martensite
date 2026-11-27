//! One redraw of a view: the whole frame, or for `redraw_immediate` an immediate-only frame
//! over the retained main pass when its elision holds (immediate_elision.rs).
use super::Graphic3dRenderView;
use crate::{
    FrameCapture, RenderError, RenderFrame, RenderedFrame, WorkPhase,
    immediate_main::MainObservation, redraw_capture::RedrawCapture, view_capture::HostCaptureSteps,
};
#[cfg(not(feature = "raytrace-input-diagnostics"))]
use crate::{immediate_elision::ImmediateViewInputs, immediate_main::eligible_start};
use martensite_tkservice::graphic3d::*;
#[cfg(feature = "raytrace-input-diagnostics")]
use std::sync::Arc;

/// A rendered frame, the scaled scene capture it blits, if any, and its device frame.
type RenderedScene = (RenderFrame, Option<FrameCapture>, RenderedFrame);

impl Graphic3dRenderView {
    pub(super) fn render_current(&self, immediate: bool) -> Result<RedrawCapture, RenderError> {
        self.release_unread_frame();
        #[cfg(feature = "raytrace-input-diagnostics")]
        let observation = self.begin_raytrace_observation()?;
        self.with_measured_frame(|work| {
            #[cfg(feature = "raytrace-input-diagnostics")]
            let (mut frame, scaled, rendered) =
                self.rendered_whole_frame(work, immediate, None, observation.as_ref())?;
            #[cfg(not(feature = "raytrace-input-diagnostics"))]
            let (mut frame, scaled, rendered) = self.rendered_scene(work, immediate)?;
            #[cfg(feature = "command-state-diagnostics")]
            let observed_transforms = [
                usize::from(scaled.is_some()),
                frame.subview_blits.len(),
                usize::from(frame.flip_output),
                usize::from(frame.gui_draw_start.is_some()),
            ];
            // The host steps wait for the first capture, as the readback does (#594).
            let capture = RedrawCapture::new(rendered, HostCaptureSteps::taken(&mut frame, scaled));
            #[cfg(feature = "raytrace-input-diagnostics")]
            self.finish_raytrace_observation(observation.as_ref(), capture.pixels(&self.device)?);
            #[cfg(feature = "command-state-diagnostics")]
            crate::scene::replay_diagnostic::rendered(
                self as *const Self as usize,
                capture.pixels(&self.device)?,
                observed_transforms,
            );
            Ok(capture)
        })
    }

    /// An immediate-only frame when the retained main pass holds; otherwise, or when the
    /// backend can no longer restore the main images, the whole frame.
    #[cfg(not(feature = "raytrace-input-diagnostics"))]
    fn rendered_scene(
        &self,
        work: &crate::frame_work::FrameWorkScope,
        immediate: bool,
    ) -> Result<RenderedScene, RenderError> {
        let observed = immediate.then(|| self.observe_main()).flatten();
        let elided = work.phase(WorkPhase::Scene, || self.elided_frame(observed.as_ref()))?;
        if let Some(frame) = elided {
            match self.render_target(&frame) {
                Err(error) if error.is_main_restore_unavailable() => self.immediate_main.discard(),
                rendered => {
                    let rendered = rendered?;
                    self.elided_redraws
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    return Ok((frame, None, rendered));
                }
            }
        }
        self.rendered_whole_frame(work, immediate, observed, None)
    }

    /// The whole frame of a redraw; an immediate redraw plans to keep its main pass.
    fn rendered_whole_frame(
        &self,
        work: &crate::frame_work::FrameWorkScope,
        immediate: bool,
        observed: Option<MainObservation>,
        #[cfg(feature = "raytrace-input-diagnostics")] observation: Option<
            &Arc<crate::raytrace_input_observation::RayInputObservation>,
        >,
        #[cfg(not(feature = "raytrace-input-diagnostics"))] _observation: Option<()>,
    ) -> Result<RenderedScene, RenderError> {
        let budget = (!immediate)
            .then(|| self.native_assembly_budget())
            .flatten();
        let (mut frame, scaled) = work.phase(WorkPhase::Scene, || {
            self.prepare_current_frame(!immediate && budget.is_none())
        })?;
        frame.assembly_upload_budget = budget.filter(|_| scaled.is_none());
        self.attach_outputs(&mut frame);
        if immediate
            && frame.immediate_start.is_some()
            && scaled.is_none()
            && self.keeps_main_pass()
        {
            frame.immediate_main = self.immediate_main.plan(&frame, observed);
        }
        #[cfg(feature = "raytrace-input-diagnostics")]
        self.attach_raytrace_observation(&mut frame, observation, scaled.is_some());
        let rendered = self.render_target(&frame)?;
        Ok((frame, scaled, rendered))
    }

    /// The flipped output and the subview copies, which redrawImmediate applies before the
    /// swap and the flipped output blit.
    fn attach_outputs(&self, frame: &mut RenderFrame) {
        frame.flip_output = self.retained.flips_output(&self.rendering_params());
        frame.subview_blits = self.subview_blits(frame.extent());
    }

    /// Whether this view can keep a main pass between immediate redraws.
    fn keeps_main_pass(&self) -> bool {
        self.assembly_streaming.read().budget().is_none()
            && !self.checked_gui_policy()
            && self.frame_hooks.read().is_none()
            && *self
                .immediate_main_supported
                .get_or_init(|| self.device.read().allows_main_copy())
    }

    /// The notice count, then the in-place view state, read before an immediate frame is
    /// built; none for a view that cannot keep its main pass.
    #[cfg(not(feature = "raytrace-input-diagnostics"))]
    fn observe_main(&self) -> Option<MainObservation> {
        if !self.keeps_main_pass() {
            return None;
        }
        let notices = self.retained.main_notices.count();
        let extent = self.redraw_extent().ok()?;
        Some((notices, ImmediateViewInputs::capture(self, extent)))
    }

    /// The immediate-only frame over the retained main pass, when its elision holds.
    #[cfg(not(feature = "raytrace-input-diagnostics"))]
    fn elided_frame(
        &self,
        observed: Option<&MainObservation>,
    ) -> Result<Option<RenderFrame>, RenderError> {
        let Some((notices, inputs)) = observed else {
            return Ok(None);
        };
        let Some((plan, elision)) = self.immediate_main.elided(*notices, inputs) else {
            return Ok(None);
        };
        let extent = self.redraw_extent()?;
        if self.gui_layers.read().snapshot(extent)?.is_some() {
            return Ok(None);
        }
        let mut frame = self.with_frame_extras(|clipping, extras| {
            crate::view_scene::immediate_frame_for_view(self, extent, clipping, extras, &elision)
        })?;
        self.attach_feedback(&mut frame);
        let mut frame = frame.with_line_width_policy(self.line_widths);
        self.attach_outputs(&mut frame);
        frame.immediate_main = Some(plan);
        if eligible_start(&frame) != Some(elision.start()) {
            return Ok(None);
        }
        #[cfg(feature = "draw-record-verification")]
        self.verify_elided_frame(elision.start())?;
        Ok(Some(frame))
    }

    /// The immediate redraws that kept their main pass without building it;
    /// `view.elided_redraws()`.
    #[cfg(test)]
    /// Count of immediate redraws that reused retained main images without rebuilding them.
    #[doc(hidden)]
    pub fn elided_redraws(&self) -> u64 {
        self.elided_redraws
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    #[cfg(all(
        feature = "draw-record-verification",
        not(feature = "raytrace-input-diagnostics")
    ))]
    fn verify_elided_frame(&self, start: usize) -> Result<(), RenderError> {
        let (whole, _) = self.prepare_current_frame(false)?;
        self.immediate_main.verify_elided(&whole, start);
        Ok(())
    }
}
