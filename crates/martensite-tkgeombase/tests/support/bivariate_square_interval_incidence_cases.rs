//! Late caller interval capture follows transposition while local bounds stay copied.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarSquareBufferEvaluation as Call, AdvApp2VarSquareIntegerBindings as Bindings,
    AdvApp2VarSquareIntervalAxis as Axis, AdvApp2VarSquareOperationFrame as Frame,
    AdvApp2VarSquareSampleFrame as Samples, AdvApp2VarSquareTable as Table,
};
const UROOTS: [f64; 3] = [-0.7745966692414834, 0., 0.7745966692414834];
const VROOTS: [f64; 5] = [
    -0.906179845938664,
    -0.5384693101056831,
    0.,
    0.5384693101056831,
    0.906179845938664,
];
fn interval_frame(iso: i32) -> Frame {
    Frame::new(
        vec![1, 3, 5, iso, -1],
        Bindings {
            dimension: 0,
            root_counts: [1, 2],
            favorite_iso: 3,
        },
    )
    .unwrap()
    .with_owned_status_slot(4)
    .unwrap()
}
fn interval_request(iso: i32) -> SampledSquare<'static> {
    SampledSquare {
        ndimen: 1,
        uintfn: [0., 2.],
        vintfn: [0., 1.],
        nbpntu: 3,
        nbpntv: 5,
        urootb: &UROOTS,
        vrootb: &VROOTS,
        isofav: iso,
    }
}
fn interval_samples() -> Samples {
    let mut values = vec![7.; 128];
    for index in 0..6 {
        values[32 + index] = 2. * index as f64;
    }
    Samples::new(values, 96, 104)
        .unwrap()
        .with_table_origins([32; 4])
        .unwrap()
        .with_interval_origin(Axis::U, 32)
        .unwrap()
}
fn inverse_permutation(values: [f64; 6]) -> [f64; 6] {
    [0, 3, 1, 4, 2, 5].map(|index| values[index])
}
fn expected_table(iso: i32, mutated: bool) -> [f64; 6] {
    let weights = if iso == 1 {
        inverse_permutation(inverse_permutation([1., 2., 2., 2., 4., 4.]))
    } else {
        [1., 2., 2., 4., 2., 4.]
    };
    let delta = if !mutated {
        [0.; 6]
    } else if iso == 1 {
        inverse_permutation(inverse_permutation([-3., -2., 0., 0., 0., 0.]))
    } else {
        [-3., 4., 0., 0., 0., 0.]
    };
    std::array::from_fn(|index| 2. * index as f64 + weights[index] + delta[index])
}
fn run_interval_case(iso: i32, mutate: bool) {
    let mut frame = interval_frame(iso);
    let mut request = interval_request(iso);
    let mut samples = interval_samples();
    let calls = std::sync::Mutex::new(0);
    let evaluator = |call: &mut Call<'_, '_>| {
        let mut calls = calls.lock().unwrap();
        let index = *calls;
        *calls += 1;
        let local_u = if iso == 1 { [0., 8.] } else { [0., 2.] };
        assert_eq!(*call.u_bounds(), local_u);
        assert_eq!(*call.v_bounds(), [0., 1.]);
        let constants: Vec<f64> = if iso == 1 {
            UROOTS.map(|root| 4. + 4. * root).to_vec()
        } else {
            VROOTS.map(|root| 0.5 + 0.5 * root).to_vec()
        };
        assert_eq!(*call.constant_parameter(), constants[index]);
        let parameters: Vec<f64> = if iso == 1 {
            VROOTS.map(|root| 0.5 + 0.5 * root).to_vec()
        } else {
            UROOTS.map(|root| 1. + root).to_vec()
        };
        for (index, value) in parameters.iter().enumerate() {
            assert_eq!(call.buffers().parameter(index), *value);
        }
        if index == 0 {
            assert_eq!(
                [
                    call.caller_interval(Axis::U, 0).unwrap(),
                    call.caller_interval(Axis::U, 1).unwrap()
                ],
                local_u
            );
            if mutate {
                call.set_caller_interval(Axis::U, 0, -3.).unwrap();
                call.set_caller_interval(Axis::U, 1, 6.).unwrap();
                assert_eq!(call.caller_table(Table::SumSum, 0).unwrap(), -3.);
                assert_eq!(call.caller_table(Table::SumDifference, 1).unwrap(), 6.);
                assert_eq!(*call.u_bounds(), local_u);
            }
        }
        assert!(call.caller_interval(Axis::V, 0).is_err());
        assert!(call.caller_interval(Axis::U, 2).is_err());
        assert!(call.set_caller_interval(Axis::U, usize::MAX, 0.).is_err());
        for index in 0..parameters.len() {
            call.buffers().set_result(index, 1.);
        }
        call.set_status(0).unwrap();
    };
    frame
        .sample_square_owned_tables(&mut request, &evaluator, &mut samples)
        .unwrap();
    assert_eq!(*calls.lock().unwrap(), if iso == 1 { 3 } else { 5 });
    assert_eq!(frame.status().unwrap(), 0);
    let expected = expected_table(iso, mutate);
    for role in [
        Table::SumSum,
        Table::DifferenceSum,
        Table::SumDifference,
        Table::DifferenceDifference,
    ] {
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(samples.table(role, index).unwrap(), *value);
        }
        assert_eq!(samples.table(role, 6).unwrap(), 7.);
        assert_eq!(samples.table(role, 7).unwrap(), 7.);
    }
    assert_eq!(request.uintfn, [expected[0], expected[1]]);
    assert_eq!(request.vintfn, [0., 1.]);
    assert_eq!(
        [
            samples.interval(Axis::U, 0).unwrap(),
            samples.interval(Axis::U, 1).unwrap()
        ],
        request.uintfn
    );
}
#[test]
fn iso1_interval_capture_follows_actual_initial_table_transposition() {
    run_interval_case(1, false);
}
#[test]
fn iso2_interval_capture_retains_bound_slots() {
    run_interval_case(2, false);
}
#[test]
fn iso1_caller_interval_writes_leave_copied_bounds_and_affines_unchanged() {
    run_interval_case(1, true);
}
#[test]
fn iso2_caller_interval_writes_leave_copied_bounds_and_affines_unchanged() {
    run_interval_case(2, true);
}
#[test]
fn interval_origins_require_two_initialized_bit_consistent_slots() {
    let mut frame = interval_frame(1);
    let mut request = interval_request(1);
    let mut samples = interval_samples();
    let evaluator =
        |_: &mut Call<'_, '_>| panic!("invalid interval contract must fail before evaluator");
    request.uintfn = [-0., 2.];
    assert!(
        frame
            .sample_square_owned_tables(&mut request, &evaluator, &mut samples)
            .is_err()
    );
    assert!(samples.interval(Axis::V, 0).is_err());
    assert!(samples.interval(Axis::U, 2).is_err());
    assert!(
        Samples::new(vec![7.; 4], 0, 0)
            .unwrap()
            .with_interval_origin(Axis::U, 3)
            .is_err()
    );
    assert!(
        Samples::new(vec![7.; 4], 0, 0)
            .unwrap()
            .with_interval_origin(Axis::V, usize::MAX)
            .is_err()
    );
    fn send_sync<T: Send + Sync>() {}
    send_sync::<Axis>();
}
