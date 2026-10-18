//! Captured piece indices can address initialized caller prefixes while the live cursor changes.
use super::iso_integer_output_cases::{Bindings, Call, Frame, Int, Output, ROOTS8, Reals};
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerCurveSlots as CurveSlots, AdvApp2VarCallerRealArray as Array,
    AdvApp2VarEndpointOutput as Endpoint, AdvApp2VarErrorOutput as ErrorRole,
    AdvApp2VarFoldedOutput as Folded, approx_f2var::IsoApproxCallerOutput,
};
fn initial_integers(iso: i32) -> Vec<i32> {
    let mut values = vec![-7; 128];
    values[32..43].copy_from_slice(&[1, 1, iso, 8, -1, 0, 2, 1, 3, -7, -7]);
    values[48..50].copy_from_slice(&[1, 1]);
    values
}
fn initial_reals() -> Vec<f64> {
    let mut values = vec![7.; 1100];
    values[16..20].copy_from_slice(&[0., 1., 0., 1.]);
    values[32..40].copy_from_slice(&ROOTS8);
    values[608..612].fill(1e-6);
    values[1024] = 0.5;
    values
}
fn integer_frame(iso: i32) -> Frame {
    Frame::new(
        initial_integers(iso),
        Bindings {
            dimension: 32,
            favorite_iso: 34,
            parameter_count: 0,
            u_order: 1,
            v_order: 2,
        },
        initial_reals(),
        Reals {
            domain: [16, 17, 18, 19],
            constant_parameter: 1024,
        },
    )
    .unwrap()
    .with_subspace_count_slot(33)
    .unwrap()
    .with_caller_order_slots([36, 37])
    .unwrap()
    .with_caller_sampling_slots([35, 38])
    .unwrap()
    .with_caller_limit_slots([39, 40])
    .unwrap()
    .with_caller_dimension_slots(vec![48, 49])
    .unwrap()
    .with_caller_output_slots([42, 41])
    .unwrap()
    .with_coefficient_count_origin(64)
    .unwrap()
}
fn signed_piece_frame(iso: i32) -> (Frame, CurveSlots) {
    let mut frame = integer_frame(iso)
        .with_caller_array_slots(Array::IsoRoots, (32..40).collect())
        .unwrap()
        .with_caller_array_slots(Array::SubspaceTolerances, (608..612).collect())
        .unwrap()
        .with_cut_parameter_origin(448)
        .unwrap()
        .with_folded_output_origins([192, 256])
        .unwrap()
        .with_endpoint_output_origins([320, 384])
        .unwrap()
        .with_error_output_origins([512, 576])
        .unwrap();
    let curve = frame.caller_curve_slots(64, 128).unwrap();
    (frame, curve)
}
fn signed_piece_request(iso: i32) -> IsoApproxRequest<'static> {
    IsoApproxRequest {
        ndimen: 1,
        nbsesp: 1,
        ndimse: &[1, 1],
        uvfonc: [0., 1., 0., 1.],
        tconst: 0.5,
        isofav: iso,
        nbroot: 8,
        rootlg: &ROOTS8,
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 3,
        epsapr: &[1e-6; 4],
    }
}
fn signed_piece_output(
    tables: &mut CurveStorage,
    curve: CurveSlots,
) -> IsoApproxCallerOutput<'_, CurveSlots> {
    IsoApproxCallerOutput {
        ncoeff: &mut tables.counts,
        courbe: curve,
        nbcrbe: -7,
        somtab: &mut tables.sums,
        diftab: &mut tables.differences,
        contr1: &mut tables.first,
        contr2: &mut tables.second,
        tabdec: &mut tables.cuts,
        errmax: &mut tables.maximum,
        errmoy: &mut tables.mean,
    }
}
fn evaluate_constant(call: &mut Call<'_, '_, '_>, ordinal: i32, prefix: bool) {
    assert_eq!(call.integer(Int::Dimension), 1);
    assert_eq!(call.integer(Int::ParameterCount), 8);
    assert_eq!(call.integer(Int::UOrder), 0);
    assert_eq!(call.integer(Int::VOrder), 0);
    assert_eq!(
        call.caller_output(Output::CompletedPieces).unwrap(),
        if ordinal == 2 { -1 } else { 0 }
    );
    for (index, root) in ROOTS8.iter().enumerate() {
        let parameter = if ordinal == 2 {
            -3.5 * root + 3.5
        } else {
            0.5 * root + 0.5
        };
        assert_eq!(
            call.buffers().parameter(index).to_bits(),
            parameter.to_bits()
        );
        call.buffers().set_result(index, 1.);
    }
    if prefix && ordinal == 1 {
        call.set_caller_output(Output::CompletedPieces, -2).unwrap();
    }
    call.set_caller_output(Output::Status, 0).unwrap();
}
fn run_signed_piece(iso: i32, prefix: bool) {
    let (mut frame, curve) = signed_piece_frame(iso);
    let (mut request, mut tables) = (signed_piece_request(iso), CurveStorage::new());
    let mut output = signed_piece_output(&mut tables, curve);
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        let mut ordinal = calls.lock().unwrap();
        *ordinal += 1;
        assert!(*ordinal <= 3);
        evaluate_constant(call, *ordinal, prefix);
    };
    frame
        .approximate_iso_owned_output_curve(&mut request, &evaluator, &mut output)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), if prefix { 3 } else { 1 });
    assert_eq!(frame.caller_output(Output::Status).unwrap(), 0);
    assert_eq!(output.nbcrbe, 1);
    assert_eq!(output.ncoeff, &[1]);
    assert_eq!(output.tabdec, &[0., 1.]);
    expectations::check_published_owner(&frame, iso, prefix);
}
#[test]
fn signed_piece_control_iso1() {
    run_signed_piece(1, false);
}
#[test]
fn signed_piece_control_iso2() {
    run_signed_piece(2, false);
}
#[test]
fn signed_piece_prefix_iso1() {
    run_signed_piece(1, true);
}
#[test]
fn signed_piece_prefix_iso2() {
    run_signed_piece(2, true);
}

#[path = "bivariate_signed_piece_contracts.rs"]
mod contracts;
#[path = "bivariate_signed_piece_expectations.rs"]
mod expectations;

#[path = "bivariate_entry_output_layout_cases.rs"]
mod entry_layout;
