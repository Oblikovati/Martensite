//! Owned fixed-grid point rectangles, independent of any vendor or camera.
use crate::RenderError;
const GRID: f32 = 256.;
#[cfg(test)]
const UNIT12_GRID: f32 = 4096.;

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FixedGridPointRectangle {
    pub minimum: [i32; 2],
    pub extent: u32,
}

pub(crate) fn fixed_grid_point_extent(requested: f32) -> Result<u32, RenderError> {
    if !requested.is_finite() || requested <= 0. || requested * GRID > (i32::MAX - 255) as f32 {
        return Err(RenderError::rejected(
            requested,
            "positive finite point size whose 8-bit fixed extent fits i32",
        ));
    }
    let fixed = (requested * GRID).round_ties_even() as i64;
    Ok((((fixed + 127) & !255).max(256) / 256) as u32)
}

#[cfg(test)]
pub(crate) fn fixed_grid_point_rectangle(
    center: [f32; 2],
    requested: f32,
) -> Result<FixedGridPointRectangle, RenderError> {
    let extent = fixed_grid_point_extent(requested)?;
    let snapped = [
        snapped_coordinate(center[0])?,
        snapped_coordinate(center[1])?,
    ];
    let half = i64::from(extent / 2);
    let shift = if extent % 2 == 0 { 128 } else { 0 };
    let minimum = snapped.map(|value| ((value + shift).div_euclid(256) - half) as i32);
    Ok(FixedGridPointRectangle { minimum, extent })
}

#[cfg(test)]
pub(crate) fn fixed_grid_unit12_rectangle(
    center: [f32; 2],
) -> Result<FixedGridPointRectangle, RenderError> {
    let [x, y] = center.map(snapped_unit12_coordinate);
    let snapped = [x?, y?];
    let minimum = [
        snapped[0].div_euclid(4096) as i32,
        (snapped[1] - 1).div_euclid(4096) as i32,
    ];
    Ok(FixedGridPointRectangle { minimum, extent: 1 })
}

#[cfg(test)]
fn snapped_unit12_coordinate(value: f32) -> Result<i64, RenderError> {
    if !value.is_finite()
        || (f64::from(value) * f64::from(UNIT12_GRID)).abs() >= f64::from(i32::MAX - 1)
    {
        return Err(RenderError::rejected(
            value,
            "finite window coordinate whose 12-bit fixed value and bottom adjustment fit i32",
        ));
    }
    Ok((value * UNIT12_GRID).round_ties_even() as i64)
}

pub(crate) fn validate_fixed_grid_unit12_viewport(extent: [u32; 2]) -> Result<(), RenderError> {
    const MAXIMUM: u32 = (i32::MAX as u32 - 1) / 4096;
    if extent
        .into_iter()
        .any(|value| value == 0 || value > MAXIMUM)
    {
        return Err(RenderError::rejected(
            extent,
            "positive viewport extents with 12-bit fixed coordinates and bottom adjustment fitting i32",
        ));
    }
    Ok(())
}

#[cfg(test)]
fn snapped_coordinate(value: f32) -> Result<i64, RenderError> {
    if !value.is_finite() || (value * GRID).abs() >= i32::MAX as f32 {
        return Err(RenderError::rejected(
            value,
            "finite window coordinate whose 8-bit fixed representation fits i32",
        ));
    }
    Ok((value * GRID).round_ties_even() as i64)
}

pub(crate) fn validate_fixed_grid_viewport(extent: [u32; 2]) -> Result<(), RenderError> {
    const MAXIMUM: u32 = (i32::MAX as u32 - 128) / 256;
    if extent
        .into_iter()
        .any(|value| value == 0 || value > MAXIMUM)
    {
        return Err(RenderError::rejected(
            extent,
            "positive viewport extents with 8-bit fixed coordinates and even adjustment fitting i32",
        ));
    }
    Ok(())
}
pub(crate) fn validate_draw_viewport(
    semantic_fixed_grid_point: bool,
    encoded_fixed_grid_point: bool,
    encoded_unit12: bool,
    extent: [u32; 2],
) -> Result<(), RenderError> {
    if !semantic_fixed_grid_point && !encoded_fixed_grid_point {
        return Ok(());
    }
    if encoded_fixed_grid_point && encoded_unit12 {
        return validate_fixed_grid_unit12_viewport(extent);
    }
    validate_fixed_grid_viewport(extent)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Frozen rational-model expectations; fixture SHA-256:
    // ac57426fc4943068b6d26aabef178890bc7857aada2b63723a091f756a579973.
    const UNIT12_CALIBRATION: [(u32, u32, i32, i32); 17] = [
        (16, 1, 16, 15),
        (447, 16, 27, 27),
        (641, 16, 40, 40),
        (3327, 64, 51, 51),
        (4097, 64, 64, 64),
        (19455, 256, 75, 75),
        (22529, 256, 88, 88),
        (51199, 512, 99, 99),
        (57345, 512, 112, 112),
        (126975, 1024, 123, 123),
        (139265, 1024, 136, 136),
        (606207, 4096, 147, 147),
        (655361, 4096, 160, 160),
        (2818047, 16384, 172, 171),
        (3014657, 16384, 184, 183),
        (12845055, 65536, 196, 195),
        (13631489, 65536, 208, 207),
    ];

    #[test]
    fn requested_half_sizes_have_half_down_extents_after_grid_rounding() {
        for (size, extent) in [
            (0.01, 1),
            (1., 1),
            (1.5, 1),
            (2., 2),
            (2.5, 2),
            (3., 3),
            (3.5, 3),
            (7., 7),
        ] {
            assert_eq!(fixed_grid_point_extent(size).unwrap(), extent, "{size}");
        }
        assert_eq!(fixed_grid_point_extent(1.5 + 1. / 256.).unwrap(), 2);
        assert_eq!(fixed_grid_point_extent(2.5 + 1. / 256.).unwrap(), 3);
        assert_eq!(fixed_grid_point_extent(65536.5).unwrap(), 65536);
        assert_eq!(fixed_grid_point_extent(65537.5).unwrap(), 65537);
    }

    #[test]
    fn grid_ties_even_and_odd_even_anchors_are_independent() {
        for (center, size, minimum) in [
            ([16.999, 17.], 1., [17, 17]),
            ([16.997, 17.], 1., [16, 17]),
            ([17.5, 17.5], 2., [17, 17]),
            ([-0.003, -0.5], 3., [-2, -2]),
            ([-0.5, -0.5], 4., [-2, -2]),
        ] {
            assert_eq!(
                fixed_grid_point_rectangle(center, size).unwrap().minimum,
                minimum
            );
        }
        assert_eq!(snapped_coordinate(1. / 512.).unwrap(), 0);
        assert_eq!(snapped_coordinate(3. / 512.).unwrap(), 2);
    }

    #[test]
    fn fixed_grid_viewport_limit_protects_integer_even_adjustment() {
        assert!(validate_fixed_grid_viewport([8388607, 8388607]).is_ok());
        assert!(validate_fixed_grid_viewport([8388608, 1]).is_err());
        assert!(validate_fixed_grid_viewport([0, 1]).is_err());
    }
    #[test]
    fn private_encoding_keeps_viewport_domain_validation() {
        assert!(validate_draw_viewport(false, true, false, [8388608, 1]).is_err());
        assert!(validate_draw_viewport(false, true, false, [1, 1]).is_ok());
        assert!(validate_draw_viewport(false, false, false, [8388608, 1]).is_ok());
    }
    #[test]
    fn encoded_unit12_uses_its_tighter_integer_viewport_domain() {
        assert!(validate_draw_viewport(false, true, true, [524287, 1]).is_ok());
        assert!(validate_draw_viewport(false, true, true, [524288, 1]).is_err());
        assert!(validate_draw_viewport(false, true, false, [524288, 1]).is_ok());
    }
    #[test]
    fn invalid_inputs_refuse_before_integer_conversion() {
        for size in [f32::NAN, f32::INFINITY, 0., -1., f32::MAX] {
            assert!(fixed_grid_point_extent(size).is_err());
        }
        for coordinate in [f32::NAN, f32::INFINITY, f32::MAX] {
            assert!(fixed_grid_point_rectangle([coordinate, 0.], 1.).is_err());
        }
    }

    #[test]
    fn unit12_grid_rounds_ties_even_then_applies_one_lattice_unit() {
        for (center, minimum) in [
            ([16.0, 16.0], [16, 15]),
            ([16.0001, 15.9999], [16, 15]),
            ([-16.0, -16.0], [-16, -17]),
            ([15.9998, -16.0002], [15, -17]),
        ] {
            assert_eq!(
                fixed_grid_unit12_rectangle(center).unwrap().minimum,
                minimum
            );
        }
        assert_eq!(snapped_unit12_coordinate(1. / 8192.).unwrap(), 0);
        assert_eq!(snapped_unit12_coordinate(3. / 8192.).unwrap(), 2);
    }

    #[test]
    fn unit12_rectangle_refuses_signed_domain_overflow() {
        assert!(fixed_grid_unit12_rectangle([f32::MAX, 0.]).is_err());
        assert!(fixed_grid_unit12_rectangle([f32::NAN, 0.]).is_err());
        assert_eq!(
            fixed_grid_unit12_rectangle([8192., 8192.]).unwrap().minimum,
            [8192, 8191]
        );
        assert_eq!(
            fixed_grid_unit12_rectangle([-8192., -8192.])
                .unwrap()
                .minimum,
            [-8192, -8193]
        );
        assert!(validate_fixed_grid_unit12_viewport([524287, 1]).is_ok());
        assert!(validate_fixed_grid_unit12_viewport([524288, 1]).is_err());
    }

    #[test]
    fn unit12_rectangles_match_the_frozen_34_unit_calibration_coordinates() {
        for (numerator, denominator, expected_x, expected_y) in UNIT12_CALIBRATION {
            let coordinate = numerator as f32 / denominator as f32;
            let horizontal = fixed_grid_unit12_rectangle([coordinate, 16.]).unwrap();
            let vertical = fixed_grid_unit12_rectangle([16., coordinate]).unwrap();
            assert_eq!(horizontal.minimum, [expected_x, 15]);
            assert_eq!(vertical.minimum, [16, expected_y]);
        }
    }
}
