//! Bsplslib tests.
#![allow(non_snake_case)]
use martensite_tkernel::ncollection::Array2;
use martensite_tkmath::bsplslib;

// Doubles are equal within four units in the last place.
fn EXPECT_DOUBLE_EQ(a: f64, b: f64) {
    assert!(!a.is_nan() && !b.is_nan());
    assert!(ordered_bits(a).abs_diff(ordered_bits(b)) <= 4, "{a} != {b}");
}
fn ordered_bits(value: f64) -> u64 {
    let bits: u64 = value.to_bits();
    if bits >> 63 != 0 {
        (!bits).wrapping_add(1)
    } else {
        bits | (1 << 63)
    }
}

// Array2::col_length and RowLength.
fn col_length(array: &Array2<'_, f64>) -> i32 {
    array.nb_rows() as i32
}
fn row_length(array: &Array2<'_, f64>) -> i32 {
    array.nb_columns() as i32
}

#[test]
fn bspline_surface_unit_weights_small_surface_returns_non_owning() {
    let aNbU: i32 = 4;
    let aNbV: i32 = 5;
    let aWeights: Array2<'static, f64> = bsplslib::unit_weights(aNbU, aNbV).unwrap();

    assert_eq!(col_length(&aWeights), aNbU);
    assert_eq!(row_length(&aWeights), aNbV);
    assert_eq!(aWeights.len() as i32, aNbU * aNbV);
    assert!(!aWeights.is_deletable());

    for i in 1..=aNbU {
        for j in 1..=aNbV {
            EXPECT_DOUBLE_EQ(*aWeights.value(i, j).unwrap(), 1.0);
        }
    }
}

#[test]
fn bspline_surface_unit_weights_max_bezier_size_returns_non_owning() {
    // Max Bezier: 26x26 = 676 poles, well within limit
    let aNbU: i32 = 26;
    let aNbV: i32 = 26;
    let aWeights: Array2<'static, f64> = bsplslib::unit_weights(aNbU, aNbV).unwrap();

    assert_eq!(aWeights.len() as i32, aNbU * aNbV);
    assert!(!aWeights.is_deletable());
    EXPECT_DOUBLE_EQ(*aWeights.value(1, 1).unwrap(), 1.0);
    EXPECT_DOUBLE_EQ(*aWeights.value(aNbU, aNbV).unwrap(), 1.0);
}

#[test]
fn bspline_surface_unit_weights_at_max_limit_returns_non_owning() {
    // Exactly at the limit: 2049 total poles
    let aNbU: i32 = 3;
    let aNbV: i32 = 683;
    let aWeights: Array2<'static, f64> = bsplslib::unit_weights(aNbU, aNbV).unwrap();

    assert_eq!(aWeights.len() as i32, aNbU * aNbV);
    assert!(!aWeights.is_deletable());
    EXPECT_DOUBLE_EQ(*aWeights.value(1, 1).unwrap(), 1.0);
    EXPECT_DOUBLE_EQ(*aWeights.value(aNbU, aNbV).unwrap(), 1.0);
}

#[test]
fn bspline_surface_unit_weights_over_max_limit_returns_owning() {
    // Over the limit: needs allocation
    let aNbU: i32 = 50;
    let aNbV: i32 = 50;
    let aWeights: Array2<'static, f64> = bsplslib::unit_weights(aNbU, aNbV).unwrap();

    assert_eq!(col_length(&aWeights), aNbU);
    assert_eq!(row_length(&aWeights), aNbV);
    assert_eq!(aWeights.len() as i32, aNbU * aNbV);
    assert!(aWeights.is_deletable());

    for i in 1..=aNbU {
        for j in 1..=aNbV {
            EXPECT_DOUBLE_EQ(*aWeights.value(i, j).unwrap(), 1.0);
        }
    }
}

#[test]
fn bspline_surface_unit_weights_single_element() {
    let aWeights: Array2<'static, f64> = bsplslib::unit_weights(1, 1).unwrap();

    assert_eq!(aWeights.len(), 1);
    assert!(!aWeights.is_deletable());
    EXPECT_DOUBLE_EQ(*aWeights.value(1, 1).unwrap(), 1.0);
}
