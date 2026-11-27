//! Fixed-function backgrounds: encoded gradient corners, linear elliptical colours and the
//! cyan-modulated image (hand-computed from createGradientArray and Graphic3dAspects).
use super::*;
use crate::{PrimitiveDraw, RenderVertex};

fn colour_layer(colors: [[f32; 4]; 3]) -> RenderLayer {
    let vertices = colors
        .iter()
        .map(|color| RenderVertex::new([0.; 3], *color))
        .collect();
    let draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap();
    RenderLayer::new(vec![draw], false, false, false)
}

const LINEAR: [[f32; 4]; 3] = [[0.9, 0.2, 0.1, 1.], [0.1, 0.3, 0.8, 1.], [0., 1., 0.5, 1.]];

#[test]
fn quad_gradients_encode_their_corner_colours_and_write_unencoded() {
    let mut layer = colour_layer(LINEAR);
    encode_fixed_gradient(&mut layer, AspectGradientFillMethod::Vertical);
    let draw = &layer.draws[0];
    assert!(draw.compatibility_color_target);
    let encoded = LINEAR.map(ColorRgba::linear_to_srgb_vec4);
    let colours: Vec<[f32; 4]> = draw.vertices.iter().map(|v| v.color).collect();
    assert_eq!(colours, encoded.to_vec());
    // 0.9 linear is 0.954 encoded (243 of 255), as the native FFP gradient's first row shows.
    assert_eq!((colours[0][0] * 255.).round(), 243.);
}

#[test]
fn elliptical_gradients_keep_their_linear_colours() {
    let mut layer = colour_layer(LINEAR);
    encode_fixed_gradient(&mut layer, AspectGradientFillMethod::Elliptical);
    let draw = &layer.draws[0];
    assert!(draw.compatibility_color_target);
    let colours: Vec<[f32; 4]> = draw.vertices.iter().map(|v| v.color).collect();
    assert_eq!(colours, LINEAR.to_vec());
}

#[test]
fn the_image_quad_is_modulated_by_the_default_cyan_interior_colour() {
    assert_eq!(default_interior_color(), [0., 1., 1., 1.]);
    let mut layer = colour_layer([[1.; 4]; 3]);
    encode_fixed_image(&mut layer);
    let colours: Vec<[f32; 4]> = layer.draws[0].vertices.iter().map(|v| v.color).collect();
    assert_eq!(colours, vec![[0., 1., 1., 1.]; 3]);
}
