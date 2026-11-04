//! Admit raster-stage world clipping only when the selected device and draw fit its version.
use crate::{
    PrimitiveDraw, PrimitiveTopology, RenderError, RenderShading,
    RenderWorldClippingBehavior as Behavior, RenderWorldClippingFailure as Failure,
    RenderWorldClippingRequirement as Requirement,
};
use ash::{Instance, vk};

#[derive(Clone, Copy, Default)]
pub(super) struct WorldClipDevice {
    selected: bool,
    shader_distances: bool,
    max_distances: u32,
    max_combined: u32,
}
impl WorldClipDevice {
    pub(super) fn query(
        instance: &Instance,
        physical: vk::PhysicalDevice,
        behavior: Behavior,
    ) -> Self {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let features = unsafe { instance.get_physical_device_features(physical) };
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let limits = unsafe { instance.get_physical_device_properties(physical) }.limits;
        Self::observed(
            behavior,
            features.shader_clip_distance == vk::TRUE,
            limits.max_clip_distances,
            limits.max_combined_clip_and_cull_distances,
        )
    }
    fn observed(behavior: Behavior, feature: bool, clip: u32, combined: u32) -> Self {
        Self {
            selected: matches!(
                behavior,
                Behavior::RasterClipDistanceV2
                    | Behavior::RasterClipDistanceV3
                    | Behavior::RasterFixedFunctionV4
                    | Behavior::RasterFixedFunctionClipSpaceV5
            ),
            shader_distances: feature,
            max_distances: clip,
            max_combined: combined,
        }
    }
    pub(super) fn enabled(self) -> bool {
        self.selected
    }
    pub(super) fn admit_device(self, behavior: Behavior) -> Result<(), RenderError> {
        if !self.selected
            || self.shader_distances && self.max_distances >= 8 && self.max_combined >= 8
        {
            return Ok(());
        }
        Err(self
            .failure(behavior, Requirement::ClipDistanceCapacity)
            .into())
    }
    pub(super) fn admit_draw(
        self,
        behavior: Behavior,
        draw: &PrimitiveDraw,
        compatible_policies: bool,
    ) -> Result<bool, RenderError> {
        if behavior == Behavior::FragmentV1 || !self.selected || !has_world_planes(draw) {
            return Ok(false);
        }
        self.admit_device(behavior)?;
        let unsupported = match behavior {
            Behavior::RasterClipDistanceV2 => unsupported_draw(draw, compatible_policies),
            Behavior::RasterClipDistanceV3 => unsupported_draw_v3(draw, compatible_policies),
            // `fixed_function_triangles` already selected only the draws this version serves.
            Behavior::FragmentV1
            | Behavior::RasterFixedFunctionV4
            | Behavior::RasterFixedFunctionClipSpaceV5 => None,
        };
        if let Some(actual) = unsupported {
            return Err(self
                .failure(behavior, Requirement::UnsupportedDraw { actual })
                .into());
        }
        if let Some(actual) = unsupported_world_input(draw) {
            return Err(self
                .failure(behavior, Requirement::UnsupportedDraw { actual })
                .into());
        }
        self.require_single_plane_chains(behavior, draw)?;
        Ok(true)
    }
    fn require_single_plane_chains(
        self,
        behavior: Behavior,
        draw: &PrimitiveDraw,
    ) -> Result<(), RenderError> {
        let Some(clipping) = &draw.clipping else {
            return Ok(());
        };
        let planes = plane_count(draw);
        if planes > 8 || clipping.chains().iter().any(|chain| chain.len() != 1) {
            let actual = format!("{planes} planes in {:?}", clipping.chains());
            return Err(self
                .failure(behavior, Requirement::UnsupportedPlaneChains { actual })
                .into());
        }
        Ok(())
    }
    fn failure(self, behavior: Behavior, reason: Requirement) -> Failure {
        Failure {
            behavior,
            reason,
            shader_clip_distance: self.shader_distances,
            max_clip_distances: self.max_distances,
            max_combined_clip_and_cull_distances: self.max_combined,
        }
    }
}
/// Triangles of a fixed-function frame whose stock vertex stage has a raster-clip interface.
/// Others keep the discard, so one such draw does not leave the whole frame unrendered.
pub(super) fn fixed_function_triangles(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Triangles
        && (draw.fixed_function_lighting || draw.fixed_function_mask)
        && has_world_planes(draw)
        && single_plane_chains(draw)
        && !draw.lighting.as_ref().is_some_and(|light| light.is_pbr())
        // A stencil mask carries only positions: apply_native_stencil_aspect removed the rest.
        && (draw.fixed_function_mask || unsupported_overlay(draw).is_none())
        && unsupported_world_input(draw).is_none()
}
fn single_plane_chains(draw: &PrimitiveDraw) -> bool {
    plane_count(draw) <= 8
        && draw
            .clipping
            .as_ref()
            .is_some_and(|clip| clip.chains().iter().all(|chain| chain.len() == 1))
}
fn has_world_planes(draw: &PrimitiveDraw) -> bool {
    draw.clipping
        .as_ref()
        .is_some_and(|clip| clip.chains().iter().any(|chain| !chain.is_empty()))
}
fn plane_count(draw: &PrimitiveDraw) -> usize {
    draw.clipping
        .as_ref()
        .map_or(0, |clip| clip.chains().iter().map(Vec::len).sum())
}
fn unsupported_draw(draw: &PrimitiveDraw, compatible_policies: bool) -> Option<String> {
    if draw.topology() != PrimitiveTopology::Triangles {
        return Some(format!("topology {:?}", draw.topology()));
    }
    if !built_in_phong(draw) {
        return Some("draw is not built-in Phong lit".into());
    }
    if !compatible_policies {
        return Some("interpolation, light-dot or triangle-coverage policy".into());
    }
    if translucent_source(draw) {
        return Some("nonopaque material or vertex alpha".into());
    }
    unsupported_overlay(draw).map(str::to_owned)
}
fn unsupported_draw_v3(draw: &PrimitiveDraw, compatible_policies: bool) -> Option<String> {
    if draw.topology() == PrimitiveTopology::Lines {
        return unsupported_solid_line(draw, compatible_policies);
    }
    unsupported_draw(draw, compatible_policies)
}
fn unsupported_solid_line(draw: &PrimitiveDraw, compatible_policies: bool) -> Option<String> {
    let Some(style) = draw.line_style else {
        return Some("line draw has no native line style".into());
    };
    if !compatible_policies || draw.fixed_function_lighting || draw.lighting.is_some() {
        return Some("line raster or lighting policy".into());
    }
    if style.pattern != 0xffff
        || style.alpha_blend
        || draw.vertices.iter().any(|vertex| vertex.color[3] != 1.)
    {
        return Some("line is stippled or nonopaque".into());
    }
    if crate::fixed_line_stipple::fixed_line_stipple(draw).is_some()
        || draw.line_style.is_some_and(|line| line.context_width)
    {
        return Some("line uses a fixed-function stipple or context width".into());
    }
    if let Some(overlay) = unsupported_overlay(draw) {
        return Some(overlay.to_owned());
    }
    if draw.source_color_array && draw.vertices.iter().any(|vertex| vertex.color[3] != 1.) {
        return Some("nonopaque source line colors".into());
    }
    None
}
fn built_in_phong(draw: &PrimitiveDraw) -> bool {
    draw.lighting
        .as_ref()
        .is_some_and(|light| !light.is_pbr() && light.shading() == RenderShading::Phong)
}
fn translucent_source(draw: &PrimitiveDraw) -> bool {
    let material = draw.lighting.as_ref().is_some_and(|light| {
        light
            .material
            .common
            .iter()
            .any(|face| face.diffuse[3] != 1.)
            || light
                .fixed_source_opacity
                .is_some_and(|opacity| opacity.iter().any(|a| *a != 1.))
    });
    let vertex_colors = draw.source_color_array
        || draw
            .lighting
            .as_ref()
            .is_some_and(|light| light.vertex_colors);
    material || vertex_colors && draw.vertices.iter().any(|vertex| vertex.color[3] != 1.)
}
fn unsupported_world_input(draw: &PrimitiveDraw) -> Option<String> {
    draw.clipping.as_ref()?;
    unsupported_homogeneous_weights(draw).or_else(|| unsupported_world_magnitudes(draw))
}
fn unsupported_homogeneous_weights(draw: &PrimitiveDraw) -> Option<String> {
    if let Some((index, vertex)) = draw
        .vertices
        .iter()
        .enumerate()
        .find(|(_, vertex)| !vertex.weight.is_finite() || vertex.weight < f32::MIN_POSITIVE)
    {
        return Some(format!(
            "vertex {index} homogeneous weight {} (0x{:08x}); expected positive normal finite W",
            vertex.weight,
            vertex.weight.to_bits()
        ));
    }
    None
}
fn unsupported_world_magnitudes(draw: &PrimitiveDraw) -> Option<String> {
    let clipping = draw.clipping.as_ref()?;
    let model = clipping.world_from_object();
    for (index, vertex) in draw.vertices.iter().enumerate() {
        if world_bound_exceeds_f32(model, vertex) {
            return Some(format!("vertex {index} world transform may overflow f32"));
        }
        if plane_bound_exceeds_f32(clipping.chains(), model, vertex) {
            return Some(format!(
                "vertex {index} world-plane distance may overflow f32"
            ));
        }
    }
    None
}
fn world_bound_exceeds_f32(model: &[f32; 16], vertex: &crate::RenderVertex) -> bool {
    (0..4).any(|row| world_component_bound(model, vertex, row) >= f32::MAX as f64 / 4.)
}
fn world_component_bound(model: &[f32; 16], vertex: &crate::RenderVertex, row: usize) -> f64 {
    vertex
        .position
        .into_iter()
        .chain(std::iter::once(vertex.weight))
        .enumerate()
        .map(|(column, value)| (model[column * 4 + row] as f64 * value as f64).abs())
        .sum::<f64>()
}
fn plane_bound_exceeds_f32(
    chains: &[Vec<[f32; 4]>],
    model: &[f32; 16],
    vertex: &crate::RenderVertex,
) -> bool {
    chains.iter().flatten().any(|plane| {
        (0..4)
            .map(|row| plane[row].abs() as f64 * world_component_bound(model, vertex, row))
            .sum::<f64>()
            >= f32::MAX as f64 / 4.
    })
}
fn unsupported_overlay(draw: &PrimitiveDraw) -> Option<&'static str> {
    unsupported_material_overlay(draw).or_else(|| unsupported_geometry_overlay(draw))
}
fn unsupported_material_overlay(draw: &PrimitiveDraw) -> Option<&'static str> {
    [
        ("registered user program", draw.user_program.is_some()),
        ("surface texture", draw.surface_texture.is_some()),
        ("texture-only binding", draw.surface_texture_binding_only),
        (
            "material maps",
            crate::backend::material_map_pipeline::draw_uses_material_maps(draw),
        ),
        ("surface cutoff", draw.surface_cutoff.is_some()),
        (
            "native material cutoff",
            draw.native_material_cutoff.is_some(),
        ),
        (
            "surface blend",
            draw.surface_blend || draw.surface_forced_blend,
        ),
    ]
    .into_iter()
    .find_map(|(name, present)| present.then_some(name))
}
fn unsupported_geometry_overlay(draw: &PrimitiveDraw) -> Option<&'static str> {
    [
        ("mesh edges", draw.mesh_edges.is_some()),
        ("polygon edge style", draw.polygon_edge_style.is_some()),
        ("polygon points", draw.polygon_points),
        ("back color", draw.unlit_back_color.is_some()),
        ("depth pass", draw.depth_pass != Default::default()),
        ("polygon face lines", draw.polygon_face_lines),
        (
            "capping pass",
            draw.capping_pass != crate::RenderCappingPass::None,
        ),
    ]
    .into_iter()
    .find_map(|(name, present)| present.then_some(name))
}

#[cfg(test)]
#[path = "world_clip_device_tests.rs"]
mod tests;
