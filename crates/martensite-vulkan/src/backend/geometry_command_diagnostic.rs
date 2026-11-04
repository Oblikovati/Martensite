//! Temporary upload-input evidence for point-cloud diagnosis; remove after input equivalence closes.
//! These bytes are owned CPU upload shadows, never GPU buffer or post-vertex readback.
use crate::PrimitiveDraw;
use ash::{vk, vk::Handle};

pub(super) struct GeometryArguments {
    vertices: Vec<u8>,
    indices: Vec<u8>,
    properties: String,
    encoded_vertex_count: usize,
}
impl GeometryArguments {
    pub(super) fn new(
        draw: &PrimitiveDraw,
        vertices: &[u8],
        indices: &[u8],
        encoded_vertex_count: usize,
    ) -> Self {
        Self {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
            properties: properties(draw),
            encoded_vertex_count,
        }
    }
    fn json(&self, identity: [u64; 6], count: u32, instances: u32) -> String {
        format!(
            "{{\"kind\":\"draw-geometry-upload-input\",\"command_ordinal_pipeline_vertex_index_layout\":{identity:?},\"index_count\":{count},\"instance_count\":{instances},\"gpu_readback\":false,\"post_vertex_observed\":false,\"encoded_vertex_count\":{},\"vertex_bytes\":{:?},\"index_bytes\":{:?},{}}}",
            self.encoded_vertex_count, self.vertices, self.indices, self.properties,
        )
    }
}
fn properties(draw: &PrimitiveDraw) -> String {
    let point = draw.point_style.as_ref();
    let size = point.map_or_else(
        || "null".to_owned(),
        |style| style.size.to_bits().to_string(),
    );
    let effective = point.filter(|_| draw.fixed_grid_point).map(|style| {
        crate::fixed_grid_point::fixed_grid_point_extent(style.size)
            .expect("validated fixed-grid point size")
    });
    format!(
        "\"topology\":\"{:?}\",\"source_vertex_count\":{},\"source_color_array\":{},\"surface_image\":{},\"surface_unavailable\":{},\"point_size_bits\":{},\"point_policy\":\"{:?}\",\"lighting\":\"{:?}\",\"fixed_grid_extent\":{}",
        draw.topology(),
        draw.vertices.len(),
        draw.source_color_array,
        draw.surface_texture.is_some(),
        draw.surface_texture_unavailable.is_some(),
        size,
        point.map(|style| style.color_policy),
        draw.lighting.as_ref().map(|lighting| lighting.shading()),
        effective.map_or_else(|| "null".to_owned(), |value| value.to_string()),
    )
}
pub(super) fn before_draw(
    (command, ordinal, pipeline): (vk::CommandBuffer, usize, vk::Pipeline),
    draw: &super::frame::UploadedDraw,
) {
    let identity = [
        command.as_raw(),
        ordinal as u64,
        pipeline.as_raw(),
        draw.vertices.handle.as_raw(),
        draw.indices.handle.as_raw(),
        draw.pipeline.layout.as_raw(),
    ];
    eprintln!(
        "{}",
        draw.geometry_arguments
            .json(identity, draw.count, draw.instances)
    );
    emit_modules(identity, draw);
}

#[cfg(test)]
#[path = "geometry_command_diagnostic_tests.rs"]
mod tests;

pub(super) fn shader_created(shader: vk::ShaderModule, bytes: &[u8]) {
    eprintln!("{}", shader_json(shader.as_raw(), bytes));
}
fn shader_json(shader: u64, bytes: &[u8]) -> String {
    format!(
        "{{\"kind\":\"shader-module-create-input\",\"module\":{shader},\"gpu_readback\":false,\"spirv_bytes\":{bytes:?}}}"
    )
}

#[derive(Debug, PartialEq)]
enum PipelineJoinIncomplete {
    CustomProgram,
    OitRoute,
    DifferentPipeline,
}
fn require_base_pipeline(
    actual: u64,
    base: u64,
    custom: bool,
    oit: bool,
) -> Result<(), PipelineJoinIncomplete> {
    if custom {
        return Err(PipelineJoinIncomplete::CustomProgram);
    }
    if oit {
        return Err(PipelineJoinIncomplete::OitRoute);
    }
    if actual != base {
        return Err(PipelineJoinIncomplete::DifferentPipeline);
    }
    Ok(())
}
fn emit_modules(identity: [u64; 6], draw: &super::frame::UploadedDraw) {
    let json = modules_json(
        identity,
        draw.pipeline.handle.as_raw(),
        draw.user_program.is_some(),
        draw.oit.is_some(),
        &draw.pipeline.diagnostic_modules(),
    );
    eprintln!("{json}");
}
fn modules_json(identity: [u64; 6], base: u64, custom: bool, oit: bool, modules: &[u64]) -> String {
    let join = require_base_pipeline(identity[2], base, custom, oit);
    let attributable = if join.is_ok() { modules } else { &[] };
    format!(
        "{{\"kind\":\"draw-pipeline-modules\",\"command_ordinal_pipeline_vertex_index_layout\":{identity:?},\"actual_pipeline_handle\":{},\"base_pipeline_handle\":{base},\"custom_program\":{custom},\"oit_route\":{oit},\"base_pipeline_join\":\"{join:?}\",\"modules_vertex_fragment_optional_geometry\":{attributable:?}}}",
        identity[2],
    )
}
