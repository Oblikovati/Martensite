//! Captured caller dimensions determine which initialized components truncation examines.
use super::*;
use martensite_tkgeombase::adv_app2var::{AdvApp2VarBufferEvaluation, AdvApp2VarIsoCallerIntegers};
struct ShrinkingCallerDimensions {
    dimensions: Mutex<[i32; 1]>,
    calls: Mutex<usize>,
}
impl AdvApp2VarEvaluatorFunc2Var for ShrinkingCallerDimensions {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("captured caller dimensions use the indexed callback interface")
    }
    fn caller_subspace_dimensions<'a>(&'a self, _: &'a [i32]) -> std::borrow::Cow<'a, [i32]> {
        std::borrow::Cow::Owned(self.dimensions.lock().unwrap().to_vec())
    }
    fn evaluate_iso_buffers(
        &self,
        call: &mut AdvApp2VarBufferEvaluation<'_>,
        _: &mut AdvApp2VarIsoCallerIntegers<'_>,
        error: &mut i32,
    ) {
        assert_eq!((*call.dimension, *call.nb_params), (4, 8));
        assert_eq!((call.orders.u(), call.orders.v()), (0, 0));
        let mut dimensions = self.dimensions.lock().unwrap();
        assert_eq!(*dimensions, [4]);
        *call.dimension = 2;
        dimensions[0] = 2;
        for index in 0..40 {
            call.buffers.set_result(index, 0.);
        }
        for index in 64..72 {
            call.buffers.set_result(index, 1.);
        }
        *self.calls.lock().unwrap() += 1;
        *error = 0;
    }
}
fn caller_dimension_case(iso: i32) {
    const ROOTS: [f64; 8] = [
        -0.9602898564975363,
        -0.7966664774136267,
        -0.525532409916329,
        -0.1834346424956498,
        0.1834346424956498,
        0.525532409916329,
        0.7966664774136267,
        0.9602898564975363,
    ];
    let mut request = curve_request(iso);
    request.ndimen = 4;
    request.ndimse = &[4];
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.iordre = -1;
    request.ndgjac = 3;
    request.nbcrmx = 1;
    request.ncflim = 1;
    let evaluator = ShrinkingCallerDimensions {
        dimensions: Mutex::new([4]),
        calls: Mutex::new(0),
    };
    let mut tables = [[7.; 64]; 5];
    let [curve, sums, differences, first, second] = &mut tables;
    let (mut counts, mut cuts, mut maximum, mut mean) = ([-7], [7.; 2], [7.], [7.]);
    let mut output = IsoApproxOutput {
        ncoeff: &mut counts,
        courbe: curve,
        nbcrbe: -7,
        somtab: sums,
        diftab: differences,
        contr1: first,
        contr2: second,
        tabdec: &mut cuts,
        errmax: &mut maximum,
        errmoy: &mut mean,
    };
    let mut error = -1;
    mma2fnc_mut_(&mut request, &evaluator, &mut output, &mut error);
    assert_eq!((request.ndimen, error, output.nbcrbe), (2, 0, 1));
    assert_eq!(*evaluator.dimensions.lock().unwrap(), [2]);
    assert_eq!(*evaluator.calls.lock().unwrap(), 1);
    assert_eq!(output.ncoeff, [1]);
    assert_eq!(output.tabdec, [0., 1.]);
    assert_eq!(&output.courbe[..4], &[0.; 4]);
    assert_eq!(&output.courbe[4..], &[7.; 60]);
    assert_eq!(output.errmax, [0.]);
    assert_eq!(output.errmoy, [0.]);
    assert_eq!(output.contr1, [7.; 64]);
    assert_eq!(output.contr2, [7.; 64]);
}
#[test]
fn first_iso_truncation_uses_updated_owned_caller_dimensions() {
    caller_dimension_case(1);
}
#[test]
fn second_iso_truncation_uses_updated_owned_caller_dimensions() {
    caller_dimension_case(2);
}
