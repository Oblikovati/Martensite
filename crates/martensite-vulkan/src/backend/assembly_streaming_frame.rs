//! Admit only cold geometry that fits this native redraw. Upload and retirement use the normal frame path.
use super::{
    VulkanRenderer,
    frame::{geometry_key, vertex_stream_bytes},
    geometry_residency::GeometryKey,
    runtime::VulkanRuntime,
};
use crate::{
    PrimitiveDraw, PrimitiveTopology, RenderAssemblyStreamingProgress, RenderFrame,
    depth_prepass::RenderDepthPass,
};
use std::{borrow::Cow, collections::HashMap};

pub(super) struct StreamingFrame<'a> {
    pub frame: Cow<'a, RenderFrame>,
    pub progress: RenderAssemblyStreamingProgress,
}

struct Part {
    needs_upload: u64,
    focus_distance: f32,
    first_draw: usize,
    admitted: bool,
}

impl VulkanRenderer {
    pub(super) fn plan_assembly_streaming<'a>(
        &self,
        frame: &'a RenderFrame,
    ) -> Option<StreamingFrame<'a>> {
        let budget = frame.assembly_upload_budget?;
        if !admits_frame(&self.runtime, frame)
            || super::needs_stencil_pipeline(frame, frame.initial_stencil(self.stencil))
        {
            return None;
        }
        // Stencil work uses a separate pipeline and geometry store.
        let store = &self.pipeline.geometry_residency;
        let mut parts = HashMap::new();
        let keys = collect_parts(&self.runtime, frame, &mut parts, |key| {
            store.find(key).is_some()
        })?;
        if !store.admits_streaming(parts.values().map(|part| part.needs_upload), parts.len()) {
            return None;
        }
        choose_parts(&mut parts, budget)?;
        let include = keys.iter().map(|key| parts[key].admitted);
        let drawn = include.clone().filter(|selected| *selected).count();
        let visible = frame.draws.len();
        Some(StreamingFrame {
            frame: if drawn == visible {
                Cow::Borrowed(frame)
            } else {
                Cow::Owned(partial_frame(frame, include, drawn))
            },
            progress: RenderAssemblyStreamingProgress {
                drawn,
                visible,
                uploaded_bytes: 0,
                budget_bytes: budget,
            },
        })
    }
}

fn admits_frame(runtime: &VulkanRuntime, frame: &RenderFrame) -> bool {
    runtime.triangle_coverage == crate::RenderTriangleCoverageBehavior::HardwareV1
        && runtime.projection_behavior == crate::RenderProjectionBehavior::PrecomposedV1
        && runtime.mesh_vertex_behavior == crate::RenderMeshVertexBehavior::Float32V1
        && !runtime.meshlets_requested
        && !frame.uses_mesh_lod
        && frame.geometry_batch.is_none()
        && frame.raytrace.is_none()
        && frame.shadow_maps.is_none()
        && frame.gui_draw_start.is_none()
        && frame.shader_grid.is_none()
        && frame.stereo_pair.is_none()
        && frame.subview_blits.is_empty()
        && frame.immediate_main.is_none()
        && !frame.raster.fixed_function
        && frame.raster.transparency == crate::RenderTransparency::Unordered
        && frame
            .layers
            .iter()
            .all(|layer| layer.stencil_boundaries.is_empty())
}

fn collect_parts(
    runtime: &VulkanRuntime,
    frame: &RenderFrame,
    parts: &mut HashMap<GeometryKey, Part>,
    mut resident: impl FnMut(GeometryKey) -> bool,
) -> Option<Vec<GeometryKey>> {
    frame
        .draws
        .iter()
        .enumerate()
        .map(|(index, draw)| {
            if !admits_draw(draw) {
                return None;
            }
            let key = geometry_key(runtime, draw, false)?;
            let distance = focus_distance(draw);
            let part = parts.entry(key).or_insert_with(|| Part {
                needs_upload: if resident(key) { 0 } else { stream_bytes(draw) },
                focus_distance: distance,
                first_draw: index,
                admitted: false,
            });
            part.focus_distance = part.focus_distance.min(distance);
            Some(key)
        })
        .collect()
}

fn admits_draw(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Triangles
        && draw.instances.is_none()
        && draw.clipping.is_none()
        && draw.mesh_lod.is_none()
        && draw.depth_pass == RenderDepthPass::Shaded
        && draw.capping_pass == crate::RenderCappingPass::None
        && !draw.stencil_rect
        && !draw.is_surface_blended()
        && !draw.annotation
        && draw.textured_style.is_none()
        && draw.user_program.is_none()
        && draw.mesh_edges.is_none()
        && !draw.polygon_face_lines
        && !draw.polygon_points
        && !super::fixed_vertex_sources::uses_fixed_flat_vertices(draw)
        && draw.quad_corners.is_empty()
        && draw.vertex_revision().is_none()
        && draw.index_revision().is_none()
        && draw.combined_revision().is_none()
}

fn stream_bytes(draw: &PrimitiveDraw) -> u64 {
    // A four-byte index is also an upper bound for an even-padded 16-bit triangle stream.
    vertex_stream_bytes(draw).saturating_add(4 * draw.indices.len() as u64)
}

fn focus_distance(draw: &PrimitiveDraw) -> f32 {
    let Some(vertex) = draw.vertices.first() else {
        return f32::INFINITY;
    };
    let [x, y, z] = vertex.position;
    let clip = |row| {
        let m = &draw.clip_transform;
        m[row] * x + m[4 + row] * y + m[8 + row] * z + m[12 + row] * vertex.weight
    };
    let w = clip(3);
    if w <= 0.0 {
        return f32::INFINITY;
    }
    let [x, y] = [clip(0) / w, clip(1) / w];
    let distance = x * x + y * y;
    if distance.is_finite() {
        distance
    } else {
        f32::INFINITY
    }
}

fn choose_parts(parts: &mut HashMap<GeometryKey, Part>, mut remaining: u64) -> Option<()> {
    if parts.values().any(|part| part.needs_upload > remaining) {
        return None;
    }
    let mut ranked: Vec<_> = parts.values_mut().collect();
    ranked.sort_unstable_by(|a, b| {
        a.focus_distance
            .total_cmp(&b.focus_distance)
            .then(a.first_draw.cmp(&b.first_draw))
    });
    for part in ranked {
        if let Some(unspent) = remaining.checked_sub(part.needs_upload) {
            part.admitted = true;
            remaining = unspent;
        }
    }
    Some(())
}

fn partial_frame(
    frame: &RenderFrame,
    include: impl Iterator<Item = bool>,
    drawn: usize,
) -> RenderFrame {
    let mut counts = Vec::with_capacity(frame.draws.len() + 1);
    let mut draws = Vec::with_capacity(drawn);
    counts.push(0);
    for (draw, selected) in frame.draws.iter().zip(include) {
        if selected {
            draws.push(draw.clone());
        }
        counts.push(draws.len());
    }
    let mut reduced = frame.clone();
    reduced.draws = draws;
    for layer in &mut reduced.layers {
        layer.draw_range = counts[layer.draw_range.start]..counts[layer.draw_range.end];
        layer.placement_sources = Default::default();
    }
    reduced
}

#[cfg(test)]
#[path = "assembly_streaming_frame_tests.rs"]
mod tests;
