//! Scoped output access allows one owner without keeping borrows across evaluation.
use super::curve::IsoApproxCallerOutput;
use super::curve_integers::BorrowedCurveIntegers;
// Preserve disjoint borrows of original fields without moving any caller storage.
pub(crate) struct CurveCallerTables<'a, Curve> {
    pub(crate) ncoeff: &'a mut [i32],
    pub(crate) courbe: &'a mut Curve,
    pub(crate) somtab: &'a mut [f64],
    pub(crate) diftab: &'a mut [f64],
    pub(crate) contr1: &'a mut [f64],
    pub(crate) contr2: &'a mut [f64],
    pub(crate) tabdec: &'a mut [f64],
    pub(crate) errmax: &'a mut [f64],
    pub(crate) errmoy: &'a mut [f64],
}
pub(crate) fn borrowed_curve_outputs<'a, Curve>(
    out: &'a mut IsoApproxCallerOutput<'_, Curve>,
    status: &'a mut i32,
) -> (CurveCallerTables<'a, Curve>, BorrowedCurveIntegers<'a>) {
    let (tables, pieces) = borrowed_curve_tables(out);
    (tables, BorrowedCurveIntegers { status, pieces })
}
pub(crate) fn borrowed_curve_tables<'a, Curve>(
    out: &'a mut IsoApproxCallerOutput<'_, Curve>,
) -> (CurveCallerTables<'a, Curve>, &'a mut i32) {
    (
        CurveCallerTables {
            ncoeff: out.ncoeff,
            courbe: &mut out.courbe,
            somtab: out.somtab,
            diftab: out.diftab,
            contr1: out.contr1,
            contr2: out.contr2,
            tabdec: out.tabdec,
            errmax: out.errmax,
            errmoy: out.errmoy,
        },
        &mut out.nbcrbe,
    )
}

pub(crate) trait CurveCoefficientStorage {
    fn with_curve_values<ResultValue>(
        &mut self,
        operation: impl FnOnce(&mut [f64]) -> ResultValue,
    ) -> ResultValue;
    fn with_curve_values_at<ResultValue>(
        &mut self,
        origin: i32,
        operation: impl FnOnce(&mut [f64]) -> ResultValue,
    ) -> ResultValue {
        self.with_curve_values(|values| operation(&mut values[origin as usize..]))
    }
}
impl CurveCoefficientStorage for &mut [f64] {
    fn with_curve_values<ResultValue>(
        &mut self,
        operation: impl FnOnce(&mut [f64]) -> ResultValue,
    ) -> ResultValue {
        operation(self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scoped_slice_access_preserves_length_and_unwritten_tail() {
        let mut values = [7.; 4];
        let length = (&mut values[..]).with_curve_values(|curve| {
            curve[..2].fill(0.);
            curve.len()
        });
        assert_eq!(length, 4);
        assert_eq!(values, [0., 0., 7., 7.]);
    }
    #[test]
    fn repeated_scopes_observe_previous_output_writes() {
        let mut values = [1., 2.];
        (&mut values[..]).with_curve_values(|curve| curve[0] = 3.);
        let sum = (&mut values[..]).with_curve_values(|curve| curve.iter().sum::<f64>());
        assert_eq!(sum, 5.);
    }
    #[test]
    fn offset_curve_scope_keeps_the_suffix_and_unwritten_prefix() {
        let mut values = [7.; 5];
        (&mut values[..]).with_curve_values_at(2, |suffix| {
            assert_eq!(suffix.len(), 3);
            suffix[0] = 3.;
        });
        assert_eq!(values, [7., 7., 3., 7., 7.]);
    }
}
