//! Updated caller roots drive both parameter generation and constraint sampling.
use super::owned_array_cases::{ArrayMode, array_frame};
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarBufferEvaluation, AdvApp2VarCallerSamplingField, AdvApp2VarIsoCallerIntegers,
};
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerRealArray as Array, AdvApp2VarCombinedBufferEvaluation as IndexedCall,
    AdvApp2VarCombinedEvaluation as CombinedCall, AdvApp2VarIntegerField as Integer,
};
const INITIAL_ROOTS: [f64; 10] = [
    -0.9739065285171717,
    -0.8650633666889845,
    -0.6794095682990244,
    -0.4333953941292472,
    -0.14887433898163122,
    0.14887433898163122,
    0.4333953941292472,
    0.6794095682990244,
    0.8650633666889845,
    0.9739065285171717,
];
const REPLACEMENT_ROOTS: [f64; 8] = [
    -0.9602898564975363,
    -0.7966664774136267,
    -0.525532409916329,
    -0.1834346424956498,
    0.1834346424956498,
    0.525532409916329,
    0.7966664774136267,
    0.9602898564975363,
];
struct ReplacingCallerRoots {
    roots: Mutex<[f64; 10]>,
    calls: Mutex<Vec<(i32, i32, i32)>>,
}
impl AdvApp2VarEvaluatorFunc2Var for ReplacingCallerRoots {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("mutable caller roots use the indexed callback interface")
    }
    fn caller_iso_roots<'a>(&'a self, _: &'a [f64]) -> std::borrow::Cow<'a, [f64]> {
        std::borrow::Cow::Owned(self.roots.lock().unwrap().to_vec())
    }
    fn evaluate_iso_buffers(
        &self,
        call: &mut AdvApp2VarBufferEvaluation<'_>,
        callers: &mut AdvApp2VarIsoCallerIntegers<'_>,
        error: &mut i32,
    ) {
        assert_eq!(*call.dimension, 2);
        assert_eq!((call.orders.u(), call.orders.v()), (0, 0));
        let [roots, degree] = callers.sampling().unwrap();
        self.calls
            .lock()
            .unwrap()
            .push((roots, degree, *call.nb_params));
        callers
            .set_sampling(AdvApp2VarCallerSamplingField::RootCount, 8)
            .unwrap();
        callers
            .set_sampling(AdvApp2VarCallerSamplingField::JacobiDegree, 4)
            .unwrap();
        self.roots.lock().unwrap()[..8].copy_from_slice(&REPLACEMENT_ROOTS);
        for point in 0..*call.nb_params as usize {
            let x = 2. * call.buffers.parameter(point) - 1.;
            for component in 0..2 {
                call.buffers.set_result(point * 2 + component, x * x);
            }
        }
        *error = 0;
    }
}
fn caller_roots_case(iso: i32) {
    caller_roots_mode(iso, ArrayMode::Raw);
}
pub(super) fn caller_roots_mode(iso: i32, mode: ArrayMode) {
    let mut request = curve_request(iso);
    request.nbroot = 10;
    request.rootlg = &INITIAL_ROOTS;
    request.iordre = 0;
    request.ndgjac = 5;
    request.nbcrmx = 3;
    request.ncflim = 3;
    let evaluator = ReplacingCallerRoots {
        roots: Mutex::new(INITIAL_ROOTS),
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
    let mut frame = array_frame(Array::IsoRoots);
    let slice = |call: &mut CombinedCall<'_, '_>, result: &mut [f64], error: &mut i32| {
        prepare_owned_roots(call, &evaluator.calls);
        for point in 0..call.integer(Integer::ParameterCount) as usize {
            let x = 2. * call.parameters()[point] - 1.;
            for component in 0..2 {
                result[point * 2 + component] = x * x;
            }
        }
        *error = 0;
    };
    let indexed = |call: &mut IndexedCall<'_, '_, '_>, error: &mut i32| {
        prepare_owned_roots(call, &evaluator.calls);
        for point in 0..call.integer(Integer::ParameterCount) as usize {
            let x = 2. * call.buffers().parameter(point) - 1.;
            for component in 0..2 {
                call.buffers().set_result(point * 2 + component, x * x);
            }
        }
        *error = 0;
    };
    let final_roots = match mode {
        ArrayMode::Raw => {
            mma2fnc_mut_(&mut request, &evaluator, &mut output, &mut error);
            evaluator.roots.lock().unwrap().to_vec()
        }
        ArrayMode::Slice => {
            frame
                .approximate_iso(&mut request, &slice, &mut output, &mut error)
                .unwrap();
            frame.caller_array(Array::IsoRoots).unwrap().to_vec()
        }
        ArrayMode::Indexed => {
            frame
                .approximate_iso_buffers(&mut request, &indexed, &mut output, &mut error)
                .unwrap();
            frame.caller_array(Array::IsoRoots).unwrap().to_vec()
        }
    };
    assert_eq!(
        (request.nbroot, request.ndgjac, error, output.nbcrbe),
        (8, 4, 0, 2)
    );
    assert_eq!(output.ncoeff, [3, 3, -7]);
    assert_eq!(output.tabdec, [0., 0.5, 1., 7.]);
    let expected = [
        0.25, -0.5, 0.25, 0.25, -0.5, 0.25, 0.25, 0.5, 0.25, 0.25, 0.5, 0.25,
    ];
    for (actual, expected) in output.courbe[..12].iter().zip(expected) {
        assert!(
            (actual - expected).abs() <= 4. * f64::EPSILON,
            "{actual} != {expected}"
        );
    }
    assert_eq!(&output.courbe[12..18], &[0.; 6]);
    assert_eq!(&output.courbe[18..], &[7.; 46]);
    for error in output.errmax[..2].iter().chain(output.errmoy[..2].iter()) {
        assert!(error.abs() <= 4. * f64::EPSILON);
    }
    assert_eq!((output.errmax[2], output.errmoy[2]), (7., 7.));
    assert_eq!(
        *evaluator.calls.lock().unwrap(),
        [(10, 5, 12), (8, 4, 10), (8, 4, 10)]
    );
    let mut expected_roots = INITIAL_ROOTS;
    expected_roots[..8].copy_from_slice(&REPLACEMENT_ROOTS);
    assert_eq!(final_roots, expected_roots);
}
fn prepare_owned_roots<Parameters>(
    call: &mut CombinedCall<'_, '_, Parameters>,
    calls: &Mutex<Vec<(i32, i32, i32)>>,
) {
    use AdvApp2VarCallerSamplingField::{JacobiDegree, RootCount};
    assert_eq!(call.integer(Integer::Dimension), 2);
    assert_eq!(
        (call.integer(Integer::UOrder), call.integer(Integer::VOrder)),
        (0, 0)
    );
    calls.lock().unwrap().push((
        call.caller_sampling(RootCount).unwrap(),
        call.caller_sampling(JacobiDegree).unwrap(),
        call.integer(Integer::ParameterCount),
    ));
    call.set_caller_sampling(RootCount, 8).unwrap();
    call.set_caller_sampling(JacobiDegree, 4).unwrap();
    call.caller_array_mut(Array::IsoRoots).unwrap()[..8].copy_from_slice(&REPLACEMENT_ROOTS);
}

#[test]
fn first_iso_replacing_caller_roots_preserves_the_exact_quadratic() {
    caller_roots_case(1);
}
#[test]
fn second_iso_replacing_caller_roots_preserves_the_exact_quadratic() {
    caller_roots_case(2);
}
