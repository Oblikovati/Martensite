//! Combined slice bridge tests behavior and supporting operations.
use super::*;
use crate::adv_app2var::slice_callback_fixture as fixture;
#[test]
fn combined_slice_callback_has_no_integer_guard_and_preserves_error_writeback() {
    probe(false);
}
#[test]
fn combined_slice_callback_unwind_restores_actual_integer_storage_without_poison() {
    probe(true);
}
fn probe(unwind: bool) {
    let mut integers = fixture::integers();
    let state = Mutex::new(CombinedState {
        frame: Some(&mut integers),
        failure: None,
    });
    let evaluator =
        |call: &mut AdvApp2VarCombinedEvaluation<'_, '_>, _: &mut [f64], error: &mut i32| {
            assert!(
                state.try_lock().is_ok(),
                "evaluator must not hold the integer state guard"
            );
            call.set_integer(AdvApp2VarIntegerField::Dimension, 2);
            call.set_real(AdvApp2VarRealField::ConstantParameter, 0.75);
            if unwind {
                panic!("slice integer callback unwind fixture");
            }
            *error = 1;
        };
    let bridge = CombinedBridge {
        state: &state,
        evaluator: &evaluator,
    };
    let mut reals = fixture::reals();
    let mut caller = fixture::request();
    let mut tables = fixture::SliceCallbackFixture::new();
    let mut out = tables.output();
    let mut code = -7;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        reals.approximate_iso(&mut caller, &bridge, &mut out, &mut code)
    }));
    assert_eq!(result.is_err(), unwind);
    assert!(state.try_lock().is_ok());
    assert_eq!(reals.real(AdvApp2VarRealField::ConstantParameter), 0.75);
    if !unwind {
        assert!(result.unwrap().is_ok());
        assert_eq!(code, 101);
        assert_eq!(caller.ndimen, 2);
    }
    drop(state);
    assert_eq!(integers.values()[0], 2);
    assert_eq!(&out.courbe[..3], &[0.; 3]);
}
#[test]
fn combined_slice_stage_getter_fails_explicitly_while_callback_owns_the_frame() {
    let mut integers = fixture::integers();
    let state = Mutex::new(CombinedState {
        frame: Some(&mut integers),
        failure: None,
    });
    let evaluator = |_: &mut AdvApp2VarCombinedEvaluation<'_, '_>, _: &mut [f64], _: &mut i32| {
        panic!("unexpected evaluator")
    };
    let bridge = CombinedBridge {
        state: &state,
        evaluator: &evaluator,
    };
    let lease = crate::adv_app2var::operation_frame_lease::OperationFrameLease::take(
        &state,
        poisoned_combined_frame,
        active_integer_callback,
    )
    .unwrap();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || bridge.caller_subspace_dimensions(&[9])
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
