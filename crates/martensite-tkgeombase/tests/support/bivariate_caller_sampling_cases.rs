//! Actual root-count/degree caller identity, separately from generated sample count.
use super::*;
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarCallerSamplingField as Sampling, AdvApp2VarCombinedEvaluation as Call,
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
fn sampling_frame(slots: Option<[usize; 2]>) -> Frame {
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
        Some(slots) => frame.with_caller_sampling_slots(slots).unwrap(),
        None => frame,
    }
}
fn sampling_pair(call: &Call<'_, '_>) -> (i32, i32) {
    (
        call.caller_sampling(Sampling::RootCount).unwrap(),
        call.caller_sampling(Sampling::JacobiDegree).unwrap(),
    )
}
fn run_sampling_case(mode: i32, iso: i32) {
    let mut request = curve_request(iso);
    request.nbcrmx = 3;
    if (2..=4).contains(&mode) {
        request.nbroot = 8;
        request.rootlg = &ROOTS;
    }
    if mode <= 2 || mode >= 5 {
        request.ndgjac = request.nbroot;
    }
    let shared = mode <= 2 || mode >= 5;
    let mut frame = sampling_frame(Some(if shared { [5, 5] } else { [5, 6] }));
    if mode == 6 {
        request.iordre = 1;
    }
    let calls = Mutex::new(0);
    let original_roots = request.nbroot;
    let original_degree = request.ndgjac;
    let replacement = if mode == 7 { 2 } else { 1 };
    let evaluator = |call: &mut Call<'_, '_>, values: &mut [f64], error: &mut i32| {
        let mut count = calls.lock().unwrap();
        *count += 1;
        if mode == 6 && *count == 2 {
            assert_eq!(sampling_pair(call), (1, 1));
            assert_eq!(call.integer(Local::ParameterCount), 1);
            assert_eq!(
                (call.integer(Local::UOrder), call.integer(Local::VOrder)),
                if iso == 1 { (0, 1) } else { (1, 0) }
            );
            *error = 1;
            return;
        }
        let parameters_count = original_roots + if mode == 6 { 2 } else { 0 };
        assert_eq!(sampling_pair(call), (original_roots, original_degree));
        assert_eq!(call.integer(Local::ParameterCount), parameters_count);
        call.set_caller_sampling(Sampling::JacobiDegree, replacement)
            .unwrap();
        assert_eq!(
            sampling_pair(call),
            (
                if shared { replacement } else { original_roots },
                replacement
            )
        );
        assert_eq!(call.integer(Local::ParameterCount), parameters_count);
        if mode == 0 {
            *error = 1;
            return;
        }
        if mode == 2 || mode == 4 {
            call.set_caller_sampling(Sampling::JacobiDegree, original_degree)
                .unwrap();
            assert_eq!(sampling_pair(call), (original_roots, original_degree));
        }
        let parameters = call.parameters().to_vec();
        for (i, value) in values[..(2 * parameters_count) as usize]
            .iter_mut()
            .enumerate()
        {
            *value = if mode >= 5 { parameters[i / 2] } else { 1. };
        }
        *error = 0;
    };
    let mut storage = CurveStorage::new();
    let (mut counts, mut cuts, mut maximum, mut mean) = ([-7; 3], [7.; 4], [7.; 3], [7.; 3]);
    let mut output = storage.output();
    output.ncoeff = &mut counts;
    output.tabdec = &mut cuts;
    output.errmax = &mut maximum;
    output.errmoy = &mut mean;
    let mut error = -1;
    frame
        .approximate_iso(&mut request, &evaluator, &mut output, &mut error)
        .unwrap();
    let expected_error = if mode == 0 || mode == 6 {
        101
    } else if mode == 1 || mode == 2 || mode == 5 || mode == 7 {
        1
    } else {
        0
    };
    assert_eq!(error, expected_error);
    assert_eq!(
        request.nbroot,
        if shared && mode != 2 {
            replacement
        } else {
            original_roots
        }
    );
    assert_eq!(
        request.ndgjac,
        if mode == 2 || mode == 4 {
            original_degree
        } else {
            replacement
        }
    );
    assert_eq!(frame.integer_values()[5], request.nbroot);
    assert_eq!(&output.courbe[18..], &[7.; 14]);
    if error != 0 {
        assert_eq!(output.nbcrbe, 0);
        assert_eq!(output.ncoeff, [-7; 3]);
        assert_eq!(&output.courbe[..18], &[0.; 18]);
    } else {
        assert_eq!((output.nbcrbe, output.ncoeff[0]), (1, 1));
        let mut expected = [0.; 18];
        expected[0] = 1.;
        if mode == 4 {
            expected[3] = 1.;
        }
        for (actual, expected) in output.courbe[..18].iter().zip(expected) {
            assert!(
                (actual - expected).abs() <= 8. * f64::EPSILON,
                "{actual} != {expected}"
            );
        }
    }
    if mode == 6 {
        assert_eq!(*calls.lock().unwrap(), 2);
        assert_eq!(&output.contr1[..4], &[0., 0., 7., 7.]);
        assert_eq!(&output.contr2[..4], &[0.5, 0.5, 7., 7.]);
    }
    let expected_sums: Vec<f64> = match mode {
        0 => vec![7.; 10],
        1 => vec![1., 2., 1., 2., 7., 7., 7., 7., 7., 7.],
        5..=7 => {
            let positive = (1. - 0.7745966692414834) / 2. + 0.5;
            let middle = if mode == 7 { 0. } else { 0.5 };
            vec![middle, positive, middle, positive, 7., 7., 7., 7., 7., 7.]
        }
        _ => vec![0., 2., 2., 2., 2., 0., 2., 2., 2., 2.],
    };
    assert_eq!(&output.somtab[..10], expected_sums);
}
#[test]
fn shared_root_degree_three_to_one_failure_keeps_parameter_count_and_clear_extent() {
    for iso in [1, 2] {
        run_sampling_case(0, iso);
    }
}
#[test]
fn shared_root_degree_constant_successful_callback_reaches_projection_failure() {
    run_sampling_case(1, 2);
}
#[test]
fn restored_shared_root_degree_eight_still_reaches_invalid_degree_failure() {
    run_sampling_case(2, 2);
}
#[test]
fn distinct_degree_can_shrink_while_root_count_stays_eight() {
    run_sampling_case(3, 2);
}
#[test]
fn restored_distinct_degree_preserves_constant_approximation() {
    for iso in [1, 2] {
        run_sampling_case(4, iso);
    }
}
#[test]
fn linear_values_use_initial_half_count_with_live_positive_sample_index_before_failure() {
    for iso in [1, 2] {
        run_sampling_case(5, iso);
    }
}
#[test]
fn sampling_slot_bindings_reject_callback_locals_and_slots_outside_owned_storage() {
    for slots in [[2, 5], [5, 3], [4, 5], [7, 5]] {
        let failure = sampling_frame(None)
            .with_caller_sampling_slots(slots)
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

#[test]
fn live_root_count_selects_endpoint_constraint_sample_before_second_callback_failure() {
    for iso in [1, 2] {
        run_sampling_case(6, iso);
    }
}

#[test]
fn live_even_root_count_clears_middle_terms_without_changing_initial_pair_count() {
    run_sampling_case(7, 2);
}
