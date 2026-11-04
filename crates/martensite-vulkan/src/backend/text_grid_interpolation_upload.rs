//! Explicit text-grid admission refuses unsupported interfaces before draw recording.
use crate::{
    PrimitiveDraw, PrimitiveTopology, RenderError, RenderTextInterpolationBehavior as Behavior,
    RenderTextInterpolationFailure as Failure,
};

pub(super) fn admit(
    behavior: Behavior,
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
) -> Result<(), RenderError> {
    if behavior == Behavior::HardwareV1 || draw.textured_style.is_none() {
        return Ok(());
    }
    admit_text_interface(draw)?;
    if behavior == Behavior::ProjectedGrid12FlatCornersV4 {
        super::text_flat_corners::admit(draw)?;
    }
    admit_text_triangles(draw, viewport)
}

fn admit_text_interface(draw: &PrimitiveDraw) -> Result<(), RenderError> {
    if draw.lighting.is_some()
        || draw.fixed_function_lighting
        || draw.surface_texture.is_some()
        || draw.surface_texture_unavailable.is_some()
        || draw.surface_cutoff.is_some()
        || draw.unlit_back_color.is_some()
        || draw.mesh_edges.is_some()
        || draw.polygon_edge_style.is_some()
        || draw.point_style.is_some()
        || draw.fixed_grid_point
        || draw.polygon_points
        || draw.surface_texture_binding_only
        || draw.polygon_face_lines
    {
        return Err(unsupported(
            "textured draw requests an interface outside the plain glyph-text pipeline".into(),
        ));
    }
    Ok(())
}

fn admit_text_triangles(draw: &PrimitiveDraw, viewport: [f32; 4]) -> Result<(), RenderError> {
    if draw.topology() != PrimitiveTopology::Triangles || !draw.indices.len().is_multiple_of(3) {
        return Err(unsupported(format!("topology {:?}", draw.topology())));
    }
    let clips = clipped_vertices(draw);
    let world = clipping_vertices(draw);
    for (triangle, indices) in draw.indices.as_chunks::<3>().0.iter().enumerate() {
        admit_triangle(triangle, indices, &clips, world.as_deref(), viewport)?;
    }
    Ok(())
}

fn clipping_vertices(draw: &PrimitiveDraw) -> Option<Vec<[f32; 4]>> {
    let clipping = draw.clipping.as_ref()?;
    Some(
        draw.vertices
            .iter()
            .map(|vertex| {
                super::phong_interpolation_upload::clip_position(
                    *clipping.world_from_object(),
                    *vertex,
                )
            })
            .collect(),
    )
}

fn clipped_vertices(draw: &PrimitiveDraw) -> Vec<[f32; 4]> {
    draw.vertices
        .iter()
        .map(|vertex| {
            super::phong_interpolation_upload::clip_position(draw.clip_transform, *vertex)
        })
        .collect()
}

fn admit_triangle(
    triangle: usize,
    indices: &[u32],
    clips: &[[f32; 4]],
    world: Option<&[[f32; 4]]>,
    viewport: [f32; 4],
) -> Result<(), RenderError> {
    let Some(clip) = triangle_clips(indices, clips) else {
        return Err(unsupported(format!(
            "triangle {triangle} indices {indices:?}"
        )));
    };
    if valid_world_triangle(indices, world) && valid_projected_triangle(clip, viewport) {
        return Ok(());
    }
    Err(Failure::InvalidTriangle {
        triangle,
        clip_bits: clip.map(|vertex| vertex.map(f32::to_bits)),
    }
    .into())
}

fn valid_world_triangle(indices: &[u32], world: Option<&[[f32; 4]]>) -> bool {
    let Some(world) = world else { return true };
    let Some(points) = triangle_clips(indices, world) else {
        return false;
    };
    points.iter().all(|point| supported_world_point(*point))
}

fn triangle_clips(indices: &[u32], clips: &[[f32; 4]]) -> Option<[[f32; 4]; 3]> {
    let [a, b, c] = indices else { return None };
    Some([
        *clips.get(*a as usize)?,
        *clips.get(*b as usize)?,
        *clips.get(*c as usize)?,
    ])
}

fn valid_projected_triangle(clip: [[f32; 4]; 3], viewport: [f32; 4]) -> bool {
    if !supported_viewport(viewport) {
        return false;
    }
    if clip.iter().any(|value| !supported_clip_w(value[3])) {
        return false;
    }
    let Some(points) = clip
        .map(|value| super::phong_interpolation_math::window_point(value, viewport))
        .into_iter()
        .collect::<Option<Vec<_>>>()
    else {
        return false;
    };
    let Some(snapped) = points
        .into_iter()
        .map(super::text_grid_interpolation_math::snap_window_point)
        .collect::<Option<Vec<_>>>()
    else {
        return false;
    };
    let Ok(points) = snapped.try_into() else {
        return false;
    };
    super::text_grid_interpolation_math::triangle_area(points).is_some()
}

fn supported_viewport(viewport: [f32; 4]) -> bool {
    viewport.iter().all(|value| value.is_finite()) && viewport[2] > 0.0 && viewport[3] != 0.0
}

fn supported_clip_w(value: f32) -> bool {
    value.is_finite() && value > 0.0 && (1.0 / value).is_normal()
}

fn supported_world_point(point: [f32; 4]) -> bool {
    if !point.iter().all(|value| value.is_finite()) || !supported_clip_w(point[3]) {
        return false;
    }
    let divided = [
        point[0] / point[3],
        point[1] / point[3],
        point[2] / point[3],
    ];
    divided.iter().all(|value| value.is_finite())
}

pub(super) fn require_interface(
    behavior: Behavior,
    draw: &PrimitiveDraw,
    registered: bool,
) -> Result<(), RenderError> {
    if behavior == Behavior::HardwareV1 || draw.textured_style.is_none() {
        return Ok(());
    }
    admit_text_interface(draw)?;
    if behavior == Behavior::ProjectedGrid12FlatCornersV4 {
        super::text_flat_corners::admit(draw)?;
    }
    if !registered {
        return Err(unsupported(
            "selected textured draw has no registered text-grid shader".into(),
        ));
    }
    Ok(())
}

fn unsupported(actual: String) -> RenderError {
    Failure::UnsupportedInterface { actual }.into()
}

#[cfg(test)]
#[path = "text_grid_interpolation_upload_tests.rs"]
mod tests;
