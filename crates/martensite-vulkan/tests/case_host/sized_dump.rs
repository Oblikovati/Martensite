//! Dump-specific extents preserve the same session, original publication and END contracts.
use super::*;
use martensite_tkservice::graphic3d::Graphic3dTypeOfLimit;

impl SessionRun {
    /// Read the actual active driver's advertised texture limit; `run.maximum_texture_size()`.
    pub(crate) fn maximum_texture_size(&mut self) -> i32 {
        self.session()
            .view()
            .viewer()
            .driver()
            .inquire_limit(Graphic3dTypeOfLimit::MaxTextureSize)
    }

    /// Dump at explicit dimensions without changing the window; `run.dump_sized("wide.png", (4096, 100))?`.
    pub(crate) fn dump_sized(
        &mut self,
        basename: &'static str,
        size: (i32, i32),
    ) -> Result<(), Box<dyn Error>> {
        let view = render_view(self.session())?;
        let view = view
            .get()
            .ok_or("a Vulkan render view behind the active view")?;
        let record = SubmittedDraws::install(view);
        let captured = self.capture_sized(basename, size);
        view.set_frame_hooks(None);
        captured?;
        self.observe_dump(view, &record, basename);
        write_ties_sidecar(&record, basename, size)
    }

    fn capture_sized(
        &mut self,
        basename: &'static str,
        size: (i32, i32),
    ) -> Result<(), Box<dyn Error>> {
        let stage = self
            .stage
            .take()
            .expect("a run is never left without a stage");
        let captured = sized_stage(stage, basename, size)?;
        self.stage = Some(Stage::Captured(captured));
        Ok(())
    }
}

fn sized_stage(
    stage: Stage,
    basename: &'static str,
    size: (i32, i32),
) -> Result<SourceCompletion<ViewerSession>, Box<dyn Error>> {
    match stage {
        Stage::Open(driver, session) => {
            source_output::sized_capture::capture_case_sized(driver, session, basename, size)
        }
        Stage::Captured(mut completion) => {
            completion.capture_additional_sized(basename, size)?;
            Ok(completion)
        }
    }
}
