use super::*;
#[test]
fn exact_ratios_cover_unequal_w_signed_zero_and_repeated_vertices() {
    let x = [[1., -1., 0., 2.], [2., 2., 0., 4.], [4., 0., 0., 8.]];
    assert!(constant_axis(x));
    assert!(inside_clip(x));
    let y = x.map(|p| [p[1], p[0], p[2], p[3]]);
    assert!(constant_axis(y));
    assert!(constant_axis([[0., -0., 0., 1.]; 3]));
    assert!(!nonzero_determinant(x));
}
#[test]
fn homogeneous_orientation_is_exact_even_when_screen_area_rounds_away() {
    let q = [
        [0., 0., 0., 1.],
        [1., 1., 0., 1.],
        [f32::from_bits(1f32.to_bits() + 1), 1., 0., 1.],
    ];
    assert!(nonzero_determinant(q));
    assert!(!constant_axis(q));
    assert!(nonzero_determinant([q[2], q[1], q[0]]));
    let collinear = [0.1, 0.2, 0.3].map(|v| [v, v, 0., 1.]);
    assert!(!constant_axis(collinear));
    assert!(!nonzero_determinant(collinear));
}
#[test]
fn extreme_finite_products_are_exact_without_float64_overflow() {
    for x in [f32::from_bits(1), f32::MIN_POSITIVE, 1., f32::MAX] {
        let q = [[x, 0., 0., 1.], [x, 0.25, 0., 1.], [x, 0.5, 0., 1.]];
        assert!(constant_axis(q));
        assert!(!nonzero_determinant(q));
    }
    for bad in [f32::NAN, f32::INFINITY, -1., 0.] {
        assert!(!inside_clip([[0., 0., 0., bad]; 3]));
    }
    assert!(!inside_clip([[f32::MAX, 0., f32::MAX, f32::MAX]; 3]));
}
#[test]
fn filtered_determinants_agree_with_exact_rationals_over_generated_bits() {
    let mut seed = 0x912b_u32;
    for _ in 0..2000 {
        let q = std::array::from_fn(|_| {
            std::array::from_fn(|_| {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                f32::from_bits(seed & 0xfeffffff)
            })
        });
        let expected = exact_determinant(q).unwrap() != BigRational::from_integer(0.into());
        assert_eq!(nonzero_determinant(q), expected);
    }
}
