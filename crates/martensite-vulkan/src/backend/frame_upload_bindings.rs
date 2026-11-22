//! Bind per-draw material and storage payloads after selecting the raster pipeline.
use super::{DrawPipeline, DrawUploadContext, PreparedBindings};
use crate::backend::{
    host_upload_pages::FrameUploadPages, storage_descriptor::UploadedStorage,
    surface_image_descriptor::SurfaceImageBinding,
};
use crate::{RenderError, error::rejected};
use std::sync::Arc;

impl DrawUploadContext<'_> {
    pub(super) fn upload_bindings(
        &self,
        pipeline: &DrawPipeline,
        storage: &mut FrameUploadPages<'_>,
    ) -> Result<PreparedBindings, RenderError> {
        let sprite = self.upload_sprite(pipeline)?;
        self.prepare_surface_cache()?;
        let surface_image = self.upload_surface(pipeline)?;
        let clipping = self.upload_clipping(pipeline, storage)?;
        let lighting = self.upload_lighting(pipeline, storage)?;
        let sphere_map = self.upload_sphere_map(pipeline, storage)?;
        let pbr_environment = pipeline
            .kind
            .pbr
            .then(|| self.pipelines.pbr_environment(self.runtime))
            .transpose()?;
        let material_maps = self.upload_material_maps(pipeline)?;
        let shadow_maps = pipeline
            .shadow_layout
            .is_some()
            .then(|| self.pipelines.shadow_maps(self.runtime))
            .transpose()?;
        Ok(PreparedBindings {
            shadow_maps,
            lighting,
            sphere_map,
            clipping,
            surface_image,
            sprite,
            pbr_environment,
            material_maps,
        })
    }
    fn upload_sprite(
        &self,
        pipeline: &DrawPipeline,
    ) -> Result<Option<Arc<UploadedStorage>>, RenderError> {
        let (Some(layout), Some(style)) = (&pipeline.sprite_layout, &self.draw.point_style) else {
            return Ok(None);
        };
        if !cfg!(feature = "command-state-diagnostics")
            && style.color_policy == crate::point_style::PointColorPolicy::Plain
        {
            return self
                .pipelines
                .plain_point_sprite
                .binding(self.runtime, layout, &style.image)
                .map(Some);
        }
        crate::backend::point_sprite::upload_sprite(self.runtime, layout, &style.image)
            .map(Arc::new)
            .map(Some)
    }

    fn prepare_surface_cache(&self) -> Result<(), RenderError> {
        let draw = self.draw;
        if let Some(unavailable) = &draw.surface_texture_unavailable {
            self.pipelines
                .surface_images
                .forget(&unavailable.resource_id)?;
        }
        if draw.surface_texture_binding_only
            && let Some(style) = &draw.surface_texture
        {
            self.touch_surface(style)?;
        }
        Ok(())
    }
    fn touch_surface(
        &self,
        style: &crate::surface_texture::SurfaceTextureStyle,
    ) -> Result<(), RenderError> {
        self.pipelines.surface_images.touch(
            self.runtime,
            &style.image,
            crate::fixed_surface_environment::uses_fixed_surface_environment(self.draw),
            self.retirements,
        )
    }
    fn upload_surface(
        &self,
        pipeline: &DrawPipeline,
    ) -> Result<Option<Arc<SurfaceImageBinding>>, RenderError> {
        if let Some(style) = &self.draw.textured_style {
            return self.glyph_atlas(pipeline, &style.image).map(Some);
        }
        self.draw
            .surface_texture
            .as_ref()
            .filter(|_| !self.draw.surface_texture_binding_only)
            .map(|style| self.selected_surface(pipeline, style))
            .transpose()
    }
    fn glyph_atlas(
        &self,
        pipeline: &DrawPipeline,
        atlas: &crate::RasterImage,
    ) -> Result<Arc<SurfaceImageBinding>, RenderError> {
        let layout = pipeline
            .surface_layout
            .as_ref()
            .expect("glyph atlas image layout");
        self.pipelines
            .glyph_atlases
            .binding(self.runtime, layout, atlas, self.retirements)
    }

    fn selected_surface(
        &self,
        pipeline: &DrawPipeline,
        style: &crate::surface_texture::SurfaceTextureStyle,
    ) -> Result<Arc<SurfaceImageBinding>, RenderError> {
        let layout = pipeline
            .surface_layout
            .as_ref()
            .expect("surface image layout");
        self.pipelines.surface_images.binding(
            self.runtime,
            layout,
            style,
            crate::fixed_surface_environment::uses_fixed_surface_environment(self.draw),
            self.retirements,
        )
    }
    fn upload_material_maps(
        &self,
        pipeline: &DrawPipeline,
    ) -> Result<Option<Arc<crate::backend::material_map_descriptor::MaterialMapBinding>>, RenderError>
    {
        let (Some(layout), Some(image_layout)) =
            (&pipeline.material_map_layout, &pipeline.surface_layout)
        else {
            return Ok(None);
        };
        let style = self.draw.surface_texture.as_ref();
        let Some(maps) = style.and_then(|style| style.material_maps.as_ref()) else {
            return Ok(None);
        };
        let pipelines = self.pipelines;
        let mocks = pipelines.material_map_mocks(self.runtime, image_layout, self.retirements)?;
        let request = crate::backend::material_map_descriptor::MaterialMapRequest {
            runtime: self.runtime,
            layout,
            image_layout,
            cache: &pipelines.surface_images,
            mocks: &mocks,
            retirements: self.retirements,
        };
        request.bind(maps).map(Some)
    }
    fn upload_clipping(
        &self,
        pipeline: &DrawPipeline,
        storage: &mut FrameUploadPages<'_>,
    ) -> Result<Option<UploadedStorage>, RenderError> {
        let bytes = if pipeline.kind.world_clip_v2 || pipeline.kind.world_clip_v3_line {
            crate::retained_hatch::raster_world_clip_fragment_mask_bytes(
                self.draw,
                self.extent.height,
            )
        } else {
            crate::retained_hatch::fragment_mask_bytes(self.draw, self.extent.height)
        };
        bytes
            .as_ref()
            .map(|bytes| {
                let layout = pipeline
                    .clipping_layout
                    .as_ref()
                    .expect("clipping pipeline layout");
                UploadedStorage::for_frame(self.runtime, layout, bytes, storage)
            })
            .transpose()
    }

    fn upload_lighting(
        &self,
        pipeline: &DrawPipeline,
        storage: &mut FrameUploadPages<'_>,
    ) -> Result<Option<Arc<UploadedStorage>>, RenderError> {
        self.draw
            .lighting
            .as_ref()
            .map(|lighting| {
                let layout = pipeline
                    .lighting_layout
                    .as_ref()
                    .expect("lighting pipeline layout");
                let retain =
                    lighting.fixed.is_none() && !lighting.pbr && self.draw.user_program.is_none();
                self.pipelines.lighting_bindings.bind_draw(
                    self.runtime,
                    layout,
                    lighting,
                    storage,
                    retain,
                )
            })
            .transpose()
    }

    fn upload_sphere_map(
        &self,
        pipeline: &DrawPipeline,
        storage: &mut FrameUploadPages<'_>,
    ) -> Result<Option<UploadedStorage>, RenderError> {
        let Some(layout) = &pipeline.sphere_layout else {
            return Ok(None);
        };
        let style = self.draw.surface_texture.as_ref();
        let transform = style
            .and_then(|style| style.sphere_map.as_ref())
            .ok_or_else(|| rejected("no sphere map", "an environment-mapped draw"))?;
        UploadedStorage::for_frame(self.runtime, layout, &transform.encode(), storage).map(Some)
    }
    /// The draw's registered user program with the selected variant's fixed state.
    pub(super) fn upload_user_program(
        &self,
        pipeline: &Arc<DrawPipeline>,
    ) -> Result<Option<crate::backend::user_program_draw::UploadedUserProgram>, RenderError> {
        let Some(program) = &self.draw.user_program else {
            return Ok(None);
        };
        let variant = Arc::as_ptr(pipeline) as usize;
        let fixed = crate::backend::user_program_pipeline::UserPipelineState::single(
            pipeline.kind,
            variant,
        );
        let linked =
            self.pipelines
                .user_programs
                .pipeline(self.runtime, &program.modules, fixed)?;
        let textures =
            self.pipelines
                .user_program_textures(self.runtime, self.draw, self.retirements)?;
        crate::backend::user_program_draw::UploadedUserProgram::new(
            self.runtime,
            (linked, fixed),
            self.draw,
            program,
            textures,
        )
        .map(Some)
    }
}
