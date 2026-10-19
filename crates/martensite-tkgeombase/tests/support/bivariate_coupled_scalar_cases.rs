//! Completed scalar growth/shrink coupling inside the original workspace bounds.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerSamplingField as Sampling, AdvApp2VarCombinedEvaluation as Call,
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
fn coupled_frame() -> Frame {
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
    Frame::new(vec![0; 8], integers, vec![0.; 5], reals)
        .unwrap()
        .with_subspace_count_slot(5)
        .unwrap()
        .with_caller_sampling_slots([6, 7])
        .unwrap()
}
fn assert_coupled_result(output: &IsoApproxOutput<'_>) {
    assert_eq!(output.nbcrbe, 1);
    assert_eq!(output.ncoeff, [1, -7, -7]);
    assert_eq!(output.tabdec, [0., 1., 7., 7.]);
    assert_eq!(&output.courbe[..2], &[1.; 2]);
    assert_eq!(&output.courbe[2..12], &[0.; 10]);
    assert_eq!(&output.courbe[12..], &[7.; 52]);
    assert_eq!((output.errmax[0], output.errmoy[0]), (0., 0.));
    assert_eq!(&output.errmax[1..], &[7.; 5]);
    assert_eq!(&output.errmoy[1..], &[7.; 5]);
    assert_eq!(&output.contr1[..2], &[1.; 2]);
    assert_eq!(&output.contr2[..2], &[1.; 2]);
    assert_eq!(&output.contr1[2..], &[7.; 62]);
    assert_eq!(&output.contr2[2..], &[7.; 62]);
}
fn evaluate_coupled(call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32) {
    let dimension = call.integer(Local::Dimension);
    let parameters = call.integer(Local::ParameterCount);
    assert_eq!(
        (
            dimension,
            call.subspace_count(),
            call.caller_sampling(Sampling::JacobiDegree).unwrap(),
            parameters
        ),
        (4, 2, 2, 10)
    );
    assert_eq!(
        (call.integer(Local::UOrder), call.integer(Local::VOrder)),
        (0, 0)
    );
    call.set_integer(Local::Dimension, 2);
    call.set_subspace_count(1);
    call.set_caller_sampling(Sampling::JacobiDegree, 3).unwrap();
    assert_eq!(
        (
            call.integer(Local::Dimension),
            call.subspace_count(),
            call.caller_sampling(Sampling::JacobiDegree).unwrap()
        ),
        (2, 1, 3)
    );
    assert_eq!(call.caller_sampling(Sampling::RootCount).unwrap(), 8);
    values[..(dimension * parameters) as usize].fill(1.);
    *error = 0;
}
fn run_coupled_case(iso: i32) {
    let mut request = curve_request(iso);
    request.ndimen = 4;
    request.nbsesp = 2;
    request.ndimse = &[2, 2];
    request.epsapr = &[1e-6, 1e-6];
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.iordre = 0;
    request.ncflim = 1;
    request.nbcrmx = 3;
    let mut frame = coupled_frame();
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        *calls.lock().unwrap() += 1;
        evaluate_coupled(call, values, error);
    };
    let mut tables = [[7.; 64]; 5];
    let [curve, sums, differences, first, second] = &mut tables;
    let (mut counts, mut cuts, mut maximum, mut mean) = ([-7; 3], [7.; 4], [7.; 6], [7.; 6]);
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
    assert_eq!(
        (request.ndimen, request.nbsesp, request.ndgjac, error),
        (2, 1, 3, 0)
    );
    assert_eq!(request.ndimse, &[2, 2]);
    assert_eq!(*calls.lock().unwrap(), 1);
    assert_eq!(
        (
            frame.integer_values()[0],
            frame.integer_values()[5],
            frame.integer_values()[7]
        ),
        (2, 1, 3)
    );
    assert_coupled_result(&output);
}
#[test]
fn degree_growth_with_dimension_and_subspace_shrink_completes_on_first_iso_path() {
    run_coupled_case(1);
}
#[test]
fn degree_growth_with_dimension_and_subspace_shrink_completes_on_second_iso_path() {
    run_coupled_case(2);
}
