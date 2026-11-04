//! Corner gradient backgrounds (AspectGradientFillMethod Corner1..4): GlView draws them
//! with getColoredQuadProgram, a bilinear mix of the two colours that vertex colours of two
//! triangles cannot express, so the frame carries them as their own background draw.
use martensite_tkernel::quantity::Color;
use martensite_tkservice::aspect::AspectGradientFillMethod;

/// One frame's corner gradient: the GlBackgroundArray chart, uColor1/uColor2
/// (QuantityColor::Rgb, linear), the view size and the highlighted corner index.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ColoredQuadBackground {
    pub projection: [f32; 16],
    pub colors: [[f32; 3]; 2],
    pub view_size: [f32; 2],
    /// `type - AspectGradientFillMethod_Corner1` of createGradientArray's UV rotation.
    pub corner: u32,
    /// The fixed-function option writes the mix unencoded (no ToRenderSRGB): the quad renders
    /// through the attachment's UNORM alias.
    pub encoded: bool,
}

/// The UV rotation of a corner fill method, or `None` for the other methods;
/// `corner_index(AspectGradientFillMethod::Corner3) == Some(2)`.
pub(crate) fn corner_index(method: AspectGradientFillMethod) -> Option<u32> {
    use AspectGradientFillMethod::{Corner1, Corner2, Corner3, Corner4};
    match method {
        Corner1 => Some(0),
        Corner2 => Some(1),
        Corner3 => Some(2),
        Corner4 => Some(3),
        _ => None,
    }
}

/// The corner gradient of a view of `extent`; `colored_quad(method, colors, chart, [w, h])`.
pub(crate) fn colored_quad(
    method: AspectGradientFillMethod,
    colors: (Color, Color),
    projection: [f32; 16],
    extent: [u32; 2],
) -> Option<ColoredQuadBackground> {
    let rgb = |color: Color| [color.red(), color.green(), color.blue()].map(|value| value as f32);
    Some(ColoredQuadBackground {
        projection,
        colors: [rgb(colors.0), rgb(colors.1)],
        // createGradientArray: float(myViewWidth), float(myViewHeight).
        view_size: extent.map(|value| value as f32),
        corner: corner_index(method)?,
        encoded: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::quantity::NamedColor;

    #[test]
    fn corner_methods_rotate_the_highlighted_corner_and_others_have_none() {
        use AspectGradientFillMethod::*;
        let corners = [Corner1, Corner2, Corner3, Corner4].map(corner_index);
        assert_eq!(corners, [Some(0), Some(1), Some(2), Some(3)]);
        for method in [None, Horizontal, Vertical, Diagonal1, Diagonal2, Elliptical] {
            assert_eq!(corner_index(method), Option::None);
        }
    }

    #[test]
    fn the_quad_keeps_linear_colors_and_the_view_size() {
        let red = Color::named(NamedColor::RED);
        let blue = Color::named(NamedColor::BLUE);
        let method = AspectGradientFillMethod::Corner2;
        let quad = colored_quad(method, (red, blue), [0.; 16], [320, 240]).unwrap();
        assert_eq!(quad.colors, [[1., 0., 0.], [0., 0., 1.]]);
        assert_eq!((quad.view_size, quad.corner), ([320., 240.], 1));
        let vertical = AspectGradientFillMethod::Vertical;
        assert!(colored_quad(vertical, (red, blue), [0.; 16], [320, 240]).is_none());
    }
}
