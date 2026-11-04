use super::{DrawPipeline, load_shader};
use crate::RenderError;

impl DrawPipeline {
    pub(super) fn has_v4_geometry_stage(&self) -> bool {
        self.logical_facing_active() && self.shaders.len() == 3
    }

    pub(super) fn add_v4_geometry_transport(&mut self) -> Result<(), RenderError> {
        if !self.logical_facing_active() {
            return Ok(());
        }
        self.require_v4_geometry_interface()?;
        self.append_v4_geometry_module()
    }

    fn logical_facing_active(&self) -> bool {
        super::super::fixed_grid_record::logical_facing_active(
            self.runtime.triangle_coverage,
            self.triangle_grid_coverage_shader,
        )
    }

    fn require_v4_geometry_interface(&self) -> Result<(), RenderError> {
        super::super::v4_geometry_transport::require_pipeline(self.v4_pipeline_state())?;
        super::super::v4_geometry_transport::require_limits(
            super::super::v4_geometry_transport::limits_from_runtime(&self.runtime),
            self.kind.clipped,
            self.requires_original_clip_position_transport(),
        )
    }

    fn v4_pipeline_state(&self) -> super::super::v4_geometry_transport::V4GeometryPipelineState {
        super::super::v4_geometry_transport::V4GeometryPipelineState {
            supported_surface: self.supported_v4_surface(),
            shader_modules: self.shaders.len(),
        }
    }

    fn supported_v4_surface(&self) -> bool {
        self.phong_interpolation_shader
            && self.kind.lighting
            && !self.kind.fixed_lighting
            && !self.kind.pbr
            && !self.kind.material_maps
            && !self.kind.sphere_map
            && !self.kind.surface_texture
            && !self.kind.surface_alpha
            && !self.kind.mesh_edges
            && !self.kind.polygon_edges
            && !self.kind.polygon_points
            && !self.kind.fixed_grid_point
            && (!self.kind.clipped || self.supports_clipped_v4_surface())
            && !self.kind.world_clip_v2
            && !self.kind.line
            && !self.kind.point
            && !self.kind.text
            && self.oit_layout.is_none()
    }

    fn append_v4_geometry_module(&mut self) -> Result<(), RenderError> {
        let bytes = if self.requires_original_clip_position_transport() {
            super::super::v4_geometry_transport::clipped_world_clip_shader_bytes()
        } else if self.kind.clipped {
            super::super::v4_geometry_transport::clipped_shader_bytes()
        } else {
            super::super::v4_geometry_transport::shader_bytes()
        };
        self.shaders.push(load_shader(&self.runtime, bytes)?);
        #[cfg(test)]
        if self.runtime.executable_profile.is_some() {
            self.executable_bytes.push(bytes);
        }
        Ok(())
    }

    fn requires_original_clip_position_transport(&self) -> bool {
        super::super::v4_geometry_transport::requires_original_clip_positions(
            self.kind.clipped,
            self.kind.world_clip_grid12,
            self.runtime.world_clip_interpolation,
            self.runtime.triangle_coverage,
            self.runtime.phong_interpolation,
        )
    }

    fn supports_clipped_v4_surface(&self) -> bool {
        self.kind.world_clip_grid12
            || (self.runtime.world_clipping == crate::RenderWorldClippingBehavior::FragmentV1
                && self.runtime.world_clip_interpolation
                    == crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2
                && self.runtime.phong_interpolation
                    == crate::RenderPhongInterpolationBehavior::UnsnappedV3
                && matches!(
                    self.runtime.triangle_coverage,
                    crate::RenderTriangleCoverageBehavior::NativeGrid12V4 { .. }
                ))
    }
}
