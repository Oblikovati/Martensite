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
mod controls {
    use super::*;
    use martensite_tkservice::graphic3d::Graphic3dAspectsMembers;
    use martensite_tkservice::null_driver::NullGraphicDriver;
    fn null_session() -> ViewerSession {
        ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap()
    }
    #[test]
    fn actual_null_driver_query_keeps_actual_dictionary_without_a_fake_device() {
        let session = null_session();
        let response = session.graphics_information();
        assert_eq!(response, "OpenGL info:\n");
    }
    #[test]
    fn source_raytrace_and_msaa_commands_preserve_unrequested_ray_options() {
        let session = null_session();
        let before = session.view().rendering_params();
        session.set_sample_count(8).unwrap();
        assert_eq!(session.view().rendering_params().nb_msaa_samples, 8);
        session.select_ray_tracing().unwrap();
        session.set_sample_count(0).unwrap();
        let after = session.view().rendering_params();
        assert_eq!(after.nb_msaa_samples, 0);
        assert_eq!(after.method, Graphic3dRenderingMode::Raytracing);
        assert_eq!(
            after.ray_tracing.is_reflection_enabled,
            before.ray_tracing.is_reflection_enabled
        );
        assert_eq!(
            after.ray_tracing.is_antialiasing_enabled,
            before.ray_tracing.is_antialiasing_enabled
        );
    }
    fn two_objects() -> ViewerSession {
        let mut session = null_session();
        let mut builder =
            martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::from_sizes(1., 1., 1.).unwrap();
        let cube = builder.shape().unwrap();
        session
            .display("color", cube, &DisplayRequest::default())
            .unwrap();
        session
            .display("other", cube, &DisplayRequest::default())
            .unwrap();
        session
    }
    fn shading(session: &ViewerSession, name: &str) -> Graphic3dTypeOfShadingModel {
        session
            .object(name)
            .unwrap()
            .attributes()
            .shading_aspect()
            .aspect()
            .shading_model()
    }
    #[test]
    fn own_unlit_shading_keeps_context_defaults_and_other_objects() {
        let session = two_objects();
        let defaults = session.context().default_drawer().shading_aspect().aspect();
        let before = defaults.shading_model();
        let other = shading(&session, "other");
        session
            .set_object_shading("color", Graphic3dTypeOfShadingModel::Unlit)
            .unwrap();
        assert_eq!(defaults.shading_model(), before);
        assert_eq!(
            shading(&session, "color"),
            Graphic3dTypeOfShadingModel::Unlit
        );
        assert_eq!(shading(&session, "other"), other);
        assert!(
            session
                .set_object_shading("absent", Graphic3dTypeOfShadingModel::Unlit)
                .is_err()
        );
    }
}
