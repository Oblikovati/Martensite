//! Caller continuity/cross controls never become callback-local derivative slots.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerOrderField as Caller, AdvApp2VarCombinedEvaluation as Call,
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
fn order_frame(slots: Option<[usize; 2]>) -> Frame {
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
    let frame = Frame::new(vec![0; 7], integers, vec![0.; 5], reals).unwrap();
    match slots {
        Some(slots) => frame.with_caller_order_slots(slots).unwrap(),
        None => frame,
    }
}
fn order_request(complete: bool) -> IsoApproxRequest<'static> {
    let mut request = curve_request(2);
    request.iordre = 0;
    request.ideriv = 0;
    if complete {
        request.nbroot = 8;
        request.rootlg = &ROOTS;
    }
    request
}
fn caller_pair(call: &Call<'_, '_>) -> (i32, i32) {
    (
        call.caller_order(Caller::ContinuityOrder).unwrap(),
        call.caller_order(Caller::CrossDerivativeOrder).unwrap(),
    )
}
fn local_pair(call: &Call<'_, '_>) -> (i32, i32) {
    (call.integer(Local::UOrder), call.integer(Local::VOrder))
}
#[test]
fn shared_caller_zero_to_two_failure_keeps_both_local_orders_zero() {
    let mut frame = order_frame(Some([5, 5]));
    let mut request = order_request(false);
    let evaluator = |call: &mut Call<'_, '_>, _: &mut [f64], error: &mut i32| {
        assert_eq!((caller_pair(call), local_pair(call)), ((0, 0), (0, 0)));
        call.set_caller_order(Caller::ContinuityOrder, 2).unwrap();
        assert_eq!((caller_pair(call), local_pair(call)), ((2, 2), (0, 0)));
        *error = 1;
    };
    let mut storage = CurveStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!(
        (
            request.iordre,
            request.ideriv,
            error,
            output.nbcrbe,
            output.ncoeff[0]
        ),
        (2, 2, 101, 0, -7)
    );
    assert_eq!(&output.courbe[..6], &[0.; 6]);
    assert_eq!(&output.courbe[6..], &[7.; 26]);
    assert_eq!(&output.contr1[..4], &[7.; 4]);
    assert_eq!(frame.integer_values()[5], 2);
}
#[test]
fn live_caller_continuity_enables_the_bounded_second_callback_before_failure() {
    let mut frame = order_frame(Some([5, 5]));
    let mut request = order_request(false);
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        let mut count = calls.lock().unwrap();
        *count += 1;
        if *count == 1 {
            assert_eq!(call.integer(Local::ParameterCount), 5);
            assert_eq!(local_pair(call), (0, 0));
            call.set_caller_order(Caller::ContinuityOrder, 1).unwrap();
            assert_eq!(caller_pair(call), (1, 1));
            values.fill(1.);
            *error = 0;
            return;
        }
        assert_eq!(*count, 2);
        assert_eq!(call.integer(Local::ParameterCount), 1);
        assert_eq!((caller_pair(call), local_pair(call)), ((1, 1), (1, 0)));
        call.set_caller_order(Caller::CrossDerivativeOrder, 2)
            .unwrap();
        assert_eq!((caller_pair(call), local_pair(call)), ((2, 2), (1, 0)));
        *error = 1;
    };
    let mut storage = CurveStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), 2);
    assert_eq!(
        (request.iordre, request.ideriv, error, output.nbcrbe),
        (2, 2, 101, 0)
    );
    assert_eq!(&output.contr1[..4], &[1., 1., 7., 7.]);
    assert_eq!(&output.contr2[..4], &[1., 1., 7., 7.]);
    assert_eq!(&output.courbe[..6], &[0.; 6]);
    assert_eq!(&output.courbe[6..], &[7.; 26]);
}
#[test]
fn shared_caller_orders_can_change_and_restore_before_successful_approximation() {
    let mut frame = order_frame(Some([5, 5]));
    let mut request = order_request(true);
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        call.set_caller_order(Caller::ContinuityOrder, 2).unwrap();
        assert_eq!(caller_pair(call), (2, 2));
        assert_eq!(local_pair(call), (0, 0));
        call.set_caller_order(Caller::CrossDerivativeOrder, 0)
            .unwrap();
        assert_eq!(caller_pair(call), (0, 0));
        assert_eq!(local_pair(call), (0, 0));
        values.fill(1.);
        *error = 0;
    };
    let mut storage = CurveStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!(
        (
            request.iordre,
            request.ideriv,
            error,
            output.nbcrbe,
            output.ncoeff[0]
        ),
        (0, 0, 0, 1, 1)
    );
    for (actual, expected) in output.courbe[..6].iter().zip([1., 0., 0., 1., 0., 0.]) {
        assert!((actual - expected).abs() <= 8. * f64::EPSILON);
    }
    assert_eq!(&output.courbe[6..], &[7.; 26]);
}
#[test]
fn numerically_equal_caller_controls_stay_distinct_unbound_or_in_distinct_slots() {
    for slots in [None, Some([5, 6])] {
        let mut frame = order_frame(slots);
        let mut request = order_request(true);
        let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
            call.set_caller_order(Caller::ContinuityOrder, 2).unwrap();
            assert_eq!(caller_pair(call), (2, 0));
            assert_eq!(local_pair(call), (0, 0));
            call.set_caller_order(Caller::ContinuityOrder, 0).unwrap();
            values.fill(1.);
            *error = 0;
        };
        let mut output = CurveStorage::new();
        let mut error = -1;
        frame
            .approximate_iso(&mut request, &evaluator, &mut output.output(), &mut error)
            .unwrap();
        assert_eq!((request.iordre, request.ideriv, error), (0, 0, 0));
    }
}
#[test]
fn local_derivative_writes_do_not_mutate_the_shared_caller_controls() {
    let mut frame = order_frame(Some([5, 5]));
    let mut request = order_request(false);
    let evaluator = |call: &mut Call<'_, '_>, _: &mut [f64], error: &mut i32| {
        call.set_integer(Local::VOrder, 3);
        assert_eq!(local_pair(call), (0, 3));
        assert_eq!(caller_pair(call), (0, 0));
        *error = 1;
    };
    let mut error = -1;
    frame
        .approximate_iso(
            &mut request,
            &evaluator,
            &mut CurveStorage::new().output(),
            &mut error,
        )
        .unwrap();
    assert_eq!((request.iordre, request.ideriv, error), (0, 0, 101));
}
#[test]
fn caller_order_bindings_reject_out_of_range_and_actual_local_identity() {
    let failure = order_frame(None)
        .with_caller_order_slots([7, 7])
        .err()
        .unwrap();
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert!(
        failure
            .message()
            .contains("ContinuityOrder slot=7; expected slot in 0..7")
    );
    let failure = order_frame(None)
        .with_caller_order_slots([5, 4])
        .err()
        .unwrap();
    assert!(
        failure
            .message()
            .contains("caller CrossDerivativeOrder slot=4 names a callback local")
    );
    for slot in [2, 3, 4] {
        let failure = order_frame(None)
            .with_caller_order_slots([slot, 5])
            .err()
            .unwrap();
        assert_eq!(failure.exception_type(), "Standard_DomainError");
        assert!(failure.message().contains(&format!(
            "caller ContinuityOrder slot={slot} names a callback local"
        )));
    }
}
#[test]
fn shared_caller_orders_reject_conflicting_initialized_values_before_callback() {
    let mut frame = order_frame(Some([5, 5]));
    let mut request = order_request(false);
    request.ideriv = 1;
    let evaluator = |_: &mut Call<'_, '_>, _: &mut [f64], _: &mut i32| {
        panic!("conflicting initialized aliases must not call evaluator")
    };
    let mut error = -1;
    let failure = frame
        .approximate_iso(
            &mut request,
            &evaluator,
            &mut CurveStorage::new().output(),
            &mut error,
        )
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(
        failure
            .message()
            .contains("incident CrossDerivativeOrder value=1; expected shared input value=0")
    );
    assert_eq!((request.iordre, request.ideriv, error), (0, 1, 101));
}
