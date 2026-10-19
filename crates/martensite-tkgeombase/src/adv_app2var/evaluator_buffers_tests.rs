//! Evaluator buffers tests behavior and supporting operations.
use super::*;
#[test]
fn shared_suffix_borrows_actual_allocation_and_checks_origins() {
    let mut values = [1., 2., 3.];
    let mut shared = AdvApp2VarEvaluationBuffers::shared(&mut values, 0, 3, 1, 3);
    assert_eq!(shared.allocation_suffix(1).unwrap(), &[2., 3.]);
    shared.allocation_suffix_mut(1).unwrap()[0] = 4.;
    assert_eq!(shared.result(0), 4.);
    assert!(shared.allocation_suffix(3).unwrap().is_empty());
    assert!(shared.allocation_suffix(4).is_err());
    assert!(shared.allocation_suffix_mut(usize::MAX).is_err());
    let mut a = [];
    let mut b = [];
    let mut separate = AdvApp2VarEvaluationBuffers::separate(&mut a, &mut b, 0);
    assert!(separate.allocation_suffix(0).is_err());
    assert!(separate.allocation_suffix_mut(0).is_err());
}
#[test]
fn indexed_shared_regions_observe_writes_in_both_directions() {
    let mut values = [0.; 45];
    let mut buffers = AdvApp2VarEvaluationBuffers::shared(&mut values, 0, 10, 10, 30);
    buffers.set_result(0, 1.);
    assert_eq!(buffers.parameter(10), 1.);
    buffers.set_parameter(11, 2.);
    assert_eq!(buffers.result(1), 2.);
    let (parameters, results) = buffers.legacy_views().unwrap();
    assert_eq!((parameters.len(), results.len()), (10, 20));
}
#[test]
fn shifted_retained_views_keep_parameter_and_sample_limits() {
    let mut values = [0.; 45];
    let mut buffers = AdvApp2VarEvaluationBuffers::shared(&mut values, 1, 10, 12, 30);
    let (parameters, results) = buffers.legacy_views().unwrap();
    assert_eq!((parameters.len(), results.len()), (9, 18));
}
#[test]
fn extended_separate_parameters_preserve_the_retained_view_length() {
    let (mut parameters, mut results) = ([0.; 5], [0.; 2]);
    let mut buffers = AdvApp2VarEvaluationBuffers::separate(&mut parameters, &mut results, 1);
    buffers.set_parameter(4, 3.);
    assert_eq!(buffers.parameter(4), 3.);
    let (legacy, result) = buffers.legacy_views().unwrap();
    assert_eq!((legacy.len(), result.len()), (1, 2));
}
#[test]
fn separate_regions_retain_independent_writes() {
    let (mut parameters, mut results) = ([0.; 2], [0.; 4]);
    let mut buffers = AdvApp2VarEvaluationBuffers::separate(&mut parameters, &mut results, 2);
    buffers.set_parameter(0, 2.);
    buffers.set_result(0, 3.);
    assert_eq!((buffers.parameter(0), buffers.result(0)), (2., 3.));
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AdvApp2VarEvaluationBuffers<'static>>();
    assert_send_sync::<AdvApp2VarBufferEvaluation<'static>>();
}
#[test]
fn reborrow_keeps_overlapping_parameter_and_sample_slots() {
    let mut values = [7.; 8];
    let mut buffers = AdvApp2VarEvaluationBuffers::shared(&mut values, 0, 8, 0, 8);
    let mut borrowed = buffers.reborrow();
    borrowed.set_result(0, 0.25);
    assert_eq!(borrowed.parameter(0), 0.25);
    borrowed.set_parameter(1, 0.75);
    assert_eq!(borrowed.result(1), 0.75);
    assert_eq!(buffers.result(1), 0.75);
}
#[test]
fn reborrow_keeps_separate_extended_storage_and_retained_limits() {
    let (mut parameters, mut results) = ([7.; 8], [7.; 8]);
    let mut buffers = AdvApp2VarEvaluationBuffers::separate(&mut parameters, &mut results, 3);
    let mut borrowed = buffers.reborrow();
    borrowed.set_parameter(7, 0.25);
    borrowed.set_result(7, 0.75);
    assert_eq!(borrowed.legacy_views().unwrap().0.len(), 3);
    assert_eq!(buffers.parameter(7), 0.25);
    assert_eq!(buffers.result(7), 0.75);
}
#[test]
fn absolute_slots_keep_shared_allocation_bounds_and_reject_separate_storage() {
    let mut values = [7.; 4];
    let mut shared = AdvApp2VarEvaluationBuffers::shared(&mut values, 1, 4, 2, 4);
    assert_eq!(shared.allocation_capacity().unwrap(), 4);
    shared.set_allocation_value(0, 0.25).unwrap();
    assert_eq!(shared.allocation_value(0).unwrap(), 0.25);
    assert!(shared.allocation_value(4).is_err());
    assert!(shared.set_allocation_value(usize::MAX, 1.).is_err());
    let (mut parameters, mut samples) = ([0.; 1], [0.; 1]);
    let mut separate = AdvApp2VarEvaluationBuffers::separate(&mut parameters, &mut samples, 1);
    assert!(separate.allocation_capacity().is_err());
    assert!(separate.allocation_value(0).is_err());
    assert!(separate.set_allocation_value(0, 1.).is_err());
}
#[test]
fn signed_result_offsets_retain_initialized_prefix_and_physical_limits() {
    let mut values = [1., 2., 3., 4.];
    let buffers = AdvApp2VarEvaluationBuffers::shared(&mut values, 0, 4, 2, 4);
    for (offset, expected) in [(-2, 1.), (-1, 2.), (0, 3.), (1, 4.)] {
        assert_eq!(buffers.result_at_signed_offset(offset).unwrap(), expected);
    }
    for offset in [-3, 2, isize::MIN, isize::MAX] {
        assert!(buffers.result_at_signed_offset(offset).is_err());
    }
    let failure = buffers.result_at_signed_offset(-3).unwrap_err();
    assert!(failure.message().contains("result offset=-3, origin=2"));
}
#[test]
fn separate_results_have_no_owned_prefix_before_zero() {
    let (mut parameters, mut values) = ([], [5., 6.]);
    let buffers = AdvApp2VarEvaluationBuffers::separate(&mut parameters, &mut values, 0);
    assert_eq!(buffers.result_at_signed_offset(0).unwrap(), 5.);
    assert_eq!(buffers.result_at_signed_offset(1).unwrap(), 6.);
    assert!(buffers.result_at_signed_offset(-1).is_err());
    assert!(buffers.result_at_signed_offset(2).is_err());
}

fn endpoint_test_slots() -> OwnedEndpointSlots {
    OwnedEndpointSlots::new(
        std::sync::Arc::new(std::sync::Mutex::new(vec![7.; 8])),
        [2, 2],
        0,
    )
}
#[test]
fn endpoint_buffers_reborrow_keep_actual_owner_and_signed_result_origin() {
    let (mut slots, mut parameters) = (endpoint_test_slots(), [0.5]);
    let mut buffers = AdvApp2VarEvaluationBuffers::owned_endpoint(
        &mut parameters,
        &mut slots,
        AdvApp2VarEndpointOutput::Lower,
        1,
    );
    buffers.set_result(0, -2.);
    let mut reborrow = buffers.reborrow();
    reborrow.set_result(1, 3.);
    reborrow.set_parameter(0, 0.25);
    assert_eq!(reborrow.result(0), -2.);
    assert_eq!(reborrow.result_at_signed_offset(-1).unwrap(), 7.);
    assert!(reborrow.result_at_signed_offset(-4).is_err());
    assert!(reborrow.result_at_signed_offset(isize::MAX).is_err());
    assert!(reborrow.legacy_views().is_none());
    assert!(reborrow.allocation_capacity().is_err());
    assert!(reborrow.allocation_value(0).is_err());
    assert_eq!(
        slots.value(AdvApp2VarEndpointOutput::Upper, 1).unwrap(),
        -2.
    );
    assert_eq!(slots.value(AdvApp2VarEndpointOutput::Upper, 2).unwrap(), 3.);
    assert_eq!(parameters, [0.25]);
}
#[test]
fn endpoint_retained_adapter_rejects_unrepresentable_slice_before_user_code() {
    struct UnexpectedLegacy;
    impl AdvApp2VarEvaluatorFunc2Var for UnexpectedLegacy {
        fn evaluate(&self, _: &super::super::AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
            panic!("owned endpoint must use indexed callback");
        }
    }
    let (mut slots, mut parameters) = (endpoint_test_slots(), [0.5]);
    let (mut dimension, mut iso, mut count, mut u, mut v, mut constant, mut domain_u, mut domain_v) =
        (1, 1, 1, 0, 0, 0.5, [0., 1.], [0., 1.]);
    let mut call = AdvApp2VarBufferEvaluation {
        dimension: &mut dimension,
        u_start_end: &mut domain_u,
        v_start_end: &mut domain_v,
        favor_iso: &mut iso,
        const_param: &mut constant,
        nb_params: &mut count,
        orders: AdvApp2VarDerivativeOrders::Independent {
            u: &mut u,
            v: &mut v,
        },
        buffers: AdvApp2VarEvaluationBuffers::owned_endpoint(
            &mut parameters,
            &mut slots,
            AdvApp2VarEndpointOutput::Lower,
            1,
        ),
    };
    let mut error = 0;
    call.evaluate_square_legacy(&UnexpectedLegacy, &mut error);
    assert_eq!(error, 1);
    assert_eq!(call.buffers.result(0), 7.);
}
#[test]
fn endpoint_callback_descriptor_cost_is_bounded_and_owner_payload_is_not_copied() {
    let (mut slots, mut parameters) = (endpoint_test_slots(), [0.5]);
    let iterations = 10000;
    let started = std::time::Instant::now();
    let mut buffers = AdvApp2VarEvaluationBuffers::owned_endpoint(
        &mut parameters,
        &mut slots,
        AdvApp2VarEndpointOutput::Lower,
        1,
    );
    for index in 0..iterations {
        buffers.set_result(0, std::hint::black_box(index as f64));
        std::hint::black_box(buffers.result(0));
    }
    assert_eq!(buffers.result(0), 9999.);
    println!(
        "{{\"metric\":\"endpoint_callback_indexed_access\",\"iterations\":{iterations},\"elapsed_ns\":{},\"scalar_lock_calls\":{},\"new_callback_allocations\":0,\"result_payload_copies\":0,\"descriptor_bytes\":{}}}",
        started.elapsed().as_nanos(),
        iterations * 2 + 1,
        std::mem::size_of::<AdvApp2VarEvaluationBuffers<'static>>()
    );
}

#[test]
fn endpoint_callback_offset_overflow_reports_both_offending_values() {
    for (start, index) in [(1, usize::MAX), (i32::MAX, 1)] {
        let failure = std::panic::catch_unwind(|| endpoint_offset(start, index)).unwrap_err();
        let message = failure.downcast_ref::<String>().unwrap();
        assert!(message.contains(&format!("offset={start}, index={index}")));
        assert!(message.contains("expected logical result index inside i32 range"));
    }
}
