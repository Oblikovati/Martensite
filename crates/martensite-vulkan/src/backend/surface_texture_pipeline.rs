//! Surface image and mask variants have independent vertex formats and shader interfaces.
use super::pipeline_variants::PipelineVariant;
use crate::{PrimitiveDraw, RenderError, error::rejected};
use ash::vk;
macro_rules! surface_pair {
    ($name:literal) => {
        [
            include_bytes!(concat!("../../shaders/", $name, ".vert.spv")).as_slice(),
            include_bytes!(concat!("../../shaders/", $name, ".frag.spv")).as_slice(),
        ]
    };
}
pub(super) fn surface_shaders(kind: PipelineVariant) -> [&'static [u8]; 2] {
    if kind.point {
        return point_surface_shaders(kind);
    }
    let [vertex, fragment] = surface_layout_shaders(kind);
    if !kind.sphere_map {
        return [vertex, fragment];
    }
    // The unlit image fragment samples the coordinates of the TextureEnv vertex stage.
    let vertex: &[u8] = if kind.clipped {
        include_bytes!("../../shaders/surface.clipped.sphere-map.vert.spv")
    } else {
        include_bytes!("../../shaders/surface.sphere-map.vert.spv")
    };
    [vertex, fragment]
}
fn point_surface_shaders(kind: PipelineVariant) -> [&'static [u8]; 2] {
    let vertex = point_surface_vertex(kind);
    let untextured_fragment = PipelineVariant {
        surface_texture: false,
        ..kind
    };
    [vertex, surface_layout_shaders(untextured_fragment)[1]]
}
macro_rules! point_vertex {
    ($name:literal) => {
        include_bytes!(concat!("../../shaders/", $name, ".point.vert.spv")).as_slice()
    };
}
fn point_surface_vertex(kind: PipelineVariant) -> &'static [u8] {
    match (kind.lighting, kind.clipped, kind.surface_texture) {
        (false, false, false) => point_vertex!("surface"),
        (false, false, true) => point_vertex!("surface.image"),
        (false, true, false) => point_vertex!("surface.clipped"),
        (false, true, true) => point_vertex!("surface.clipped.image"),
        (true, false, false) => point_vertex!("surface.lit"),
        (true, false, true) => point_vertex!("surface.lit.image"),
        (true, true, false) => point_vertex!("surface.lit.clipped"),
        (true, true, true) => point_vertex!("surface.lit.clipped.image"),
    }
}
fn surface_layout_shaders(kind: PipelineVariant) -> [&'static [u8]; 2] {
    match (kind.lighting, kind.clipped, kind.surface_texture) {
        (false, false, false) => surface_pair!("surface"),
        (false, false, true) => surface_pair!("surface.image"),
        (false, true, false) => surface_pair!("surface.clipped"),
        (false, true, true) => surface_pair!("surface.clipped.image"),
        (true, false, false) => surface_pair!("surface.lit"),
        (true, false, true) => surface_pair!("surface.lit.image"),
        (true, true, false) => surface_pair!("surface.lit.clipped"),
        (true, true, true) => surface_pair!("surface.lit.clipped.image"),
    }
}
/// The lit layout (with normals) also feeds the sphere-map vertex stage;
/// `surface_vertex_state(kind.lighting || kind.sphere_map)`.
pub(super) fn surface_vertex_state(
    lighting: bool,
) -> vk::PipelineVertexInputStateCreateInfo<'static> {
    const UNLIT: [vk::VertexInputBindingDescription; 1] = [vk::VertexInputBindingDescription {
        binding: 0,
        stride: 48,
        input_rate: vk::VertexInputRate::VERTEX,
    }];
    const LIT: [vk::VertexInputBindingDescription; 1] = [vk::VertexInputBindingDescription {
        binding: 0,
        stride: 60,
        input_rate: vk::VertexInputRate::VERTEX,
    }];
    const ATTRIBUTES: [vk::VertexInputAttributeDescription; 4] = [
        attribute(0, 0, vk::Format::R32G32B32A32_SFLOAT),
        attribute(1, 16, vk::Format::R32G32B32A32_SFLOAT),
        attribute(2, 32, vk::Format::R32G32B32A32_SFLOAT),
        attribute(3, 48, vk::Format::R32G32B32_SFLOAT),
    ];
    vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(if lighting { &LIT } else { &UNLIT })
        .vertex_attribute_descriptions(&ATTRIBUTES[..if lighting { 4 } else { 3 }])
}
const fn attribute(
    location: u32,
    offset: u32,
    format: vk::Format,
) -> vk::VertexInputAttributeDescription {
    vk::VertexInputAttributeDescription {
        location,
        binding: 0,
        format,
        offset,
    }
}

/// Faces mapped with the view's environment texture; `draw_uses_sphere_map(&draw)`.
pub(super) fn draw_uses_sphere_map(draw: &PrimitiveDraw) -> bool {
    !draw.surface_texture_binding_only
        && draw
            .surface_texture
            .as_ref()
            .is_some_and(|style| style.sphere_map.is_some())
}
/// The mesh-edge program has no TextureEnv stage; `require_sphere_map_without_mesh_edges(&draw)?`.
pub(super) fn require_sphere_map_without_mesh_edges(
    draw: &PrimitiveDraw,
) -> Result<(), RenderError> {
    if draw.mesh_edges.is_some() && draw_uses_sphere_map(draw) {
        return Err(rejected(
            "mesh edges over an environment texture",
            "an environment-mapped face without the mesh-edge program",
        ));
    }
    Ok(())
}
