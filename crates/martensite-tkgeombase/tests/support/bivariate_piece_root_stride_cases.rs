//! Successful changed-root pieces keep the caller tables' original block offsets.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerSamplingField as Sampling, AdvApp2VarCombinedEvaluation as Call,
    AdvApp2VarIntegerBindings as Integers, AdvApp2VarIntegerField as Local,
    AdvApp2VarIsoOperationFrame as Frame, AdvApp2VarRealBindings as Reals,
};
const ROOTS: [f64; 10] = [
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
fn piece_root_frame() -> Frame {
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
        .with_caller_sampling_slots([5, 6])
        .unwrap()
}
fn constant_fold_expected(table: &mut [f64], offset: usize, roots: usize) {
    let stride = roots / 2 + 1;
    for component in 0..2 {
        let start = offset + component * stride;
        table[start] = 0.;
        table[start + 1..start + stride].fill(2.);
    }
}
fn assert_piece_root_tables(output: &IsoApproxOutput<'_>, derivative: i32) {
    let mut sums = [7.; 64];
    constant_fold_expected(&mut sums, 0, 10);
    if derivative == 1 {
        sums[10..12].fill(1.);
    }
    constant_fold_expected(&mut sums, 0, 8);
    constant_fold_expected(&mut sums, 12, 8);
    let mut differences = [7.; 64];
    differences[..22].fill(0.);
    assert_eq!(output.somtab, sums);
    assert_eq!(output.diftab, differences);
    assert_eq!(output.contr1, [7.; 64]);
    assert_eq!(output.contr2, [7.; 64]);
}
fn assert_piece_root_result(output: &IsoApproxOutput<'_>, derivative: i32) {
    assert_eq!(output.nbcrbe, 2);
    assert_eq!(output.ncoeff, [1, 1, -7]);
    assert_eq!(output.tabdec, [0., 0.5, 1., 7.]);
    for (actual, expected) in output.courbe[..6].iter().zip([1., 1., 1., 1., 0., 0.]) {
        assert!(
            (actual - expected).abs() <= 8. * f64::EPSILON,
            "{actual} != {expected}"
        );
    }
    assert_eq!(&output.courbe[6..], &[7.; 58]);
    for value in output.errmax[..2].iter().chain(output.errmoy[..2].iter()) {
        assert!(value.abs() <= 8. * f64::EPSILON, "{value} != 0");
    }
    assert_eq!((output.errmax[2], output.errmoy[2]), (7., 7.));
    assert_piece_root_tables(output, derivative);
}
fn run_piece_root_case(iso: i32, derivative: i32) {
    let mut request = curve_request(iso);
    request.ideriv = derivative;
    request.nbroot = 10;
    request.rootlg = &ROOTS;
    request.ncflim = 1;
    request.nbcrmx = 3;
    let mut frame = piece_root_frame();
    let calls = Mutex::new(Vec::new());
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        let mut calls = calls.lock().unwrap();
        calls.push((
            call.caller_sampling(Sampling::RootCount).unwrap(),
            call.integer(Local::ParameterCount),
        ));
        assert_eq!(
            (call.integer(Local::UOrder), call.integer(Local::VOrder)),
            if iso == 1 {
                (derivative, 0)
            } else {
                (0, derivative)
            }
        );
        if calls.len() == 1 {
            call.set_caller_sampling(Sampling::RootCount, 8).unwrap();
        }
        assert_eq!(call.caller_sampling(Sampling::JacobiDegree).unwrap(), 2);
        values[..2 * call.integer(Local::ParameterCount) as usize].fill(1.);
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
    assert_eq!(error, 0);
    assert_eq!((request.nbroot, request.ndgjac), (8, 2));
    assert_eq!(*calls.lock().unwrap(), [(10, 10), (8, 8), (8, 8)]);
    assert_piece_root_result(&output, derivative);
}
#[test]
fn changed_root_count_keeps_piece_table_blocks_on_the_first_iso_path() {
    run_piece_root_case(1, 0);
}
#[test]
fn changed_root_count_keeps_piece_table_blocks_on_the_second_iso_path() {
    run_piece_root_case(2, 0);
}

#[test]
fn changed_root_derivative_tables_rescale_only_live_rows_in_piece_blocks() {
    for iso in [1, 2] {
        run_piece_root_case(iso, 1);
    }
}
