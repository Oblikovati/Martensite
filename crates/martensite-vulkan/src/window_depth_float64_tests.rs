//! Independent closed forms and exact rational expectations pin the V5 arithmetic domain.
fn reference(clip: [[f32; 4]; 3], sample: [f64; 2], bias: [f64; 2]) -> Option<f64> {
    let mut points = [[0.; 3]; 3];
    for (point, clip) in points.iter_mut().zip(clip) {
        if !clip.iter().all(|v| v.is_finite())
            || clip[3] <= 0.
            || clip[..3].iter().any(|v| v.abs() > clip[3])
        {
            return None;
        }
        for axis in 0..2 {
            let window = (f64::from(clip[axis]) / f64::from(clip[3]) * 0.5 + 0.5) * 409.;
            point[axis] = (window * 4096.).round_ties_even() / 4096.;
        }
        point[2] = f64::from(clip[2]) / f64::from(clip[3]) * 0.5 + 0.5;
    }
    plane(points, sample, bias)
}
fn plane(p: [[f64; 3]; 3], sample: [f64; 2], bias: [f64; 2]) -> Option<f64> {
    let a: [f64; 3] = std::array::from_fn(|i| p[1][i] - p[0][i]);
    let b: [f64; 3] = std::array::from_fn(|i| p[2][i] - p[0][i]);
    let determinant = a[0] * b[1] - b[0] * a[1];
    if determinant == 0. {
        return None;
    }
    let x = (a[2] * b[1] - b[2] * a[1]) / determinant;
    let y = (a[0] * b[2] - b[0] * a[2]) / determinant;
    let z = (x * (sample[0] - p[0][0]) + y * (sample[1] - p[0][1])) + p[0][2];
    Some(z + (x.abs().max(y.abs()) * bias[0] + bias[1] / 8388608.))
}
#[test]
fn actual_pair_preserves_exact_rational_depth_order_without_offsets() {
    let expected_ring = 14539093290187_f64 / 61495224303616_f64;
    let actual_ring = reference(
        super::window_depth_float64_fixture::RING,
        [185.5, 222.5],
        [1., 1.],
    )
    .unwrap();
    assert!((actual_ring - expected_ring).abs() < 1e-15);
    let expected = 1130237446894763_f64 / 4780508919627776_f64;
    assert!(expected < actual_ring);
    assert!(
        (reference(
            super::window_depth_float64_fixture::SECTOR,
            [185.5, 222.5],
            [1., 1.]
        )
        .unwrap()
            - expected)
            .abs()
            < 1e-15
    );
}
#[test]
fn nearest_even_grid_ties_and_adjacent_values_remain_distinct() {
    for integer in [-4_f64, -3., 0., 1., 4095.] {
        let tie = integer + 0.5;
        assert_eq!(
            tie.round_ties_even(),
            if integer as i64 % 2 == 0 {
                integer
            } else {
                integer + 1.
            }
        );
        assert_eq!((tie - 0.00000001).round_ties_even(), integer);
        assert_eq!((tie + 0.00000001).round_ties_even(), integer + 1.);
    }
}
#[test]
fn generated_affine_planes_match_closed_form_and_bias_terms() {
    for x in -4..5 {
        for y in -4..5 {
            let a = x as f64 / 64.;
            let b = y as f64 / 64.;
            let points = [
                [0., 0., 0.25],
                [8., 0., 0.25 + 8. * a],
                [0., 8., 0.25 + 8. * b],
            ];
            for bias in [[0., 0.], [1., 0.], [0., 1.], [1., 1.]] {
                let expected =
                    0.25 + 2. * a + 3. * b + a.abs().max(b.abs()) * bias[0] + bias[1] / 8388608.;
                assert_eq!(plane(points, [2., 3.], bias).unwrap(), expected);
            }
        }
    }
}
#[test]
fn degenerate_clipped_nonpositive_and_nonfinite_inputs_never_produce_depth() {
    assert!(reference([[0., 0., 0., 1.]; 3], [1., 1.], [0., 0.]).is_none());
    for clip in [
        [2., 0., 0., 1.],
        [0., 0., 2., 1.],
        [0., 0., 0., 0.],
        [f32::NAN, 0., 0., 1.],
    ] {
        assert!(reference([clip; 3], [1., 1.], [0., 0.]).is_none());
    }
}
#[test]
fn raw_gl_z_is_not_recovered_from_lossy_binary32_remap() {
    let raw = 0.1_f32;
    let exact = f64::from(raw) * 0.5 + 0.5;
    let rounded = f64::from(0.5_f32 * (raw + 1.));
    assert_ne!(exact, rounded);
}
