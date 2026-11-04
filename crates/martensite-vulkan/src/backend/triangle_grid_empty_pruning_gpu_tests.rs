//! Whole-frame neutrality of a certified-empty first primitive and an ordinary visible successor.
use super::*;

#[test]
fn empty_grid_v3_preserves_following_primitive_color_depth_and_order() {
    if crate::backend::phong_interpolation_device::skip_without_barycentric() {
        return;
    }
    let frame = empty_then_visible_frame();
    let mut control = renderer(hardware_behavior());
    let expected = control.render(&frame).unwrap();
    for origin in [
        RenderTriangleGridOrigin::TopDown,
        RenderTriangleGridOrigin::BottomUp,
    ] {
        let configuration = hardware_behavior()
            .with_triangle_coverage(RenderTriangleCoverageBehavior::FixedGrid12V3 { origin });
        let mut selected = renderer(configuration);
        let actual = selected.render(&frame).unwrap();
        assert_eq!(actual.rgba, expected.rgba);
        assert_eq!(actual.depth, expected.depth);
        assert_red_at(&actual, 204, 204);
        assert!(actual.depth[204 * 409 + 204] < 1.0);
        assert!(selected.validation_errors().unwrap().is_empty());
    }
    assert!(control.validation_errors().unwrap().is_empty());
}

fn empty_then_visible_frame() -> RenderFrame {
    let bits = [
        [3204550651, 1055134992, 0],
        [3204546180, 1055120595, 0],
        [3204540855, 1055107392, 0],
    ];
    let mut vertices: Vec<_> = bits
        .map(|p| RenderVertex::new(p.map(f32::from_bits), [1.0; 4]))
        .to_vec();
    vertices.extend(
        [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]]
            .map(|p| RenderVertex::new(p, [1.0; 4])),
    );
    let mut lighting = phong_emission();
    lighting.normals.extend([[0.0, 0.0, 1.0]; 3]);
    let draw = PrimitiveDraw::new(vertices, vec![0, 1, 2, 3, 4, 5], identity_clip())
        .unwrap()
        .with_lighting(lighting)
        .unwrap();
    RenderFrame::new(409, 409, [0.0, 0.0, 0.0, 1.0], vec![draw]).unwrap()
}
