//! Owned status preserves initial clearing and original caller integer identity.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarIntegerField as Argument, AdvApp2VarSquareEvaluation as Call,
    AdvApp2VarSquareIntegerBindings as Bindings, AdvApp2VarSquareIntegerField as Field,
    AdvApp2VarSquareOperationFrame as Frame, AdvApp2VarSquareStatusEvaluator,
};
#[path = "bivariate_square_status_callback_cases.rs"]
mod callback_cases;
fn bindings() -> Bindings {
    Bindings {
        dimension: 0,
        root_counts: [1, 2],
        favorite_iso: 3,
    }
}
fn status_request(iso: i32) -> SampledSquare<'static> {
    let mut request = SquareStorage::new().request(iso);
    request.ndimen = 3;
    request
}
fn status_frame(iso: i32, slot: usize) -> Frame {
    Frame::new(vec![3, 3, 3, iso], bindings())
        .unwrap()
        .with_owned_status_slot(slot)
        .unwrap()
}
struct StatusStorage<const N: usize> {
    sums: [f64; N],
    u_difference: [f64; N],
    v_difference: [f64; N],
    both_difference: [f64; N],
    samples: [f64; 64],
    parameters: [f64; 64],
}
impl<const N: usize> StatusStorage<N> {
    fn new() -> Self {
        Self {
            sums: [0.; N],
            u_difference: [0.; N],
            v_difference: [0.; N],
            both_difference: [0.; N],
            samples: [7.; 64],
            parameters: [7.; 64],
        }
    }
    fn run(
        &mut self,
        frame: &mut Frame,
        request: &mut SampledSquare<'_>,
        evaluator: &impl AdvApp2VarSquareStatusEvaluator,
    ) -> Result<i32, martensite_tkernel::standard::Failure> {
        let mut tables = SquareTables {
            sosotb: &mut self.sums,
            disotb: &mut self.u_difference,
            soditb: &mut self.v_difference,
            diditb: &mut self.both_difference,
        };
        frame.sample_square_owned_status(
            request,
            evaluator,
            &mut tables,
            &mut self.samples,
            &mut self.parameters,
        )
    }
}
fn status_clear_case(iso: i32, slot: usize) {
    let mut frame = status_frame(iso, slot);
    let mut request = status_request(iso);
    let mut storage = StatusStorage::<64>::new();
    let local_iso = if slot == 3 { 2 } else { iso };
    let varying = if local_iso == 1 { 2 } else { 1 };
    let nonvarying = 3 - varying;
    let dimension = if slot == 0 { 0 } else { 3 };
    let count = if slot == varying { 0 } else { 3 };
    let trace = Mutex::new(Vec::new());
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64]| {
        assert_eq!(call.status().unwrap(), 0);
        assert_eq!(call.argument(Argument::Dimension), dimension);
        assert_eq!(call.argument(Argument::ParameterCount), count);
        assert_eq!(call.argument(Argument::FavoriteIso), local_iso);
        assert_eq!(
            call.integer(Field::FavoriteIso),
            if slot == 3 { 0 } else { iso }
        );
        assert_eq!(call.argument(Argument::UOrder), 0);
        assert_eq!(call.argument(Argument::VOrder), 0);
        trace.lock().unwrap().push(*call.constant_parameter());
        values[..(dimension * count) as usize].fill(1.);
        call.set_status(0).unwrap();
    };
    assert_eq!(
        storage.run(&mut frame, &mut request, &evaluator).unwrap(),
        0
    );
    assert_eq!(frame.status().unwrap(), 0);
    assert_eq!(request.ndimen, dimension);
    assert_eq!(
        [request.nbpntu, request.nbpntv],
        [if slot == 1 { 0 } else { 3 }, if slot == 2 { 0 } else { 3 }]
    );
    assert_eq!(request.isofav, if slot == 3 { 0 } else { iso });
    let trace = trace.lock().unwrap();
    assert_eq!(trace.len(), if slot == nonvarying { 0 } else { 3 });
    if !trace.is_empty() {
        let root = (3_f64 / 5.).sqrt();
        for (actual, expected) in trace.iter().zip([(1. - root) / 2., 0.5, (1. + root) / 2.]) {
            assert!((*actual - expected).abs() <= f64::EPSILON);
        }
    }
    let mut expected = [0.; 64];
    if slot == 3 {
        for component in 0..3 {
            expected[4 * component..4 * component + 4].copy_from_slice(&[1., 2., 2., 4.]);
        }
    }
    assert_eq!(storage.sums, expected);
    assert_eq!(storage.u_difference, [0.; 64]);
    assert_eq!(storage.v_difference, [0.; 64]);
    assert_eq!(storage.both_difference, [0.; 64]);
    assert_eq!(storage.samples[9..], [7.; 55]);
    if slot != 3 {
        assert_eq!(storage.samples, [7.; 64]);
    }
    assert_eq!(storage.parameters[3..], [7.; 61]);
    if slot == varying {
        assert_eq!(storage.parameters, [7.; 64]);
    }
    assert_eq!((request.uintfn, request.vintfn), ([0., 1.], [0., 1.]));
}
#[test]
fn first_iso_status_and_dimension_share_initial_clear() {
    status_clear_case(1, 0);
}
#[test]
fn second_iso_status_and_dimension_share_initial_clear() {
    status_clear_case(2, 0);
}
#[test]
fn first_iso_status_and_u_count_share_initial_clear() {
    status_clear_case(1, 1);
}
#[test]
fn second_iso_status_and_u_count_share_initial_clear() {
    status_clear_case(2, 1);
}
#[test]
fn first_iso_status_and_v_count_share_initial_clear() {
    status_clear_case(1, 2);
}
#[test]
fn second_iso_status_and_v_count_share_initial_clear() {
    status_clear_case(2, 2);
}
#[test]
fn first_iso_status_and_caller_preference_share_clear_before_normalization() {
    status_clear_case(1, 3);
}
#[test]
fn second_iso_status_and_caller_preference_share_clear_before_normalization() {
    status_clear_case(2, 3);
}
#[test]
fn owned_square_status_rejects_missing_bounds_conflicting_seed_and_separate_error_entrypoint() {
    let missing = Frame::new(vec![3, 3, 3, 2], bindings()).unwrap();
    assert_eq!(
        missing.status().unwrap_err().exception_type(),
        "Standard_DomainError"
    );
    assert_eq!(
        missing
            .with_owned_status_slot(4)
            .err()
            .unwrap()
            .exception_type(),
        "Standard_OutOfRange"
    );
    let mut missing = Frame::new(vec![3, 3, 3, 2], bindings()).unwrap();
    let mut storage = StatusStorage::<64>::new();
    let mut request = status_request(2);
    let forbidden = |_: &mut Call<'_, '_>, _: &mut [f64]| {
        panic!("invalid owned status must fail before evaluation")
    };
    assert!(storage.run(&mut missing, &mut request, &forbidden).is_err());
    let mut mismatch = Frame::new(vec![0, 3, 3, 2], bindings())
        .unwrap()
        .with_owned_status_slot(0)
        .unwrap();
    let failure = storage
        .run(&mut mismatch, &mut request, &forbidden)
        .unwrap_err();
    assert!(
        failure
            .message()
            .contains("ErrorCode value=0; expected shared initial value=3")
    );
    let mut configured = status_frame(2, 0);
    let legacy = |_: &mut Call<'_, '_>, _: &mut [f64], _: &mut i32| {
        panic!("wrong entrypoint must stop before evaluation")
    };
    let mut tables = SquareTables {
        sosotb: &mut storage.sums,
        disotb: &mut storage.u_difference,
        soditb: &mut storage.v_difference,
        diditb: &mut storage.both_difference,
    };
    let mut code = -1;
    let failure = configured
        .sample_square(
            &mut request,
            &legacy,
            &mut tables,
            &mut storage.samples,
            &mut storage.parameters,
            &mut code,
        )
        .unwrap_err();
    assert!(
        failure
            .message()
            .contains("expected owned-status entrypoint")
    );
    assert_eq!(code, -1);
    assert_eq!(storage.sums, [0.; 64]);
}
#[test]
fn distinct_owned_square_status_preserves_constant_one_in_both_iso_directions() {
    for iso in [1, 2] {
        let mut frame = Frame::new(vec![3, 3, 3, iso, -1], bindings())
            .unwrap()
            .with_owned_status_slot(4)
            .unwrap();
        let mut request = status_request(iso);
        let mut storage = StatusStorage::<64>::new();
        let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64]| {
            values[..9].fill(1.);
            call.set_status(0).unwrap();
        };
        assert_eq!(
            storage.run(&mut frame, &mut request, &evaluator).unwrap(),
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
            (3, 3, 3, iso)
        );
        assert_eq!(
            &storage.sums[..12],
            &[1., 2., 2., 4., 1., 2., 2., 4., 1., 2., 2., 4.]
        );
        assert_eq!(storage.sums[12..], [0.; 52]);
    }
}
