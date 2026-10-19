//! Exact full86-slot retained callback with shared NDIMEN/NDIMSE identity.
use super::dimension_slot_cases::dimension_incidence_case;
#[test]
fn full_buffer_shared_dimension_u_success() {
    dimension_incidence_case(1, 0, true);
}
#[test]
fn full_buffer_shared_dimension_v_success() {
    dimension_incidence_case(2, 0, true);
}
#[test]
fn full_buffer_shared_dimension_u_failure() {
    dimension_incidence_case(1, 1, true);
}
#[test]
fn full_buffer_shared_dimension_v_failure() {
    dimension_incidence_case(2, 1, true);
}

#[test]
fn indexed_combined_view_and_operation_frame_are_send_sync() {
    use martensite_tkgeombase::adv_app2var::{
        AdvApp2VarCombinedBufferEvaluation, AdvApp2VarIsoOperationFrame,
    };
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AdvApp2VarCombinedBufferEvaluation<'static, 'static, 'static>>();
    assert_send_sync::<AdvApp2VarIsoOperationFrame>();
}

#[test]
fn indexed_callback_rejects_invalid_caller_dimension_shape_before_evaluation() {
    check_indexed_binding_failure(vec![0, 5], "len=1; expected 2");
}

#[test]
fn indexed_callback_rejects_contradictory_incident_values_before_evaluation() {
    check_indexed_binding_failure(vec![1], "incident SubspaceDimension(0) value=2");
}

fn check_indexed_binding_failure(slots: Vec<usize>, expected: &str) {
    use super::dimension_slot_cases::dimension_frame;
    use super::*;
    use martensite_tkgeombase::adv_app2var::AdvApp2VarCombinedBufferEvaluation;
    let mut frame = dimension_frame(Some(slots)).unwrap();
    let mut request = curve_request(1);
    let mut storage = CurveStorage::new();
    let mut error = -1;
    let evaluator = |_: &mut AdvApp2VarCombinedBufferEvaluation<'_, '_, '_>, _: &mut i32| {
        panic!("invalid incidence must stop before the evaluator");
    };
    let failure = frame
        .approximate_iso_buffers(&mut request, &evaluator, &mut storage.output(), &mut error)
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(
        failure.message().contains(expected),
        "{}",
        failure.message()
    );
    assert_eq!(request.ndimen, 2);
}
