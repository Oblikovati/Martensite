//! Bounded caller-real incidence with independent expected caller state.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarIntegerField as Integer, AdvApp2VarIsoRealFrame as RealFrame,
    AdvApp2VarRealBindings as RealBindings, AdvApp2VarRealEvaluation as RealCall,
    AdvApp2VarRealField as Real,
};
const ENDPOINTS: [Real; 4] = [Real::UStart, Real::UEnd, Real::VStart, Real::VEnd];
fn bindings(constant_parameter: usize) -> RealBindings {
    RealBindings {
        domain: [0, 1, 2, 3],
        constant_parameter,
    }
}
fn stopping_reals(call: &mut RealCall<'_, '_>, _: &mut [f64], error: &mut i32) {
    call.set_integer(Integer::Dimension, 1);
    call.set_integer(Integer::ParameterCount, 2);
    call.set_integer(Integer::FavoriteIso, 3 - call.integer(Integer::FavoriteIso));
    call.set_integer(Integer::UOrder, 7);
    assert_eq!(call.integer(Integer::VOrder), 0);
    call.set_integer(Integer::VOrder, 9);
    assert_eq!(call.integer(Integer::UOrder), 7);
    call.set_real(Real::ConstantParameter, 0.75);
    call.u_bounds()[0] = -0.5;
    call.v_bounds()[1] = 0.75;
    *error = 1;
}
#[test]
fn constant_shares_the_caller_endpoint_in_the_failure_case() {
    let mut frame = RealFrame::new(vec![0.; 4], bindings(0)).unwrap();
    let mut request = curve_request(2);
    request.tconst = 0.;
    let mut storage = CurveStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    assert_eq!(
        frame
            .approximate_iso(&mut request, &stopping_reals, &mut output, &mut error)
            .unwrap(),
        0
    );
    assert_eq!(
        (request.ndimen, request.isofav, request.tconst, error),
        (1, 1, 0.75, 101)
    );
    assert_eq!(request.uvfonc, [0.75, 1., 0., 1.]);
    assert_eq!(
        frame.real(Real::ConstantParameter),
        frame.real(Real::UStart)
    );
    assert_eq!(output.nbcrbe, 0);
    assert_eq!(&output.courbe[..6], &[0.; 6]);
    assert_eq!(&output.courbe[6..], &[7.; 26]);
}
#[test]
fn every_caller_endpoint_alias_observes_writes_in_both_directions_on_both_axes() {
    for iso in [1, 2] {
        for (index, endpoint) in ENDPOINTS.into_iter().enumerate() {
            let mut frame = RealFrame::new(vec![0.; 4], bindings(index)).unwrap();
            let mut request = curve_request(iso);
            request.tconst = request.uvfonc[index];
            let evaluator = |call: &mut RealCall<'_, '_>, values: &mut [f64], error: &mut i32| {
                assert_eq!(*call.u_bounds(), [0., 1.]);
                assert_eq!(*call.v_bounds(), [0., 1.]);
                assert_eq!(call.parameters()[1], 0.5);
                stopping_reals(call, values, error);
                assert_eq!(call.real(endpoint), 0.75);
                call.set_real(endpoint, 0.625);
                assert_eq!(call.real(Real::ConstantParameter), 0.625);
            };
            let mut storage = CurveStorage::new();
            let mut error = -1;
            frame
                .approximate_iso(&mut request, &evaluator, &mut storage.output(), &mut error)
                .unwrap();
            let mut expected_domain = [0., 1., 0., 1.];
            expected_domain[index] = 0.625;
            assert_eq!(request.uvfonc, expected_domain);
            assert_eq!(request.tconst, 0.625);
            assert_eq!(error, 101);
        }
    }
}
#[test]
fn equal_endpoint_values_do_not_invent_shared_identity() {
    let mut frame = RealFrame::new(vec![0.; 5], bindings(4)).unwrap();
    let mut request = curve_request(2);
    let evaluator = |call: &mut RealCall<'_, '_>, _: &mut [f64], error: &mut i32| {
        call.set_real(Real::UStart, 0.25);
        assert_eq!(call.real(Real::VStart), 0.);
        assert_eq!(call.real(Real::ConstantParameter), 0.5);
        assert_eq!(*call.u_bounds(), [0., 1.]);
        call.u_bounds()[0] = -0.5;
        assert_eq!(call.real(Real::UStart), 0.25);
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
    assert_eq!(request.uvfonc, [0.25, 1., 0., 1.]);
    assert_eq!(request.tconst, 0.5);
    assert_eq!(error, 101);
}
#[test]
fn invalid_or_impossible_domain_slot_shapes_are_typed_errors() {
    let invalid = RealFrame::new(vec![0.; 4], bindings(4)).err().unwrap();
    assert_eq!(invalid.exception_type(), "Standard_OutOfRange");
    assert!(invalid.message().contains("slot=4; expected slot in 0..4"));
    let mut duplicate = bindings(4);
    duplicate.domain[2] = 0;
    let impossible = RealFrame::new(vec![0.; 5], duplicate).err().unwrap();
    assert_eq!(impossible.exception_type(), "Standard_DomainError");
    assert!(impossible.message().contains("expected distinct slots"));
}
#[test]
fn conflicting_shared_input_bits_are_rejected_without_running_or_clearing_outputs() {
    let mut frame = RealFrame::new(vec![0.; 4], bindings(0)).unwrap();
    let mut request = curve_request(2);
    request.tconst = -0.;
    let mut error = -7;
    let mut storage = CurveStorage::new();
    let failure = frame
        .approximate_iso(
            &mut request,
            &stopping_reals,
            &mut storage.output(),
            &mut error,
        )
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(
        failure
            .message()
            .contains("expected shared input value=0.0 bits=0000000000000000")
    );
    assert_eq!(storage.curve, [7.; 32]);
    assert_eq!(error, -7);
}
#[test]
fn real_frame_and_callback_view_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RealFrame>();
    assert_send_sync::<RealCall<'static, 'static>>();
    assert_send_sync::<RealBindings>();
    assert_send_sync::<Real>();
}

#[test]
fn real_alias_writes_persist_in_source_order_between_endpoint_callbacks() {
    let mut frame = RealFrame::new(vec![0.; 4], bindings(0)).unwrap();
    let mut request = curve_request(2);
    request.tconst = 0.;
    request.iordre = 1;
    request.ndgjac = 3;
    request.ncflim = 4;
    let calls = Mutex::new(0);
    let evaluator = |call: &mut RealCall<'_, '_>, values: &mut [f64], error: &mut i32| {
        let mut count = calls.lock().unwrap();
        *count += 1;
        if *count == 1 {
            assert_eq!(call.integer(Integer::ParameterCount), 5);
            assert_eq!(call.real(Real::ConstantParameter), 0.);
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
        assert_eq!(call.real(Real::UStart), 0.75);
        call.set_real(Real::UStart, 0.625);
        assert_eq!(call.real(Real::ConstantParameter), 0.625);
        *error = 1;
    };
    let mut storage = CurveStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), 2);
    assert_eq!(request.uvfonc, [0.625, 1., 0., 1.]);
    assert_eq!(
        (request.ndimen, request.isofav, request.tconst, error),
        (2, 2, 0.625, 101)
    );
    assert_eq!(&output.courbe[..8], &[0.; 8]);
    assert_eq!(&output.courbe[8..], &[7.; 24]);
}
