//! Bivariate cut output contracts behavior and supporting operations.
use super::*;
fn contract_frame(nan: Option<f64>) -> Frame {
    let mut values = vec![7.; 16];
    values[..5].copy_from_slice(&[0., 1., 0., 1., 0.5]);
    if let Some(value) = nan {
        values[8] = value;
    }
    Frame::new(
        vec![1, 1, 0, 0, 0, -7, -7],
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
    .with_caller_output_slots([5, 6])
    .unwrap()
}
fn bound_contract_frame(origin: usize, nan: Option<f64>) -> Frame {
    contract_frame(nan)
        .with_cut_parameter_origin(origin)
        .unwrap()
}
fn contract_request() -> IsoApproxRequest<'static> {
    let mut request = super::super::curve_request(1);
    request.ndimen = 1;
    request.ndimse = &[1];
    request
}
#[test]
fn cut_origins_and_signed_getters_require_initialized_owner_slots() {
    let frame = contract_frame(None);
    assert!(frame.cut_parameter(0).is_err());
    assert!(contract_frame(None).with_cut_parameter_origin(16).is_err());
    assert!(
        contract_frame(None)
            .with_cut_parameter_origin(usize::MAX)
            .is_err()
    );
    let frame = bound_contract_frame(8, None);
    assert_eq!(frame.cut_parameter(-1).unwrap(), 7.);
    assert!(frame.cut_parameter(-9).is_err());
    assert!(frame.cut_parameter(i32::MAX).is_err());
}
#[test]
fn callback_cut_writes_are_immediate_and_survive_unwind_with_the_actual_owner() {
    let mut frame = bound_contract_frame(0, None);
    let mut request = contract_request();
    let mut tables = super::super::CurveStorage::new();
    tables.cuts = [0., 1.];
    let mut output = tables.output();
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        assert!(call.set_cut_parameter(-1, 1.).is_err());
        call.set_cut_parameter(0, 0.25).unwrap();
        assert_eq!(call.real(Real::UStart), 0.25);
        call.set_real(Real::UStart, 0.75);
        assert_eq!(call.cut_parameter(0).unwrap(), 0.75);
        panic!("owned cut callback unwind fixture");
    };
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = frame.approximate_iso_owned_outputs(&mut request, &evaluator, &mut output);
        }))
        .is_err()
    );
    assert_eq!(frame.cut_parameter(0).unwrap(), 0.75);
    assert_eq!(frame.real(Real::UStart), 0.75);
    assert_eq!(frame.caller_output(Output::Status).unwrap(), 0);
    assert_eq!(&output.courbe[..3], &[0.; 3]);
}
#[test]
fn cut_mirror_validation_distinguishes_signed_zero_and_nan_payloads_before_clear() {
    for same in [false, true] {
        let actual = f64::from_bits(0x7ff8_0000_0000_0001);
        let supplied = if same {
            actual
        } else {
            f64::from_bits(0x7ff8_0000_0000_0002)
        };
        let mut frame = bound_contract_frame(8, Some(actual));
        let mut request = contract_request();
        let mut tables = super::super::CurveStorage::new();
        tables.cuts = [supplied, 7.];
        let mut output = tables.output();
        let calls = std::sync::Mutex::new(0);
        let evaluator = |call: &mut Call<'_, '_, '_>| {
            *calls.lock().unwrap() += 1;
            call.set_caller_output(Output::Status, 1).unwrap();
        };
        let result = frame.approximate_iso_owned_outputs(&mut request, &evaluator, &mut output);
        assert_eq!(result.is_ok(), same);
        assert_eq!(*calls.lock().unwrap(), i32::from(same));
        assert_eq!(
            frame.caller_output(Output::Status).unwrap(),
            if same { 101 } else { -7 }
        );
        if !same {
            assert_eq!(output.courbe, &[7.; 32]);
        }
    }
    let mut frame = bound_contract_frame(0, None);
    let mut request = contract_request();
    let mut tables = super::super::CurveStorage::new();
    tables.cuts = [-0., 1.];
    let mut output = tables.output();
    let evaluator = |_: &mut Call<'_, '_, '_>| panic!("unexpected callback on signedzero mismatch");
    assert!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .is_err()
    );
    assert_eq!(output.courbe, &[7.; 32]);
}
#[test]
fn unbound_callback_cut_access_fails_without_inventing_a_storage_identity() {
    let mut frame = contract_frame(None);
    let mut request = contract_request();
    let mut tables = super::super::CurveStorage::new();
    let mut output = tables.output();
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        assert!(call.cut_parameter(0).is_err());
        assert!(call.set_cut_parameter(0, 1.).is_err());
        call.set_caller_output(Output::Status, 1).unwrap();
    };
    assert_eq!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .unwrap(),
        0
    );
    assert_eq!(frame.caller_output(Output::Status).unwrap(), 101);
}
