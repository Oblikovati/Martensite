//! Surface-only lighting and culling do not reject native line presentations.
use crate::{LineStyle, RenderError, error::rejected};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
pub(crate) fn primitive_style(
    kind: Graphic3dTypeOfPrimitiveArray,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    closed: bool,
) -> Result<([f32; 4], Option<LineStyle>), RenderError> {
    if kind == Graphic3dTypeOfPrimitiveArray::Points {
        let style = aspects
            .get()
            .ok_or_else(|| rejected("null point aspects", "explicit Graphic3d aspects"))?;
        validate_line_shading(style, true)?;
        return Ok((rgba_color(style), None));
    }
    if !is_line_kind(kind) {
        return Ok((crate::style::surface_color(aspects, closed)?, None));
    }
    let style = aspects
        .get()
        .ok_or_else(|| rejected("null line aspects", "explicit Graphic3d aspects"))?;
    validate_line_shading(style, true)?;
    if is_adjacency_line(kind) {
        return adjacency_line_parameters(style);
    }
    line_parameters(style)
}

/// Arrays GlPrimitiveArray rasterizes as lines; `is_line_kind(Kind::Segments)`.
fn is_line_kind(kind: Graphic3dTypeOfPrimitiveArray) -> bool {
    matches!(
        kind,
        Graphic3dTypeOfPrimitiveArray::Segments
            | Graphic3dTypeOfPrimitiveArray::Polylines
            | Graphic3dTypeOfPrimitiveArray::LinesAdjacency
            | Graphic3dTypeOfPrimitiveArray::LineStripAdjacency
    )
}
fn is_adjacency_line(kind: Graphic3dTypeOfPrimitiveArray) -> bool {
    matches!(
        kind,
        Graphic3dTypeOfPrimitiveArray::LinesAdjacency
            | Graphic3dTypeOfPrimitiveArray::LineStripAdjacency
    )
}

/// Line adjacency binds the face program (GlPrimitiveArray::Render's default branch),
/// which has no stipple, and sets neither the aspect's pattern nor its width: the segments are
/// solid at the GL line width the frame last set, else the width GlWorkspace::Activate
/// resets each frame to (the default aspect's 1). Witness: the "-after-wide-segments" cases of
/// the recorded adjacency_arrays capture.
fn adjacency_line_parameters(
    style: &dyn Graphic3dAspectsSource,
) -> Result<([f32; 4], Option<LineStyle>), RenderError> {
    let line = LineStyle::new(1., 0xffff, 1)?
        .with_context_width()
        .with_alpha_blending(style.alpha_mode() == Graphic3dAlphaMode::MaskBlend);
    Ok((rgba_color(style), Some(line)))
}

pub(crate) fn primitive_style_in_pipeline(
    kind: Graphic3dTypeOfPrimitiveArray,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    closed: bool,
    pipeline: crate::RenderLightingPipeline,
    unlit: crate::RenderUnlitFaceColorPolicy,
) -> Result<([f32; 4], Option<LineStyle>), RenderError> {
    let nodal = kind == Graphic3dTypeOfPrimitiveArray::Points || is_line_kind(kind);
    if !nodal {
        return Ok((
            crate::style::surface_color_with_policies(aspects, pipeline, unlit)?,
            None,
        ));
    }
    primitive_style(kind, aspects, closed)
}

fn line_parameters(
    style: &dyn Graphic3dAspectsSource,
) -> Result<([f32; 4], Option<LineStyle>), RenderError> {
    let rgba = rgba_color(style);
    let line = LineStyle::new(
        style.line_width(),
        style.line_pattern(),
        style.line_stipple_factor(),
    )?
    .with_alpha_blending(style.alpha_mode() == Graphic3dAlphaMode::MaskBlend);
    Ok((rgba, Some(line)))
}

fn validate_line_shading(
    style: &dyn Graphic3dAspectsSource,
    ignores_surface_sampler: bool,
) -> Result<(), RenderError> {
    if let Some(program) = style.shader_program().get() {
        return Err(crate::user_program::refuse_user_program(program));
    }
    if style.to_map_texture() && !ignores_surface_sampler {
        return Err(rejected(
            (style.to_map_texture(), !style.shader_program().is_null()),
            "built-in line shader without sampling surface textures; unmapped point aspect",
        ));
    }
    Ok(())
}

fn rgba_color(style: &dyn Graphic3dAspectsSource) -> [f32; 4] {
    let color = style.interior_color_rgba();
    [
        color.rgb().red() as f32,
        color.rgb().green() as f32,
        color.rgb().blue() as f32,
        color.alpha(),
    ]
}
