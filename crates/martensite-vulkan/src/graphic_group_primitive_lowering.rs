//! Preserve retained primitive style and upload behavior before group policies apply.
use super::*;
pub(super) fn lower_styled_primitive(
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    closed: bool,
    matrix: [f32; 16],
    highlighted: bool,
    lighting_pipeline: crate::RenderLightingPipeline,
    unlit: crate::RenderUnlitFaceColorPolicy,
) -> Result<Vec<PrimitiveDraw>, RenderError> {
    let (color, line) =
        primitive_style_in_pipeline(primitive.kind, aspects, closed, lighting_pipeline, unlit)?;
    let (mode, factor, units) = aspects.polygon_offsets();
    let offset = crate::RenderPolygonOffset::new(mode, factor, units)?;
    if primitive.kind != Graphic3dTypeOfPrimitiveArray::Points {
        record_surface_upload(primitive, &**aspects);
        let culling = crate::face_culling::resolve_culling(
            aspects.face_culling(),
            closed && crate::face_culling::allow_surface_auto_culling(&**aspects),
        );
        return primitive.draws(color, matrix, line, None).map(|draws| {
            draws
                .into_iter()
                .map(|draw| {
                    surface_alpha_policy(
                        draw.with_face_culling(culling).with_polygon_offset(offset),
                        aspects,
                    )
                })
                .collect()
        });
    }
    if aspects.marker_type() == martensite_tkservice::aspect::AspectTypeOfMarker::Empty {
        return Ok(vec![]);
    }
    primitive.upload.record_primitive(primitive);
    let vertex_colors = primitive
        .attributes
        .attribute_data(Graphic3dTypeOfAttribute::Color)
        .is_some();
    let sprite = crate::point_style::marker_style(&**aspects, highlighted, vertex_colors)?;
    primitive
        .draws(color, matrix, line, Some(sprite))
        .map(|draws| {
            draws
                .into_iter()
                .map(|draw| surface_alpha_policy(draw.with_polygon_offset(offset), aspects))
                .collect()
        })
}

/// GlPrimitiveArray::Render builds buffers before drawing, except for empty line types.
fn record_surface_upload(primitive: &PrimitiveSubmission, aspects: &dyn Graphic3dAspectsSource) {
    let line = crate::element_stats::is_line_array(primitive.kind);
    if !line || aspects.line_type() != martensite_tkservice::aspect::AspectTypeOfLine::Empty {
        primitive.upload.record_primitive(primitive);
    }
}
pub(super) fn surface_alpha_policy(
    mut draw: PrimitiveDraw,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
) -> PrimitiveDraw {
    crate::surface_alpha::apply_surface_alpha(&mut draw, &**aspects);
    draw
}
