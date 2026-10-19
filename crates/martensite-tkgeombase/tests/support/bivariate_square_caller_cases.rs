//! Non-varying caller counts share real integer slots, not callback-local controls.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarIntegerField as Argument, AdvApp2VarSquareEvaluation as Call,
    AdvApp2VarSquareIntegerBindings as Bindings, AdvApp2VarSquareIntegerField as Field,
    AdvApp2VarSquareOperationFrame as Frame,
};
fn square_caller_bindings(iso: i32) -> Bindings {
    Bindings {
        dimension: 0,
        root_counts: if iso == 1 { [0, 1] } else { [1, 0] },
        favorite_iso: 2,
    }
}
fn square_caller_case(iso: i32, fail: bool) {
    let mut request = SquareStorage::new().request(iso);
    request.ndimen = 3;
    let mut frame = Frame::new(vec![3, 3, iso], square_caller_bindings(iso)).unwrap();
    let trace = Mutex::new(Vec::new());
    let nonvarying = if iso == 1 {
        Field::URootCount
    } else {
        Field::VRootCount
    };
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        let dimension = call.argument(Argument::Dimension);
        assert_eq!(call.integer(nonvarying), dimension);
        assert_eq!(call.argument(Argument::ParameterCount), 3);
        assert_eq!(call.argument(Argument::FavoriteIso), iso);
        assert_eq!(call.argument(Argument::UOrder), 0);
        assert_eq!(call.argument(Argument::VOrder), 0);
        assert_eq!(*call.u_bounds(), [0., 1.]);
        assert_eq!(*call.v_bounds(), [0., 1.]);
        let first = (1. - (3_f64 / 5.).sqrt()) / 2.;
        assert!((call.parameters()[0] - first).abs() <= f64::EPSILON);
        assert_eq!(call.parameters()[1], 0.5);
        trace
            .lock()
            .unwrap()
            .push((dimension, *call.constant_parameter()));
        values[..(dimension * 3) as usize].fill(1.);
        call.set_argument(Argument::Dimension, 1);
        assert_eq!(call.integer(nonvarying), 1);
        *error = i32::from(fail);
    };
    let (mut sums, mut u_differences, mut v_differences, mut both_differences) =
        ([0.; 64], [0.; 64], [0.; 64], [0.; 64]);
    let mut tables = SquareTables {
        sosotb: &mut sums,
        disotb: &mut u_differences,
        soditb: &mut v_differences,
        diditb: &mut both_differences,
    };
    let (mut samples, mut parameters) = ([7.; 64], [7.; 64]);
    let mut error = -1;
    assert_eq!(
        frame
            .sample_square(
                &mut request,
                &evaluator,
                &mut tables,
                &mut samples,
                &mut parameters,
                &mut error
            )
            .unwrap(),
        0
    );
    assert_eq!(error, if fail && iso == 2 { 201 } else { 0 });
    assert_eq!(request.ndimen, 1);
    assert_eq!(
        [request.nbpntu, request.nbpntv],
        if iso == 1 { [1, 3] } else { [3, 1] }
    );
    assert_eq!(request.isofav, iso);
    assert_eq!((request.uintfn, request.vintfn), ([0., 1.], [0., 1.]));
    let trace = trace.lock().unwrap();
    let first = (1. - (3_f64 / 5.).sqrt()) / 2.;
    assert_eq!(trace.len(), if fail { 1 } else { 3 });
    assert_eq!(trace[0].0, 3);
    assert!((trace[0].1 - first).abs() <= f64::EPSILON);
    if !fail {
        assert_eq!(trace[1..], [(1, 0.5), (1, 0.5)]);
    }
    let expected = if fail { [0.; 4] } else { [1., 2., 2., 4.] };
    assert_eq!(tables.sosotb[..4], expected);
    assert_eq!(tables.sosotb[4..], [0.; 60]);
    assert_eq!(tables.disotb, [0.; 64]);
    assert_eq!(tables.soditb, [0.; 64]);
    assert_eq!(tables.diditb, [0.; 64]);
    assert_eq!(frame.integer(nonvarying), 1);
    assert_eq!(samples[9..], [7.; 55]);
    assert_eq!(parameters[3..], [7.; 61]);
}
#[test]
fn first_iso_shared_dimension_nonvarying_count_changes_live_root_lookup() {
    square_caller_case(1, false);
}
#[test]
fn second_iso_shared_dimension_nonvarying_count_changes_live_root_lookup() {
    square_caller_case(2, false);
}
#[test]
fn first_iso_shared_caller_count_survives_retained_error_clearing() {
    square_caller_case(1, true);
}
#[test]
fn second_iso_shared_caller_count_survives_evaluator_error201() {
    square_caller_case(2, true);
}
#[test]
fn square_shared_integer_slots_reject_bounds_and_inconsistent_initial_inputs() {
    let mut bad = square_caller_bindings(2);
    bad.dimension = 3;
    assert_eq!(
        Frame::new(vec![3, 3, 2], bad)
            .err()
            .unwrap()
            .exception_type(),
        "Standard_OutOfRange"
    );
    let mut frame = Frame::new(vec![0; 3], square_caller_bindings(2)).unwrap();
    let mut request = SquareStorage::new().request(2);
    let mut storage = SquareStorage::new();
    let callback = |_: &mut Call<'_, '_>, _: &mut [f64], _: &mut i32| {
        panic!("inconsistent shared seed must fail before callback")
    };
    let mut tables = SquareTables {
        sosotb: &mut storage.soso,
        disotb: &mut storage.diso,
        soditb: &mut storage.sodi,
        diditb: &mut storage.didi,
    };
    let failure = frame
        .sample_square(
            &mut request,
            &callback,
            &mut tables,
            &mut storage.samples,
            &mut storage.parameters,
            &mut -1,
        )
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_DomainError");
    assert!(
        failure
            .message()
            .contains("VRootCount value=3; expected shared initial value=2")
    );
    assert_eq!(tables.sosotb, [0.; 8]);
}
