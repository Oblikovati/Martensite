//! Closed-form constant approximation and clear-before-validation chronology.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCombinedBufferEvaluation as Call, AdvApp2VarIntegerBindings as Bindings,
    AdvApp2VarIntegerField as Integer, AdvApp2VarIsoOperationFrame as Frame,
    AdvApp2VarRealBindings as Reals, AdvApp2VarRealField as Real,
    approx_f2var::IsoApproxCallerOutput,
};
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
fn scalar_curve_frame(domain: bool) -> (Frame, usize) {
    let constant = if domain { 65 } else { 4 };
    let start = if domain { 1 } else { 4 };
    let mut values = vec![7.; if domain { 66 } else { 68 }];
    values[..4].copy_from_slice(&[0., 1., 0., 1.]);
    values[constant] = 0.5;
    let frame = Frame::new(
        vec![0; 5],
        Bindings {
            dimension: 0,
            favorite_iso: 1,
            parameter_count: 2,
            u_order: 3,
            v_order: 4,
        },
        values,
        Reals {
            domain: [0, 1, 2, 3],
            constant_parameter: constant,
        },
    )
    .unwrap();
    (frame, start)
}
fn scalar_curve_request(iso: i32) -> IsoApproxRequest<'static> {
    let mut request = curve_request(iso);
    request.ndimen = 1;
    request.ndimse = &[1];
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.ndgjac = 3;
    request.ncflim = 1;
    request
}
fn scalar_curve_case(iso: i32, domain: bool) {
    let mut request = scalar_curve_request(iso);
    let (mut frame, start) = scalar_curve_frame(domain);
    let curve = frame.caller_scalar_curve_slots(start, 64).unwrap();
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>, error: &mut i32| {
        assert!(!domain, "collapsed domain must fail before evaluation");
        assert_eq!(call.integer(Integer::Dimension), 1);
        assert_eq!(call.integer(Integer::ParameterCount), 8);
        assert_eq!(call.integer(Integer::UOrder), 0);
        assert_eq!(call.integer(Integer::VOrder), 0);
        assert_eq!(call.real(Real::ConstantParameter), 0.);
        assert_eq!(call.caller_curve_value(0).unwrap(), 0.);
        call.set_real(Real::ConstantParameter, 0.25);
        assert_eq!(call.caller_curve_value(0).unwrap(), 0.25);
        call.set_caller_curve_value(0, 0.75).unwrap();
        assert_eq!(call.real(Real::ConstantParameter), 0.75);
        call.set_real(Real::ConstantParameter, 0.);
        for point in 0..8 {
            call.buffers().set_result(point, 1.);
        }
        *calls.lock().unwrap() += 1;
        *error = 0;
    };
    let (mut sums, mut differences, mut first, mut second) =
        ([7.; 64], [7.; 64], [7.; 64], [7.; 64]);
    let (mut count, mut cuts, mut maximum, mut mean) = ([-7], [7.; 2], [7.], [7.]);
    let mut output = IsoApproxCallerOutput {
        ncoeff: &mut count,
        courbe: curve,
        nbcrbe: -7,
        somtab: &mut sums,
        diftab: &mut differences,
        contr1: &mut first,
        contr2: &mut second,
        tabdec: &mut cuts,
        errmax: &mut maximum,
        errmoy: &mut mean,
    };
    let mut error = -1;
    frame
        .approximate_iso_owned_curve(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    if domain {
        assert_eq!((error, output.nbcrbe, *calls.lock().unwrap()), (1, -7, 0));
        assert_eq!(output.ncoeff, [-7]);
        assert_eq!(output.tabdec, [7.; 2]);
        assert_eq!(output.errmax, [7.]);
        assert_eq!(output.errmoy, [7.]);
        assert_eq!(output.somtab, [7.; 64]);
        assert_eq!(output.diftab, [7.; 64]);
        assert_eq!(request.uvfonc, [0., 0., 0., 1.]);
        assert_eq!(request.tconst, 0.5);
        for (index, expected) in [0., 0., 1.].into_iter().enumerate() {
            assert_eq!(frame.caller_curve_value(index).unwrap(), expected);
        }
        for index in 3..64 {
            assert_eq!(frame.caller_curve_value(index).unwrap(), 7.);
        }
    } else {
        assert_eq!((error, output.nbcrbe, *calls.lock().unwrap()), (0, 1, 1));
        assert_eq!(output.ncoeff, [1]);
        assert_eq!(output.tabdec, [0., 1.]);
        let coefficient = frame.caller_curve_value(0).unwrap();
        assert!((coefficient - 1.).abs() <= 8. * f64::EPSILON);
        assert_eq!(request.tconst, coefficient);
        assert_eq!(frame.real(Real::ConstantParameter), coefficient);
        assert_eq!(request.uvfonc, [0., 1., 0., 1.]);
        for index in 1..64 {
            assert_eq!(frame.caller_curve_value(index).unwrap(), 7.);
        }
        assert!(output.errmax[0].abs() <= 8. * f64::EPSILON);
        assert!(output.errmoy[0].abs() <= 8. * f64::EPSILON);
    }
    assert_eq!(output.contr1, [7.; 64]);
    assert_eq!(output.contr2, [7.; 64]);
    assert_eq!(request.rootlg, ROOTS);
}
#[test]
fn first_iso_constant_and_curve_share_initial_clear_callback_and_final_writes() {
    scalar_curve_case(1, false);
}
#[test]
fn second_iso_constant_and_curve_share_initial_clear_callback_and_final_writes() {
    scalar_curve_case(2, false);
}
#[test]
fn first_iso_output_clear_collapses_shared_domain_before_validation() {
    scalar_curve_case(1, true);
}
#[test]
fn second_iso_output_clear_collapses_shared_domain_before_validation() {
    scalar_curve_case(2, true);
}
