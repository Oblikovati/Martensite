//! Native text decorations precede the foreground, as GlText.
use crate::text_layout::GlyphLayoutView;
use crate::text_transform::TextTransform;
use crate::{PrimitiveDraw, RenderError};
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkservice::{aspect::*, graphic3d::*};
/// Lower native decoration order using per-pass state; `lower_text_draws(text, aspect, layout, transform)` shares one atlas.
pub(crate) fn lower_text_draws<'a>(
    text: &Graphic3dText,
    aspect: &dyn Graphic3dAspectsSource,
    layout: impl Into<GlyphLayoutView<'a>>,
    transform: &TextTransform,
) -> Result<Vec<PrimitiveDraw>, RenderError> {
    let layout = layout.into();
    let offsets = native_text_offsets(aspect.text_display_type());
    let clips = transform.pass_clips(text, aspect, layout.point_size(), offsets)?;
    let last = clips.len() - 1;
    let mut draws = clips
        .iter()
        .enumerate()
        .map(|(pass, clip)| native_text_pass(layout, aspect, *clip, pass, last))
        .collect::<Result<Vec<_>, _>>()?;
    if aspect.text_display_type() == AspectTypeOfDisplayText::Dimension {
        draws.push(crate::text_background::dimension_rectangle(
            aspect,
            layout.formatter_bounds(),
            clips[last],
        )?);
    }
    Ok(draws)
}
/// Place draws that `lower_text_draws` built with other clip transforms: pass `i` takes
/// `clips[i]` and a dimension rectangle the last one; anything else returns `false`.
/// `place_text_draws(&mut kept, &clips)`
pub(crate) fn place_text_draws(draws: &mut [PrimitiveDraw], clips: &[[f32; 16]]) -> bool {
    let Some(last) = clips.last() else {
        return false;
    };
    if !(draws.len() == clips.len() || draws.len() == clips.len() + 1)
        || clips.iter().flatten().any(|value| !value.is_finite())
    {
        return false;
    }
    for (index, draw) in draws.iter_mut().enumerate() {
        draw.clip_transform = *clips.get(index).unwrap_or(last);
    }
    true
}
// GlText::render at preserves this exact outline order before the foreground.
#[cfg(test)]
const DECAL_PIXEL_OFFSETS: [[f64; 2]; 4] = [[1., 1.], [-1., -1.], [-1., 1.], [1., -1.]];
/// The pixel offset of each pass of a display type, in drawing order;
/// `native_text_offsets(AspectTypeOfDisplayText::Shadow).len() == 2`.
pub(crate) fn native_text_offsets(display: AspectTypeOfDisplayText) -> &'static [[f64; 2]] {
    match display {
        AspectTypeOfDisplayText::Dekale => &[[1., 1.], [-1., -1.], [-1., 1.], [1., -1.], [0., 0.]],
        AspectTypeOfDisplayText::Shadow => &[[1., -1.], [0., 0.]],
        AspectTypeOfDisplayText::Subtitle => &[[0., 0.], [0., 0.]],
        _ => &[[0., 0.]],
    }
}
fn native_text_pass(
    layout: GlyphLayoutView<'_>,
    aspect: &dyn Graphic3dAspectsSource,
    clip: [f32; 16],
    pass: usize,
    last: usize,
) -> Result<PrimitiveDraw, RenderError> {
    if aspect.text_display_type() == AspectTypeOfDisplayText::Subtitle && pass == 0 {
        return crate::text_background::subtitle_rectangle(aspect, layout.formatter_bounds(), clip);
    }
    let color = if pass == last {
        aspect.interior_color_rgba()
    } else {
        aspect.edge_color_rgba()
    };
    let mut draw = glyph_draw(layout, aspect, clip, color)?;
    draw.text_shadow_bias = pass != last && !draw.annotation;
    draw.text_logic_xor = aspect.text_display_type() == AspectTypeOfDisplayText::Blend;
    Ok(draw)
}
fn glyph_draw(
    layout: GlyphLayoutView<'_>,
    aspect: &dyn Graphic3dAspectsSource,
    clip: [f32; 16],
    color: ColorRgba,
) -> Result<PrimitiveDraw, RenderError> {
    let mut draw = layout.draw(text_vertex_rgba(color), clip)?;
    if let Some(style) = &mut draw.textured_style {
        style.blend = aspect.alpha_mode() == Graphic3dAlphaMode::MaskBlend;
    }
    let (mode, factor, units) = aspect.polygon_offsets();
    draw = draw.with_polygon_offset(crate::RenderPolygonOffset::new(mode, factor, units)?);
    draw.annotation = aspect.text_style() == AspectTypeOfStyleText::Annotation;
    Ok(draw)
}

/// Retain linear RGB and independent alpha; `text_vertex_rgba(ColorRgba::default())` feeds native text colors.
pub(crate) fn text_vertex_rgba(color: ColorRgba) -> [f32; 4] {
    let rgb = color.rgb();
    [
        rgb.red() as f32,
        rgb.green() as f32,
        rgb.blue() as f32,
        color.alpha(),
    ]
}
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::GlyphLayout;
    use martensite_tkernel::ncollection::Mat4;
    #[test]
    fn annotation_shadow_has_no_temporary_depth_bias_and_retains_subtitle_alpha() {
        let aspect = Graphic3dAspectText3d::default();
        aspect.set_text_display_type(AspectTypeOfDisplayText::Shadow);
        aspect.set_text_style(AspectTypeOfStyleText::Annotation);
        aspect.set_edge_color_rgba(ColorRgba::new(1., 0., 0., 0.5).unwrap());
        let text = Graphic3dText::new(16.);
        let layout = minimal_glyph_layout();
        let transform = identity_text_transform();
        let draws = lower_text_draws(&text, &aspect, &layout, &transform).unwrap();
        assert!(
            draws
                .iter()
                .all(|d| d.annotation && !d.text_shadow_bias && !d.surface_blend)
        );
        assert_eq!(draws[0].vertices()[0].color, [1., 0., 0., 0.5]);
        aspect.set_text_style(AspectTypeOfStyleText::Normal);
        let draws = lower_text_draws(&text, &aspect, &layout, &transform).unwrap();
        assert!(draws[0].text_shadow_bias);
        assert!(!draws[1].text_shadow_bias);
    }
    #[test]
    fn placed_draws_take_the_clips_lowering_would_give_them() {
        let aspect = Graphic3dAspectText3d::default();
        aspect.set_text_display_type(AspectTypeOfDisplayText::Dimension);
        let text = Graphic3dText::new(16.);
        let layout = minimal_glyph_layout();
        let moved = |shift: f32| {
            let mut transform = identity_text_transform();
            transform.model_view.set_value(0, 3, f64::from(shift));
            transform
        };
        let mut kept = lower_text_draws(&text, &aspect, &layout, &moved(0.)).unwrap();
        let expected = lower_text_draws(&text, &aspect, &layout, &moved(0.5)).unwrap();
        let offsets = native_text_offsets(aspect.text_display_type());
        let clips = moved(0.5).pass_clips(&text, &aspect, 16, offsets).unwrap();
        assert!(place_text_draws(&mut kept, &clips));
        assert!(
            kept == expected,
            "the dimension rectangle takes the last clip"
        );
        assert!(!place_text_draws(&mut kept, &[[f32::NAN; 16]]));
    }
    pub(crate) fn minimal_glyph_layout() -> GlyphLayout {
        GlyphLayout {
            positions: vec![[0.; 2]; 3],
            uv: vec![[0.; 2]; 3],
            indices: vec![0, 1, 2],
            atlas: crate::RasterImage::new([1, 1], vec![[1.; 4]]).unwrap(),
            formatter_bounds: martensite_tkservice::font::FontRect::default(),
            point_size: 16,
        }
    }
    pub(crate) fn identity_text_transform() -> TextTransform {
        TextTransform {
            projection: Mat4::identity(),
            model_view: Mat4::identity(),
            model_world: Mat4::identity(),
            camera_rotation: Mat4::identity(),
            viewport: [0, 0, 64, 64],
            pixel_coordinates: false,
            zoomable_scale: 1.,
            unview: Default::default(),
        }
    }
}

#[cfg(test)]
#[path = "text_decal_tests.rs"]
mod decal_tests;

#[cfg(test)]
#[path = "text_xor_tests.rs"]
mod xor_tests;
