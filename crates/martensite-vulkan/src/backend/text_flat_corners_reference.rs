//! Test-only exact source reference; never an image-grading rule or raw-linear observation.
use num_rational::BigRational;
use std::cmp::Ordering;

fn ratio(numerator: i64, denominator: i64) -> BigRational {
    BigRational::new(numerator.into(), denominator.into())
}

pub(super) fn clipped_fixture_rgb() -> [BigRational; 3] {
    // Pixel center (9/2,9/2), viewport(0,8,8,-8), identity clip and W=1.
    let ndc_x = ratio(2, 1) * ratio(9, 2) / ratio(8, 1) - ratio(1, 1);
    let ndc_y = ratio(1, 1) - ratio(2, 1) * ratio(9, 2) / ratio(8, 1);
    let uv_x = (ndc_x + ratio(2, 1)) / ratio(4, 1);
    let uv_y = (ndc_y + ratio(1, 1)) / ratio(2, 1);
    // The 2x2 linear UNORM atlas has R=column, G=row; both weights are dyadic.
    [
        ratio(2, 1) * uv_x - ratio(1, 2),
        ratio(2, 1) * uv_y - ratio(1, 2),
        ratio(0, 1),
    ]
}

pub(super) fn require_clipped_pixel(pixel: [u8; 4]) {
    for (actual, linear) in pixel[..3].iter().zip(clipped_fixture_rgb()) {
        let permitted = permitted_srgb8(&linear);
        assert!(
            permitted.contains(actual),
            "color={actual}, linear={linear}; expected {permitted:?}"
        );
    }
    assert_eq!(
        pixel[3], 255,
        "alpha={}; expected exact opaque255",
        pixel[3]
    );
}

pub(super) fn permitted_srgb8(linear: &BigRational) -> Vec<u8> {
    assert!(linear >= &ratio(0, 1) && linear <= &ratio(1, 1));
    // Vulkan fundamentals-fixedfpconv permits the two adjacent integers, not nearest±1.
    for code in 0..=255 {
        match compare_code(linear, code) {
            Ordering::Equal => return vec![code],
            Ordering::Less => return vec![code - 1, code],
            Ordering::Greater => (),
        }
    }
    unreachable!("bounded linear value must reach encoded endpoint 255")
}

fn compare_code(linear: &BigRational, code: u8) -> Ordering {
    if linear <= &ratio(7827, 2_500_000) {
        return (linear * ratio(323 * 255, 25)).cmp(&ratio(i64::from(code), 1));
    }
    // Invert sRGB monotonically: ((code/255 + 11/200)/(211/200))^12 vs linear^5.
    let encoded_root = ratio(200 * i64::from(code) + 2805, 53805);
    linear.pow(5).cmp(&encoded_root.pow(12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_clipped_fixture_has_source_derived_exact_dyadic_linear_color() {
        assert_eq!(
            clipped_fixture_rgb(),
            [ratio(9, 16), ratio(3, 8), ratio(0, 1)]
        );
        // The selected triangle's exact shader area/cofactors are -128 and (-60,-12,-56).
        let weights = [-60_f32 / -128., -12_f32 / -128., -56_f32 / -128.];
        assert_eq!(weights.iter().sum::<f32>(), 1.);
        assert_eq!(weights[1] + weights[2], 17. / 32.);
        assert_eq!(weights[2], 7. / 16.);
    }

    #[test]
    fn exact_transfer_comparison_allows_only_the_two_adjacent_red_codes() {
        let allowed = permitted_srgb8(&ratio(9, 16));
        assert_eq!(allowed, [197, 198]);
        assert!(!allowed.contains(&196));
        assert!(!allowed.contains(&199));
        assert_eq!(permitted_srgb8(&ratio(3, 8)), [164, 165]);
    }

    #[test]
    fn integer_endpoints_and_exact_linear_segment_codes_have_no_extra_neighbor() {
        assert_eq!(permitted_srgb8(&ratio(0, 1)), [0]);
        assert_eq!(permitted_srgb8(&ratio(1, 1)), [255]);
        assert_eq!(permitted_srgb8(&ratio(25, 323 * 255)), [1]);
    }
    #[test]
    fn clipped_pixel_accepts_exactly_the_source_derived_adjacent_rgb_codes() {
        for red in [197, 198] {
            for green in [164, 165] {
                require_clipped_pixel([red, green, 0, 255]);
            }
        }
    }

    #[test]
    fn clipped_pixel_refuses_outer_neighbors_and_nonexact_integer_endpoints() {
        for pixel in [
            [196, 165, 0, 255],
            [199, 165, 0, 255],
            [197, 165, 1, 255],
            [197, 165, 0, 254],
        ] {
            assert!(
                std::panic::catch_unwind(|| require_clipped_pixel(pixel)).is_err(),
                "pixel={pixel:?}; expected source-derived quantization refusal"
            );
        }
    }
}
