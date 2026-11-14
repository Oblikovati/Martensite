//! Retain one angular-control conversion while decoded verification remains independent.
pub(super) fn angular_controls(source: &[[f64; 2]], radius: f64) -> Vec<[f64; 2]> {
    source.iter().map(|[s, z]| [s / radius, *z]).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn division_preserves_height_order_and_signed_zero_bits() {
        let source = [[1., 7.], [3.5, -2.], [-0., 0.]];
        let actual = angular_controls(&source, 2.);
        assert_eq!(actual, [[0.5, 7.], [1.75, -2.], [-0., 0.]]);
        assert_eq!(actual[2][0].to_bits(), (-0_f64).to_bits());
        assert_eq!(
            angular_controls(&[[3., 1.]], 10.)[0][0].to_bits(),
            0.3_f64.to_bits()
        );
    }
}
