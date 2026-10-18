//! A callback's stricter requested limit must not be replaced with its earlier value.
use super::owned_array_cases::{ArrayMode, array_frame};
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerRealArray as Array, AdvApp2VarCombinedBufferEvaluation as IndexedCall,
    AdvApp2VarCombinedEvaluation as CombinedCall, AdvApp2VarIntegerField as Integer,
};
struct TighteningCallerTolerance {
    tolerances: Mutex<[f64; 1]>,
    calls: Mutex<usize>,
}
impl AdvApp2VarEvaluatorFunc2Var for TighteningCallerTolerance {
    fn caller_subspace_tolerances<'a>(&'a self, _: &'a [f64]) -> std::borrow::Cow<'a, [f64]> {
        std::borrow::Cow::Owned(self.tolerances.lock().unwrap().to_vec())
    }
    fn evaluate(&self, call: &AdvApp2VarEvaluation<'_>, result: &mut [f64], error: &mut i32) {
        assert_eq!(
            (call.dimension, call.nb_params, call.u_order, call.v_order),
            (1, 8, 0, 0)
        );
        let mut tolerances = self.tolerances.lock().unwrap();
        assert_eq!(*tolerances, [2.]);
        tolerances[0] = 1e-12;
        for (result, parameter) in result.iter_mut().zip(&call.parameters[..8]) {
            *result = 2. * parameter - 1.;
        }
        *self.calls.lock().unwrap() += 1;
        *error = 0;
    }
}
fn caller_tolerance_case(iso: i32) {
    caller_tolerance_mode(iso, ArrayMode::Raw);
}
pub(super) fn caller_tolerance_mode(iso: i32, mode: ArrayMode) {
    caller_tolerance_configuration(iso, mode, false);
}
pub(super) fn caller_tolerance_configuration(iso: i32, mode: ArrayMode, shared: bool) {
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
    request.ndimen = 1;
    request.ndimse = &[1];
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.iordre = -1;
    request.ndgjac = 3;
    request.nbcrmx = 1;
    request.ncflim = 1;
    request.epsapr = if shared { &[0.5] } else { &[2.] };
    let evaluator = TighteningCallerTolerance {
        tolerances: Mutex::new([2.]),
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
    let mut error = 0;
    let mut frame = array_frame(Array::SubspaceTolerances);
    if shared {
        frame = frame
            .with_caller_array_slots(Array::SubspaceTolerances, vec![4])
            .unwrap();
    }
    let slice = |call: &mut CombinedCall<'_, '_>, result: &mut [f64], error: &mut i32| {
        prepare_owned_tolerance(call, &evaluator.calls, shared);
        for (point, value) in result.iter_mut().enumerate().take(8) {
            *value = 2. * call.parameters()[point] - 1.;
        }
        *error = 0;
    };
    let indexed = |call: &mut IndexedCall<'_, '_, '_>, error: &mut i32| {
        prepare_owned_tolerance(call, &evaluator.calls, shared);
        for point in 0..8 {
            let x = 2. * call.buffers().parameter(point) - 1.;
            call.buffers().set_result(point, x);
        }
        *error = 0;
    };
    let final_tolerances = match mode {
        ArrayMode::Raw => {
            mma2fnc_mut_(&mut request, &evaluator, &mut output, &mut error);
            evaluator.tolerances.lock().unwrap().to_vec()
        }
        ArrayMode::Slice => {
            frame
                .approximate_iso(&mut request, &slice, &mut output, &mut error)
                .unwrap();
            if shared {
                vec![
                    frame
                        .caller_array_value(Array::SubspaceTolerances, 0)
                        .unwrap(),
                ]
            } else {
                frame
                    .caller_array(Array::SubspaceTolerances)
                    .unwrap()
                    .to_vec()
            }
        }
        ArrayMode::Indexed => {
            frame
                .approximate_iso_buffers(&mut request, &indexed, &mut output, &mut error)
                .unwrap();
            if shared {
                vec![
                    frame
                        .caller_array_value(Array::SubspaceTolerances, 0)
                        .unwrap(),
                ]
            } else {
                frame
                    .caller_array(Array::SubspaceTolerances)
                    .unwrap()
                    .to_vec()
            }
        }
    };
    if shared {
        assert_eq!(request.tconst, 1e-12);
        assert_eq!(
            frame.real(martensite_tkgeombase::adv_app2var::AdvApp2VarRealField::ConstantParameter),
            1e-12
        );
    }
    assert_eq!((error, output.nbcrbe), (-1, 1));
    assert_eq!(final_tolerances, [1e-12]);
    assert_eq!(*evaluator.calls.lock().unwrap(), 1);
    assert_eq!(output.ncoeff, [1]);
    assert_eq!(output.tabdec, [0., 1.]);
    assert!(output.courbe[0].abs() <= 8. * f64::EPSILON);
    assert_eq!(&output.courbe[1..], &[7.; 63]);
    assert!((output.errmax[0] - 1.).abs() <= 8. * f64::EPSILON);
    assert!((output.errmoy[0] - 1. / 3_f64.sqrt()).abs() <= 8. * f64::EPSILON);
    assert!(output.errmax[0] > final_tolerances[0]);
    assert_eq!(output.contr1, [7.; 64]);
    assert_eq!(output.contr2, [7.; 64]);
}
fn prepare_owned_tolerance<Parameters>(
    call: &mut CombinedCall<'_, '_, Parameters>,
    calls: &Mutex<usize>,
    shared: bool,
) {
    assert_eq!(
        (
            call.integer(Integer::Dimension),
            call.integer(Integer::ParameterCount),
            call.integer(Integer::UOrder),
            call.integer(Integer::VOrder)
        ),
        (1, 8, 0, 0)
    );
    if shared {
        use martensite_tkgeombase::adv_app2var::AdvApp2VarRealField::ConstantParameter;
        assert_eq!(call.real(ConstantParameter), 0.5);
        assert_eq!(
            call.caller_array_value(Array::SubspaceTolerances, 0)
                .unwrap(),
            0.5
        );
        call.set_caller_array_value(Array::SubspaceTolerances, 0, 1e-12)
            .unwrap();
        assert_eq!(call.real(ConstantParameter), 1e-12);
        call.set_real(ConstantParameter, 0.5);
        assert_eq!(
            call.caller_array_value(Array::SubspaceTolerances, 0)
                .unwrap(),
            0.5
        );
        call.set_caller_array_value(Array::SubspaceTolerances, 0, 1e-12)
            .unwrap();
    } else {
        assert_eq!(call.caller_array(Array::SubspaceTolerances).unwrap(), [2.]);
        call.caller_array_mut(Array::SubspaceTolerances).unwrap()[0] = 1e-12;
    }
    *calls.lock().unwrap() += 1;
}

#[test]
fn first_iso_retains_the_stricter_live_tolerance_and_reports_unmet_precision() {
    caller_tolerance_case(1);
}
#[test]
fn second_iso_retains_the_stricter_live_tolerance_and_reports_unmet_precision() {
    caller_tolerance_case(2);
}
