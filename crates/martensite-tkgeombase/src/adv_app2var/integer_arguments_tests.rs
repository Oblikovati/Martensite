//! Both buffer representations use the same integer incidence transaction.
use super::*;
use crate::adv_app2var::integer_frame::IntegerFrame;
use crate::adv_app2var::{
    AdvApp2VarDerivativeOrders as Orders, AdvApp2VarEvaluationBuffers as Buffers,
    AdvApp2VarIntegerBindings as Bindings,
};
use martensite_tkernel::standard::Failure;

fn check_fields(call: &mut impl IntegerArguments, shared: bool) {
    use AdvApp2VarIntegerField::*;
    assert_eq!(call.shared_orders(), shared);
    for (field, value) in [
        (Dimension, 3),
        (FavoriteIso, 2),
        (ParameterCount, 5),
        (UOrder, 7),
        (VOrder, 9),
    ] {
        call.set_argument_value(field, value);
        assert_eq!(call.argument_value(field), value);
    }
    assert_eq!(call.argument_value(UOrder), if shared { 9 } else { 7 });
}

#[test]
fn retained_slice_arguments_keep_independent_orders() {
    let (mut dimension, mut iso, mut count, mut u, mut v) = (0, 0, 0, 0, 0);
    let (mut ub, mut vb, mut constant, mut parameters) = ([0.; 2], [0.; 2], 0., [0.; 1]);
    let mut call = AdvApp2VarEvaluationSlots {
        dimension: &mut dimension,
        favor_iso: &mut iso,
        nb_params: &mut count,
        u_start_end: &mut ub,
        v_start_end: &mut vb,
        const_param: &mut constant,
        orders: Orders::Independent {
            u: &mut u,
            v: &mut v,
        },
        parameters: &mut parameters,
    };
    check_fields(&mut call, false);
}

#[test]
fn indexed_arguments_keep_shared_orders_and_full_buffer_identity() {
    let (mut dimension, mut iso, mut count, mut order) = (0, 0, 0, 0);
    let (mut ub, mut vb, mut constant, mut storage) = ([0.; 2], [0.; 2], 0., [0.; 86]);
    let mut call = AdvApp2VarBufferEvaluation {
        dimension: &mut dimension,
        favor_iso: &mut iso,
        nb_params: &mut count,
        u_start_end: &mut ub,
        v_start_end: &mut vb,
        const_param: &mut constant,
        orders: Orders::Shared(&mut order),
        buffers: Buffers::shared(&mut storage, 0, 10, 14, 50),
    };
    check_fields(&mut call, true);
    call.buffers.set_result(71, 1.);
    assert_eq!(call.buffers.parameter(85), 1.);
    let bindings = Bindings {
        dimension: 0,
        favorite_iso: 1,
        parameter_count: 2,
        u_order: 3,
        v_order: 3,
    };
    let mut frame = IntegerFrame::new(vec![0; 4], bindings).unwrap();
    frame.bind_caller_dimensions(vec![0]).unwrap();
    frame.seed_caller_dimensions(&[3]).unwrap();
    let outcome = frame.evaluate_integer_arguments(
        &mut call,
        None,
        None,
        None,
        None,
        |call, values, bindings, _, _, _, _, _, _, _, _, dimensions| {
            let slot = dimensions.unwrap()[0];
            values[bindings.dimension] = 2;
            assert_eq!(values[slot], 2);
            call.buffers.set_result(64, 1.);
            Err(Failure::with_message(
                "BoundedCallbackFailure",
                "owned frame rollback=none; expected writeback",
            ))
        },
    );
    assert!(outcome.is_err());
    assert_eq!(call.argument_value(AdvApp2VarIntegerField::Dimension), 2);
    assert_eq!(frame.caller_dimensions(), Some(vec![2]));
    assert_eq!(call.buffers.parameter(78), 1.);
}
