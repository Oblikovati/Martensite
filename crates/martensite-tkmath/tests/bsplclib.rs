//! Bsplclib tests.
#![allow(non_snake_case)]
use martensite_tkernel::ncollection::Array1;
use martensite_tkmath::bsplclib;

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

#[test]
fn bspline_curve_unit_weights_small_size_returns_non_owning() {
    let aNbElems: i32 = 10;
    let aWeights: Array1<'static, f64> = bsplclib::unit_weights(aNbElems).unwrap();

    assert_eq!(aWeights.lower(), 1);
    assert_eq!(aWeights.upper(), aNbElems);
    assert_eq!(aWeights.len() as i32, aNbElems);
    assert!(!aWeights.is_deletable());

    for i in 1..=aNbElems {
        EXPECT_DOUBLE_EQ(aWeights[i], 1.0);
    }
}

#[test]
fn bspline_curve_unit_weights_max_size_returns_non_owning() {
    let aNbElems: i32 = bsplclib::max_unit_weights_size();
    let aWeights: Array1<'static, f64> = bsplclib::unit_weights(aNbElems).unwrap();

    assert_eq!(aWeights.len() as i32, aNbElems);
    assert!(!aWeights.is_deletable());

    EXPECT_DOUBLE_EQ(aWeights[1], 1.0);
    EXPECT_DOUBLE_EQ(aWeights[aNbElems], 1.0);
}

#[test]
fn bspline_curve_unit_weights_over_max_size_returns_owning() {
    let aNbElems: i32 = bsplclib::max_unit_weights_size() + 1;
    let aWeights: Array1<'static, f64> = bsplclib::unit_weights(aNbElems).unwrap();

    assert_eq!(aWeights.lower(), 1);
    assert_eq!(aWeights.upper(), aNbElems);
    assert_eq!(aWeights.len() as i32, aNbElems);
    assert!(aWeights.is_deletable());

    for i in 1..=aNbElems {
        EXPECT_DOUBLE_EQ(aWeights[i], 1.0);
    }
}

#[test]
fn bspline_curve_unit_weights_single_element() {
    let aWeights: Array1<'static, f64> = bsplclib::unit_weights(1).unwrap();

    assert_eq!(aWeights.len(), 1);
    assert!(!aWeights.is_deletable());
    EXPECT_DOUBLE_EQ(aWeights[1], 1.0);
}

#[test]
fn bspline_curve_max_unit_weights_size_is_positive() {
    assert!(bsplclib::max_unit_weights_size() > 0);
    assert_eq!(bsplclib::max_unit_weights_size(), 2049);
}
