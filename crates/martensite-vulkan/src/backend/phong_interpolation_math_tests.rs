use super::*;
// Independent closed-form fixtures reused; no captured-image expectations.
type Clip = [f64; 4];
fn area(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}
fn subtract(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}
fn project(c: Clip, v: [f64; 4]) -> Option<[f64; 2]> {
    if c.iter().any(|x| !x.is_finite()) || c[3] <= 0. {
        return None;
    }
    Some([
        v[0] + (c[0] / c[3] + 1.) * v[2] / 2.,
        v[1] + (c[1] / c[3] + 1.) * v[3] / 2.,
    ])
}
fn weights(clips: [Clip; 3], viewport: [f64; 4], pixel: [f64; 2]) -> Option<[f64; 3]> {
    let points = [
        project(clips[0], viewport)?,
        project(clips[1], viewport)?,
        project(clips[2], viewport)?,
    ];
    let d = area(
        subtract(points[1], points[0]),
        subtract(points[2], points[0]),
    );
    if d == 0. || !d.is_finite() {
        return None;
    }
    normalized_weights(points, clips.map(|c| c[3]), pixel, d)
}
fn normalized_weights(
    points: [[f64; 2]; 3],
    ws: [f64; 3],
    pixel: [f64; 2],
    d: f64,
) -> Option<[f64; 3]> {
    let mut w = [0.; 3];
    for i in 0..3 {
        w[i] = area(
            subtract(points[(i + 1) % 3], pixel),
            subtract(points[(i + 2) % 3], pixel),
        ) / d
            / ws[i];
    }
    let sum = w.iter().sum::<f64>();
    if sum == 0. || !sum.is_finite() {
        return None;
    }
    Some(w.map(|x| x / sum))
}

fn assert_weights(actual: [f64; 3], expected: [f64; 3]) {
    for (a, b) in actual.into_iter().zip(expected) {
        assert!((a - b).abs() < 1e-12, "{a:?} != independent weight {b:?}");
    }
}
fn triangle(ws: [f64; 3]) -> [Clip; 3] {
    [
        [-ws[0], -ws[0], 0., ws[0]],
        [ws[1], -ws[1], 0., ws[1]],
        [-ws[2], ws[2], 0., ws[2]],
    ]
}
#[test]
fn orthographic_interpolation_has_closed_form_half_quarter_quarter() {
    assert_weights(
        weights(triangle([1.; 3]), [0., 0., 4., 4.], [1., 1.]).unwrap(),
        [0.5, 0.25, 0.25],
    );
}
#[test]
fn unequal_clip_w_requires_perspective_weights() {
    assert_weights(
        weights(triangle([1., 2., 4.]), [0., 0., 4., 4.], [1., 1.]).unwrap(),
        [8. / 11., 2. / 11., 1. / 11.],
    );
}
#[test]
fn actual_negative_viewport_height_and_offset_preserve_weights() {
    assert_weights(
        weights(triangle([1., 2., 4.]), [0., 5., 4., -4.], [1., 4.]).unwrap(),
        [8. / 11., 2. / 11., 1. / 11.],
    );
}
#[test]
fn vertex_permutation_permutes_only_weight_ownership() {
    let t = triangle([1., 2., 4.]);
    let expected = [8. / 11., 2. / 11., 1. / 11.];
    let permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for p in permutations {
        assert_weights(
            weights(p.map(|i| t[i]), [0., 0., 4., 4.], [1., 1.]).unwrap(),
            p.map(|i| expected[i]),
        );
    }
}
#[test]
fn degenerate_nonfinite_and_nonpositive_w_refuse() {
    assert!(weights([[0., 0., 0., 1.]; 3], [0., 0., 4., 4.], [1., 1.]).is_none());
    for w in [0., -1., f64::NAN, f64::INFINITY] {
        let mut t = triangle([1.; 3]);
        t[0][3] = w;
        assert!(weights(t, [0., 0., 4., 4.], [1., 1.]).is_none());
    }
}
fn interpolate(normals: [[f64; 3]; 3], weights: [f64; 3]) -> [f64; 3] {
    if normals[0] == normals[1] && normals[0] == normals[2] {
        return normals[0];
    }
    std::array::from_fn(|k| {
        (weights[0] * normals[0][k] + weights[1] * normals[1][k]) + weights[2] * normals[2][k]
    })
}
#[test]
fn constant_normal_is_exact_even_for_extreme_perspective_weights() {
    let constant = [0.125, -0.75, 0.5];
    for ws in [[1., 1., 1.], [1e-12, 1e12, 2.], [4., 2., 1.]] {
        let w = weights(triangle(ws), [0., 0., 4., 4.], [1., 1.]).unwrap();
        assert_eq!(interpolate([constant; 3], w), constant);
    }
}
#[test]
fn perspective_normal_reconstruction_uses_owned_vertex_values() {
    let ns = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
    let w = weights(triangle([1., 2., 4.]), [0., 0., 4., 4.], [1., 1.]).unwrap();
    assert_weights(interpolate(ns, w), [8. / 11., 2. / 11., 1. / 11.]);
}
#[test]
fn perspective_normal_is_invariant_under_matching_vertex_permutation() {
    let t = triangle([1., 2., 4.]);
    let ns = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
    let permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for p in permutations {
        let w = weights(p.map(|i| t[i]), [0., 0., 4., 4.], [1., 1.]).unwrap();
        assert_weights(
            interpolate(p.map(|i| ns[i]), w),
            [8. / 11., 2. / 11., 1. / 11.],
        );
    }
}

#[test]
fn production_domain_refuses_overflow_nonfinite_and_collapsed_triangles() {
    let finite = triangle([1.; 3]).map(|v| v.map(|x| x as f32));
    let viewport = [0., 0., 4., 4.];
    assert!(admitted_triangle(finite, viewport));
    assert_eq!(window_point(finite[0], viewport), Some([0., 0.]));
    assert_eq!(signed_area([0., 0.], [4., 0.], [0., 4.]), 16.);
    assert!(!admitted_triangle([finite[0]; 3], viewport));
    for w in [0., -1., f32::NAN, f32::INFINITY, f32::from_bits(1)] {
        let mut invalid = finite;
        invalid[0][3] = w;
        assert!(!admitted_triangle(invalid, viewport));
    }
    assert!(!admitted_triangle(finite, [0., 0., f32::INFINITY, 4.]));
}
fn scalar_interpolation(ws: [f64; 3]) -> f64 {
    let weights = weights(triangle(ws), [0., 0., 4., 4.], [1., 1.]).unwrap();
    weights
        .into_iter()
        .zip([2., 14., 22.])
        .map(|(w, v)| w * v)
        .sum()
}
#[test]
fn affine_scalar_plane_and_rational_projective_value_are_independent() {
    // Affine f(x,y)=2+3x+5y on the 4x4 reference triangle gives f(1,1)=10.
    assert_eq!(scalar_interpolation([1.; 3]), 10.);
    // Rational projective weights 8/11,2/11,1/11 give (16+28+22)/11=6.
    assert!((scalar_interpolation([1., 2., 4.]) - 6.).abs() < 1e-12);
}
#[test]
fn projective_fixture_satisfies_independent_exact_homogeneous_constraints() {
    use num_rational::Ratio;
    let q: [Ratio<i64>; 3] = [8, 2, 1].map(|n| Ratio::new(n, 11));
    // Independent homogeneous equations at screen (1,1), clip w=(1,2,4).
    for coefficients in [[-1, 6, -4], [-1, -2, 12]] {
        assert_eq!(
            coefficients
                .into_iter()
                .zip(q)
                .map(|(a, b)| b * a)
                .sum::<Ratio<i64>>(),
            Ratio::from_integer(0)
        );
    }
    assert_eq!(q.into_iter().sum::<Ratio<i64>>(), Ratio::from_integer(1));
}
#[test]
fn v3_admits_only_exact_axis_degenerates_while_v2_replay_refuses() {
    let axis = [[1., -1., 0., 2.], [2., 2., 0., 4.], [4., 0., 0., 8.]];
    let viewport = [0., 0., 409., -409.];
    assert!(!admitted_triangle(axis, viewport));
    assert!(admitted_triangle_v3(axis, viewport));
    assert!(admitted_triangle_v3(
        axis.map(|p| [p[1], p[0], p[2], p[3]]),
        viewport
    ));
    assert!(admitted_triangle_v3([axis[0]; 3], viewport));
    assert!(!admitted_triangle_v3([[0., 0., 2., 1.]; 3], viewport));
}
#[test]
fn v3_refuses_general_collinearity_and_rounded_cancellation() {
    let collinear = [0.1, 0.2, 0.3].map(|v| [v, v, 0., 1.]);
    assert!(!admitted_triangle_v3(collinear, [3., 7., 409., 311.]));
    let nonzero = [
        [0., 0., 0., 1.],
        [1., 1., 0., 1.],
        [f32::from_bits(1f32.to_bits() + 1), 1., 0., 1.],
    ];
    assert!(super::super::phong_projected_axis::nonzero_determinant(
        nonzero
    ));
    assert!(!admitted_triangle(nonzero, [0., 0., 4., 4.]));
    assert!(!admitted_triangle_v3(nonzero, [0., 0., 4., 4.]));
}
