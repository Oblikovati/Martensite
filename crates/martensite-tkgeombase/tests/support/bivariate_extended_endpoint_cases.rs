//! Zero-function endpoint derivatives read the live parameter region after root growth.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarBufferEvaluation, AdvApp2VarCallerSamplingField, AdvApp2VarIsoCallerIntegers,
};
type EndpointObservation = (i32, i32, i32, i32, f64, f64);
struct ExtendedZero {
    calls: Mutex<Vec<EndpointObservation>>,
}
impl AdvApp2VarEvaluatorFunc2Var for ExtendedZero {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("extended endpoint callback must use indexed buffer access")
    }
    fn evaluate_iso_buffers(
        &self,
        call: &mut AdvApp2VarBufferEvaluation<'_>,
        callers: &mut AdvApp2VarIsoCallerIntegers<'_>,
        error: &mut i32,
    ) {
        let roots = callers.sampling().unwrap()[0];
        let count = *call.nb_params;
        let parameter = call.buffers.parameter((count - 1) as usize);
        callers
            .set_sampling(AdvApp2VarCallerSamplingField::RootCount, 10)
            .unwrap();
        for index in 0..if count > 1 { 24 } else { 2 } {
            call.buffers.set_result(index, 0.);
        }
        let after = call.buffers.parameter((count - 1) as usize);
        self.calls.lock().unwrap().push((
            roots,
            count,
            call.orders.u(),
            call.orders.v(),
            parameter,
            after,
        ));
        *error = 0;
    }
}
fn extended_endpoint_case(iso: i32) {
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
    request.iordre = 1;
    request.ndgjac = 4;
    request.nbcrmx = 3;
    request.ncflim = 4;
    let evaluator = ExtendedZero {
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
    assert_eq!(&output.courbe[..24], &[0.; 24]);
    assert_eq!(&output.courbe[24..], &[7.; 40]);
    assert_eq!(&output.errmax[..2], &[0., 0.]);
    assert_eq!(&output.errmoy[..2], &[0., 0.]);
    assert_eq!((output.errmax[2], output.errmoy[2]), (7., 7.));
    let (u, v) = if iso == 1 { (0, 1) } else { (1, 0) };
    assert_eq!(
        *evaluator.calls.lock().unwrap(),
        [
            (8, 10, 0, 0, 1., 1.),
            (10, 1, u, v, 0., 0.),
            (10, 1, u, v, 0., 0.),
            (10, 12, 0, 0, 0.5, 0.),
            (10, 1, u, v, 0., 0.),
            (10, 1, u, v, 0., 0.),
            (10, 12, 0, 0, 1., 0.),
            (10, 1, u, v, 0.5, 0.5),
            (10, 1, u, v, 0., 0.),
        ]
    );
}
#[test]
fn first_iso_derivative_endpoint_uses_extended_parameter_region() {
    extended_endpoint_case(1);
}
#[test]
fn second_iso_derivative_endpoint_uses_extended_parameter_region() {
    extended_endpoint_case(2);
}
