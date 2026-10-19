//! Sequential caller status and piece count retain their original storage identity.
use super::curve::IsoApproxRequest;
pub(crate) trait CurveIntegerOutputs: super::curve_cut_storage::CurveCutParameters {
    fn endpoint_piece(&self, _: i32) -> Option<crate::adv_app2var::real_slots::OwnedEndpointSlots> {
        None
    }
    fn error_piece(&self, _: i32) -> Option<crate::adv_app2var::real_slots::OwnedErrorSlots> {
        None
    }
    fn subspace_tolerance(&self, values: &[f64], index: usize) -> f64 {
        values[index]
    }

    fn folded_piece(&self, _: i32) -> Option<crate::adv_app2var::real_slots::OwnedFoldedSlots> {
        None
    }
    fn prepare_sampling(&mut self, _: &mut IsoApproxRequest<'_>) {}
    fn refresh_request(&self, _: &mut IsoApproxRequest<'_>) {}
    fn coefficient_count(&self, values: &[i32], index: i32) -> i32 {
        values[index as usize]
    }
    fn set_coefficient_count(
        &mut self,
        values: &mut [i32],
        index: i32,
        value: i32,
        _: &mut IsoApproxRequest<'_>,
    ) {
        values[index as usize] = value;
    }
    fn subspace_dimension(&self, values: &[i32], index: usize) -> i32 {
        values[index]
    }
    fn status(&self) -> i32;
    fn set_status(&mut self, value: i32, request: &mut IsoApproxRequest<'_>);
    fn pieces(&self) -> i32;
    fn set_pieces(&mut self, value: i32, request: &mut IsoApproxRequest<'_>);
    fn with_status<ResultValue>(
        &mut self,
        operation: impl FnOnce(&mut i32) -> ResultValue,
    ) -> ResultValue;
}
pub(super) struct CurveCoefficientCount<'a, Integers> {
    pub(super) integers: &'a mut Integers,
    pub(super) values: &'a mut [i32],
    pub(super) index: i32,
}
impl<Integers: CurveIntegerOutputs> CurveCoefficientCount<'_, Integers> {
    pub(super) fn dimension(&self, values: &[i32], index: usize) -> i32 {
        self.integers.subspace_dimension(values, index)
    }
    pub(super) fn get(&self) -> i32 {
        self.integers.coefficient_count(self.values, self.index)
    }
    pub(super) fn set(&mut self, value: i32, request: &mut IsoApproxRequest<'_>) {
        self.integers
            .set_coefficient_count(self.values, self.index, value, request);
    }
}
pub(crate) struct BorrowedCurveIntegers<'a> {
    pub(crate) status: &'a mut i32,
    pub(crate) pieces: &'a mut i32,
}
impl super::curve_cut_storage::CurveCutParameters for BorrowedCurveIntegers<'_> {}
impl CurveIntegerOutputs for BorrowedCurveIntegers<'_> {
    fn status(&self) -> i32 {
        *self.status
    }
    fn set_status(&mut self, value: i32, _: &mut IsoApproxRequest<'_>) {
        *self.status = value;
    }
    fn pieces(&self) -> i32 {
        *self.pieces
    }
    fn set_pieces(&mut self, value: i32, _: &mut IsoApproxRequest<'_>) {
        *self.pieces = value;
    }
    fn with_status<ResultValue>(
        &mut self,
        operation: impl FnOnce(&mut i32) -> ResultValue,
    ) -> ResultValue {
        operation(self.status)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn borrowed_outputs_preserve_actual_status_reference_and_sequential_values() {
        let (mut status, mut pieces) = (-1, -7);
        let pointer = &mut status as *mut i32;
        let mut outputs = BorrowedCurveIntegers {
            status: &mut status,
            pieces: &mut pieces,
        };
        let mut request = count_test_request();
        assert_eq!((outputs.status(), outputs.pieces()), (-1, -7));
        outputs.set_status(0, &mut request);
        outputs.set_pieces(2, &mut request);
        let result = outputs.with_status(|actual| {
            assert_eq!(actual as *mut i32, pointer);
            *actual = 101;
            7
        });
        assert_eq!(result, 7);
        assert_eq!((outputs.status(), outputs.pieces()), (101, 2));
        assert_eq!((request.ndimen, request.nbroot), (1, 3));
        assert_eq!((status, pieces), (101, 2));
    }
    #[test]
    fn coefficient_count_view_writes_the_selected_slice_slot() {
        let (mut status, mut pieces) = (0, 0);
        let mut outputs = BorrowedCurveIntegers {
            status: &mut status,
            pieces: &mut pieces,
        };
        let mut values = [-7, -7, -7];
        let mut count = CurveCoefficientCount {
            integers: &mut outputs,
            values: &mut values,
            index: 1,
        };
        let mut request = count_test_request();
        assert_eq!(count.get(), -7);
        count.set(0, &mut request);
        assert_eq!(count.get(), 0);
        count.set(2, &mut request);
        assert_eq!(count.get(), 2);
        assert_eq!(values, [-7, 2, -7]);
        assert_eq!(
            (request.ndimen, request.nbroot, status, pieces),
            (1, 3, 0, 0)
        );
    }
    fn count_test_request() -> IsoApproxRequest<'static> {
        IsoApproxRequest {
            ndimen: 1,
            nbsesp: 1,
            ndimse: &[1],
            uvfonc: [0., 1., 0., 1.],
            tconst: 0.5,
            isofav: 2,
            nbroot: 3,
            rootlg: &[],
            iordre: -1,
            ideriv: 0,
            ndgjac: 2,
            nbcrmx: 1,
            ncflim: 3,
            epsapr: &[1e-6],
        }
    }
}
