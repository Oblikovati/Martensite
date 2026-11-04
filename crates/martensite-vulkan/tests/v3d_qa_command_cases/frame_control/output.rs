//! Restored media precedes every positive trace; output failures remain incomplete.
use super::*;
#[cfg(test)]
mod controls;
use super::super::Outcome;
use std::ffi::OsStr;

trait ScaleFrameCaptureOutput {
    fn write_uncovered(&mut self, session: &ViewerSession) -> Outcome;
    fn write_restored(&mut self, session: &ViewerSession) -> Outcome;
    fn write_trace(&mut self, trace: &trace::FrameTrace) -> Outcome;
}
struct CaseScaleFrameOutput;
impl ScaleFrameCaptureOutput for CaseScaleFrameOutput {
    fn write_uncovered(&mut self, session: &ViewerSession) -> Outcome {
        crate::source_output::control_capture::control_dump(session, "bug12584.png", "uncovered")
    }
    fn write_restored(&mut self, session: &ViewerSession) -> Outcome {
        crate::source_output::control_capture::control_dump(session, "bug12584.png", "restored")
    }
    fn write_trace(&mut self, trace: &trace::FrameTrace) -> Outcome {
        let path = crate::source_output::capture_path("bug12584.png.frame-control.json")?;
        trace.write_exclusive(&path)?;
        Ok(())
    }
}
/// Keep optional output ordered; `capture_requested(session, scale, Some(flag))?`.
pub(super) fn capture_requested(
    session: &ViewerSession,
    scale: &Handle<AisColorScale>,
    flag: Option<&OsStr>,
) -> Outcome {
    capture_with_output(session, scale, flag, &mut CaseScaleFrameOutput)
}
fn capture_with_output(
    session: &ViewerSession,
    scale: &Handle<AisColorScale>,
    flag: Option<&OsStr>,
    output: &mut dyn ScaleFrameCaptureOutput,
) -> Outcome {
    if !requested(flag)? {
        return Ok(());
    }
    let trace = with_frame_control(session, scale, || output.write_uncovered(session))?;
    output.write_restored(session)?;
    output.write_trace(&trace)
}
