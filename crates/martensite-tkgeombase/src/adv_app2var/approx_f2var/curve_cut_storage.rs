//! Sequential cut-table access keeps borrowed defaults and owned origins in one pipeline.
use super::curve::IsoApproxRequest;
use crate::adv_app2var::sys_base::mcrfill_within;
pub(crate) trait CurveCutParameters {
    fn cut_parameter(&self, values: &[f64], index: i32) -> f64 {
        values[index as usize]
    }
    fn set_cut_parameter(
        &mut self,
        values: &mut [f64],
        index: i32,
        value: f64,
        _: &mut IsoApproxRequest<'_>,
    ) {
        values[index as usize] = value;
    }
    fn shift_cut_parameters(
        &mut self,
        bytes: i32,
        values: &mut [f64],
        source: i32,
        target: i32,
        _: &mut IsoApproxRequest<'_>,
    ) {
        mcrfill_within(bytes, values, source as usize, target as usize);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    struct BorrowedCuts;
    impl CurveCutParameters for BorrowedCuts {}
    #[test]
    fn borrowed_cut_access_preserves_signed_zero_and_overlapping_copy_order() {
        let mut request = crate::adv_app2var::slice_callback_fixture::request();
        let mut storage = BorrowedCuts;
        let mut values = [1., 2., 3., 4.];
        assert_eq!(storage.cut_parameter(&values, 2), 3.);
        storage.set_cut_parameter(&mut values, 1, -0., &mut request);
        assert_eq!(
            storage.cut_parameter(&values, 1).to_bits(),
            (-0_f64).to_bits()
        );
        storage.shift_cut_parameters(16, &mut values, 0, 1, &mut request);
        assert_eq!(
            values.map(f64::to_bits),
            [
                1_f64.to_bits(),
                1_f64.to_bits(),
                (-0_f64).to_bits(),
                4_f64.to_bits()
            ]
        );
        assert_eq!(
            (request.ndimen, request.nbcrmx, request.uvfonc),
            (1, 1, [0., 1., 0., 1.])
        );
    }
}
