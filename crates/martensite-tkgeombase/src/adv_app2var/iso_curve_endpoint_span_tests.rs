//! The endpoint normalization captures its axis before mutable derivative callbacks.
use super::*;
#[path = "iso_curve_endpoint_span_expected_tests.rs"]
mod expected;
fn endpoint_request(iso: i32) -> IsoApproxRequest<'static> {
    let mut request = fixture::request(iso);
    (request.iordre, request.ndgjac, request.ncflim) = (1, 4, 5);
    request.uvfonc = expected::domain(iso);
    request
}
fn endpoint_frame(
    iso: i32,
) -> (
    AdvApp2VarIsoOperationFrame,
    crate::adv_app2var::AdvApp2VarCallerCurveSlots,
) {
    let mut integers = fixture::integer_owner(iso);
    (integers[36], integers[38], integers[40]) = (1, 4, 5);
    let (mut frame, _) = fixture::configured_frame(integers, 42);
    let mut owner = frame.caller_scalar_curve_slots(0, 1100).unwrap();
    owner.with_curve_values(|values| values[16..20].copy_from_slice(&expected::domain(iso)));
    let curve = frame.caller_curve_slots(64, 128).unwrap();
    (frame, curve)
}
fn evaluate_endpoint(call: &mut Call<'_, '_, '_>, calls: &Mutex<usize>, iso: i32, flip: bool) {
    let mut count = calls.lock().unwrap();
    *count += 1;
    let favorite = if *count == 3 && flip { 3 - iso } else { iso };
    assert_eq!(call.integer(AdvApp2VarIntegerField::FavoriteIso), favorite);
    expected::check_callback(call, *count, iso);
    fill_affine_endpoint(call, *count);
    write_endpoint_controls(call, *count, flip);
}
fn fill_affine_endpoint(call: &mut Call<'_, '_, '_>, count: usize) {
    let length = call.integer(AdvApp2VarIntegerField::ParameterCount) as usize;
    for index in 0..length {
        let value = if count == 1 {
            0.5 + call.buffers().parameter(index)
        } else {
            1.
        };
        call.buffers().set_result(index, value);
    }
}
fn write_endpoint_controls(call: &mut Call<'_, '_, '_>, count: usize, flip: bool) {
    if count == 2 && flip {
        let favorite = call.integer(AdvApp2VarIntegerField::FavoriteIso);
        call.set_integer(AdvApp2VarIntegerField::FavoriteIso, 3 - favorite);
    }
    if count == 3 {
        call.set_caller_sampling(
            crate::adv_app2var::AdvApp2VarCallerSamplingField::JacobiDegree,
            8,
        )
        .unwrap();
    }
    call.set_caller_output(AdvApp2VarIsoIntegerOutput::Status, 0)
        .unwrap();
}
fn run_endpoint_span(iso: i32, flip: bool) {
    let (mut frame, curve) = endpoint_frame(iso);
    let mut request = endpoint_request(iso);
    let mut buffers = fixture::ResourceBuffers::new();
    let mut output = buffers.output(curve);
    let calls = Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_, '_>| evaluate_endpoint(call, &calls, iso, flip);
    frame
        .approximate_iso_owned_output_curve(&mut request, &evaluator, &mut output)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), 3);
    check_endpoint_outputs(&frame, &request, output.nbcrbe, flip, iso);
    expected::check_owner(&mut frame, iso);
    println!(
        "endpoint_span_contract|iso={iso}|flip={flip}|callbacks=3|captured_halfspan=1|status=1|made=0|all_caller_real_slots_verified=true"
    );
}
fn check_endpoint_outputs(
    frame: &AdvApp2VarIsoOperationFrame,
    request: &IsoApproxRequest<'_>,
    made: i32,
    flip: bool,
    iso: i32,
) {
    assert_eq!(
        (request.isofav, request.ndgjac, made),
        (if flip { 3 - iso } else { iso }, 8, 0)
    );
    assert_eq!(
        frame
            .caller_output(AdvApp2VarIsoIntegerOutput::Status)
            .unwrap(),
        1
    );
    assert_eq!(frame.coefficient_count(0).unwrap(), -7);
}

#[test]
fn endpoint_derivative_span_remains_captured_after_iso_one_callback_mutation() {
    run_endpoint_span(1, true);
}
#[test]
fn endpoint_derivative_span_remains_captured_after_iso_two_callback_mutation() {
    run_endpoint_span(2, true);
}
#[test]
fn endpoint_derivative_span_control_for_iso_one() {
    run_endpoint_span(1, false);
}
#[test]
fn endpoint_derivative_span_control_for_iso_two() {
    run_endpoint_span(2, false);
}
