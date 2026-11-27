//! Same-view accepted-publication join; the requested observer never redraws to create evidence.
use super::*;
use crate::raytrace_input_observation::{self as observation, RayInputObservation};
use std::sync::atomic::Ordering;
impl Graphic3dRenderView {
    pub(super) fn begin_raytrace_observation(
        &self,
    ) -> Result<Option<Arc<RayInputObservation>>, RenderError> {
        if !observation::requested()? {
            return Ok(None);
        }
        self.raytrace_observation
            .write()
            .begin(self as *const Self as usize, self.identification() as i64)
            .map(Some)
    }
    pub(super) fn attach_raytrace_observation(
        &self,
        frame: &mut crate::RenderFrame,
        token: Option<&Arc<RayInputObservation>>,
        scaled: bool,
    ) {
        let Some(token) = token else { return };
        if !supported_frame(frame, scaled) {
            token.refuse("expected one direct mono non-FSAA non-pathtraced scene without subview composition");
        }
        frame.raytrace_observation = Some(token.clone());
    }
    pub(super) fn finish_raytrace_observation(
        &self,
        token: Option<&Arc<RayInputObservation>>,
        capture: &FrameCapture,
    ) {
        if let Some(token) = token {
            self.raytrace_observation.write().finished(token, capture);
        }
    }
    pub(super) fn publish_raytrace_observation(&self, outcome: &Result<FrameCapture, RenderError>) {
        self.raytrace_observation
            .write()
            .publish(outcome, self.redraw_serial.load(Ordering::Relaxed));
    }
    pub(super) fn observe_raytrace_buffer_dump(
        &self,
        before: u64,
        kind: &Graphic3dBufferType,
        capture: &FrameCapture,
        copied: bool,
    ) {
        let color = matches!(kind, Graphic3dBufferType::Rgb | Graphic3dBufferType::Rgba);
        let current = self.redraw_serial.load(Ordering::Relaxed);
        self.raytrace_observation.write().dumped(
            before,
            capture,
            copied && color && before == current,
        );
    }
    /// Read accepted input evidence without redraw; `view.capture_with_raytrace_input_witness()?`.
    /// Missing, concurrent, unsupported or changed-frame observations refuse instead of inferring identity.
    pub fn capture_with_raytrace_input_witness(
        &self,
    ) -> Result<(FrameCapture, Option<String>), RenderError> {
        let before = self.redraw_serial.load(Ordering::Relaxed);
        let capture = self.capture()?;
        if !observation::requested()? {
            return Ok((capture, None));
        }
        let state = self.raytrace_observation.read();
        let json = state.witness(self as *const Self as usize, before, &capture)?;
        if before != self.redraw_serial.load(Ordering::Relaxed) {
            return Err(rejected(
                before,
                "unchanged accepted publication during diagnostic dump",
            ));
        }
        Ok((capture, Some(json)))
    }
}

fn supported_frame(frame: &crate::RenderFrame, scaled: bool) -> bool {
    let ray = frame
        .raytrace
        .as_ref()
        .map(|ray| (ray.antialiasing, ray.path_tracing.is_some()));
    observation::supported_scope(
        ray,
        scaled,
        frame.stereo_pair.is_some(),
        !frame.subview_blits.is_empty(),
    )
}
