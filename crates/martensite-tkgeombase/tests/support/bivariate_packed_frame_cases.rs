//! Packing and canonical conversion retain shared slots in the original allocation.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarBufferEvaluation, AdvApp2VarCallerSamplingField, AdvApp2VarIsoCallerIntegers,
};
struct GrowingPackedConstant {
    calls: Mutex<Vec<(i32, i32, i32, i32)>>,
}
impl AdvApp2VarEvaluatorFunc2Var for GrowingPackedConstant {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("original allocation incidence requires indexed callback access")
    }
    fn evaluate_iso_buffers(
        &self,
        call: &mut AdvApp2VarBufferEvaluation<'_>,
        callers: &mut AdvApp2VarIsoCallerIntegers<'_>,
        error: &mut i32,
    ) {
        let stride = *call.dimension;
        let count = *call.nb_params;
        self.calls
            .lock()
            .unwrap()
            .push((stride, count, call.orders.u(), call.orders.v()));
        assert_eq!(callers.sampling(), Some([8, 7]));
        *call.dimension = 64;
        callers
            .set_sampling(AdvApp2VarCallerSamplingField::JacobiDegree, 0)
            .unwrap();
        for index in 0..((count - 1) * stride + *call.dimension) as usize {
            call.buffers.set_result(index, 1.);
        }
        *error = 0;
    }
}
fn packed_frame_case(iso: i32) {
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
    request.ndimen = 8;
    request.ndimse = &[8];
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.iordre = -1;
    request.ndgjac = 7;
    request.nbcrmx = 1;
    request.ncflim = 1;
    let evaluator = GrowingPackedConstant {
        calls: Mutex::new(Vec::new()),
    };
    let mut tables = [[7.; 512]; 5];
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
    assert_eq!(
        (request.ndimen, request.ndgjac, error, output.nbcrbe),
        (64, 0, 0, 1)
    );
    assert_eq!(request.ndimse, [8]);
    assert_eq!(output.ncoeff, [1]);
    assert_eq!(output.tabdec, [0., 1.]);
    for (component, actual) in output.courbe[..64].iter().enumerate() {
        let expected = if component < 60 {
            1.
        } else {
            std::f64::consts::FRAC_1_SQRT_2
        };
        assert!(
            (actual - expected).abs() <= 4. * f64::EPSILON,
            "component {component}: {actual} != {expected}"
        );
    }
    assert_eq!(&output.courbe[64..], &[7.; 448]);
    assert_eq!(output.errmax, [0.]);
    assert_eq!(output.errmoy, [0.]);
    assert_eq!(output.contr1, [7.; 512]);
    assert_eq!(output.contr2, [7.; 512]);
    assert_eq!(*evaluator.calls.lock().unwrap(), [(8, 8, 0, 0)]);
}
#[test]
fn first_iso_packed_scratch_and_canonical_coefficients_share_slots() {
    packed_frame_case(1);
}
#[test]
fn second_iso_packed_scratch_and_canonical_coefficients_share_slots() {
    packed_frame_case(2);
}
