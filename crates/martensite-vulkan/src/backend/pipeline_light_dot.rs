//! Versioned light terms change in vertex/fragment stages; geometry transforms and contracts remain.
use super::*;
impl DrawPipeline {
    pub(super) fn selected_builtin_vertex(
        &self,
        stock: &'static [u8],
    ) -> Result<&'static [u8], RenderError> {
        super::super::factorized_projection::admit_vertex_composition(self.kind)?;
        if !self.shaders.is_empty() {
            return Ok(stock);
        }
        let stock = self.admitted_line_vertex(stock)?;
        let selected = super::super::light_dot_shader::selected_vertex(
            self.runtime.light_dot_behavior,
            crate::backend::light_dot_shader::builtin_phong_kind(self.kind),
            stock,
        )?;
        let selected = selected.unwrap_or(stock);
        if self.kind.fixed_grid_point_sprite {
            return super::super::fixed_grid_sprite_shader::vertex(selected);
        }
        if self.kind.fixed_grid_point {
            return super::super::fixed_grid_shader::select_fixed_grid_vertex(
                selected,
                self.kind.fixed_grid_point_unit12,
            );
        }
        if self.kind.window_depth.is_some_and(|p| p.clipped_fan()) {
            return super::super::clipped_depth_shader::vertex(selected);
        }
        if self.kind.world_clip_v2 {
            if self.runtime.world_clipping
                == crate::RenderWorldClippingBehavior::RasterFixedFunctionClipSpaceV5
            {
                return super::super::world_clip_shader::select_vertex_v5(selected);
            }
            return super::super::world_clip_shader::select_vertex_v2(selected);
        }
        if self.kind.world_clip_v3_line {
            return super::super::world_clip_shader::select_line_vertex_v3(selected);
        }
        let selected = super::super::factorized_projection::vertex(
            self.runtime.projection_behavior,
            self.kind,
            selected,
        )?;
        let selected = super::super::window_depth_shader::vertex(self.kind, selected)?;
        if !self.runtime.projection_behavior.composes_model_view() {
            return Ok(selected);
        }
        Ok(
            super::super::fixed_lighting_pipeline::fixed_sum_order_vertex(selected)
                .unwrap_or(selected),
        )
    }

    pub(super) fn apply_sprite_fragment(&mut self) -> Result<(), RenderError> {
        if self.kind.fixed_grid_point_sprite {
            self.replace_fragment(super::super::fixed_grid_sprite_shader::fragment(
                self.fragment_bytes,
            )?)?;
        }
        Ok(())
    }

    fn admitted_line_vertex(&self, stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
        if !self.kind.line {
            return Ok(stock);
        }
        super::super::line_clip_shader::select_admitted_line_vertex(
            stock,
            self.runtime.admit_line_clipping(false),
        )
    }
    pub(super) fn apply_fixed_reciprocal_fragment(&mut self) -> Result<(), RenderError> {
        if !self.runtime.projection_behavior.composes_model_view() {
            return Ok(());
        }
        let stock = self.fragment_bytes;
        match super::super::fixed_lighting_pipeline::fixed_reciprocal_fragment(stock) {
            Some(bytes) => self.replace_fragment(bytes),
            None => Ok(()),
        }
    }
    pub(super) fn apply_light_dot_behavior(&mut self) -> Result<(), RenderError> {
        let phong = crate::backend::light_dot_shader::builtin_phong_kind(self.kind);
        let selected = super::super::light_dot_shader::selected_fragment(
            self.runtime.light_dot_behavior,
            phong,
            self.fragment_bytes,
        )?;
        if let Some(bytes) = selected {
            self.replace_fragment(bytes)?;
        }
        Ok(())
    }
}
