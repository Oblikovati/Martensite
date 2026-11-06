//! Retained text uses the current aspect and view resolution on every extraction.
use crate::{Graphic3dDrawGroup, PrimitiveDraw, RenderError, TextLayoutSource, error::rejected};
use martensite_tkernel::{
    ncollection::{Mat4, Vec3},
    standard::Handle,
};
use martensite_tkservice::{aspect::*, graphic3d::*};
#[derive(Clone, Copy)]
pub(crate) struct TextSceneContext<'a> {
    pub group: &'a Graphic3dDrawGroup,
    pub camera: &'a Graphic3dCamera,
    pub extent: Option<[u32; 2]>,
    pub structure: &'a Graphic3dCStructureFields,
    pub rendering: &'a Graphic3dRenderingParams,
    pub fonts: &'a dyn TextLayoutSource,
    pub model_override: Option<Mat4<f32>>,
    /// Group persistence uses the actual incoming model for its local anchor.
    pub view_override: Option<Mat4<f32>>,
    pub clipping: Option<&'a crate::RenderClipping>,
    /// The view's environment texture while the layer uses it (`GlWorkspace`).
    pub environment: Option<&'a Handle<Graphic3dTextureEnv>>,
    pub fixed_function_lighting: bool,
}
impl TextSceneContext<'_> {
    /// Choose the binding path before lowering UVs; `context.texture_coordinates(aspect)`.
    pub fn texture_coordinates(
        &self,
        aspects: &dyn Graphic3dAspectsSource,
    ) -> crate::RenderTextureCoordinatePipeline {
        self.group.texture_images.coordinate_pipeline.effective(
            self.fixed_function_lighting,
            !aspects.shader_program().is_null(),
        )
    }

    /// The viewport of transform persistence; see `crate::render_scale::persistence_extent`.
    pub fn persistence_extent(&self) -> Option<[u32; 2]> {
        self.extent
            .map(|raster| crate::render_scale::persistence_extent(raster, self.rendering))
    }
    pub fn draws(
        &self,
        text: &Graphic3dText,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        upload: Option<&crate::element_upload::ElementUploadSlot>,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        let Some((aspect, layout)) = self.recorded_layout(text, aspects, upload)? else {
            return Ok(vec![]);
        };
        crate::text_drawing::lower_text_draws(text, aspect, &layout, &self.transform()?)
    }
    /// Draws of a 2D text element (`GlText::Set2D`): the text model replaces the
    /// world-view under the group's projection and no depth test applies.
    pub fn pixel_draws(
        &self,
        text: &Graphic3dText,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        upload: &crate::element_upload::ElementUploadSlot,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        let Some((aspect, layout)) = self.recorded_layout(text, aspects, Some(upload))? else {
            return Ok(vec![]);
        };
        let mut transform = self.transform()?;
        transform.pixel_coordinates = true;
        transform.camera_rotation = Mat4::identity();
        transform.zoomable_scale = 1.;
        let mut draws = crate::text_drawing::lower_text_draws(text, aspect, &layout, &transform)?;
        for draw in &mut draws {
            draw.annotation = true;
            draw.text_shadow_bias = false;
        }
        Ok(draws)
    }

    /// The glyph layout of a text with its validated aspect; records the glyph buffers.
    fn recorded_layout<'s>(
        &self,
        text: &Graphic3dText,
        aspects: &'s Handle<dyn Graphic3dAspectsSource>,
        upload: Option<&crate::element_upload::ElementUploadSlot>,
    ) -> Result<Option<(&'s dyn Graphic3dAspectsSource, crate::GlyphLayout)>, RenderError> {
        let aspect = aspects
            .get()
            .ok_or_else(|| rejected("null text aspect", "a retained text style"))?;
        validate_text_style(aspect)?;
        let Some(layout) = self.fonts.layout(text, aspect, self.rendering)? else {
            return Ok(None);
        };
        if let Some(upload) = upload {
            // GlTextBuilder emits six vertices per visible glyph, like the layout indices.
            let subtitle = aspect.text_display_type() == AspectTypeOfDisplayText::Subtitle;
            upload.record_text(layout.indices.len(), subtitle, layout.formatter_bounds);
        }
        Ok(Some((aspect, layout)))
    }
    fn transform(&self) -> Result<crate::text_transform::TextTransform, RenderError> {
        let extent = self
            .extent
            .ok_or_else(|| rejected(self.extent, "a viewport for text rendering"))?;
        let (projection, model_view) = self.text_scene_matrices(extent)?;
        let mut camera_rotation = self.camera.orientation_matrix();
        camera_rotation.set_column3(3, Vec3::new(0., 0., 0.));
        Ok(crate::text_transform::TextTransform {
            projection,
            model_view,
            model_world: self
                .model_override
                .map(|m| Mat4::from_other(&m))
                .unwrap_or_else(|| {
                    crate::persistent_transform::model_transform(self.group, self.structure)
                }),
            camera_rotation,
            viewport: [0, 0, extent[0] as i32, extent[1] as i32],
            pixel_coordinates: false,
            zoomable_scale: crate::render_scale::zoomable_text_scale(self.rendering),
        })
    }
    fn text_scene_matrices(&self, extent: [u32; 2]) -> Result<(Mat4<f64>, Mat4<f64>), RenderError> {
        let persistence = crate::render_scale::persistence_extent(extent, self.rendering);
        let (mut projection, mut model_view) = crate::persistent_transform::text_matrices(
            self.group,
            self.camera,
            persistence,
            self.structure,
        )?;
        if let Some(model) = self.model_override {
            let (_, view) = crate::persistent_transform::lighting_matrices(self)?;
            // GlText promotes the float GL matrices before multiplying.
            model_view = Mat4::<f64>::from_other(&Mat4::<f32>::from_data(view))
                * Mat4::<f64>::from_other(&model);
            projection = Mat4::from_other(&Mat4::<f32>::from_other(&projection));
        }
        Ok((projection, model_view))
    }
}
/// Reject unsupported shader requests; `validate_text_style(&aspect)?` preserves native paths.
/// Every display type is drawn: GlText::render has no default branch, so the values outside
/// AspectTypeOfDisplayText (Native6, Native7) draw only the main pass, as Normal does
/// (the recorded text_display_type capture: identical dumps on llvmpipe and radeonsi).
pub(crate) fn validate_text_style(aspect: &dyn Graphic3dAspectsSource) -> Result<(), RenderError> {
    if let Some(program) = aspect.shader_program().get() {
        return Err(crate::user_program::refuse_user_program(program));
    }
    Ok(())
}
