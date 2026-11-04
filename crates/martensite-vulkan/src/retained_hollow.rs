//! Hollow (empty) interiors, as GlPrimitiveArray::Render resolves them.
//!
//! An empty interior draws no fill. With edges requested in a non-solid line type, the draw
//! switches to polygon-mode edges and skips the filled array (`toDrawArray = false`), so only
//! the edge draws remain. A hollow interior with solid edges draws the triangles through the
//! GLSL mesh-edge program: `PushInteriorState` sets `occWireframeColor` to -1 (edges keep the
//! interior colour, alpha `1 - mix`) and `GlContext::SetShadingMaterial` the 0.285 alpha
//! cutoff. Solid edges in polygon mode (no geometry stage, or the polygon-mode caps) draw the
//! array once with the face program in `GL_LINE` polygon mode (`toSetLinePolygMode`) and skip
//! `drawEdges`. Under `ffpEnable` the GLSL edges have no program to run in
//! (`GlShaderManager::BindFaceProgram` binds none), so solid edges leave the faces drawn
//! filled by the fixed-function pipeline, with the 0.285 alpha test of a hollow interior with
//! edges (witness the recorded hollow_solid_edges capture).
use crate::{LineStyle, PrimitiveDraw, RenderError, error::rejected};
use martensite_tkservice::{
    aspect::{AspectInteriorStyle, AspectTypeOfLine},
    graphic3d::*,
};

/// `Graphic3dAspects` default line width: the GL line width when no earlier draw of the frame
/// set one (`GlWorkspace::ResetAppliedAspect`). `resolve_ordered_line_widths` replaces it.
const DEFAULT_CONTEXT_LINE_WIDTH: f32 = 1.;

/// Drop the fill of a hollow interior, keeping its polygon-mode edges; runs after the raster
/// widths. Example: `apply_hollow_interior(&mut draws, &*aspect, &rendering)?`.
pub(crate) fn apply_hollow_interior(
    draws: &mut Vec<PrimitiveDraw>,
    aspect: &dyn Graphic3dAspectsSource,
    rendering: &Graphic3dRenderingParams,
) -> Result<(), RenderError> {
    if aspect.interior_style() != AspectInteriorStyle::Empty {
        return Ok(());
    }
    if !aspect.to_draw_edges() {
        draws.clear();
        return Ok(());
    }
    if draws.iter().any(|draw| draw.mesh_edges.is_some()) {
        hollow_mesh_edges(draws);
        return Ok(());
    }
    let polygon_edges = draws.iter().any(PrimitiveDraw::is_polygon_edge_pass);
    if !polygon_edges && aspect.line_type() == AspectTypeOfLine::Solid {
        fixed_function_hollow_faces(draws);
        return Ok(());
    }
    if !polygon_edges {
        return Err(rejected(
            (aspect.line_type(), polygon_edges),
            "a hollow interior with GLSL mesh edges or polygon-mode edges",
        ));
    }
    if aspect.line_type() == AspectTypeOfLine::Solid {
        let scale = crate::retained_mesh_edges::context_line_width_scale(rendering);
        return face_polygon_lines(draws, DEFAULT_CONTEXT_LINE_WIDTH * scale);
    }
    draws.retain(PrimitiveDraw::is_polygon_edge_pass);
    Ok(())
}

/// The GLSL mesh-edge path of a hollow interior: wireframe colour -1 and alpha cutoff 0.285.
fn hollow_mesh_edges(draws: &mut [PrimitiveDraw]) {
    for draw in draws {
        if let Some(edges) = draw.mesh_edges.as_mut() {
            edges.color = [-1.; 4];
            draw.surface_cutoff = Some(0.285);
        }
    }
}

/// The fixed-function faces of a hollow interior with solid edges: filled, no edges, and the
/// `GlContext::SetShadingMaterial` cutoff 0.285 (`GL_ALPHA_TEST` with `GL_GEQUAL`).
fn fixed_function_hollow_faces(draws: &mut [PrimitiveDraw]) {
    for draw in draws {
        draw.surface_cutoff = Some(0.285);
    }
}

/// Solid polygon-mode edges of a hollow interior: the faces in line polygon mode, no `drawEdges`.
/// Neither stipple nor width is set here, so the lines are solid at the context line width.
fn face_polygon_lines(draws: &mut Vec<PrimitiveDraw>, width: f32) -> Result<(), RenderError> {
    draws.retain(|draw| !draw.is_polygon_edge_pass());
    let style = LineStyle::new(width, 0xffff, 1)?.with_context_width();
    for draw in draws {
        draw.polygon_edge_style = Some(style);
        draw.polygon_face_lines = true;
    }
    Ok(())
}

/// Give each face-program line draw the GL line width the frame sets before it: the last
/// `GL_LINES` aspect width or `drawEdges` edge width (`GlContext::SetLineWidth` is sticky).
/// Context-width styles keep their default when no earlier draw set a width.
/// Example: `resolve_ordered_line_widths(&mut frame_draws)`.
pub(crate) fn resolve_ordered_line_widths(draws: &mut [PrimitiveDraw]) {
    let mut current = None;
    for draw in draws {
        for style in [&mut draw.line_style, &mut draw.polygon_edge_style]
            .into_iter()
            .flatten()
        {
            match (style.context_width, current) {
                (false, _) => current = Some(style.width),
                (true, Some(width)) => style.width = width,
                (true, None) => {}
            }
        }
    }
}

#[cfg(test)]
#[path = "retained_hollow_tests.rs"]
mod tests;
