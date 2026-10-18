//! Constant pieces pin immediate parameter/result incidence in original storage.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarBufferEvaluation, AdvApp2VarCallerSamplingField, AdvApp2VarIsoCallerIntegers,
};
type ParameterResultObservation = (i32, i32, [f64; 2], [f64; 2]);
struct SharedConstant {
    calls: Mutex<Vec<ParameterResultObservation>>,
}
impl AdvApp2VarEvaluatorFunc2Var for SharedConstant {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("shared storage must use indexed callback access")
    }
    fn evaluate_iso_buffers(
        &self,
        call: &mut AdvApp2VarBufferEvaluation<'_>,
        callers: &mut AdvApp2VarIsoCallerIntegers<'_>,
        error: &mut i32,
    ) {
        let roots = callers.sampling().unwrap()[0];
        let count = *call.nb_params as usize;
        assert_eq!((call.orders.u(), call.orders.v()), (0, 0));
        let before = [
            call.buffers.parameter(count - 2),
            call.buffers.parameter(count - 1),
        ];
        callers
            .set_sampling(AdvApp2VarCallerSamplingField::RootCount, 10)
            .unwrap();
        for index in 0..24 {
            call.buffers.set_result(index, 1.);
        }
        let after = [
            call.buffers.parameter(count - 2),
            call.buffers.parameter(count - 1),
        ];
        self.calls
            .lock()
            .unwrap()
            .push((roots, count as i32, before, after));
        *error = 0;
    }
}
fn parameter_result_case(iso: i32) {
    const ROOTS: [f64; 10] = [
        -0.9602898564975363,
        -0.7966664774136267,
        -0.525532409916329,
        -0.1834346424956498,
        0.1834346424956498,
        0.525532409916329,
        0.7966664774136267,
        0.9602898564975363,
        0.,
        0.,
    ];
    let mut request = curve_request(iso);
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.iordre = 0;
    request.nbcrmx = 3;
    request.ncflim = 1;
    let evaluator = SharedConstant {
        calls: Mutex::new(Vec::new()),
    };
    let mut tables = [[7.; 64]; 5];
    let [curve, sums, differences, first, second] = &mut tables;
    let (mut counts, mut cuts, mut maximum, mut mean) = ([-7; 3], [7.; 4], [7.; 3], [7.; 3]);
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
    assert_eq!((request.nbroot, error, output.nbcrbe), (10, 0, 2));
    assert_eq!(output.ncoeff, [1, 1, -7]);
    assert_eq!(output.tabdec, [0., 0.5, 1., 7.]);
    assert_eq!(&output.courbe[..6], &[1., 1., 1., 1., 0., 0.]);
    assert_eq!(&output.courbe[6..], &[7.; 58]);
    assert_eq!(&output.errmax[..2], &[0., 0.]);
    assert_eq!(&output.errmoy[..2], &[0., 0.]);
    assert_eq!((output.errmax[2], output.errmoy[2]), (7., 7.));
    let calls = evaluator.calls.lock().unwrap();
    assert_eq!(calls.len(), 3);
    assert_eq!(
        calls[0],
        (8, 10, [0.9801449282487682, 1.], [0.9801449282487682, 1.])
    );
    assert_eq!(calls[1], (10, 12, [0.25, 0.5], [1., 1.]));
    assert_eq!(calls[2], (10, 12, [0.75, 1.], [1., 1.]));
}
#[test]
fn first_iso_parameter_result_overlap_retains_immediate_observation() {
    parameter_result_case(1);
}
#[test]
fn second_iso_parameter_result_overlap_retains_immediate_observation() {
    parameter_result_case(2);
}
