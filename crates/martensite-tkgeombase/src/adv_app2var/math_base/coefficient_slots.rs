//! Coefficient reads and writes retain their scalar evaluation order.
pub(crate) trait PolynomialCoefficientSlots {
    fn input_coefficient(&self, index: usize) -> f64;
    fn write_output_coefficient(&mut self, index: usize, value: f64);
}
pub(super) struct SeparateCoefficientSlots<'a> {
    pub(super) input: &'a [f64],
    pub(super) output: &'a mut [f64],
}
impl PolynomialCoefficientSlots for SeparateCoefficientSlots<'_> {
    fn input_coefficient(&self, index: usize) -> f64 {
        self.input[index]
    }
    fn write_output_coefficient(&mut self, index: usize, value: f64) {
        self.output[index] = value;
    }
}
#[cfg(test)]
mod tests {
    use super::super::{mmapcmp_, mmjacan_};
    #[test]
    fn disjoint_packing_preserves_odd_padding_in_each_component() {
        let mut packed = [99.; 8];
        mmapcmp_(2, 4, 3, &[1., 2., 3., 4., 5., 6., 7., 8.], &mut packed);
        assert_eq!(packed, [1., 3., 2., 99., 5., 7., 6., 99.]);
    }
    #[test]
    fn disjoint_conversion_matches_normalized_legendre_closed_forms() {
        let packed = [2_f64.sqrt(), (8_f64 / 5.).sqrt(), (2_f64 / 3.).sqrt(), 99.];
        let mut canonical = [99.; 3];
        mmjacan_(-1, 2, &packed, &mut canonical);
        for (actual, expected) in canonical.into_iter().zip([0., 1., 3.]) {
            assert!((actual - expected).abs() <= 4. * f64::EPSILON);
        }
    }
}
