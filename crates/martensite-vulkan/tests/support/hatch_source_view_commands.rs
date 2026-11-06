//! Direct view commands keep their native updater ownership, rather than a generic redraw policy.
use super::*;
use martensite_tkernel::standard::catch_native_failure;

impl HatchSourceView {
    /// Keep the fresh-process Z-up axonometric view without another update; `stage.source_axo()?`.
    pub fn source_axo(&self) -> Result<(), Failure> {
        catch_native_failure(|| {
            self.view
                .set_proj_orientation(V3dTypeOfOrientation::XposYnegZpos, false)
        })
    }

    /// Edit actual rendering parameters under the command updater; `stage.source_resolution(144)?`.
    pub fn source_resolution(&self, resolution: u32) -> Result<(), Failure> {
        catch_native_failure(|| {
            self.source_update(|view| {
                view.change_rendering_params(&mut |params| params.resolution = resolution);
                Ok(())
            })
        })
    }
}
