//! The glyph clip transform follows the native float matrix pipeline (bug22368).
use super::*;
use martensite_tkmath::gp::Point3;

/// The float projection of the bug22368 view (409x409, orthographic), promoted to double.
const BUG22368_PROJECTION: [f64; 16] = [
    0.0020000000949949026,
    0.,
    0.,
    0.,
    0.,
    0.0020000000949949026,
    0.,
    0.,
    0.,
    0.,
    -0.011346877552568913,
    0.,
    -0.,
    -0.,
    -4.696545600891113,
    1.,
];
/// The float WorldView * ModelWorld product of the trihedron moved to (100, 100, 100).
const BUG22368_MODEL_VIEW: [f64; 16] = [
    0.7071068286895752,
    -0.4082483649253845,
    0.5773503184318542,
    0.,
    0.7071068286895752,
    0.4082483649253845,
    -0.5773503184318542,
    0.,
    0.,
    0.816496729850769,
    0.5773503184318542,
    0.,
    141.42137145996094,
    81.64967346191406,
    -442.2650146484375,
    1.,
];

fn bug22368_transform() -> TextTransform {
    TextTransform {
        projection: Mat4::from_data(BUG22368_PROJECTION),
        model_view: Mat4::from_data(BUG22368_MODEL_VIEW),
        model_world: Mat4::identity(),
        camera_rotation: Mat4::identity(),
        viewport: [0, 0, 409, 409],
        pixel_coordinates: false,
        zoomable_scale: 1.,
    }
}

/// Column-major float product, summed in shader order (k = 0..3), without fused operations.
fn float_product(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    std::array::from_fn(|i| {
        let (column, row) = (i / 4, i % 4);
        (0..4).fold(0_f32, |sum, k| sum + a[k * 4 + row] * b[column * 4 + k])
    })
}

#[test]
fn glyph_clip_is_the_float_product_of_the_float_projection_and_world_view() {
    // The X label of bug22368: native draws its glyph over the end of its axis line at
    // (291, 187); the double product rounded once moved the glyph depth behind the line.
    let text = Graphic3dText::new(16.);
    text.set_position(&Point3::new(100., 0., 0.));
    let aspect = Graphic3dAspectText3d::default();
    let transform = bug22368_transform();
    let clip = transform.clip(&text, &aspect, 16).unwrap();
    let win = transform.projected_anchor(&text).unwrap();
    let model = transform
        .text_model(&text, &aspect, win, &transform.projection)
        .unwrap();
    let mut scaled = model;
    tr::scale(&mut scaled, [transform.pixel_scale(win, 16).unwrap(); 3]);
    let to_f32 = |m: &Mat4<f64>| m.data().map(|v| v as f32);
    let expected = float_product(&to_f32(&transform.projection), &to_f32(&scaled));
    assert_eq!(clip, expected, "native float pipeline clip");
    let double_rounded = to_f32(&(transform.projection * scaled));
    assert_ne!(
        clip[14], double_rounded[14],
        "the bug22368 anchor separates the two roundings of the glyph depth"
    );
}

#[test]
fn gpu_clip_rounds_both_matrices_before_multiplying() {
    let mut projection = Mat4::<f64>::identity();
    projection.set_value(0, 0, 1. + 1e-9);
    let world_view = Mat4::<f64>::identity();
    let clip = gpu_clip(&projection, &world_view);
    assert_eq!(clip[0], 1., "1 + 1e-9 rounds to 1 in float");
}
