//! Capture ordinary controls through existing frame hooks and owned witness encoding.
use super::*;
use martensite_vulkan::Graphic3dRenderView;
use std::io::Write;
pub(super) struct SphereImageSink;
impl SphereControlSink for SphereImageSink {
    fn capture(&mut self, session: &ViewerSession, image: &str, suffix: &str) -> Outcome {
        let active = session
            .view()
            .get()
            .ok_or_else(|| refused("active view null"))?;
        let view = active.view().cast::<Graphic3dRenderView>();
        let actual = view.get().ok_or_else(|| refused("render view null"))?;
        capture_numeric(session, actual, image, suffix)
    }
    fn identity(&mut self, image: &str, state: &SphereIdentity) -> Outcome {
        let path = crate::source_output::capture_path(&format!("{image}.sphere-unlit-state.txt"))?;
        let mut output = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|error| {
                refused(format!(
                    "metadata path={path:?}: {error}; expected new writable file"
                ))
            })?;
        writeln!(output, "restored=true\n{state:#?}")?;
        Ok(())
    }
}
fn capture_numeric(
    session: &ViewerSession,
    view: &Graphic3dRenderView,
    image: &str,
    suffix: &str,
) -> Outcome {
    let record = crate::tie_edges::SubmittedDraws::install(view);
    let reset = SphereHookReset(view);
    let result = crate::source_output::control_capture::control_dump(session, image, suffix);
    drop(reset);
    result?;
    crate::session_run::capture_witness::observe(view, &record, &format!("{image}.{suffix}.png"))?;
    Ok(())
}

struct SphereHookReset<'a>(&'a Graphic3dRenderView);
impl Drop for SphereHookReset<'_> {
    fn drop(&mut self) {
        self.0.set_frame_hooks(None);
    }
}
