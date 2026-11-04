//! Selected-domain refusals happen before encoding, allocating or recording a draw.
use crate::{
    PrimitiveDraw, PrimitiveTopology, RenderError, RenderPhongInterpolationBehavior as Behavior,
    RenderPhongInterpolationFailure as Failure, RenderShading,
};
use ash::vk;
#[cfg(test)]
pub(super) fn admit(
    behavior: Behavior,
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
) -> Result<(), RenderError> {
    admit_with_world_clip(behavior, draw, viewport, false)
}
pub(super) fn admit_with_world_clip(
    behavior: Behavior,
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
    grid12_world_clip: bool,
) -> Result<(), RenderError> {
    if behavior == Behavior::HardwareV1 || !selected_phong(draw) {
        return Ok(());
    }
    admit_interface(draw, grid12_world_clip)?;
    admit_triangles(behavior, draw, viewport)
}
/// Phong admission where V4 owns coverage: its 12-bit weights never divide by the unsnapped area,
/// so that numeric domain does not apply. `admit_for_coverage(b, d, v, false, true)`.
pub(super) fn admit_for_coverage(
    behavior: Behavior,
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
    grid12_world_clip: bool,
    native_grid_coverage: bool,
) -> Result<(), RenderError> {
    if native_grid_coverage && behavior != Behavior::HardwareV1 && selected_phong(draw) {
        return admit_interface(draw, grid12_world_clip);
    }
    admit_with_world_clip(behavior, draw, viewport, grid12_world_clip)
}
fn admit_triangles(
    behavior: Behavior,
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
) -> Result<(), RenderError> {
    let clips: Vec<_> = draw
        .vertices
        .iter()
        .map(|v| clip_position(draw.clip_transform, *v))
        .collect();
    for (triangle, indices) in draw.indices.as_chunks::<3>().0.iter().enumerate() {
        let clip = std::array::from_fn(|i| clips[indices[i] as usize]);
        // A private zero-area carrier covers no sample, so no interpolation can read it.
        if indices[0] == indices[1] && indices[1] == indices[2] {
            continue;
        }
        if !admitted_clip(behavior, clip, viewport) {
            return Err(Failure::InvalidTriangle {
                behavior,
                triangle,
                clip_bits: clip.map(|v| v.map(f32::to_bits)),
            }
            .into());
        }
    }
    Ok(())
}
pub(super) fn require_interface(
    behavior: Behavior,
    draw: &PrimitiveDraw,
    registered: bool,
) -> Result<(), RenderError> {
    if behavior != Behavior::HardwareV1 && selected_phong(draw) && !registered {
        return Err(Failure::UnsupportedInterface {
            actual: "selected unregistered triangle pipeline".into(),
        }
        .into());
    }
    Ok(())
}
fn selected_phong(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Triangles
        && draw
            .lighting
            .as_ref()
            .is_some_and(|l| !l.is_pbr() && l.shading() == RenderShading::Phong)
        // A draw outside the registered interface stays on hardware interpolation; clipping keeps
        // its own admission in `admit_interface`.
        && unsupported_interface(draw, true).is_none()
}
fn admit_interface(draw: &PrimitiveDraw, grid12_world_clip: bool) -> Result<(), RenderError> {
    let Some(actual) = unsupported_interface(draw, grid12_world_clip) else {
        return Ok(());
    };
    Err(Failure::UnsupportedInterface {
        actual: format!("{actual}=true"),
    }
    .into())
}
fn unsupported_interface(draw: &PrimitiveDraw, grid12_world_clip: bool) -> Option<&'static str> {
    [
        ("fixed lighting", draw.fixed_function_lighting),
        ("custom program", draw.user_program.is_some()),
        ("clipping", draw.clipping.is_some() && !grid12_world_clip),
        ("surface texture", draw.surface_texture.is_some()),
        ("mesh edges", draw.mesh_edges.is_some()),
        ("polygon points", draw.polygon_points),
        ("polygon lines", draw.polygon_face_lines),
        ("polygon edge style", draw.polygon_edge_style.is_some()),
        ("textured style", draw.textured_style.is_some()),
        ("alpha blending", draw.surface_blend),
        ("alpha cutoff", draw.surface_cutoff.is_some()),
        ("shadow-casting lights", shadow_casting(draw)),
    ]
    .into_iter()
    .find_map(|(name, present)| present.then_some(name))
}
fn shadow_casting(draw: &PrimitiveDraw) -> bool {
    draw.lighting
        .as_ref()
        .is_some_and(|l| l.lights.sources.iter().any(|source| source.cast_shadows))
}
pub(super) fn clip_position(matrix: [f32; 16], vertex: crate::RenderVertex) -> [f32; 4] {
    // Domain screening follows the existing source's ordered products; it does not replace the VS.
    std::array::from_fn(|i| {
        ((matrix[4 + i] * vertex.position[1] + matrix[i] * vertex.position[0])
            + matrix[8 + i] * vertex.position[2])
            + matrix[12 + i] * vertex.weight
    })
}
pub(super) fn viewport(
    coordinates: crate::RenderCoordinateConvention,
    extent: vk::Extent2D,
) -> [f32; 4] {
    let full = vk::Rect2D {
        offset: Default::default(),
        extent,
    };
    let v = coordinates.raster_regions(extent, full, full).viewport;
    [v.x, v.y, v.width, v.height]
}
#[cfg(test)]
#[path = "phong_interpolation_upload_tests.rs"]
mod tests;

fn admitted_clip(behavior: Behavior, clip: [[f32; 4]; 3], viewport: [f32; 4]) -> bool {
    match behavior {
        Behavior::UnsnappedV3 => {
            super::phong_interpolation_math::admitted_triangle_v3(clip, viewport)
        }
        _ => super::phong_interpolation_math::admitted_triangle(clip, viewport),
    }
}

#[cfg(test)]
#[test]
fn native_grid_coverage_does_not_apply_the_unsnapped_area_domain() {
    let mut draw = tests::phong_triangle_for_coverage();
    draw.vertices[1] = draw.vertices[0];
    assert!(admit(Behavior::UnsnappedV2, &draw, [0., 0., 4., 4.]).is_err());
    assert!(
        admit_for_coverage(Behavior::UnsnappedV2, &draw, [0., 0., 4., 4.], false, true).is_ok()
    );
}
