//! Owned scalar write-back, derivative-slot identity and fixed-storage regressions.
use martensite_tkgeombase::adv_app2var::approx_f2var::{
    IsoApproxOutput, IsoApproxRequest, SampledSquare, SquareTables, mma2ds1_, mma2ds1_mut_,
    mma2fnc_, mma2fnc_mut_, mma2roo_,
};
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarDerivativeOrders, AdvApp2VarEvaluation, AdvApp2VarEvaluationMut,
    AdvApp2VarEvaluationSlots, AdvApp2VarEvaluatorFunc2Var,
};
use std::sync::Mutex;

struct ShrinkingError;
impl AdvApp2VarEvaluatorFunc2Var for ShrinkingError {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("mutable callback must use its slot interface")
    }
    fn evaluate_slots(
        &self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        _: &mut [f64],
        error: &mut i32,
    ) {
        assert_slot_identity(&mut call.orders);
        *call.dimension = 1;
        *call.nb_params = 2;
        *call.favor_iso = 3 - *call.favor_iso;
        *call.const_param = 0.75;
        call.u_start_end[0] = -0.5;
        call.v_start_end[1] = 0.75;
        *error = 1;
    }
}

fn assert_slot_identity(orders: &mut AdvApp2VarDerivativeOrders<'_>) {
    let shared = matches!(orders, AdvApp2VarDerivativeOrders::Shared(_));
    let previous_v = orders.v();
    orders.set_u(7);
    assert_eq!(orders.v(), if shared { 7 } else { previous_v });
    orders.set_v(9);
    assert_eq!(orders.u(), if shared { 9 } else { 7 });
}

struct SquareStorage {
    roots: [f64; 3],
    soso: [f64; 8],
    diso: [f64; 2],
    sodi: [f64; 2],
    didi: [f64; 8],
    samples: [f64; 6],
    parameters: [f64; 3],
}
impl SquareStorage {
    fn new() -> Self {
        let mut roots = [0.; 3];
        mma2roo_(3, 3, &mut roots, &mut [0.; 3]);
        Self {
            roots,
            soso: [0.; 8],
            diso: [0.; 2],
            sodi: [0.; 2],
            didi: [0.; 8],
            samples: [0.; 6],
            parameters: [0.; 3],
        }
    }
    fn request(&self, iso: i32) -> SampledSquare<'static> {
        // Independent, exact three-point Gauss roots; request arrays do not alias scratch.
        const ROOTS: [f64; 3] = [-0.7745966692414834, 0., 0.7745966692414834];
        assert_eq!(self.roots, ROOTS);
        SampledSquare {
            ndimen: 2,
            uintfn: [0., 1.],
            vintfn: [0., 1.],
            nbpntu: 3,
            nbpntv: 3,
            urootb: &ROOTS,
            vrootb: &ROOTS,
            isofav: iso,
        }
    }
    fn sample(
        &mut self,
        request: &mut SampledSquare<'_>,
        evaluator: &dyn AdvApp2VarEvaluatorFunc2Var,
    ) -> i32 {
        let mut error = -1;
        let mut tables = SquareTables {
            sosotb: &mut self.soso,
            disotb: &mut self.diso,
            soditb: &mut self.sodi,
            diditb: &mut self.didi,
        };
        mma2ds1_mut_(
            request,
            evaluator,
            &mut tables,
            &mut self.samples,
            &mut self.parameters,
            &mut error,
        );
        error
    }
}

#[test]
fn square_error_retains_caller_dimension_and_axis_count_writes() {
    for iso in [1, 2] {
        let mut storage = SquareStorage::new();
        let mut request = storage.request(iso);
        assert_eq!(
            storage.sample(&mut request, &ShrinkingError),
            if iso == 1 { 0 } else { 201 }
        );
        assert_eq!(request.ndimen, 1);
        assert_eq!(
            (request.nbpntu, request.nbpntv),
            if iso == 1 { (3, 2) } else { (2, 3) }
        );
        assert_eq!(request.isofav, iso);
        assert_eq!((request.uintfn, request.vintfn), ([0., 1.], [0., 1.]));
    }
}

#[test]
fn transposed_square_uses_the_changed_dimension_on_error_cleanup() {
    let mut storage = SquareStorage::new();
    storage.soso = [1., 2., 3., 4., 5., 6., 7., 8.];
    storage.didi = storage.soso;
    let mut request = storage.request(1);
    assert_eq!(storage.sample(&mut request, &ShrinkingError), 0);
    assert_eq!(storage.soso, [1., 2., 3., 4., 5., 7., 6., 8.]);
    assert_eq!(storage.didi, storage.soso);
}

#[derive(Default)]
struct AliasedSquareEvaluator {
    seen: Mutex<Vec<(i32, i32, i32)>>,
    shrink: bool,
}
impl AdvApp2VarEvaluatorFunc2Var for AliasedSquareEvaluator {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("shared-slot provider must observe actual slot identity")
    }
    fn evaluate_slots(
        &self,
        call: &mut AdvApp2VarEvaluationSlots<'_>,
        result: &mut [f64],
        error: &mut i32,
    ) {
        self.seen
            .lock()
            .unwrap()
            .push((call.orders.u(), *call.dimension, *call.nb_params));
        call.orders.set_u(7);
        assert_eq!(call.orders.v(), 7);
        call.orders.set_v(9);
        assert_eq!(call.orders.u(), 9);
        result.fill(1.);
        if self.shrink {
            *call.dimension = 1;
            *call.nb_params = 2;
        }
        *error = 0;
    }
}

#[test]
fn square_derivative_arguments_share_one_live_slot_during_and_between_calls() {
    for iso in [1, 2] {
        let evaluator = AliasedSquareEvaluator::default();
        let mut storage = SquareStorage::new();
        let mut request = storage.request(iso);
        request.ndimen = 1;
        assert_eq!(storage.sample(&mut request, &evaluator), 0);
        assert_eq!(
            *evaluator.seen.lock().unwrap(),
            [(0, 1, 3), (9, 1, 3), (9, 1, 3)]
        );
        assert_eq!(&storage.soso[..4], &[1., 2., 2., 4.]);
        assert_eq!(storage.diso, [0.; 2]);
        assert_eq!(storage.sodi, [0.; 2]);
        assert_eq!(storage.didi, [0.; 8]);
    }
}

#[test]
fn shrinking_success_uses_the_live_parameter_count_for_middle_sample_folding() {
    for iso in [1, 2] {
        let evaluator = AliasedSquareEvaluator {
            shrink: true,
            ..Default::default()
        };
        let mut storage = SquareStorage::new();
        let mut request = storage.request(iso);
        assert_eq!(storage.sample(&mut request, &evaluator), 0);
        assert_eq!(
            *evaluator.seen.lock().unwrap(),
            [(0, 2, 3), (9, 1, 2), (9, 1, 2)]
        );
        let expected = if iso == 1 {
            [0., 0., 2., 4.]
        } else {
            [0., 2., 0., 4.]
        };
        assert_eq!(&storage.soso[..4], &expected);
        assert_eq!(&storage.soso[4..], &[0.; 4]);
        assert_eq!(request.ndimen, 1);
    }
}

struct CurveStorage {
    counts: [i32; 1],
    curve: [f64; 32],
    sums: [f64; 32],
    differences: [f64; 32],
    first: [f64; 32],
    second: [f64; 32],
    cuts: [f64; 2],
    maximum: [f64; 1],
    mean: [f64; 1],
}
impl CurveStorage {
    fn new() -> Self {
        Self {
            counts: [-7],
            curve: [7.; 32],
            sums: [7.; 32],
            differences: [7.; 32],
            first: [7.; 32],
            second: [7.; 32],
            cuts: [7.; 2],
            maximum: [7.],
            mean: [7.],
        }
    }
    fn output(&mut self) -> IsoApproxOutput<'_> {
        IsoApproxOutput {
            ncoeff: &mut self.counts,
            courbe: &mut self.curve,
            nbcrbe: -7,
            somtab: &mut self.sums,
            diftab: &mut self.differences,
            contr1: &mut self.first,
            contr2: &mut self.second,
            tabdec: &mut self.cuts,
            errmax: &mut self.maximum,
            errmoy: &mut self.mean,
        }
    }
}
fn curve_request(iso: i32) -> IsoApproxRequest<'static> {
    IsoApproxRequest {
        ndimen: 2,
        nbsesp: 1,
        ndimse: &[2],
        uvfonc: [0., 1., 0., 1.],
        tconst: 0.5,
        isofav: iso,
        nbroot: 3,
        rootlg: &[-0.7745966692414834, 0., 0.7745966692414834],
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 3,
        epsapr: &[1e-6],
    }
}

#[test]
fn iso_error_retains_caller_scalars_but_not_local_domain_mutations() {
    for iso in [1, 2] {
        let mut request = curve_request(iso);
        let mut storage = CurveStorage::new();
        let mut output = storage.output();
        let mut error = -1;
        mma2fnc_mut_(&mut request, &ShrinkingError, &mut output, &mut error);
        assert_eq!((error, output.nbcrbe), (101, 0));
        assert_eq!(
            (request.ndimen, request.isofav, request.tconst),
            (1, 3 - iso, 0.75)
        );
        assert_eq!(request.uvfonc, [0., 1., 0., 1.]);
        assert_eq!(&output.courbe[..6], &[0.; 6]);
        assert_eq!(&output.courbe[6..], &[7.; 26]);
        assert_eq!(output.tabdec, &[0., 1.]);
    }
}

#[test]
fn immutable_iso_convenience_keeps_its_request_snapshot() {
    let request = curve_request(1);
    let mut storage = CurveStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    mma2fnc_(&request, &ShrinkingError, &mut output, &mut error);
    assert_eq!(
        (request.ndimen, request.isofav, request.tconst, error),
        (2, 1, 0.5, 101)
    );
}

#[test]
fn immutable_square_convenience_keeps_its_request_snapshot() {
    let mut storage = SquareStorage::new();
    let request = storage.request(2);
    let mut error = -1;
    let mut tables = SquareTables {
        sosotb: &mut storage.soso,
        disotb: &mut storage.diso,
        soditb: &mut storage.sodi,
        diditb: &mut storage.didi,
    };
    mma2ds1_(
        &request,
        &ShrinkingError,
        &mut tables,
        &mut storage.samples,
        &mut storage.parameters,
        &mut error,
    );
    assert_eq!((request.ndimen, request.nbpntu, error), (2, 3, 201));
}

#[test]
fn independent_derivative_slots_do_not_gain_aliases() {
    let (mut u, mut v) = (1, 2);
    let mut orders = AdvApp2VarDerivativeOrders::Independent {
        u: &mut u,
        v: &mut v,
    };
    orders.set_u(3);
    assert_eq!((orders.u(), orders.v()), (3, 2));
    orders.set_v(4);
    assert_eq!((orders.u(), orders.v()), (3, 4));
}

#[derive(Default)]
struct LegacyDerivativeEvaluator(Mutex<Vec<i32>>);
impl AdvApp2VarEvaluatorFunc2Var for LegacyDerivativeEvaluator {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("legacy mutable evaluator uses its established adapter")
    }
    fn evaluate_native(
        &self,
        call: &mut AdvApp2VarEvaluationMut<'_>,
        result: &mut [f64],
        error: &mut i32,
    ) {
        self.0.lock().unwrap().push(*call.u_order);
        *call.u_order = 4;
        *call.v_order = 5;
        result.fill(1.);
        *error = 0;
    }
}

#[test]
fn retained_independent_reference_adapter_keeps_its_shared_order_conflict_rule() {
    let evaluator = LegacyDerivativeEvaluator::default();
    let mut storage = SquareStorage::new();
    let mut request = storage.request(2);
    request.ndimen = 1;
    assert_eq!(storage.sample(&mut request, &evaluator), 0);
    // After order4 is retained, U4 is unchanged and the old adapter selects V5.
    assert_eq!(*evaluator.0.lock().unwrap(), [0, 4, 5]);
    assert_eq!(&storage.soso[..4], &[1., 2., 2., 4.]);
}

#[test]
fn borrowed_callback_views_are_send_and_sync_without_global_state() {
    fn assert_send_sync<T: Send + Sync>() {
        let _ = std::marker::PhantomData::<T>;
    }
    assert_send_sync::<AdvApp2VarDerivativeOrders<'static>>();
    assert_send_sync::<AdvApp2VarEvaluationSlots<'static>>();
}

#[path = "support/bivariate_scalar_provider_cases.rs"]
mod scalar_provider_cases;

#[path = "support/bivariate_real_frame_cases.rs"]
mod real_frame_cases;

#[path = "support/bivariate_combined_frame_cases.rs"]
mod combined_frame_cases;

#[path = "support/bivariate_subspace_count_cases.rs"]
mod subspace_count_cases;

#[path = "support/bivariate_caller_order_cases.rs"]
mod caller_order_cases;

#[path = "support/bivariate_caller_limit_cases.rs"]
mod caller_limit_cases;

#[path = "support/bivariate_caller_sampling_cases.rs"]
mod caller_sampling_cases;

#[path = "support/bivariate_endpoint_reread_cases.rs"]
mod endpoint_reread_cases;

#[path = "support/bivariate_piece_root_stride_cases.rs"]
mod piece_root_stride_cases;

#[path = "support/bivariate_continuity_layout_cases.rs"]
mod continuity_layout_cases;

#[path = "support/bivariate_hermite_sample_cases.rs"]
mod hermite_sample_cases;

#[path = "support/bivariate_hermite_jacobi_cases.rs"]
mod hermite_jacobi_cases;

#[path = "support/bivariate_coupled_scalar_cases.rs"]
mod coupled_scalar_cases;

#[path = "support/bivariate_parameter_result_cases.rs"]
mod parameter_result_cases;

#[path = "support/bivariate_extended_endpoint_cases.rs"]
mod extended_endpoint_cases;

#[path = "support/bivariate_four_component_cases.rs"]
mod four_component_cases;

#[path = "support/bivariate_packed_frame_cases.rs"]
mod packed_frame_cases;

#[path = "support/bivariate_caller_dimension_cases.rs"]
mod caller_dimension_cases;

#[path = "support/bivariate_caller_tolerance_cases.rs"]
mod caller_tolerance_cases;

#[path = "support/bivariate_caller_root_cases.rs"]
mod caller_root_cases;

#[path = "support/bivariate_dimension_slot_cases.rs"]
mod dimension_slot_cases;

#[path = "support/bivariate_dimension_buffer_cases.rs"]
mod bivariate_dimension_buffer_cases;

#[path = "support/bivariate_owned_array_cases.rs"]
mod owned_array_cases;

#[path = "support/bivariate_root_curve_cases.rs"]
mod root_curve_cases;

#[path = "support/bivariate_scalar_curve_cases.rs"]
mod scalar_curve_cases;

#[path = "support/bivariate_square_caller_cases.rs"]
mod square_caller_cases;

#[path = "support/bivariate_square_status_cases.rs"]
mod square_status_cases;

#[path = "support/bivariate_square_shared_sample_cases.rs"]
mod square_shared_sample_cases;

#[path = "support/bivariate_square_root_parameter_cases.rs"]
mod square_root_parameter_cases;

#[path = "support/bivariate_square_table_incidence_cases.rs"]
mod square_table_incidence_cases;

#[path = "support/bivariate_square_interval_incidence_cases.rs"]
mod square_interval_incidence_cases;

#[path = "support/bivariate_square_signed_root_cases.rs"]
mod square_signed_root_cases;

#[path = "support/bivariate_iso_integer_output_cases.rs"]
mod iso_integer_output_cases;

#[path = "support/bivariate_coefficient_count_cases.rs"]
mod coefficient_count_cases;

#[path = "support/bivariate_cut_domain_cases.rs"]
mod cut_domain_cases;

#[path = "support/bivariate_folded_output_cases.rs"]
mod folded_output_cases;

#[path = "support/bivariate_error_output_cases.rs"]
mod error_output_cases;

#[path = "support/bivariate_endpoint_output_cases.rs"]
mod endpoint_output_cases;

#[path = "support/bivariate_signed_piece_origin_cases.rs"]
mod signed_piece_origin_cases;
