//! Shader stage selection for the stock draw pipelines.
use super::super::pipeline_shader_sources::{clipping_shaders, lighting_shaders};
use super::DrawPipeline;
use crate::RenderError;

impl DrawPipeline {
    pub(super) fn replace_oit_fragment(
        &mut self,
        target: super::super::oit_pipeline::OitPipelineTarget,
    ) -> Result<(), RenderError> {
        let bytes = super::super::oit_pipeline::oit_fragment_bytes(self.kind, target)?;
        self.replace_fragment(bytes)
    }
    pub(super) fn apply_coordinate_fragment(&mut self) -> Result<(), RenderError> {
        let legacy = super::super::attachment_coordinates::AttachmentCoordinates::LegacyBottomUpV1;
        if self.runtime.attachment_coordinates == legacy {
            return Ok(());
        }
        let bytes = super::super::coordinate_shader::coordinate_fragment(self.fragment_bytes)?;
        self.replace_fragment(bytes)
    }
    pub(super) fn load_compact_mesh_shaders(
        &mut self,
        directional: bool,
    ) -> Result<(), RenderError> {
        let [vertex, fragment] = if directional {
            [
                super::super::mesh_vertex_pipeline::DIRECTIONAL_VERTEX,
                super::super::directional_phong::SHADERS[1],
            ]
        } else {
            [
                super::super::mesh_vertex_pipeline::STOCK_VERTEX,
                lighting_shaders(false, false, false)[1],
            ]
        };
        self.add_shader(vertex)?;
        self.add_shader(fragment)
    }
    pub(super) fn load_shaders(
        &mut self,
        line: bool,
        point: bool,
        text: bool,
        clipped: bool,
        lighting: bool,
    ) -> Result<(), RenderError> {
        if let Some(shaders) = super::super::text_grid_interpolation_shader::selected_pipeline(
            self.runtime.text_interpolation,
            text,
            clipped,
        ) {
            self.text_grid_interpolation_shader = true;
            for bytes in shaders {
                self.add_shader(bytes)?;
            }
            return Ok(());
        }
        let shaders: [&[u8]; 2] = if lighting {
            lighting_shaders(clipped, point, line)
        } else if clipped {
            clipping_shaders(line, point, text)
        } else if text {
            [
                include_bytes!("../../shaders/text.vert.spv"),
                include_bytes!("../../shaders/text.frag.spv"),
            ]
        } else if point {
            [
                include_bytes!("../../shaders/point.vert.spv"),
                include_bytes!("../../shaders/point.frag.spv"),
            ]
        } else if line {
            let vertex: &[u8] = if self.kind.line_tint.uses_draw_color() {
                include_bytes!("../../shaders/line.draw-color.vert.spv")
            } else {
                include_bytes!("../../shaders/line.vert.spv")
            };
            [vertex, include_bytes!("../../shaders/line.frag.spv")]
        } else {
            [
                include_bytes!("../../shaders/unlit.vert.spv"),
                include_bytes!("../../shaders/unlit.frag.spv"),
            ]
        };
        for bytes in shaders {
            self.add_shader(bytes)?;
        }
        Ok(())
    }
}
