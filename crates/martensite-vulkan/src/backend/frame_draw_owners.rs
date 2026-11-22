//! After recording, the fence keeps GPU resources alive without keeping CPU draw data.
use super::super::{
    geometry_buffer::GeometryBuffer, oit_descriptor::OitPassDescriptors,
    oit_upload::OitUploadedVariants, pipeline::DrawPipeline, storage_descriptor::UploadedStorage,
    user_program_draw::UploadedUserProgram, window_depth_status::WindowDepthStatus,
};
use super::{DrawBindings, UploadedDraw};
use std::sync::Arc;

#[derive(Default)]
pub(super) struct DrawGpuOwners {
    vertices: Vec<Arc<GeometryBuffer>>,
    indices: Vec<Arc<GeometryBuffer>>,
    pipelines: Vec<Arc<DrawPipeline>>,
    lighting: Vec<Arc<UploadedStorage>>,
    extras: Vec<ExtraDrawOwners>,
}

struct ExtraDrawOwners {
    _bindings: DrawBindings,
    _native_alpha: Option<Arc<OitPassDescriptors>>,
    _oit_alpha: Vec<(ash::vk::DescriptorSet, Arc<OitPassDescriptors>)>,
    _oit_pipelines: Option<OitUploadedVariants>,
    _user_program: Option<Box<UploadedUserProgram>>,
    _window_depth: Option<Box<WindowDepthStatus>>,
    #[cfg(feature = "vertex-output-diagnostics")]
    _vertex_output: Option<super::super::vertex_output::VertexOutputResources>,
}

impl DrawGpuOwners {
    pub(super) fn from_draws(draws: Vec<UploadedDraw>) -> Self {
        let mut owners = Self::default();
        for draw in draws {
            owners.retain_draw(draw);
        }
        owners
    }

    pub(super) fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    fn retain_draw(&mut self, draw: UploadedDraw) {
        let UploadedDraw {
            vertices,
            indices,
            pipeline,
            mut bindings,
            native_alpha,
            oit_alpha,
            oit,
            user_program,
            window_depth_status,
            #[cfg(feature = "vertex-output-diagnostics")]
            vertex_output,
            #[cfg(feature = "command-state-diagnostics")]
                geometry_arguments: _,
            window_depth_expected: _,
            sample_alpha_cutoff: _,
            count: _,
            instances: _,
            point_viewports: _,
            transform: _,
            line: _,
            presentation: _,
        } = draw;
        keep_adjacent_owner(&mut self.vertices, vertices);
        keep_adjacent_owner(&mut self.indices, indices);
        keep_adjacent_owner(&mut self.pipelines, pipeline);
        if let Some(lighting) = bindings.lighting.take() {
            keep_adjacent_owner(&mut self.lighting, lighting);
        }
        let extras = ExtraDrawOwners {
            _bindings: bindings,
            _native_alpha: native_alpha,
            _oit_alpha: oit_alpha,
            _oit_pipelines: oit,
            _user_program: user_program,
            _window_depth: window_depth_status,
            #[cfg(feature = "vertex-output-diagnostics")]
            _vertex_output: vertex_output,
        };
        if extras.have_owners() {
            self.extras.push(extras);
        }
    }
}

impl ExtraDrawOwners {
    fn have_owners(&self) -> bool {
        #[cfg(feature = "vertex-output-diagnostics")]
        if self._vertex_output.is_some() {
            return true;
        }
        let DrawBindings {
            clipping,
            lighting: _,
            sprite,
            surface_image,
            pbr_environment,
            material_maps,
            sphere_map,
            shadow_maps,
        } = &self._bindings;
        self._native_alpha.is_some()
            || !self._oit_alpha.is_empty()
            || self._oit_pipelines.is_some()
            || self._user_program.is_some()
            || self._window_depth.is_some()
            || clipping.is_some()
            || sprite.is_some()
            || surface_image.is_some()
            || pbr_environment.is_some()
            || material_maps.is_some()
            || sphere_map.is_some()
            || shadow_maps.is_some()
    }
}

/// The submission only needs one reference per owner, even when successive draws share it.
fn keep_adjacent_owner<T>(owners: &mut Vec<Arc<T>>, owner: Arc<T>) {
    if !owners.last().is_some_and(|last| Arc::ptr_eq(last, &owner)) {
        owners.push(owner);
    }
}
