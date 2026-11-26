//! Narrow rendering command setters keep unrelated ray-tracing defaults intact.
use super::aspects::synchronize;
use super::*;
use martensite_tkservice::graphic3d::{Graphic3dRenderingMode, Graphic3dTypeOfShadingModel};

#[path = "../../viewer_commands/opengl_commands/glinfo_format.rs"]
mod glinfo_format;

impl ViewerSession {
    /// Read the actual driver's basic diagnostic dictionary; `session.graphics_information()`.
    pub fn graphics_information(&self) -> String {
        use martensite_tkservice::graphic3d::{Graphic3dDiagnosticInfo, Graphic3dFrameStatsDict};
        let mut dictionary = Graphic3dFrameStatsDict::default();
        self.view()
            .diagnostic_information(&mut dictionary, Graphic3dDiagnosticInfo::BASIC);
        glinfo_format::format_information(&dictionary, 80)
    }
    /// Set just the requested sample count; `session.set_sample_count(8)?`.
    pub fn set_sample_count(&self, samples: i32) -> Result<(), Failure> {
        self.view()
            .change_rendering_params(&mut |parameters| parameters.nb_msaa_samples = samples);
        self.view().redraw();
        Ok(())
    }
    /// Select ray tracing without changing its reflection/AA options; `session.select_ray_tracing()?`.
    pub fn select_ray_tracing(&self) -> Result<(), Failure> {
        self.view().change_rendering_params(&mut |parameters| {
            parameters.method = Graphic3dRenderingMode::Raytracing
        });
        self.view().redraw();
        Ok(())
    }
    /// Own the object's shading model, preserving linked defaults; `session.set_object_shading("r", Unlit)?`.
    pub fn set_object_shading(
        &self,
        name: &str,
        model: Graphic3dTypeOfShadingModel,
    ) -> Result<(), Failure> {
        let object = self.named(name)?;
        let drawer = object.attributes();
        self.with_update(|context| {
            if drawer.set_shading_model(model, true)? {
                return context.redisplay(&object, false, false);
            }
            synchronize(&object);
            Ok(())
        })
    }
}

#[cfg(test)]
support_unit_tests! {
    #[path = "material_rendering_controls.rs"]
    mod controls;
}
