//! Explicit buffers and layer/shadow options share the original capture/witness lifecycle.
use super::*;
use martensite_tkv3d::v3d::V3dImageDumpOptions;

impl SessionRun {
    /// Capture exactly the requested public options; `run.dump_options("depth.png", &options)?`.
    pub(crate) fn dump_options(
        &mut self,
        basename: &'static str,
        options: &V3dImageDumpOptions,
    ) -> Result<(), Box<dyn Error>> {
        self.dump_using(basename, Some(options), |run| {
            run.capture_options(basename, options)
        })
    }

    fn capture_options(
        &mut self,
        basename: &'static str,
        options: &V3dImageDumpOptions,
    ) -> Result<(), Box<dyn Error>> {
        let stage = self
            .stage
            .take()
            .expect("a run is never left without a stage");
        let captured = capture_stage(stage, basename, options)?;
        self.stage = Some(Stage::Captured(captured));
        Ok(())
    }
}

fn capture_stage(
    stage: Stage,
    basename: &'static str,
    options: &V3dImageDumpOptions,
) -> Result<SourceCompletion<ViewerSession>, Box<dyn Error>> {
    match stage {
        Stage::Open(driver, scene) => {
            source_output::options_capture::capture_case_options(driver, scene, basename, options)
        }
        Stage::Captured(mut captured) => {
            captured.capture_additional_options(basename, options)?;
            Ok(captured)
        }
    }
}
