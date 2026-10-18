//! Defined integer aliases characterized by the bounded retained source probe.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarIntegerBindings as Bindings, AdvApp2VarIntegerField as Field,
    AdvApp2VarScalarEvaluation, AdvApp2VarScalarProvider,
};

fn curve_alias_bindings() -> Bindings {
    Bindings {
        dimension: 0,
        favorite_iso: 0,
        parameter_count: 1,
        u_order: 2,
        v_order: 3,
    }
}
fn square_alias_bindings() -> Bindings {
    Bindings {
        dimension: 0,
        favorite_iso: 1,
        parameter_count: 0,
        u_order: 2,
        v_order: 2,
    }
}
fn curve_alias_callback(call: &mut AdvApp2VarScalarEvaluation<'_>, _: &mut [f64], error: &mut i32) {
    call.set_integer(Field::Dimension, 1);
    assert_eq!(call.integer(Field::FavoriteIso), 1);
    call.set_integer(Field::ParameterCount, 2);
    call.set_integer(Field::FavoriteIso, 2);
    assert_eq!(call.integer(Field::Dimension), 2);
    call.set_integer(Field::UOrder, 7);
    assert_eq!(call.integer(Field::VOrder), 0);
    call.set_integer(Field::VOrder, 9);
    assert_eq!(call.integer(Field::UOrder), 7);
    *call.const_param = 0.75;
    call.u_start_end[0] = -0.5;
    call.v_start_end[1] = 0.75;
    *error = 1;
}

#[test]
fn same_initialized_integer_slot_is_visible_as_both_curve_dimension_and_iso() {
    let provider = AdvApp2VarScalarProvider::new(
        vec![2, 3, 0, 0],
        curve_alias_bindings(),
        curve_alias_callback,
    )
    .unwrap();
    for _ in 0..2 {
        let mut request = curve_request(2);
        let mut storage = CurveStorage::new();
        let mut output = storage.output();
        let mut error = -1;
        mma2fnc_mut_(&mut request, &provider, &mut output, &mut error);
        assert_eq!(
            (request.ndimen, request.isofav, request.tconst, error),
            (2, 2, 0.75, 101)
        );
        assert_eq!(request.uvfonc, [0., 1., 0., 1.]);
        assert_eq!(output.nbcrbe, 0);
        assert_eq!(&output.courbe[..6], &[0.; 6]);
        assert_eq!(&output.courbe[6..], &[7.; 26]);
        assert!(provider.last_failure().unwrap().is_none());
        assert_eq!(provider.integer_values().unwrap(), [2, 2, 7, 9]);
    }
}

fn square_alias_callback(
    call: &mut AdvApp2VarScalarEvaluation<'_>,
    _: &mut [f64],
    error: &mut i32,
) {
    call.set_integer(Field::Dimension, 1);
    assert_eq!(call.integer(Field::ParameterCount), 1);
    call.set_integer(Field::ParameterCount, 2);
    assert_eq!(call.integer(Field::Dimension), 2);
    call.set_integer(Field::UOrder, 7);
    assert_eq!(call.integer(Field::VOrder), 7);
    call.set_integer(Field::VOrder, 9);
    assert_eq!(call.integer(Field::UOrder), 9);
    *call.const_param = 0.75;
    call.u_start_end[0] = -0.5;
    call.v_start_end[1] = 0.75;
    *error = 1;
}

#[test]
fn same_initialized_integer_slot_is_visible_as_both_square_dimension_and_count() {
    let provider = AdvApp2VarScalarProvider::new(
        vec![2, 2, 0],
        square_alias_bindings(),
        square_alias_callback,
    )
    .unwrap();
    let mut storage = SquareStorage::new();
    let mut request = storage.request(2);
    request.nbpntu = 2;
    request.urootb = &[-0.5773502691896257, 0.5773502691896257];
    assert_eq!(storage.sample(&mut request, &provider), 201);
    assert_eq!((request.ndimen, request.nbpntu, request.nbpntv), (2, 2, 3));
    assert_eq!((request.uintfn, request.vintfn), ([0., 1.], [0., 1.]));
    assert_eq!(provider.integer_values().unwrap(), [2, 2, 9]);
    assert!(provider.last_failure().unwrap().is_none());
}

#[test]
fn invalid_slot_index_is_rejected_before_evaluation() {
    let result =
        AdvApp2VarScalarProvider::new(vec![0], curve_alias_bindings(), curve_alias_callback);
    let failure = result.err().expect("slot1 does not fit in one value");
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert!(failure.message().contains("slot=1"));
    assert!(failure.message().contains("0..1"));
}

#[test]
fn inconsistent_shared_initial_values_are_reported_and_repairable() {
    let calls = Mutex::new(0);
    let evaluator = |_: &mut AdvApp2VarScalarEvaluation<'_>, _: &mut [f64], error: &mut i32| {
        *calls.lock().unwrap() += 1;
        *error = 1;
    };
    let provider =
        AdvApp2VarScalarProvider::new(vec![0; 4], curve_alias_bindings(), evaluator).unwrap();
    let mut storage = CurveStorage::new();
    let mut error = -1;
    mma2fnc_mut_(
        &mut curve_request(1),
        &provider,
        &mut storage.output(),
        &mut error,
    );
    assert_eq!(*calls.lock().unwrap(), 0);
    assert_eq!(error, 101);
    let failure = provider.last_failure().unwrap().unwrap();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(failure.message().contains("expected shared input value=2"));
    mma2fnc_mut_(
        &mut curve_request(2),
        &provider,
        &mut storage.output(),
        &mut error,
    );
    assert_eq!(*calls.lock().unwrap(), 1);
    assert!(provider.last_failure().unwrap().is_none());
}

#[test]
fn derived_order_identity_cannot_be_replaced_by_wrong_provider_bindings() {
    let mut bindings = square_alias_bindings();
    bindings.v_order = 3;
    let provider =
        AdvApp2VarScalarProvider::new(vec![0; 4], bindings, square_alias_callback).unwrap();
    let mut storage = SquareStorage::new();
    let mut request = storage.request(2);
    request.nbpntu = 2;
    request.urootb = &[-0.5773502691896257, 0.5773502691896257];
    assert_eq!(storage.sample(&mut request, &provider), 201);
    let failure = provider.last_failure().unwrap().unwrap();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(failure.message().contains("expected shared=true"));
}

#[test]
fn immutable_evaluation_rejection_retains_typed_diagnostics() {
    let provider =
        AdvApp2VarScalarProvider::new(vec![0; 4], curve_alias_bindings(), curve_alias_callback)
            .unwrap();
    let request = AdvApp2VarEvaluation {
        dimension: 2,
        u_start_end: [0., 1.],
        v_start_end: [0., 1.],
        favor_iso: 2,
        const_param: 0.5,
        nb_params: 3,
        parameters: &[0., 0.5, 1.],
        u_order: 0,
        v_order: 0,
    };
    let mut error = -1;
    provider.evaluate(&request, &mut [0.; 6], &mut error);
    assert_eq!(error, 1);
    let failure = provider.last_failure().unwrap().unwrap();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(
        failure
            .message()
            .contains("expected an evaluate_slots call")
    );
}

#[test]
fn scalar_provider_keeps_the_constant_square_fold_on_both_axes() {
    for iso in [1, 2] {
        let bindings = Bindings {
            dimension: 0,
            favorite_iso: 1,
            parameter_count: 2,
            u_order: 3,
            v_order: 3,
        };
        let evaluator =
            |_: &mut AdvApp2VarScalarEvaluation<'_>, result: &mut [f64], error: &mut i32| {
                result.fill(1.);
                *error = 0;
            };
        let provider = AdvApp2VarScalarProvider::new(vec![0; 4], bindings, evaluator).unwrap();
        let mut storage = SquareStorage::new();
        let mut request = storage.request(iso);
        assert_eq!(storage.sample(&mut request, &provider), 0);
        assert_eq!(storage.soso, [1., 2., 2., 4., 1., 2., 2., 4.]);
        assert_eq!(storage.diso, [0.; 2]);
        assert_eq!(storage.sodi, [0.; 2]);
        assert_eq!(storage.didi, [0.; 8]);
        assert!(provider.last_failure().unwrap().is_none());
    }
}
#[test]
fn scalar_provider_and_owned_identities_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    type Callback = fn(&mut AdvApp2VarScalarEvaluation<'_>, &mut [f64], &mut i32);
    assert_send_sync::<AdvApp2VarScalarProvider<Callback>>();
    assert_send_sync::<AdvApp2VarScalarEvaluation<'static>>();
    assert_send_sync::<Bindings>();
    assert_send_sync::<Field>();
}
