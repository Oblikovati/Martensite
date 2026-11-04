//! Retain the reference fixture's result measurement checks and allowances.
#![allow(non_snake_case)]
use super::*;
fn expect_near(actual: f64, expected: f64, tolerance: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{what}: {actual} differs from {expected} by more than {tolerance}"
    );
}
/// Validate result properties against expected values (`ValidateResult` of both fixtures).
pub fn ValidateResult(result: &Shape, surface_area: f64, volume: f64, empty: bool) {
    let tolerance = DefaultTolerance();
    if empty {
        assert!(IsEmpty(result, tolerance), "Result should be empty");
        return;
    }
    assert!(!result.is_null(), "Result shape should not be null");
    if surface_area >= 0.0 {
        expect_near(
            GetSurfaceArea(result),
            surface_area,
            5000.0,
            "Surface area mismatch",
        );
    }
    if volume >= 0.0 {
        expect_near(GetVolume(result), volume, tolerance, "Volume mismatch");
    }
}

#[cfg(test)]
mod tests {
    use super::expect_near;

    #[test]
    fn reference_surface_allowance_accepts_the_boundary_difference() {
        expect_near(131_500.0, 134_500.0, 5_000.0, "surface area mismatch");
    }
}
