//! Combined callback domains with owned independent expected state.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCombinedEvaluation as CombinedCall, AdvApp2VarIntegerBindings as Integers,
    AdvApp2VarIntegerField as Integer, AdvApp2VarIsoOperationFrame as Frame,
    AdvApp2VarRealBindings as Reals, AdvApp2VarRealField as Real,
};
fn shared_integers() -> Integers {
    Integers {
        dimension: 0,
        favorite_iso: 0,
        parameter_count: 1,
        u_order: 2,
        v_order: 3,
    }
}
fn shared_reals() -> Reals {
    Reals {
        domain: [0, 1, 2, 3],
        constant_parameter: 0,
    }
}
fn combined_stop(call: &mut CombinedCall<'_, '_>, _: &mut [f64], error: &mut i32) {
    call.set_integer(Integer::Dimension, 1);
    assert_eq!(call.integer(Integer::FavoriteIso), 1);
    call.set_integer(Integer::FavoriteIso, 2);
    assert_eq!(call.integer(Integer::Dimension), 2);
    call.set_integer(Integer::ParameterCount, 2);
    call.set_integer(Integer::UOrder, 7);
    assert_eq!(call.integer(Integer::VOrder), 0);
    call.set_integer(Integer::VOrder, 9);
    assert_eq!(call.integer(Integer::UOrder), 7);
    call.set_real(Real::ConstantParameter, 0.75);
    assert_eq!(call.real(Real::UStart), 0.75);
    call.set_real(Real::UStart, 0.625);
    assert_eq!(call.real(Real::ConstantParameter), 0.625);
    assert_eq!(*call.u_bounds(), [0., 1.]);
    assert_eq!(*call.v_bounds(), [0., 1.]);
    call.u_bounds()[0] = -0.5;
    call.v_bounds()[1] = 0.75;
    *error = 1;
}
#[test]
fn simultaneous_integer_and_real_aliases_match_exact_failure_and_reseed_on_reuse() {
    let mut frame = Frame::new(vec![0; 4], shared_integers(), vec![0.; 4], shared_reals()).unwrap();
    for _ in 0..2 {
        let mut request = curve_request(2);
        request.tconst = 0.;
        let mut storage = CurveStorage::new();
        let mut output = storage.output();
        let mut error = -1;
        let evaluator = |call: &mut CombinedCall<'_, '_>, values: &mut [f64], error: &mut i32| {
            assert_eq!(call.parameters()[1], 0.5);
            combined_stop(call, values, error);
        };
        assert_eq!(
            frame
                .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
                .unwrap(),
            0
        );
        assert_eq!(
            (
                request.ndimen,
                request.isofav,
                request.nbsesp,
                request.tconst,
                error
            ),
            (2, 2, 1, 0.625, 101)
        );
        assert_eq!(request.uvfonc, [0.625, 1., 0., 1.]);
        assert_eq!(frame.integer_values(), [2, 2, 7, 9]);
        assert_eq!(frame.real(Real::UStart), 0.625);
        assert_eq!(output.nbcrbe, 0);
        assert_eq!(&output.courbe[..6], &[0.; 6]);
        assert_eq!(&output.courbe[6..], &[7.; 26]);
    }
}
#[test]
fn combined_aliases_keep_the_first_callback_write_visible_at_the_endpoint() {
    let mut frame = Frame::new(vec![0; 4], shared_integers(), vec![0.; 4], shared_reals()).unwrap();
    let mut request = curve_request(2);
    request.tconst = 0.;
    request.iordre = 1;
    request.ndgjac = 3;
    request.ncflim = 4;
    let calls = Mutex::new(0);
    let evaluator = |call: &mut CombinedCall<'_, '_>, values: &mut [f64], error: &mut i32| {
        let mut count = calls.lock().unwrap();
        *count += 1;
        if *count == 1 {
            assert_eq!(call.integer(Integer::ParameterCount), 5);
            values.fill(1.);
            call.set_real(Real::ConstantParameter, 0.75);
            *error = 0;
            return;
        }
        assert_eq!(*count, 2);
        assert_eq!(call.integer(Integer::ParameterCount), 1);
        assert_eq!(
            (call.integer(Integer::UOrder), call.integer(Integer::VOrder)),
            (1, 0)
        );
        assert_eq!(call.real(Real::ConstantParameter), 0.75);
        combined_stop(call, values, error);
    };
    let mut storage = CurveStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), 2);
    assert_eq!(
        (request.ndimen, request.isofav, request.tconst, error),
        (2, 2, 0.625, 101)
    );
    assert_eq!(request.uvfonc, [0.625, 1., 0., 1.]);
    assert_eq!(&output.courbe[..8], &[0.; 8]);
    assert_eq!(&output.courbe[8..], &[7.; 24]);
}
#[test]
fn inconsistent_integer_aliases_return_typed_failure_after_retained_initialization() {
    let mut frame = Frame::new(vec![0; 4], shared_integers(), vec![0.; 4], shared_reals()).unwrap();
    let mut request = curve_request(1);
    request.tconst = 0.;
    let mut storage = CurveStorage::new();
    let mut error = -7;
    let failure = frame
        .approximate_iso(
            &mut request,
            &combined_stop,
            &mut storage.output(),
            &mut error,
        )
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(failure.message().contains("expected shared input value=2"));
    assert_eq!(error, 101);
    assert_eq!(&storage.curve[..6], &[0.; 6]);
    assert_eq!(&storage.curve[6..], &[7.; 26]);
    assert_eq!(request.uvfonc, [0., 1., 0., 1.]);
}
#[test]
fn inconsistent_real_aliases_stop_before_integer_seeding_and_output_initialization() {
    let mut frame = Frame::new(vec![9; 4], shared_integers(), vec![0.; 4], shared_reals()).unwrap();
    let mut storage = CurveStorage::new();
    let mut error = -7;
    let failure = frame
        .approximate_iso(
            &mut curve_request(2),
            &combined_stop,
            &mut storage.output(),
            &mut error,
        )
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(
        failure
            .message()
            .contains("expected shared input value=0.0")
    );
    assert_eq!(error, -7);
    assert_eq!(frame.integer_values(), [9; 4]);
    assert_eq!(storage.curve, [7.; 32]);
}
#[test]
fn combined_constructor_retains_both_field_shape_gates() {
    let bad_integer = Frame::new(vec![], shared_integers(), vec![0.; 4], shared_reals())
        .err()
        .unwrap();
    assert_eq!(bad_integer.exception_type(), "Standard_OutOfRange");
    assert!(
        bad_integer
            .message()
            .contains("slot=0; expected slot in 0..0")
    );
    let bad_real = Frame::new(vec![0; 4], shared_integers(), vec![0.; 3], shared_reals())
        .err()
        .unwrap();
    assert_eq!(bad_real.exception_type(), "Standard_OutOfRange");
    assert!(bad_real.message().contains("slot=3; expected slot in 0..3"));
}
#[test]
fn combined_frame_keeps_iso_derivative_locals_distinct() {
    let mut integers = shared_integers();
    integers.v_order = integers.u_order;
    let mut frame = Frame::new(vec![0; 4], integers, vec![0.; 4], shared_reals()).unwrap();
    let mut request = curve_request(2);
    request.tconst = 0.;
    let mut error = -1;
    let failure = frame
        .approximate_iso(
            &mut request,
            &combined_stop,
            &mut CurveStorage::new().output(),
            &mut error,
        )
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(failure.message().contains("expected shared=false"));
    assert_eq!(error, 101);
}
#[test]
fn combined_frame_preserves_the_constant_polynomial_closed_form() {
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
    let mut frame = Frame::new(vec![0; 5], integers, vec![0.; 5], reals).unwrap();
    let evaluator = |_: &mut CombinedCall<'_, '_>, values: &mut [f64], error: &mut i32| {
        values.fill(1.);
        *error = 0;
    };
    let mut storage = CurveStorage::new();
    let mut output = storage.output();
    let mut error = -1;
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
    let mut request = curve_request(2);
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!((output.nbcrbe, output.ncoeff[0], error), (1, 1, 0));
    for (actual, expected) in output.courbe[..6].iter().zip([1., 0., 0., 1., 0., 0.]) {
        assert!(
            (actual - expected).abs() <= 8. * f64::EPSILON,
            "{actual} != {expected}"
        );
    }
    assert_eq!(&output.courbe[6..], &[7.; 26]);
}
#[test]
fn combined_storage_and_view_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Frame>();
    assert_send_sync::<CombinedCall<'static, 'static>>();
}

#[test]
fn unsupported_projection_count_still_returns_incomplete_without_a_curve() {
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
    let mut frame = Frame::new(vec![0; 5], integers, vec![0.; 5], reals).unwrap();
    let evaluator = |_: &mut CombinedCall<'_, '_>, values: &mut [f64], error: &mut i32| {
        values.fill(1.);
        *error = 0;
    };
    let mut storage = CurveStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    frame
        .approximate_iso(&mut curve_request(2), &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!((output.nbcrbe, output.ncoeff[0], error), (0, -7, 1));
    assert_eq!(&output.courbe[..6], &[0.; 6]);
    assert_eq!(&output.courbe[6..], &[7.; 26]);
}
