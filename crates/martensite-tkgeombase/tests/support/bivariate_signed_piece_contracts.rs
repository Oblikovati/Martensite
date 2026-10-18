//! Bivariate signed piece contracts behavior and supporting operations.
use super::*;
#[test]
fn signed_curve_reads_keep_positive_limits_and_check_physical_prefix() {
    let (frame, _curve) = signed_piece_frame(1);
    assert_eq!(frame.caller_curve_value_signed(-64).unwrap(), 7.);
    assert_eq!(frame.caller_curve_value_signed(-1).unwrap(), 7.);
    assert!(frame.caller_curve_value_signed(-65).is_err());
    assert!(frame.caller_curve_value_signed(128).is_err());
    assert!(frame.caller_curve_value_signed(i32::MAX).is_err());
    assert!(frame.caller_curve_value(128).is_err());
}
fn prefix_stopping_callback(call: &mut Call<'_, '_, '_>, panic: bool) {
    assert!(call.set_caller_curve_value_signed(-65, 2.).is_err());
    call.set_caller_curve_value_signed(-3, 0.25).unwrap();
    assert_eq!(call.caller_curve_value_signed(-3).unwrap(), 0.25);
    call.set_caller_curve_value_signed(-2, -0.).unwrap();
    assert_eq!(
        call.caller_curve_value_signed(-2).unwrap().to_bits(),
        (-0_f64).to_bits()
    );
    if panic {
        panic!("signed caller curve prefix unwind fixture");
    }
    call.set_caller_output(Output::Status, 5).unwrap();
}
fn run_prefix_stop(panic: bool) {
    let (mut frame, curve) = signed_piece_frame(1);
    let (mut request, mut tables) = (signed_piece_request(1), CurveStorage::new());
    let mut output = signed_piece_output(&mut tables, curve);
    let evaluator = |call: &mut Call<'_, '_, '_>| prefix_stopping_callback(call, panic);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        frame.approximate_iso_owned_output_curve(&mut request, &evaluator, &mut output)
    }));
    assert_eq!(result.is_err(), panic);
    if !panic {
        assert_eq!(result.unwrap().unwrap(), 0);
        assert_eq!(frame.caller_output(Output::Status).unwrap(), 105);
    }
    assert_eq!(frame.caller_output(Output::CompletedPieces).unwrap(), 0);
    assert_eq!(frame.caller_curve_value_signed(-3).unwrap(), 0.25);
    assert_eq!(
        frame.caller_curve_value_signed(-2).unwrap().to_bits(),
        (-0_f64).to_bits()
    );
    assert_eq!(frame.coefficient_count(0).unwrap(), -7);
}
#[test]
fn callback_failure_keeps_actual_signed_prefix_and_adds100_once() {
    run_prefix_stop(false);
}
#[test]
fn callback_unwind_keeps_actual_signed_prefix_and_restores_frame() {
    run_prefix_stop(true);
}
