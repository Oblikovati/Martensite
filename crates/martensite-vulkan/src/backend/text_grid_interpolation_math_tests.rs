use super::*;

#[test]
fn signed_half_steps_use_nearest_even_on_the_twelve_bit_lattice() {
    let step = 1.0 / 4096.0;
    assert_eq!(
        snap_window_point([0.5 * step, 1.5 * step]),
        Some([0.0, 2.0 * step])
    );
    assert_eq!(
        snap_window_point([-0.5 * step, -1.5 * step]),
        Some([0.0, -2.0 * step])
    );
}

#[test]
fn finite_window_domain_includes_exact_cap_and_refuses_outside_values() {
    assert_eq!(
        snap_window_point([4096.0, -4096.0]),
        Some([4096.0, -4096.0])
    );
    assert_eq!(snap_window_point([4096.25, 0.0]), None);
    assert_eq!(snap_window_point([f32::INFINITY, 0.0]), None);
    assert_eq!(snap_window_point([f32::NAN, 0.0]), None);
}

#[test]
fn two_quad_triangles_interpolate_the_same_closed_form_diagonal_value() {
    let positions = [[0., 0.], [2., 0.], [2., 2.], [0., 2.]];
    let uv = [[0., 0.], [1., 0.], [1., 1.], [0., 1.]];
    let w = [1.; 3];
    assert_eq!(
        interpolate_uv(
            [positions[0], positions[1], positions[2]],
            [uv[0], uv[1], uv[2]],
            w,
            [1., 1.]
        ),
        Some([0.5, 0.5])
    );
    assert_eq!(
        interpolate_uv(
            [positions[0], positions[2], positions[3]],
            [uv[0], uv[2], uv[3]],
            w,
            [1., 1.]
        ),
        Some([0.5, 0.5])
    );
}

#[test]
fn projective_quad_diagonal_uses_the_same_two_endpoint_perspective_weights() {
    let positions = [[0., 0.], [2., 0.], [2., 2.], [0., 2.]];
    let uv = [[0., 0.], [1., 0.], [1., 1.], [0., 1.]];
    let sample = [1., 1.];
    let first = interpolate_uv(
        [positions[0], positions[1], positions[2]],
        [uv[0], uv[1], uv[2]],
        [1., 2., 4.],
        sample,
    );
    let second = interpolate_uv(
        [positions[0], positions[2], positions[3]],
        [uv[0], uv[2], uv[3]],
        [1., 4., 8.],
        sample,
    );
    assert_eq!(first, Some([0.2, 0.2]));
    assert_eq!(second, Some([0.2, 0.2]));
}

#[test]
fn snap_changes_interpolation_only_when_the_vertex_grid_changes() {
    let positions = [[0.0002, 0.], [2., 0.], [0., 2.]];
    let uv = [[0., 0.], [1., 0.], [0., 1.]];
    let actual = interpolate_uv(positions, uv, [1.; 3], [0.5, 0.5]).unwrap();
    assert_eq!(snap_window_point(positions[0]), Some([1. / 4096., 0.]));
    assert!((actual[0] - 8189. / 32764.).abs() <= f32::EPSILON);
    assert_eq!(actual[1], 0.25);
}

#[test]
fn signed_viewport_height_maps_clip_edges_in_both_orientations() {
    use super::super::phong_interpolation_math::window_point;

    assert_eq!(
        window_point([-1., 1., 0., 1.], [0., 8., 8., -8.]),
        Some([0., 0.])
    );
    assert_eq!(
        window_point([-1., -1., 0., 1.], [0., 8., 8., -8.]),
        Some([0., 8.])
    );
    assert_eq!(
        window_point([-1., -1., 0., 1.], [0., 0., 8., 8.]),
        Some([0., 0.])
    );
}

#[test]
fn degenerate_grid_triangles_and_nonfinite_reciprocal_w_refuse() {
    let uv = [[0., 0.], [1., 0.], [0., 1.]];
    assert_eq!(
        interpolate_uv([[0., 0.], [1., 1.], [2., 2.]], uv, [1.; 3], [1., 1.]),
        None
    );
    assert_eq!(
        interpolate_uv([[0., 0.], [1., 0.], [0., 1.]], uv, [1., 0., 1.], [0.2, 0.2]),
        None
    );
    assert_eq!(
        interpolate_uv(
            [[0., 0.], [1., 0.], [0., 1.]],
            uv,
            [1., -1., 1.],
            [0.2, 0.2]
        ),
        None
    );
    assert_eq!(
        interpolate_uv([[0., 0.], [1., 0.], [0., 1.]], uv, [1.; 3], [f32::NAN, 0.2]),
        None
    );
}

#[test]
fn finite_inputs_that_overflow_shader_weights_or_sample_uv_refuse() {
    let uv = [[0., 0.], [1., 0.], [0., 1.]];
    let tiny_triangle = [[0., 0.], [1. / 4096., 0.], [0., 1. / 4096.]];
    assert_eq!(
        interpolate_uv(tiny_triangle, uv, [f32::MIN_POSITIVE; 3], [4096., 4096.]),
        None
    );

    let unit_triangle = [[0., 0.], [1., 0.], [0., 1.]];
    assert_eq!(
        interpolate_uv(
            unit_triangle,
            [[f32::MAX; 2]; 3],
            [f32::MIN_POSITIVE; 3],
            [0.25, 0.25]
        ),
        None
    );
}

#[test]
fn signed_triangle_area_preserves_winding_and_rejects_degeneracy() {
    assert_eq!(triangle_area([[0., 0.], [2., 0.], [0., 2.]]), Some(4.));
    assert_eq!(triangle_area([[0., 0.], [0., 2.], [2., 0.]]), Some(-4.));
    assert_eq!(triangle_area([[0., 0.], [1., 1.], [2., 2.]]), None);
    assert_eq!(
        triangle_area([[0., 0.], [f32::MAX, 0.], [0., f32::MAX]]),
        None
    );
}

#[test]
fn shader_area_bound_refuses_exact_nonzero_cancellation_that_f32_can_erase() {
    let points = [[0.0, 0.0], [4096.0, 4095.9995], [4095.9995, 4095.999]];
    assert_eq!(triangle_area(points), None);
    assert_eq!(
        triangle_area([[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]),
        Some(1.0)
    );
}

#[test]
fn shader_guard_tests_precede_every_risky_division_and_texture_sample() {
    let shader = include_str!("../../shaders/text.grid12.frag");
    let w_guard = shader.find("clip.w <= 0.0").unwrap();
    let clip_division = shader.find("clip.xy / clip.w").unwrap();
    let area_guard = shader.find("area == 0.0").unwrap();
    let area_division = shader.find("/ area").unwrap();
    let sum_guard = shader.find("sum <= 0.0").unwrap();
    let sample_write = shader.find("sampleUv = ((weights").unwrap();
    let uv_guard = shader.find("return textGridFinite(sampleUv)").unwrap();
    let sample_call_guard = shader
        .find("if (!textGridSampleUv(sampleUv)) discard")
        .unwrap();
    let viewport_guard = shader
        .find("transform.rectangle.z <= 0.0 || transform.rectangle.w == 0.0")
        .unwrap();
    let texture_sample = shader.find("texture(glyphs, sampleUv)").unwrap();
    let world_guard = shader
        .find("textGridValidWorld(clippingWorldPosition)")
        .unwrap();
    let world_division = shader.find("vec3 divided = point.xyz / point.w").unwrap();
    let clipping = include_str!("../../shaders/clipping.glsl");
    assert!(w_guard < clip_division);
    assert!(area_guard < area_division);
    assert!(sum_guard < sample_write);
    assert!(uv_guard < texture_sample);
    assert!(sample_call_guard < texture_sample);
    assert!(world_guard < shader.find("discardClippedFragment()").unwrap());
    assert!(world_division < shader.find("discardClippedFragment()").unwrap());
    assert!(viewport_guard < shader.find("clip.xy / clip.w").unwrap());
    assert!(!shader.contains("lessThanEqual(transform.rectangle.zw"));
    assert!(clipping.contains("clippingWorldPosition.xyz / clippingWorldPosition.w"));
}
