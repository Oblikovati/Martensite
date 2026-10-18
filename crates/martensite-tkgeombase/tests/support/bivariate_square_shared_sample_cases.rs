//! Shared caller parameters/results preserve initialization and immediate writes.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarIntegerField as Argument, AdvApp2VarSquareBufferEvaluation as Call,
    AdvApp2VarSquareBufferEvaluator, AdvApp2VarSquareIntegerBindings as Bindings,
    AdvApp2VarSquareIntegerField as Field, AdvApp2VarSquareOperationFrame as Frame,
    AdvApp2VarSquareSampleFrame as Samples,
};
fn shared_frame(iso: i32, status_alias: bool) -> Frame {
    let values = if status_alias {
        vec![3, 3, 3, iso]
    } else {
        vec![1, 3, 3, iso, -1]
    };
    Frame::new(
        values,
        Bindings {
            dimension: 0,
            root_counts: [1, 2],
            favorite_iso: 3,
        },
    )
    .unwrap()
    .with_owned_status_slot(if status_alias { 0 } else { 4 })
    .unwrap()
}
fn shared_request(iso: i32, status_alias: bool) -> SampledSquare<'static> {
    let mut request = SquareStorage::new().request(iso);
    request.ndimen = if status_alias { 3 } else { 1 };
    request
}
struct SharedTables {
    sums: [f64; 64],
    u: [f64; 64],
    v: [f64; 64],
    both: [f64; 64],
}
impl SharedTables {
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
        frame.sample_square_owned_samples(request, evaluator, &mut tables, samples)
    }
}
fn gauss_parameters() -> [f64; 3] {
    let root = (3_f64 / 5.).sqrt();
    [(1. - root) / 2., 0.5, (1. + root) / 2.]
}
fn shared_case(iso: i32, status_alias: bool) {
    let mut frame = shared_frame(iso, status_alias);
    let mut request = shared_request(iso, status_alias);
    let mut samples = Samples::new(vec![7.; 64], 0, 0).unwrap();
    let mut tables = SharedTables::new();
    let trace = Mutex::new(Vec::new());
    let evaluator = |call: &mut Call<'_, '_>| {
        let made = trace.lock().unwrap().len();
        let dimension = if status_alias { 0 } else { 1 };
        assert_eq!(call.status().unwrap(), 0);
        assert_eq!(call.argument(Argument::Dimension), dimension);
        assert_eq!(call.argument(Argument::ParameterCount), 3);
        assert_eq!(call.argument(Argument::FavoriteIso), iso);
        assert_eq!(call.argument(Argument::UOrder), 0);
        assert_eq!(call.argument(Argument::VOrder), 0);
        let expected = if made == 0 {
            gauss_parameters()
        } else if status_alias {
            [0.75, 0.5, gauss_parameters()[2]]
        } else {
            [1.; 3]
        };
        for (index, expected) in expected.into_iter().enumerate() {
            let parameter = call.buffers().parameter(index);
            assert!((parameter - expected).abs() <= f64::EPSILON);
            assert_eq!(call.buffers().result(index), parameter);
        }
        trace.lock().unwrap().push(*call.constant_parameter());
        call.buffers().set_result(0, 0.25);
        assert_eq!(call.buffers().parameter(0), 0.25);
        call.buffers().set_parameter(0, 0.75);
        assert_eq!(call.buffers().result(0), 0.75);
        for index in 0..dimension * 3 {
            call.buffers().set_result(index as usize, 1.);
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
    assert_eq!(
        (
            request.ndimen,
            request.nbpntu,
            request.nbpntv,
            request.isofav
        ),
        (if status_alias { 0 } else { 1 }, 3, 3, iso)
    );
    let trace = trace.lock().unwrap();
    assert_eq!(trace.len(), 3);
    for (actual, expected) in trace.iter().zip(gauss_parameters()) {
        assert!((*actual - expected).abs() <= f64::EPSILON);
    }
    let expected = if status_alias {
        [0.; 4]
    } else {
        [1., 2., 2., 4.]
    };
    assert_eq!(tables.sums[..4], expected);
    assert_eq!(tables.sums[4..], [0.; 60]);
    assert_eq!(tables.u, [0.; 64]);
    assert_eq!(tables.v, [0.; 64]);
    assert_eq!(tables.both, [0.; 64]);
    let expected = if status_alias {
        [0.75, 0.5, gauss_parameters()[2]]
    } else {
        [1.; 3]
    };
    for (index, expected) in expected.into_iter().enumerate() {
        let value = samples.parameter(index).unwrap();
        assert!((value - expected).abs() <= f64::EPSILON);
        assert_eq!(samples.sample(index).unwrap(), value);
    }
    for index in 3..64 {
        assert_eq!(
            (
                samples.parameter(index).unwrap(),
                samples.sample(index).unwrap()
            ),
            (7., 7.)
        );
    }
}
#[test]
fn first_iso_shared_parameter_and_sample_allocation_retains_callback_writes() {
    shared_case(1, false);
}
#[test]
fn second_iso_shared_parameter_and_sample_allocation_retains_callback_writes() {
    shared_case(2, false);
}
#[test]
fn first_iso_shared_real_buffers_compose_with_actual_dimension_status_identity() {
    shared_case(1, true);
}
#[test]
fn second_iso_shared_real_buffers_compose_with_actual_dimension_status_identity() {
    shared_case(2, true);
}
#[test]
fn sample_origins_and_indices_are_checked_and_owned_status_is_required() {
    assert_eq!(
        Samples::new(vec![7.; 64], 65, 0)
            .err()
            .unwrap()
            .exception_type(),
        "Standard_OutOfRange"
    );
    assert_eq!(
        Samples::new(vec![7.; 64], 0, usize::MAX)
            .err()
            .unwrap()
            .exception_type(),
        "Standard_OutOfRange"
    );
    let empty = Samples::new(Vec::new(), 0, 0).unwrap();
    assert!(empty.parameter(0).is_err());
    assert!(empty.sample(0).is_err());
    let mut samples = Samples::new(vec![7.; 64], 0, 0).unwrap();
    assert!(samples.parameter(64).is_err());
    assert!(samples.sample(usize::MAX).is_err());
    let mut frame = Frame::new(
        vec![1, 3, 3, 2],
        Bindings {
            dimension: 0,
            root_counts: [1, 2],
            favorite_iso: 3,
        },
    )
    .unwrap();
    let mut request = shared_request(2, false);
    let mut tables = SharedTables::new();
    let forbidden =
        |_: &mut Call<'_, '_>| panic!("missing owned status must fail before evaluation");
    assert!(
        tables
            .run(&mut frame, &mut request, &forbidden, &mut samples)
            .is_err()
    );
    assert_eq!(samples.parameter(0).unwrap(), 7.);
    assert_eq!(tables.sums, [0.; 64]);
}
#[test]
fn separate_origins_in_one_allocation_keep_constant_one_results() {
    for iso in [1, 2] {
        let mut frame = shared_frame(iso, false);
        let mut request = shared_request(iso, false);
        let mut samples = Samples::new(vec![7.; 128], 0, 64).unwrap();
        let mut tables = SharedTables::new();
        let evaluator = |call: &mut Call<'_, '_>| {
            assert!((call.buffers().parameter(0) - gauss_parameters()[0]).abs() <= f64::EPSILON);
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
        assert_eq!(&tables.sums[..4], &[1., 2., 2., 4.]);
        assert_eq!(samples.sample(0).unwrap(), 1.);
        assert_eq!(samples.parameter(64).unwrap(), 1.);
        assert!((samples.parameter(0).unwrap() - gauss_parameters()[0]).abs() <= f64::EPSILON);
    }
}
#[test]
fn indexed_square_unwind_retains_both_actual_integer_and_real_owner_writes() {
    let mut frame = shared_frame(2, true);
    let mut request = shared_request(2, true);
    let mut samples = Samples::new(vec![7.; 64], 0, 0).unwrap();
    let mut tables = SharedTables::new();
    let evaluator = |call: &mut Call<'_, '_>| {
        call.buffers().set_result(0, 0.25);
        assert_eq!(call.buffers().parameter(0), 0.25);
        call.set_status(1).unwrap();
        assert_eq!(call.argument(Argument::Dimension), 1);
        panic!("indexed square owner unwind fixture");
    };
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tables.run(
            &mut frame,
            &mut request,
            &evaluator,
            &mut samples
        )))
        .is_err()
    );
    assert_eq!(
        (frame.status().unwrap(), frame.integer(Field::Dimension)),
        (1, 1)
    );
    assert_eq!(
        (samples.parameter(0).unwrap(), samples.sample(0).unwrap()),
        (0.25, 0.25)
    );
    assert_eq!(tables.sums, [0.; 64]);
}
