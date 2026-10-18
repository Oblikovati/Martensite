//! Initialized root/parameter overlap follows live scalar reads, not root snapshots.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarSquareBufferEvaluation as Call, AdvApp2VarSquareBufferEvaluator,
    AdvApp2VarSquareIntegerBindings as Bindings, AdvApp2VarSquareOperationFrame as Frame,
    AdvApp2VarSquareRootAxis as Axis, AdvApp2VarSquareSampleFrame as Samples,
};
const ROOTS: [f64; 3] = [-0.7745966692414834, 0., 0.7745966692414834];
fn half_map(value: f64) -> f64 {
    (1. + value) / 2.
}
fn root_frame(iso: i32) -> Frame {
    Frame::new(
        vec![1, 3, 3, iso, -1],
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
fn root_request(iso: i32) -> SampledSquare<'static> {
    SampledSquare {
        ndimen: 1,
        uintfn: [0., 1.],
        vintfn: [0., 1.],
        nbpntu: 3,
        nbpntv: 3,
        urootb: &[],
        vrootb: &[],
        isofav: iso,
    }
}
fn root_samples(start: usize) -> Samples {
    let mut values = vec![7.; 129];
    values[..3].copy_from_slice(&ROOTS);
    Samples::new(values, start, 65)
        .unwrap()
        .with_root_origin(Axis::U, 0)
        .unwrap()
        .with_root_origin(Axis::V, 0)
        .unwrap()
}
struct RootTables {
    sums: [f64; 64],
    u: [f64; 64],
    v: [f64; 64],
    both: [f64; 64],
}
impl RootTables {
    fn new() -> Self {
        Self {
            sums: [0.; 64],
            u: [0.; 64],
            v: [0.; 64],
            both: [0.; 64],
        }
    }
    fn run(
        &mut self,
        frame: &mut Frame,
        request: &mut SampledSquare<'_>,
        evaluator: &impl AdvApp2VarSquareBufferEvaluator,
        samples: &mut Samples,
    ) -> Result<i32, martensite_tkernel::standard::Failure> {
        let mut tables = SquareTables {
            sosotb: &mut self.sums,
            disotb: &mut self.u,
            soditb: &mut self.v,
            diditb: &mut self.both,
        };
        frame.sample_square_owned_roots(request, evaluator, &mut tables, samples)
    }
    fn assert_constant_one(&self, multiplier: f64) {
        assert_eq!(
            &self.sums[..4],
            &[
                multiplier,
                2. * multiplier,
                2. * multiplier,
                4. * multiplier
            ]
        );
        assert_eq!(self.sums[4..], [0.; 60]);
        assert_eq!(self.u, [0.; 64]);
        assert_eq!(self.v, [0.; 64]);
        assert_eq!(self.both, [0.; 64]);
    }
}
fn initialized_values(start: usize) -> ([f64; 3], [f64; 3]) {
    if start == 0 {
        return (ROOTS.map(half_map), ROOTS.map(half_map));
    }
    let first = half_map(ROOTS[0]);
    let second = half_map(first);
    ([ROOTS[0], first, second], [first, second, half_map(second)])
}
fn assert_near(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= f64::EPSILON,
        "actual={actual:?}; expected={expected:?}"
    );
}
fn root_parameter_case(iso: i32, start: usize, mutate: bool) {
    let mut frame = root_frame(iso);
    let mut request = root_request(iso);
    let mut samples = root_samples(start);
    let mut tables = RootTables::new();
    let trace = Mutex::new(Vec::new());
    let (roots, parameters) = initialized_values(start);
    let evaluator = |call: &mut Call<'_, '_>| {
        let made = trace.lock().unwrap().len();
        let mut expected_roots = roots;
        let mut expected_parameters = parameters;
        if mutate && made > 0 {
            expected_roots[2] = 0.;
            expected_parameters[2 - start] = 0.;
        }
        for index in 0..3 {
            assert_near(
                call.caller_root(Axis::U, index).unwrap(),
                expected_roots[index],
            );
            assert_eq!(
                call.caller_root(Axis::U, index).unwrap(),
                call.caller_root(Axis::V, index).unwrap()
            );
            assert_near(call.buffers().parameter(index), expected_parameters[index]);
        }
        trace.lock().unwrap().push(*call.constant_parameter());
        if mutate && made == 0 {
            call.set_caller_root(Axis::V, 2, 0.).unwrap();
            assert_eq!(call.caller_root(Axis::U, 2).unwrap(), 0.);
            assert_eq!(call.buffers().parameter(2 - start), 0.);
        }
        for index in 0..3 {
            call.buffers().set_result(index, 1.);
        }
        call.set_status(0).unwrap();
    };
    assert_eq!(
        tables
            .run(&mut frame, &mut request, &evaluator, &mut samples)
            .unwrap(),
        0
    );
    assert_eq!(frame.status().unwrap(), 0);
    let expected = [
        half_map(roots[0]),
        0.5,
        if mutate { 0.5 } else { half_map(roots[2]) },
    ];
    assert_eq!(trace.lock().unwrap().len(), 3);
    for (actual, expected) in trace.lock().unwrap().iter().zip(expected) {
        assert_near(*actual, expected);
    }
    tables.assert_constant_one(1.);
    for index in 0..3 {
        assert_near(
            samples.root(Axis::U, index).unwrap(),
            if mutate && index == 2 {
                0.
            } else {
                roots[index]
            },
        );
        assert_near(
            samples.parameter(index).unwrap(),
            if mutate && index == 2 - start {
                0.
            } else {
                parameters[index]
            },
        );
        assert_eq!(samples.sample(index).unwrap(), 1.);
    }
    for index in (start + 3).max(3)..65 {
        assert_eq!(samples.root(Axis::U, index).unwrap(), 7.);
    }
    for index in 3..64 {
        assert_eq!(samples.sample(index).unwrap(), 7.);
    }
    assert!(request.urootb.is_empty() && request.vrootb.is_empty());
    assert_eq!(
        (
            request.ndimen,
            request.nbpntu,
            request.nbpntv,
            request.isofav
        ),
        (1, 3, 3, iso)
    );
}
#[test]
fn first_iso_root_parameter_identity_changes_constant_lookup_after_initialization() {
    root_parameter_case(1, 0, false);
}
#[test]
fn second_iso_root_parameter_identity_changes_constant_lookup_after_initialization() {
    root_parameter_case(2, 0, false);
}
#[test]
fn first_iso_shifted_parameter_writes_change_each_next_root_read() {
    root_parameter_case(1, 1, false);
}
#[test]
fn second_iso_shifted_parameter_writes_change_each_next_root_read() {
    root_parameter_case(2, 1, false);
}
#[test]
fn first_iso_callback_root_write_changes_later_constant_lookup() {
    root_parameter_case(1, 0, true);
}
#[test]
fn second_iso_callback_root_write_changes_later_constant_lookup() {
    root_parameter_case(2, 0, true);
}
#[test]
fn first_iso_shifted_callback_root_write_changes_parameter_and_later_constant() {
    root_parameter_case(1, 1, true);
}
#[test]
fn second_iso_shifted_callback_root_write_changes_parameter_and_later_constant() {
    root_parameter_case(2, 1, true);
}
#[test]
fn owned_roots_reject_duplicate_borrowed_inputs_and_ambiguous_entrypoints() {
    let mut frame = root_frame(2);
    let mut request = root_request(2);
    request.urootb = &ROOTS;
    let mut samples = root_samples(1);
    let mut tables = RootTables::new();
    let forbidden =
        |_: &mut Call<'_, '_>| panic!("ambiguous owned root source must fail before callback");
    let failure = tables
        .run(&mut frame, &mut request, &forbidden, &mut samples)
        .unwrap_err();
    assert!(
        failure
            .message()
            .contains("U borrowed roots len=3; expected empty")
    );
    assert_eq!(samples.root(Axis::U, 0).unwrap(), ROOTS[0]);
    request.urootb = &[];
    let mut raw = SquareTables {
        sosotb: &mut tables.sums,
        disotb: &mut tables.u,
        soditb: &mut tables.v,
        diditb: &mut tables.both,
    };
    let failure = frame
        .sample_square_owned_samples(&mut request, &forbidden, &mut raw, &mut samples)
        .unwrap_err();
    assert!(failure.message().contains("expected owned-root entrypoint"));
    let mut unbound = Samples::new(vec![7.; 129], 1, 65).unwrap();
    assert!(
        tables
            .run(&mut frame, &mut request, &forbidden, &mut unbound)
            .is_err()
    );
    assert_eq!(tables.sums, [0.; 64]);
}
#[test]
fn repeated_owned_root_operations_use_current_owner_without_stale_borrowed_seeding() {
    for iso in [1, 2] {
        for start in [0, 1] {
            let mut frame = root_frame(iso);
            let mut request = root_request(iso);
            let mut samples = root_samples(start);
            let mut tables = RootTables::new();
            let evaluator = |call: &mut Call<'_, '_>| {
                for index in 0..3 {
                    call.buffers().set_result(index, 1.);
                }
                call.set_status(0).unwrap();
            };
            for _ in 0..2 {
                assert_eq!(
                    tables
                        .run(&mut frame, &mut request, &evaluator, &mut samples)
                        .unwrap(),
                    0
                );
            }
            tables.assert_constant_one(2.);
            let (roots, parameters) = initialized_values(start);
            let expected = if start == 0 {
                roots.map(half_map)
            } else {
                roots
            };
            for (index, expected) in expected.into_iter().enumerate() {
                assert_near(samples.root(Axis::U, index).unwrap(), expected);
            }
            if start == 1 {
                for (index, expected) in parameters.into_iter().enumerate() {
                    assert_near(samples.parameter(index).unwrap(), expected);
                }
            }
        }
    }
}

#[test]
fn one_owned_root_axis_keeps_axis_identity_through_transposition() {
    for iso in [1, 2] {
        for owned in [Axis::U, Axis::V] {
            let mut frame = root_frame(iso);
            let mut request = root_request(iso);
            if owned == Axis::U {
                request.vrootb = &ROOTS;
            } else {
                request.urootb = &ROOTS;
            }
            let mut values = vec![7.; 129];
            values[..3].copy_from_slice(&ROOTS);
            let mut samples = Samples::new(values, 1, 65)
                .unwrap()
                .with_root_origin(owned, 0)
                .unwrap();
            let varying = if iso == 1 { Axis::V } else { Axis::U };
            let varying_owned = varying == owned;
            let parameters = if varying_owned {
                initialized_values(1).1
            } else {
                ROOTS.map(half_map)
            };
            let roots = if varying_owned {
                initialized_values(1).0
            } else {
                [ROOTS[0], parameters[0], parameters[1]]
            };
            let trace = Mutex::new(Vec::new());
            let evaluator = |call: &mut Call<'_, '_>| {
                for index in 0..3 {
                    assert_near(call.caller_root(owned, index).unwrap(), roots[index]);
                    assert_near(call.buffers().parameter(index), parameters[index]);
                    call.buffers().set_result(index, 1.);
                }
                let borrowed = if owned == Axis::U { Axis::V } else { Axis::U };
                assert!(call.caller_root(borrowed, 0).is_err());
                trace.lock().unwrap().push(*call.constant_parameter());
                call.set_status(0).unwrap();
            };
            let mut tables = RootTables::new();
            assert_eq!(
                tables
                    .run(&mut frame, &mut request, &evaluator, &mut samples)
                    .unwrap(),
                0
            );
            let constants = [
                half_map(ROOTS[0]),
                0.5,
                if varying_owned {
                    half_map(ROOTS[2])
                } else {
                    0.75
                },
            ];
            assert_eq!(trace.lock().unwrap().len(), 3);
            for (actual, expected) in trace.lock().unwrap().iter().zip(constants) {
                assert_near(*actual, expected);
            }
            tables.assert_constant_one(1.);
        }
    }
}
