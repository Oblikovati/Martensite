//! Private upload geometry for one closed operation; semantic draws are never mutated.
mod cells;
mod project;
use crate::error::rejected;
use crate::{PrimitiveDraw, PrimitiveTopology, RenderError, RenderThinAxisCoverage};
use project::AxisProjection;

pub(crate) fn selected_draw(
    behavior: RenderThinAxisCoverage,
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    single_sample: bool,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    let RenderThinAxisCoverage::FixedGridV2 { grid, origin } = behavior else {
        return Ok(None);
    };
    if !applicable(draw, single_sample) {
        return Ok(None);
    }
    validate_extent(extent)?;
    selected_axis_geometry(draw, extent, (grid, origin))
}
fn selected_axis_geometry(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    (grid, origin): (crate::RenderLineGrid, crate::RenderLineOrigin),
) -> Result<Option<PrimitiveDraw>, RenderError> {
    let projections = project_segments(draw, extent)?;
    if projections.iter().any(|p| p.axis().is_none()) {
        return Ok(None);
    }
    validate_styles(draw)?;
    for projection in &projections {
        projection.validate_clip()?;
    }
    expand_axis_draw(draw, &projections, extent, (grid, origin)).map(Some)
}
fn project_segments(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
) -> Result<Vec<AxisProjection>, RenderError> {
    draw.indices
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| AxisProjection::new(draw, [p[0], p[1]], extent))
        .collect()
}
#[derive(Default)]
struct AxisMesh {
    vertices: Vec<crate::RenderVertex>,
    indices: Vec<u32>,
}
fn expand_axis_draw(
    draw: &PrimitiveDraw,
    projections: &[AxisProjection],
    extent: [u32; 2],
    coverage: (crate::RenderLineGrid, crate::RenderLineOrigin),
) -> Result<PrimitiveDraw, RenderError> {
    let mut mesh = AxisMesh::default();
    for &projection in projections {
        append_quad(projection, extent, coverage, &mut mesh)?;
    }
    Ok(replacement(draw, mesh.vertices, mesh.indices))
}

fn applicable(draw: &PrimitiveDraw, single_sample: bool) -> bool {
    single_sample
        && draw.topology() == PrimitiveTopology::Lines
        && draw
            .line_style
            .is_some_and(|s| s.raster_width() == 1. && s.pattern == 0xffff)
}
fn validate_extent(extent: [u32; 2]) -> Result<(), RenderError> {
    if extent.iter().any(|&v| v == 0 || v > 32768) {
        return Err(rejected(
            extent,
            "thin-axis viewport dimensions in 1..=32768",
        ));
    }
    Ok(())
}
fn validate_styles(draw: &PrimitiveDraw) -> Result<(), RenderError> {
    if unsupported_style(draw) {
        return Err(rejected(
            draw,
            "unlit built-in independent thin-axis lines without world clipping, flat sources, blending, texture or hatch",
        ));
    }

    Ok(())
}
fn unsupported_style(draw: &PrimitiveDraw) -> bool {
    unsupported_shader(draw)
        || unsupported_surface(draw)
        || draw.clipping.is_some()
        || !draw.native_flat_sources.is_empty()
        || draw.mesh_edges.is_some()
        || draw.hatch_rows.is_some()
        || draw.face_culling != crate::RenderFaceCulling::None
}
fn unsupported_shader(draw: &PrimitiveDraw) -> bool {
    draw.user_program.is_some()
        || draw.lighting.is_some()
        || draw.fixed_function_lighting
        || draw.native_material_cutoff.is_some()
        || draw.unlit_back_color.is_some()
        || draw.polygon_face_lines
        || draw.polygon_points
        || draw.polygon_edge_style.is_some()
}
fn unsupported_surface(draw: &PrimitiveDraw) -> bool {
    draw.textured_style.is_some()
        || draw.surface_texture.is_some()
        || draw.surface_texture_unavailable.is_some()
        || draw.surface_cutoff.is_some()
        || draw.surface_blend
        || draw.surface_forced_blend
        || draw.line_style.is_some_and(|s| s.alpha_blend)
}

fn append_quad(
    projection: AxisProjection,
    extent: [u32; 2],
    (grid, origin): (crate::RenderLineGrid, crate::RenderLineOrigin),
    mesh: &mut AxisMesh,
) -> Result<(), RenderError> {
    if projection.window[0] == projection.window[1] {
        return Ok(());
    }
    let axis = projection.axis().expect("admitted axis line");
    let Some(bounds) = cells::window_bounds(projection.window, extent, grid, origin, axis) else {
        return Ok(());
    };
    let base = quad_base(mesh.vertices.len())?;
    mesh.vertices
        .extend(projection.quad_vertices(bounds, axis)?);
    mesh.indices.extend([0, 1, 2, 0, 2, 3].map(|i| base + i));
    Ok(())
}

fn quad_base(count: usize) -> Result<u32, RenderError> {
    u32::try_from(count)
        .ok()
        .filter(|&v| v <= u32::MAX - 4)
        .ok_or_else(|| rejected(count, "thin-axis quad vertex count below u32::MAX - 4"))
}

fn replacement(
    draw: &PrimitiveDraw,
    vertices: Vec<crate::RenderVertex>,
    indices: Vec<u32>,
) -> PrimitiveDraw {
    let mut selected = draw.with_replacement_geometry(vertices, indices);
    selected.clip_transform = crate::identity_clip();
    selected.line_style = None;
    selected.source_line_strip = false;
    selected.quad_corners.clear();
    selected
}
#[cfg(test)]
mod coverage_tests;
#[cfg(test)]
mod scene_tests;
#[cfg(test)]
mod tests;
