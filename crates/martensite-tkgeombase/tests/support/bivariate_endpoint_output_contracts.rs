//! Bivariate endpoint output contracts behavior and supporting operations.
use super::*;
#[test]
fn endpoint_origins_and_signed_reads_require_initialized_slots() {
    let frame = endpoint_frame(1, true);
    assert_eq!(frame.endpoint_output(Endpoint::Lower, -1).unwrap(), 7.);
    assert!(frame.endpoint_output(Endpoint::Lower, -321).is_err());
    assert!(frame.endpoint_output(Endpoint::Upper, i32::MAX).is_err());
    assert!(
        endpoint_frame(1, true)
            .with_endpoint_output_origins([usize::MAX, 320])
            .is_err()
    );
}
#[test]
fn endpoint_mirror_bit_mismatch_is_rejected_before_callback() {
    let (mut frame, mut request, mut tables) = (
        endpoint_frame(1, true),
        endpoint_request(1, 1),
        CurveStorage::new(),
    );
    tables.first[0] = -0.;
    let mut output = tables.output();
    let evaluator = |_: &mut Call<'_, '_, '_>| panic!("unexpected inconsistent endpoint callback");
    assert!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .is_err()
    );
    assert_eq!(frame.caller_output(Output::Status).unwrap(), -7);
    assert_eq!(output.courbe, &[7.; 32]);
}
#[test]
fn derivative_result_identity_is_bidirectional_and_frame_restores_on_unwind() {
    let (mut frame, mut request, mut tables) = (
        endpoint_frame(1, true),
        endpoint_request(1, 1),
        CurveStorage::new(),
    );
    let mut output = tables.output();
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        let mut ordinal = calls.lock().unwrap();
        *ordinal += 1;
        if *ordinal == 1 {
            for index in 0..10 {
                write_quadratic(call, index, false);
            }
            call.set_caller_output(Output::Status, 0).unwrap();
            return;
        }
        call.buffers().set_result(0, -2.);
        assert_eq!(call.endpoint_output(Endpoint::Upper, 1).unwrap(), -2.);
        call.set_endpoint_output(Endpoint::Upper, 1, 0.375).unwrap();
        assert_eq!(call.buffers().result(0), 0.375);
        panic!("owned derivative callback unwind fixture");
    };
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = frame.approximate_iso_owned_outputs(&mut request, &evaluator, &mut output);
        }))
        .is_err()
    );
    assert_eq!(frame.endpoint_output(Endpoint::Lower, 1).unwrap(), 0.375);
    assert_eq!(frame.endpoint_output(Endpoint::Upper, 1).unwrap(), 0.375);
    assert_eq!(frame.caller_output(Output::CompletedPieces).unwrap(), 0);
    assert_eq!(output.contr1, &[7.; 32]);
}
