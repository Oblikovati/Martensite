//! Retained text uses the current aspect and view resolution on every extraction.
use crate::{
    Graphic3dDrawGroup, PrimitiveDraw, RenderError,
    error::rejected,
    text_layout::{TextLayoutSnapshot, TextProvider},
};
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
    pub fonts: &'a TextProvider,
    /// The group's model captured before visiting commands. Callback paths can observe edits.
    pub base_model: Option<&'a Mat4<f64>>,
    pub model_override: Option<Mat4<f32>>,
    /// Group persistence uses the actual incoming model for its local anchor.
    pub view_override: Option<Mat4<f32>>,
    pub clipping: Option<&'a crate::RenderClipping>,
    /// The view's environment texture while the layer uses it (`GlWorkspace`).
    pub environment: Option<&'a Handle<Graphic3dTextureEnv>>,
    pub fixed_function_lighting: bool,
    /// The pass's shared inverse of the text projection, when a pass lowers the group.
    pub text_unview: Option<&'a crate::text_transform::SharedTextUnview>,
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
        label: Option<&crate::text_label_record::LabelSlot>,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        if let Some(draws) = self.replayed_label(text, aspects, upload, label)? {
            return Ok(draws);
        }
        let inputs = crate::text_label_record::LabelInputs::current(text, aspects, self.rendering);
        let Some((aspect, layout)) = self.recorded_layout(text, aspects, upload)? else {
            return Ok(vec![]);
        };
        let draws =
            crate::text_drawing::lower_text_draws(text, aspect, layout.view(), &self.transform()?)?;
        if let (Some(slot), Some(inputs)) = (label, inputs)
            && text.text_formatter().is_null()
        {
            slot.keep(inputs, label_replay(aspect, layout.view(), &draws));
        }
        Ok(draws)
    }

    /// The kept draws of an unchanged label, placed for the current camera.
    fn replayed_label(
        &self,
        text: &Graphic3dText,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        upload: Option<&crate::element_upload::ElementUploadSlot>,
        label: Option<&crate::text_label_record::LabelSlot>,
    ) -> Result<Option<Vec<PrimitiveDraw>>, RenderError> {
        let Some(mut replay) = label.and_then(|slot| slot.unchanged(text, aspects, self.rendering))
        else {
            return Ok(None);
        };
        let Some(aspect) = aspects.get() else {
            return Ok(None);
        };
        let offsets = crate::text_drawing::native_text_offsets(aspect.text_display_type());
        let clips = self
            .transform()?
            .pass_clips(text, aspect, replay.point_size, offsets)?;
        if !crate::text_drawing::place_text_draws(&mut replay.draws, &clips) {
            return Ok(None);
        }
        if let Some(upload) = upload {
            let (indices, subtitle, bounds) = replay.upload;
            upload.record_text(indices, subtitle, bounds);
        }
        #[cfg(feature = "draw-record-verification")]
        self.verify_label(text, aspects, &replay.draws)?;
        Ok(Some(replay.draws))
    }

    /// Lower the label again and require the replay to equal it.
    #[cfg(feature = "draw-record-verification")]
    fn verify_label(
        &self,
        text: &Graphic3dText,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        replayed: &[PrimitiveDraw],
    ) -> Result<(), RenderError> {
        let lowered = self.draws(text, aspects, None, None)?;
        assert!(
            lowered.as_slice() == replayed,
            "a replayed text label differs from lowering it again"
        );
        Ok(())
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
        let mut draws =
            crate::text_drawing::lower_text_draws(text, aspect, layout.view(), &transform)?;
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
    ) -> Result<Option<(&'s dyn Graphic3dAspectsSource, TextLayoutSnapshot)>, RenderError> {
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
            let view = layout.view();
            upload.record_text(view.index_count(), subtitle, view.formatter_bounds());
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
            unview: self
                .text_unview
                .map(|shared| std::sync::OnceLock::from(shared.inverse(&projection)))
                .unwrap_or_default(),
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
/// What a label replay needs besides its inputs; `label_replay(aspect, layout, &draws)`.
fn label_replay(
    aspect: &dyn Graphic3dAspectsSource,
    layout: crate::text_layout::GlyphLayoutView<'_>,
    draws: &[PrimitiveDraw],
) -> crate::text_label_record::LabelReplay {
    let subtitle = aspect.text_display_type() == AspectTypeOfDisplayText::Subtitle;
    crate::text_label_record::LabelReplay {
        point_size: layout.point_size(),
        upload: (layout.index_count(), subtitle, layout.formatter_bounds()),
        draws: draws.to_vec(),
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
