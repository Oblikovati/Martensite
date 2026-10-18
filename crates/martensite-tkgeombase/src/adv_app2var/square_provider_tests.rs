//! Square provider tests behavior and supporting operations.
use super::*;
use crate::adv_app2var::{AdvApp2VarDerivativeOrders, AdvApp2VarSquareIntegerBindings};
fn frame() -> Frame {
    Frame::new(
        vec![3, 3, 2],
        AdvApp2VarSquareIntegerBindings {
            dimension: 0,
            root_counts: [1, 0],
            favorite_iso: 2,
        },
    )
    .unwrap()
}
fn dispatch(evaluator: &impl AdvApp2VarEvaluatorFunc2Var) -> i32 {
    let (mut dim, mut iso, mut count, mut order) = (3, 2, 3, 0);
    let (mut ub, mut vb, mut constant, mut parameters, mut values) =
        ([0., 1.], [0., 1.], 0.5, [0.25, 0.5, 0.75], [0.; 9]);
    let mut call = AdvApp2VarEvaluationSlots {
        dimension: &mut dim,
        favor_iso: &mut iso,
        nb_params: &mut count,
        u_start_end: &mut ub,
        v_start_end: &mut vb,
        const_param: &mut constant,
        parameters: &mut parameters,
        orders: AdvApp2VarDerivativeOrders::Shared(&mut order),
    };
    let mut error = 0;
    evaluator.evaluate_slots(&mut call, &mut values, &mut error);
    error
}
#[test]
fn caller_fields_and_callback_locals_keep_their_actual_identities() {
    let mut owner = frame();
    let state = Mutex::new(OperationFrameState {
        frame: Some(&mut owner),
        failure: None,
    });
    let evaluator =
        |call: &mut AdvApp2VarSquareEvaluation<'_, '_>, _: &mut [f64], error: &mut i32| {
            call.set_argument(Argument::Dimension, 1);
            assert_eq!(call.integer(Field::VRootCount), 1);
            call.set_integer(Field::VRootCount, 2);
            assert_eq!(call.argument(Argument::Dimension), 2);
            call.set_argument(Argument::ParameterCount, 1);
            assert_eq!(call.integer(Field::URootCount), 1);
            call.set_integer(Field::URootCount, 2);
            assert_eq!(call.argument(Argument::ParameterCount), 2);
            call.set_argument(Argument::FavoriteIso, 7);
            assert_eq!(call.integer(Field::FavoriteIso), 2);
            call.set_integer(Field::FavoriteIso, 1);
            assert_eq!(call.argument(Argument::FavoriteIso), 7);
            call.set_argument(Argument::UOrder, 2);
            assert_eq!(call.argument(Argument::VOrder), 2);
            call.set_argument(Argument::VOrder, 3);
            assert_eq!(call.argument(Argument::UOrder), 3);
            call.u_bounds()[0] = -0.5;
            assert_eq!(call.v_bounds()[0], 0.);
            call.v_bounds()[1] = 0.75;
            assert_eq!(call.u_bounds()[1], 1.);
            *call.constant_parameter() = 0.75;
            assert_eq!(*call.constant_parameter(), 0.75);
            call.parameters()[0] = 0.5;
            assert_eq!(call.parameters()[0], 0.5);
            *error = 0;
        };
    let bridge = SquareCallerBridge {
        state: &state,
        evaluator: &evaluator,
        varying: OnceLock::from(Field::URootCount),
    };
    assert_eq!(dispatch(&bridge), 0);
    let state = state.into_inner().unwrap();
    assert!(state.failure.is_none());
    assert_eq!(state.frame.unwrap().root_counts(), [2, 2]);
}
#[test]
fn square_callback_releases_guard_and_reentrant_stage_and_dispatch_fail_explicitly() {
    let mut owner = frame();
    let state = Mutex::new(OperationFrameState {
        frame: Some(&mut owner),
        failure: None,
    });
    let evaluator = |call: &mut AdvApp2VarSquareEvaluation<'_, '_>,
                     _: &mut [f64],
                     error: &mut i32| {
        assert!(state.try_lock().is_ok());
        call.set_argument(Argument::Dimension, 1);
        let forbidden = |_: &mut AdvApp2VarSquareEvaluation<'_, '_>, _: &mut [f64], _: &mut i32| {
            panic!("nested callback must not execute")
        };
        let nested = SquareCallerBridge {
            state: &state,
            evaluator: &forbidden,
            varying: OnceLock::from(Field::URootCount),
        };
        assert_eq!(dispatch(&nested), 1);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || nested.caller_square_root_counts()
            ))
            .is_err()
        );
        assert!(state.try_lock().is_ok());
        *error = 1;
    };
    let bridge = SquareCallerBridge {
        state: &state,
        evaluator: &evaluator,
        varying: OnceLock::from(Field::URootCount),
    };
    assert_eq!(dispatch(&bridge), 1);
    let state = state.into_inner().unwrap();
    assert!(
        state
            .failure
            .unwrap()
            .message()
            .contains("callback active=true")
    );
    assert_eq!(state.frame.unwrap().integer(Field::Dimension), 1);
}
#[test]
fn square_frame_is_restored_without_poisoning_when_evaluator_unwinds() {
    let mut owner = frame();
    let state = Mutex::new(OperationFrameState {
        frame: Some(&mut owner),
        failure: None,
    });
    let evaluator = |call: &mut AdvApp2VarSquareEvaluation<'_, '_>, _: &mut [f64], _: &mut i32| {
        assert!(state.try_lock().is_ok());
        call.set_argument(Argument::Dimension, 1);
        panic!("owned square unwind fixture");
    };
    let bridge = SquareCallerBridge {
        state: &state,
        evaluator: &evaluator,
        varying: OnceLock::from(Field::URootCount),
    };
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dispatch(&bridge))).is_err());
    let state = state.into_inner().unwrap();
    assert!(state.failure.is_none());
    assert_eq!(state.frame.unwrap().integer(Field::VRootCount), 1);
}
#[test]
fn square_caller_storage_is_send_sync_and_reports_its_owned_metadata() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<Frame>();
    send_sync::<AdvApp2VarSquareIntegerBindings>();
    send_sync::<Field>();
    send_sync::<AdvApp2VarSquareEvaluation<'static, 'static>>();
    assert_eq!(
        std::mem::size_of::<Frame>(),
        2 * std::mem::size_of::<Vec<i32>>()
            + std::mem::size_of::<AdvApp2VarSquareIntegerBindings>()
    );
    println!(
        "{{\"metric\":\"square_caller_frame\",\"inline_bytes\":{},\"integer_payload_bytes\":12,\"callback_heap_allocations\":0}}",
        std::mem::size_of::<Frame>()
    );
}

#[test]
fn indexed_square_view_preserves_integer_orders_and_shared_real_slots() {
    let mut owner = frame();
    let (mut dimension, mut iso, mut count, mut order) = (3, 2, 3, 0);
    let (mut u, mut v, mut constant, mut values) = ([0., 1.], [0., 1.], 0.5, [7.; 9]);
    let mut arguments = AdvApp2VarBufferEvaluation {
        dimension: &mut dimension,
        favor_iso: &mut iso,
        nb_params: &mut count,
        u_start_end: &mut u,
        v_start_end: &mut v,
        const_param: &mut constant,
        orders: AdvApp2VarDerivativeOrders::Shared(&mut order),
        buffers: AdvApp2VarEvaluationBuffers::shared(&mut values, 0, 9, 0, 9),
    };
    let mut call = AdvApp2VarSquareBufferEvaluation {
        frame: &mut owner,
        call: &mut arguments,
        varying: Field::URootCount,
        callback_lifetime: std::marker::PhantomData,
        roots: SquareRootBindings::default(),
        tables: crate::adv_app2var::square_tables::SquareTableBindings::default(),
        intervals: crate::adv_app2var::square_intervals::SquareIntervalBindings::default(),
    };
    call.set_argument(Argument::Dimension, 1);
    assert_eq!(call.integer(Field::VRootCount), 1);
    call.set_argument(Argument::ParameterCount, 2);
    assert_eq!(call.integer(Field::URootCount), 2);
    call.set_argument(Argument::UOrder, 2);
    assert_eq!(call.argument(Argument::VOrder), 2);
    call.buffers().set_result(0, 0.25);
    assert_eq!(call.buffers().parameter(0), 0.25);
    call.buffers().set_parameter(1, 0.75);
    assert_eq!(call.buffers().result(1), 0.75);
    call.u_bounds()[0] = -0.5;
    assert_eq!(*call.v_bounds(), [0., 1.]);
    *call.constant_parameter() = 0.75;
    assert_eq!(*call.constant_parameter(), 0.75);
    fn send_sync<T: Send + Sync>() {}
    send_sync::<AdvApp2VarSquareBufferEvaluation<'static, 'static>>();
    assert_eq!(
        std::mem::size_of::<AdvApp2VarSquareBufferEvaluation<'_, '_>>(),
        std::mem::size_of::<AdvApp2VarSquareEvaluation<'_, '_>>()
    );
}

#[test]
fn indexed_root_view_reaches_slots_before_parameter_origin() {
    use crate::adv_app2var::AdvApp2VarSquareRootAxis::{U, V};
    let mut owner = frame();
    let (mut dimension, mut iso, mut count, mut order) = (3, 2, 3, 0);
    let (mut u, mut v, mut constant, mut values) = (
        [0., 1.],
        [0., 1.],
        0.5,
        [0.25, 0.5, 0.75, 7., 7., 7., 7., 7.],
    );
    let mut arguments = AdvApp2VarBufferEvaluation {
        dimension: &mut dimension,
        favor_iso: &mut iso,
        nb_params: &mut count,
        u_start_end: &mut u,
        v_start_end: &mut v,
        const_param: &mut constant,
        orders: AdvApp2VarDerivativeOrders::Shared(&mut order),
        buffers: AdvApp2VarEvaluationBuffers::shared(&mut values, 1, 8, 5, 8),
    };
    let mut roots = SquareRootBindings::default();
    roots.bind(U, 0);
    roots.bind(V, 0);
    let mut call = AdvApp2VarSquareBufferEvaluation {
        frame: &mut owner,
        call: &mut arguments,
        varying: Field::URootCount,
        callback_lifetime: std::marker::PhantomData,
        roots,
        tables: crate::adv_app2var::square_tables::SquareTableBindings::default(),
        intervals: crate::adv_app2var::square_intervals::SquareIntervalBindings::default(),
    };
    assert_eq!(call.caller_root(U, 0).unwrap(), 0.25);
    call.set_caller_root(U, 0, 0.125).unwrap();
    assert_eq!(call.caller_root(V, 0).unwrap(), 0.125);
    call.set_caller_root(U, 1, 0.25).unwrap();
    assert_eq!(call.buffers().parameter(0), 0.25);
    call.buffers().set_parameter(1, 0.5);
    assert_eq!(call.caller_root(V, 2).unwrap(), 0.5);
    assert!(call.caller_root(U, 8).is_err());
    assert!(call.set_caller_root(V, usize::MAX, 1.).is_err());
}
