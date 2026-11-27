//! GlWorkspace::ApplyAspects text policy.
use crate::{PrimitiveDraw, face_culling::resolve_culling};
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*};

/// Retain aspect culling on every glyph pass; `apply_text_face_culling(&mut draws, style, false)`.
pub(crate) fn apply_text_face_culling(
    draws: &mut [PrimitiveDraw],
    aspects: &dyn Graphic3dAspectsSource,
    closed: bool,
) {
    let culling = text_face_culling(aspects, closed);
    for draw in draws {
        draw.face_culling = culling;
    }
}
/// The face culling of text drawn with `aspects` in a closed or open group;
/// `text_face_culling(&aspects, closed)`.
pub(crate) fn text_face_culling(
    aspects: &dyn Graphic3dAspectsSource,
    closed: bool,
) -> crate::RenderFaceCulling {
    let automatic = closed && text_allows_automatic_culling(aspects);
    resolve_culling(aspects.face_culling(), automatic)
}

// Font coverage is blended independently, but native automatic culling still uses aspect alpha mode.
fn text_allows_automatic_culling(aspects: &dyn Graphic3dAspectsSource) -> bool {
    use Graphic3dAlphaMode as Alpha;
    if aspects.interior_style() == AspectInteriorStyle::Hatch {
        return false;
    }
    match aspects.alpha_mode() {
        Alpha::Blend | Alpha::Mask | Alpha::MaskBlend => false,
        Alpha::BlendAuto => aspects.front_material().transparency() == 0.,
        _ => true,
    }
}
