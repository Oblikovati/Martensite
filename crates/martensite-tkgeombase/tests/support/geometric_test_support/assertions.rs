//! Retained development-test assertion contracts.
#![allow(non_snake_case, dead_code)]

/// Numeric assertion `EXPECT_DOUBLE_EQ`: at most four ULPs apart, never NaN.
pub fn EXPECT_DOUBLE_EQ(a: f64, b: f64) {
    assert!(
        !a.is_nan() && !b.is_nan(),
        "EXPECT_DOUBLE_EQ on NaN: {a} vs {b}"
    );
    assert!(
        ordered_bits(a).abs_diff(ordered_bits(b)) <= 4,
        "EXPECT_DOUBLE_EQ: {a} != {b}"
    );
}

/// Numeric assertion `EXPECT_NEAR`: `|a - b| <= tolerance`.
pub fn EXPECT_NEAR(a: f64, b: f64, tolerance: f64) {
    assert!(
        (a - b).abs() <= tolerance,
        "EXPECT_NEAR: |{a} - {b}| > {tolerance}"
    );
}

// Maps doubles to integers whose order matches the numeric order (Numeric assertion's
// sign-and-magnitude to biased conversion).
fn ordered_bits(value: f64) -> u64 {
    let bits: u64 = value.to_bits();
    if bits >> 63 != 0 {
        (!bits).wrapping_add(1)
    } else {
        bits | (1 << 63)
    }
}

/// Numeric assertion `EXPECT_THROW(statement, Type)`: the statement fails with the retained
/// exception `exception`.
pub fn EXPECT_THROW<T>(result: Result<T, martensite_tkernel::standard::Failure>, exception: &str) {
    match result {
        Ok(_) => panic!("EXPECT_THROW: no exception; expected {exception}"),
        Err(error) => assert_eq!(
            error.exception_type(),
            exception,
            "EXPECT_THROW: wrong exception; expected {exception}"
        ),
    }
}
