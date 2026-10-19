//! Bounds and coefficients may overlap inside the one actual private allocation.
use super::*;
#[path = "iso_curve_truncation_overlap_fixture_tests.rs"]
mod owner;
fn verify_callback_entry(call: &mut Call<'_, '_, '_>, calls: &Mutex<usize>, warning: bool) {
    let mut count = calls.lock().unwrap();
    assert_eq!(*count, 0);
    *count += 1;
    assert_eq!(
        call.integer(AdvApp2VarIntegerField::Dimension),
        if warning { 20 } else { 9 }
    );
    assert_eq!(call.integer(AdvApp2VarIntegerField::ParameterCount), 8);
    assert_eq!(
        call.buffers().allocation_capacity().unwrap(),
        if warning { 1235 } else { 422 }
    );
}
fn evaluate_overlap(call: &mut Call<'_, '_, '_>, calls: &Mutex<usize>, warning: bool) {
    verify_callback_entry(call, calls, warning);
    // Each tail ends at its actual retained payload boundary, with no guard/padding.
    for index in 0..if warning { 1205 } else { 403 } {
        call.buffers().set_result(index, 0.);
    }
    write_overlap_controls(call, warning);
}
fn write_overlap_controls(call: &mut Call<'_, '_, '_>, warning: bool) {
    call.set_integer(
        AdvApp2VarIntegerField::Dimension,
        if warning { 406 } else { 206 },
    );
    call.set_caller_sampling(
        crate::adv_app2var::AdvApp2VarCallerSamplingField::JacobiDegree,
        if warning { 1 } else { 0 },
    )
    .unwrap();
    call.set_caller_output(AdvApp2VarIsoIntegerOutput::Status, 0)
        .unwrap();
}
fn assert_public_outputs(
    frame: &AdvApp2VarIsoOperationFrame,
    request: &IsoApproxRequest<'_>,
    made: i32,
    warning: bool,
) {
    assert_eq!(
        (request.ndimen, request.ndgjac, made),
        if warning { (406, 1, 1) } else { (206, 0, 1) }
    );
    assert_eq!(
        frame
            .caller_output(AdvApp2VarIsoIntegerOutput::Status)
            .unwrap(),
        if warning { -1 } else { 0 }
    );
    assert_eq!(frame.coefficient_count(0).unwrap(), 1);
}

fn run_overlap(iso: i32, warning: bool) {
    let (mut frame, curve) = owner::frame(iso, warning);
    let mut request = owner::request(iso, warning);
    let mut output = owner::output(curve);
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| evaluate_overlap(call, &calls, warning);
    frame
        .approximate_iso_owned_output_curve(&mut request, &evaluator, &mut output)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), 1);
    assert_public_outputs(&frame, &request, output.nbcrbe, warning);
    owner::assert_real_owner(&mut frame, warning);
    print_result_trace(iso, &frame, &request, output.nbcrbe);
}
fn print_result_trace(
    iso: i32,
    frame: &AdvApp2VarIsoOperationFrame,
    request: &IsoApproxRequest<'_>,
    made: i32,
) {
    let status = frame
        .caller_output(AdvApp2VarIsoIntegerOutput::Status)
        .unwrap();
    let retained = frame.coefficient_count(0).unwrap();
    println!(
        "curve_truncation_contract|iso={iso}|dimension={}|degree={}|status={status}|made={made}|retained={retained}|all_caller_real_slots_verified=true",
        request.ndimen, request.ndgjac
    );
}
#[test]
fn live_dimension_crosses_sample_coefficient_boundary_in_iso_one() {
    run_overlap(1, false);
}
#[test]
fn live_dimension_crosses_sample_coefficient_boundary_in_iso_two() {
    run_overlap(2, false);
}

#[test]
fn warning_dimension_crosses_sample_coefficient_boundary_in_iso_one() {
    run_overlap(1, true);
}
#[test]
fn warning_dimension_crosses_sample_coefficient_boundary_in_iso_two() {
    run_overlap(2, true);
}
