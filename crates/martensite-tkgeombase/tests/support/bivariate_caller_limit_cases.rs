//! Piece/degree limits are caller controls, with original output strides.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerLimitField as Limit, AdvApp2VarCombinedEvaluation as Call,
    AdvApp2VarIntegerBindings as Integers, AdvApp2VarIntegerField as Local,
    AdvApp2VarIsoOperationFrame as Frame, AdvApp2VarRealBindings as Reals,
};
const ROOTS: [f64; 8] = [
    -0.9602898564975363,
    -0.7966664774136267,
    -0.525532409916329,
    -0.1834346424956498,
    0.1834346424956498,
    0.525532409916329,
    0.7966664774136267,
    0.9602898564975363,
];
fn limit_frame(slots: Option<[usize; 2]>) -> Frame {
    let integers = Integers {
        dimension: 0,
        favorite_iso: 1,
        parameter_count: 2,
        u_order: 3,
        v_order: 4,
    };
    let reals = Reals {
        domain: [0, 1, 2, 3],
        constant_parameter: 4,
    };
    let frame = Frame::new(vec![0; 7], integers, vec![0.; 5], reals).unwrap();
    match slots {
        Some(slots) => frame.with_caller_limit_slots(slots).unwrap(),
        None => frame,
    }
}
fn limit_pair(call: &Call<'_, '_>) -> (i32, i32) {
    (
        call.caller_limit(Limit::MaximumPieces).unwrap(),
        call.caller_limit(Limit::CoefficientLimit).unwrap(),
    )
}
struct LimitStorage {
    original: CurveStorage,
    counts: [i32; 3],
    cuts: [f64; 4],
    maximum: [f64; 3],
    mean: [f64; 3],
}
impl LimitStorage {
    fn new() -> Self {
        Self {
            original: CurveStorage::new(),
            counts: [-7; 3],
            cuts: [7.; 4],
            maximum: [7.; 3],
            mean: [7.; 3],
        }
    }
    fn output(&mut self) -> IsoApproxOutput<'_> {
        let mut output = self.original.output();
        output.ncoeff = &mut self.counts;
        output.tabdec = &mut self.cuts;
        output.errmax = &mut self.maximum;
        output.errmoy = &mut self.mean;
        output
    }
}
fn limit_request(iso: i32) -> IsoApproxRequest<'static> {
    let mut request = curve_request(iso);
    request.nbcrmx = 3;
    request.nbroot = 8;
    request.rootlg = &ROOTS;
    request
}
#[test]
fn shared_limits_shrink_three_to_one_before_failure_and_preserve_clear_extent() {
    for iso in [1, 2] {
        let mut request = limit_request(iso);
        request.nbroot = 3;
        request.rootlg = &[-0.7745966692414834, 0., 0.7745966692414834];
        let mut frame = limit_frame(Some([5, 5]));
        let evaluator = |call: &mut Call<'_, '_>, _: &mut [f64], error: &mut i32| {
            assert_eq!(limit_pair(call), (3, 3));
            call.set_caller_limit(Limit::MaximumPieces, 1).unwrap();
            assert_eq!(limit_pair(call), (1, 1));
            assert_eq!(call.integer(Local::ParameterCount), 3);
            *error = 1;
        };
        let mut storage = LimitStorage::new();
        let mut output = storage.output();
        let mut error = -1;
        frame
            .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
            .unwrap();
        assert_eq!(
            (request.nbcrmx, request.ncflim, error, output.nbcrbe),
            (1, 1, 101, 0)
        );
        assert_eq!(output.ncoeff, [-7; 3]);
        assert_eq!(&output.courbe[..18], &[0.; 18]);
        assert_eq!(&output.courbe[18..], &[7.; 14]);
        assert_eq!(&output.contr1[..4], &[7.; 4]);
        assert_eq!(frame.integer_values()[5], 1);
    }
}
fn completed_limit_case(slots: Option<[usize; 2]>, restore: bool, iso: i32, derivative: i32) {
    let mut frame = limit_frame(slots);
    let mut request = limit_request(iso);
    request.ideriv = derivative;
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        assert_eq!(limit_pair(call), (3, 3));
        call.set_caller_limit(Limit::MaximumPieces, 1).unwrap();
        let shared = slots == Some([5, 5]);
        assert_eq!(limit_pair(call), (1, if shared { 1 } else { 3 }));
        if restore {
            call.set_caller_limit(Limit::CoefficientLimit, 3).unwrap();
            assert_eq!(limit_pair(call), (3, 3));
        }
        values.fill(1.);
        *error = 0;
    };
    let mut storage = LimitStorage::new();
    let mut output = storage.output();
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    assert_eq!((error, output.nbcrbe, output.ncoeff[0]), (0, 1, 1));
    assert_eq!(output.ncoeff[1..], [-7; 2]);
    assert!(output.errmax[0].abs() <= 8. * f64::EPSILON);
    assert!(output.errmoy[0].abs() <= 8. * f64::EPSILON);
    assert_eq!(&output.errmax[1..], &[7.; 2]);
    assert_eq!(&output.errmoy[1..], &[7.; 2]);
    let packed = slots == Some([5, 5]) && !restore;
    assert_eq!(
        (request.nbcrmx, request.ncflim),
        if restore {
            (3, 3)
        } else {
            (1, if packed { 1 } else { 3 })
        }
    );
    let mut expected = [0.; 18];
    expected[0] = 1.;
    expected[if packed { 1 } else { 3 }] = if packed && derivative == 1 { 0.5 } else { 1. };
    for (actual, expected) in output.courbe[..18].iter().zip(expected) {
        assert!(
            (actual - expected).abs() <= 8. * f64::EPSILON,
            "{actual} != {expected}"
        );
    }
    assert_eq!(&output.courbe[18..], &[7.; 14]);
    assert_eq!(&output.tabdec[..2], &[0., 1.]);
    assert_eq!(&output.tabdec[2..], &[7.; 2]);
}
#[test]
fn persistent_shared_limit_shrink_packs_constant_coefficients_using_live_matrix_stride() {
    completed_limit_case(Some([5, 5]), false, 2, 0);
}
#[test]
fn restored_shared_limit_uses_component_spacing() {
    completed_limit_case(Some([5, 5]), true, 2, 0);
}
#[test]
fn equal_distinct_limits_keep_separate_slot_identity() {
    completed_limit_case(Some([5, 6]), false, 2, 0);
}
#[test]
fn unbound_limits_still_update_the_actual_distinct_caller_controls() {
    completed_limit_case(None, false, 2, 0);
}
#[test]
fn post_store_rescaling_uses_initial_component_stride_after_live_limit_shrink() {
    for iso in [1, 2] {
        completed_limit_case(Some([5, 5]), false, iso, 1);
    }
}
#[test]
fn caller_limit_bindings_reject_local_slots_and_out_of_range_slots() {
    for slots in [[2, 5], [5, 3], [4, 5], [7, 5]] {
        let failure = limit_frame(None)
            .with_caller_limit_slots(slots)
            .err()
            .unwrap();
        assert_eq!(
            failure.exception_type(),
            if slots.contains(&7) {
                "Standard_OutOfRange"
            } else {
                "Standard_DomainError"
            }
        );
        assert!(failure.message().contains("expected"));
    }
}
