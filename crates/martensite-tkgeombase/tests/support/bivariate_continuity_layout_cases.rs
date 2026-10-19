//! A continuity shrink keeps original constraint blocks and defined workspace incidence.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerOrderField as Order, AdvApp2VarCombinedEvaluation as Call,
    AdvApp2VarIntegerBindings as Integers, AdvApp2VarIntegerField as Local,
    AdvApp2VarIsoOperationFrame as Frame, AdvApp2VarRealBindings as Reals,
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
fn continuity_frame() -> Frame {
    let integers = Integers {
        dimension: 0,
        favorite_iso: 1,
        parameter_count: 2,
        u_order: 3,
        v_order: 4,
    };
    let reals = Reals {
        domain: [0, 1, 2, 3],
        constant_parameter: 4,
    };
    Frame::new(vec![0; 7], integers, vec![0.; 5], reals)
        .unwrap()
        .with_caller_order_slots([5, 6])
        .unwrap()
}
fn assert_continuity_result(output: &IsoApproxOutput<'_>) {
    assert_eq!(output.nbcrbe, 2);
    assert_eq!(output.ncoeff, [2, 2, -7]);
    assert_eq!(output.tabdec, [0., 0.5, 1., 7.]);
    let expected = [0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0., 0., 0., 0.];
    for (actual, expected) in output.courbe[..12].iter().zip(expected) {
        assert!(
            (actual - expected).abs() <= 8. * f64::EPSILON,
            "{actual} != {expected}"
        );
    }
    assert_eq!(&output.courbe[12..], &[7.; 52]);
    for error in output.errmax[..2].iter().chain(output.errmoy[..2].iter()) {
        assert!(error.abs() <= 8. * f64::EPSILON, "{error} != 0");
    }
    assert_eq!((output.errmax[2], output.errmoy[2]), (7., 7.));
}
fn assert_continuity_constraints(output: &IsoApproxOutput<'_>) {
    let (mut first, mut second) = ([7.; 64], [7.; 64]);
    first[..2].fill(1.);
    first[6..8].fill(0.);
    second[..2].fill(0.);
    second[6..8].fill(1.);
    assert_eq!(output.contr1, first);
    assert_eq!(output.contr2, second);
}
fn run_continuity_case(iso: i32) {
    let mut request = curve_request(iso);
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.ndgjac = 4;
    request.iordre = 1;
    request.ncflim = 2;
    request.nbcrmx = 3;
    let mut frame = continuity_frame();
    let trace = Mutex::new(Vec::new());
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        let mut trace = trace.lock().unwrap();
        let continuity = call.caller_order(Order::ContinuityOrder).unwrap();
        let count = call.integer(Local::ParameterCount);
        trace.push((continuity, count));
        assert_eq!(
            (call.integer(Local::UOrder), call.integer(Local::VOrder)),
            (0, 0)
        );
        if trace.len() == 1 {
            call.set_caller_order(Order::ContinuityOrder, 0).unwrap();
        }
        assert_eq!(call.caller_order(Order::CrossDerivativeOrder).unwrap(), 0);
        for (i, value) in values[..2 * count as usize].iter_mut().enumerate() {
            *value = (2. * call.parameters()[i / 2] - 1.).abs();
        }
        *error = 0;
    };
    let mut tables = [[7.; 64]; 5];
    let [curve, sums, differences, first, second] = &mut tables;
    let (mut counts, mut cuts, mut maximum, mut mean) = ([-7; 3], [7.; 4], [7.; 3], [7.; 3]);
    let mut output = IsoApproxOutput {
        ncoeff: &mut counts,
        courbe: curve,
        nbcrbe: 0,
        somtab: sums,
        diftab: differences,
        contr1: first,
        contr2: second,
        tabdec: &mut cuts,
        errmax: &mut maximum,
        errmoy: &mut mean,
    };
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!((request.iordre, request.ideriv, error), (0, 0, 0));
    assert_eq!(*trace.lock().unwrap(), [(1, 10), (0, 10), (0, 10)]);
    assert_continuity_result(&output);
    assert_continuity_constraints(&output);
}
#[test]
fn persistent_continuity_shrink_completes_exact_v_profile_on_first_iso_path() {
    run_continuity_case(1);
}
#[test]
fn persistent_continuity_shrink_completes_exact_v_profile_on_second_iso_path() {
    run_continuity_case(2);
}
