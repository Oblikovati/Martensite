//! Flat backgrounds under the fixed-function option (`ffpEnable`). The context's
//! `GlContext::ToRenderSRGB` is off there, so every background writes encoded bytes into a
//! non-sRGB target:
//! - `createGradientArray` encodes the corner colours with `Vec4FromQuantityColor` (linear to
//!   sRGB) and the rasterizer interpolates the encoded values; the elliptical fan keeps its
//!   linear colours;
//! - the corner gradient's colored-quad program receives linear `uColor1/uColor2` and writes its
//!   mix unencoded;
//! - the image quad is an ordinary fixed-function texture: its texels are not decoded and
//!   `GL_MODULATE` multiplies them by the current colour, which `GlPrimitiveArray::Render`
//!   sets to the background aspect's interior colour (`glColor4fv`, unconverted). That aspect is
//!   a fresh `Graphic3dAspectFillArea3d` whose default interior colour is cyan.
use crate::RenderLayer;
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkservice::aspect::AspectGradientFillMethod;
use martensite_tkservice::graphic3d::{Graphic3dAspects, Graphic3dAspectsMembers as _};

/// Encode a vertex-colour gradient layer of `method` for the fixed-function target; elliptical
/// colours stay linear. Example: `encode_fixed_gradient(&mut layer, method)`.
pub(crate) fn encode_fixed_gradient(layer: &mut RenderLayer, method: AspectGradientFillMethod) {
    for draw in &mut layer.draws {
        draw.compatibility_color_target = true;
        if method == AspectGradientFillMethod::Elliptical {
            continue;
        }
        for vertex in &mut draw.vertices {
            vertex.color = ColorRgba::linear_to_srgb_vec4(vertex.color);
        }
    }
}

/// Draw the image quad as a fixed-function modulated texture of the default aspect colour.
/// Example: `encode_fixed_image(&mut layer)`.
pub(crate) fn encode_fixed_image(layer: &mut RenderLayer) {
    let color = default_interior_color();
    for draw in &mut layer.draws {
        for vertex in &mut draw.vertices {
            vertex.color = color;
        }
        if let Some(style) = draw.surface_texture.as_mut() {
            style.fixed_environment = true;
        }
    }
}

/// `Graphic3dAspects` default interior colour as raw RGBA.
fn default_interior_color() -> [f32; 4] {
    let color = Graphic3dAspects::default().interior_color_rgba();
    let rgb = color.rgb();
    [
        rgb.red() as f32,
        rgb.green() as f32,
        rgb.blue() as f32,
        color.alpha(),
    ]
}

#[cfg(test)]
#[path = "fixed_background_tests.rs"]
mod tests;
