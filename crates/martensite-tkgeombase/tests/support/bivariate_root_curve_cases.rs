//! The caller's original root and curve regions are one initialized allocation.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerRealArray::IsoRoots, AdvApp2VarCombinedBufferEvaluation as Call,
    AdvApp2VarIntegerBindings as Bindings, AdvApp2VarIntegerField as Integer,
    AdvApp2VarIsoOperationFrame as Frame, AdvApp2VarRealBindings as Reals,
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
fn root_curve_frame() -> Frame {
    let mut values = vec![7.; 69];
    values[5..13].copy_from_slice(&ROOTS);
    Frame::new(
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
            constant_parameter: 4,
        },
    )
    .unwrap()
    .with_caller_array_slots(IsoRoots, (5..13).collect())
    .unwrap()
}
fn root_curve_case(iso: i32) {
    let mut request = curve_request(iso);
    request.ndimen = 1;
    request.ndimse = &[1];
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request.ndgjac = 3;
    request.ncflim = 1;
    let mut frame = root_curve_frame();
    let curve = frame.caller_curve_slots(5, 64).unwrap();
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>, error: &mut i32| {
        assert_eq!(
            (
                call.integer(Integer::Dimension),
                call.integer(Integer::ParameterCount),
                call.integer(Integer::UOrder),
                call.integer(Integer::VOrder)
            ),
            (1, 8, 0, 0)
        );
        assert_eq!(call.caller_array_value(IsoRoots, 0).unwrap(), 0.);
        assert_eq!(call.buffers().parameter(0), 0.5);
        assert_eq!(call.caller_curve_value(0).unwrap(), 0.);
        call.set_caller_array_value(IsoRoots, 0, 0.25).unwrap();
        assert_eq!(call.caller_curve_value(0).unwrap(), 0.25);
        call.set_caller_curve_value(0, 0.75).unwrap();
        assert_eq!(call.caller_array_value(IsoRoots, 0).unwrap(), 0.75);
        call.set_caller_array_value(IsoRoots, 0, 0.).unwrap();
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
    assert_eq!((error, output.nbcrbe, *calls.lock().unwrap()), (0, 1, 1));
    assert_eq!(output.ncoeff, [1]);
    assert_eq!(output.tabdec, [0., 1.]);
    let coefficient = frame.caller_curve_value(0).unwrap();
    assert!((coefficient - 1.).abs() <= 8. * f64::EPSILON);
    assert_eq!(frame.caller_array_value(IsoRoots, 0).unwrap(), coefficient);
    for (index, expected) in ROOTS.iter().enumerate().skip(1) {
        assert_eq!(frame.caller_curve_value(index).unwrap(), *expected);
        assert_eq!(
            frame.caller_array_value(IsoRoots, index).unwrap(),
            *expected
        );
    }
    for index in 8..64 {
        assert_eq!(frame.caller_curve_value(index).unwrap(), 7.);
    }
    assert!(output.errmax[0].abs() <= 8. * f64::EPSILON);
    assert!(output.errmoy[0].abs() <= 8. * f64::EPSILON);
    assert_eq!(output.contr1, [7.; 64]);
    assert_eq!(output.contr2, [7.; 64]);
    assert_eq!(request.rootlg, ROOTS);
}
#[test]
fn first_iso_root_and_curve_share_clear_callback_and_final_coefficient_writes() {
    root_curve_case(1);
}
#[test]
fn second_iso_root_and_curve_share_clear_callback_and_final_coefficient_writes() {
    root_curve_case(2);
}
#[test]
fn caller_curve_regions_reject_bounds_scalar_overlap_and_slice_entrypoints() {
    let mut frame = root_curve_frame();
    for (start, length, kind) in [
        (69, 1, "Standard_OutOfRange"),
        (5, 0, "Standard_OutOfRange"),
        (usize::MAX, 2, "Standard_OutOfRange"),
        (4, 1, "Standard_DomainError"),
    ] {
        assert_eq!(
            frame
                .caller_curve_slots(start, length)
                .err()
                .unwrap()
                .exception_type(),
            kind
        );
    }
    assert!(frame.caller_curve_value(0).is_err());
    let _curve = frame.caller_curve_slots(5, 64).unwrap();
    assert_eq!(
        frame.caller_curve_value(64).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
    let mut request = curve_request(1);
    let mut storage = CurveStorage::new();
    let mut error = -1;
    let callback = |_: &mut Call<'_, '_, '_>, _: &mut i32| {
        panic!("wrong output entrypoint must stop before evaluation")
    };
    let failure = frame
        .approximate_iso_buffers(&mut request, &callback, &mut storage.output(), &mut error)
        .unwrap_err();
    assert!(
        failure
            .message()
            .contains("expected owned caller-curve entrypoint")
    );
}
