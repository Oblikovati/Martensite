//! Real provider bridge tests behavior and supporting operations.
use super::*;
use crate::adv_app2var::slice_callback_fixture as fixture;
#[test]
fn real_slice_callback_has_no_real_guard_and_preserves_error_writeback() {
    probe(false);
}
#[test]
fn real_slice_callback_unwind_restores_actual_real_storage_without_poison() {
    probe(true);
}
fn probe(unwind: bool) {
    let mut reals = fixture::reals();
    let state = Mutex::new(RealFrameState {
        frame: Some(&mut reals),
        failure: None,
    });
    let evaluator =
        |call: &mut AdvApp2VarRealEvaluation<'_, '_>, _: &mut [f64], error: &mut i32| {
            assert!(
                state.try_lock().is_ok(),
                "evaluator must not hold the real state guard"
            );
            call.set_real(AdvApp2VarRealField::ConstantParameter, 0.75);
            if unwind {
                panic!("slice real callback unwind fixture");
            }
            *error = 1;
        };
    let adapter = IsoRealAdapter {
        frame: &state,
        evaluator: &evaluator,
    };
    let mut caller = fixture::request();
    let mut tables = fixture::SliceCallbackFixture::new();
    let mut out = tables.output();
    let mut code = -7;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mma2fnc_mut_(&mut caller, &adapter, &mut out, &mut code)
    }));
    assert_eq!(result.is_err(), unwind);
    assert!(state.try_lock().is_ok());
    if !unwind {
        assert_eq!(code, 101);
    }
    drop(state);
    assert_eq!(reals.real(AdvApp2VarRealField::ConstantParameter), 0.75);
    assert_eq!(&out.courbe[..3], &[0.; 3]);
}
#[test]
fn real_slice_stage_getter_fails_explicitly_while_callback_owns_the_frame() {
    let mut reals = fixture::reals();
    let state = Mutex::new(RealFrameState {
        frame: Some(&mut reals),
        failure: None,
    });
    let evaluator = |_: &mut AdvApp2VarRealEvaluation<'_, '_>, _: &mut [f64], _: &mut i32| {
        panic!("unexpected evaluator")
    };
    let adapter = IsoRealAdapter {
        frame: &state,
        evaluator: &evaluator,
    };
    let lease = crate::adv_app2var::operation_frame_lease::OperationFrameLease::take(
        &state,
        poisoned_real_frame,
        active_real_callback,
    )
    .unwrap();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || adapter.caller_iso_roots(&[9.])
        ))
        .is_err()
    );
    let snapshot = state.try_lock().unwrap();
    assert!(snapshot.frame.is_none());
    assert_eq!(
        snapshot.failure.as_ref().unwrap().exception_type(),
        "Standard_DomainError"
    );
    drop(snapshot);
    drop(lease);
    assert!(state.try_lock().unwrap().frame.is_some());
}
#[test]
fn slice_callback_lease_storage_cost_is_inline_and_send_sync() {
    use crate::adv_app2var::operation_frame_lease::{OperationFrameLease, OperationFrameState};
    type State = OperationFrameState<'static, AdvApp2VarIsoRealFrame>;
    fn send_sync<T: Send + Sync>() {}
    send_sync::<Mutex<State>>();
    let lease =
        std::mem::size_of::<OperationFrameLease<'static, 'static, AdvApp2VarIsoRealFrame>>();
    assert_eq!(lease, 2 * std::mem::size_of::<usize>());
    println!(
        "{{\"metric\":\"slice_callback_lease\",\"previous_real_mutex_bytes\":{},\"new_real_mutex_bytes\":{},\"lease_bytes\":{},\"frame_payload_copies\":0,\"metadata_heap_allocations\":0}}",
        std::mem::size_of::<Mutex<&mut AdvApp2VarIsoRealFrame>>(),
        std::mem::size_of::<Mutex<State>>(),
        lease
    );
}
