//! Bivariate error output contracts behavior and supporting operations.
use super::*;
use martensite_tkgeombase::adv_app2var::AdvApp2VarRealField as Real;
#[test]
fn error_origins_and_signed_reads_require_initialized_slots() {
    let frame = error_frame(1, true);
    assert_eq!(frame.error_output(ErrorOutput::Maximum, -1).unwrap(), 7.);
    assert!(frame.error_output(ErrorOutput::Maximum, -513).is_err());
    assert!(frame.error_output(ErrorOutput::Maximum, i32::MAX).is_err());
    assert!(
        error_frame(1, true)
            .with_error_output_origins([usize::MAX, 512])
            .is_err()
    );
}
#[test]
fn error_mirror_signed_zero_mismatch_rejects_before_clear_or_callback() {
    let mut frame = error_frame_with_domain(1, false, [0., 1., 0., 1.], 0.5)
        .with_error_output_origins([16, 16])
        .unwrap();
    let mut request = error_request(1);
    request.uvfonc = [0., 1., 0., 1.];
    request.tconst = 0.5;
    let mut tables = CurveStorage::new();
    tables.maximum[0] = 0.;
    tables.mean[0] = -0.;
    let mut output = tables.output();
    let evaluator = |_: &mut Call<'_, '_, '_>| panic!("unexpected mismatched error callback");
    assert!(
        frame
            .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
            .is_err()
    );
    assert_eq!(frame.caller_output(Output::Status).unwrap(), -7);
    assert_eq!(output.courbe, &[7.; 32]);
}
#[test]
fn error_callback_identity_is_immediate_bidirectional_and_restored_on_unwind() {
    let mut frame = error_frame(1, false)
        .with_error_output_origins([16, 16])
        .unwrap();
    let mut request = error_request(1);
    let mut tables = CurveStorage::new();
    tables.maximum[0] = -1.;
    tables.mean[0] = -1.;
    let mut output = tables.output();
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        call.set_error_output(ErrorOutput::Maximum, 0, -0.25)
            .unwrap();
        assert_eq!(
            call.error_output(ErrorOutput::RootMeanSquare, 0).unwrap(),
            -0.25
        );
        assert_eq!(call.real(Real::UStart), -0.25);
        call.set_real(Real::UStart, -0.75);
        assert_eq!(call.error_output(ErrorOutput::Maximum, 0).unwrap(), -0.75);
        panic!("owned error callback unwind fixture");
    };
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = frame.approximate_iso_owned_outputs(&mut request, &evaluator, &mut output);
        }))
        .is_err()
    );
    assert_eq!(frame.error_output(ErrorOutput::Maximum, 0).unwrap(), -0.75);
    assert_eq!(
        frame.error_output(ErrorOutput::RootMeanSquare, 0).unwrap(),
        -0.75
    );
    assert_eq!(frame.real(Real::UStart), -0.75);
}
