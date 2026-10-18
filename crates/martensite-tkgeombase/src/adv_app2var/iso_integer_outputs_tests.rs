//! Iso integer outputs tests behavior and supporting operations.
use super::*;
#[test]
fn owned_output_stage_access_fails_explicitly_without_poisoning_an_active_callback_state() {
    let state = Mutex::new(buffers::IndexedCombinedState {
        frame: None,
        failure: None,
    });
    let integers = OwnedCurveIntegers {
        state: &state,
        slots: [0, 1],
    };
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| integers.status())).is_err());
    let state = state.try_lock().unwrap();
    assert!(state.frame.is_none());
    let failure = state.failure.as_ref().unwrap();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
}
#[test]
fn status_scope_keeps_no_guard_across_the_sampler_and_reports_metadata_cost() {
    let mut owner = AdvApp2VarIsoOperationFrame::new(
        vec![0, 2, 0, 0, 0, -1, -7],
        AdvApp2VarIntegerBindings {
            dimension: 0,
            favorite_iso: 1,
            parameter_count: 2,
            u_order: 3,
            v_order: 4,
        },
        vec![0., 1., 0., 1., 0.5],
        AdvApp2VarRealBindings {
            domain: [0, 1, 2, 3],
            constant_parameter: 4,
        },
    )
    .unwrap()
    .with_caller_output_slots([5, 6])
    .unwrap();
    let state = Mutex::new(buffers::IndexedCombinedState {
        frame: Some(&mut owner),
        failure: None,
    });
    let mut integers = OwnedCurveIntegers {
        state: &state,
        slots: [5, 6],
    };
    integers.with_status(|code| {
        assert!(state.try_lock().is_ok());
        *code = 101;
    });
    assert_eq!(integers.status(), 101);
    assert_eq!(integers.pieces(), -7);
    println!(
        "{{\"metric\":\"owned_iso_integer_outputs\",\"operation_frame_bytes\":{},\"backend_bytes\":{},\"output_descriptor_bytes\":{},\"metadata_heap_allocations\":0}}",
        std::mem::size_of::<AdvApp2VarIsoOperationFrame>(),
        std::mem::size_of::<OwnedCurveIntegers<'static, 'static>>(),
        std::mem::size_of::<Option<[usize; 2]>>()
    );
    fn send_sync<T: Send + Sync>() {}
    send_sync::<AdvApp2VarIsoOperationFrame>();
}
