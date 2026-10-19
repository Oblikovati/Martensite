//! Independent caller-slot stage model for initialized coefficient-count incidence.
use super::iso_integer_output_cases::{Bindings, Call, Frame, Int, Output, ROOTS8, Reals};
use super::*;
fn initial_count_slots(iso: i32, role: &str) -> (Vec<i32>, usize) {
    let mut slots = vec![-7; 128];
    slots[32..43].copy_from_slice(&[1, 1, iso, 8, -1, 0, 2, 1, 3, -7, -7]);
    slots[48..50].copy_from_slice(&[1, 1]);
    let origin = match role {
        "dimension" => {
            slots[32] = 2;
            slots[48] = 2;
            32
        }
        "spaces" => {
            slots[32] = 2;
            slots[33] = 2;
            33
        }
        "iso" => 34,
        "roots" => 35,
        "order" => 36,
        "derivative" => 37,
        "degree" => 38,
        "maximum" => 39,
        "limit" => 40,
        "made" => 41,
        "status" => 42,
        "partition_current" => 48,
        "partition_pending" => {
            slots[32] = 3;
            slots[33] = 2;
            slots[49] = 2;
            49
        }
        "status_warning" => {
            slots[40] = 1;
            42
        }
        _ => 64,
    };
    (slots, origin)
}
fn expected_count_slots(iso: i32, role: &str) -> (Vec<i32>, usize) {
    let (mut slots, origin) = initial_count_slots(iso, role);
    slots[41] = 0;
    slots[42] = 0;
    let mut calls = 0;
    while slots[41] != 1 {
        let offset = slots[41] as usize;
        slots[42] = 0;
        calls += 1;
        if calls > 1 {
            slots[42] = 101;
            break;
        }
        slots[origin + offset] = 0;
        let subspaces = slots[33];
        let required = if role == "status_warning" { 2 } else { 1 };
        let mut warning = false;
        for _ in 0..subspaces {
            if slots[40] < required {
                warning = true;
                slots[origin + offset] = slots[40];
            } else {
                slots[origin + offset] = required.max(slots[origin + offset]);
            }
        }
        if warning {
            slots[42] = -1;
        }
        slots[41] += 1;
    }
    (slots, calls)
}
fn count_frame(slots: Vec<i32>, origin: usize, warning: bool) -> Frame {
    let low = if warning { -1. } else { 0. };
    Frame::new(
        slots,
        Bindings {
            dimension: 32,
            favorite_iso: 34,
            parameter_count: 0,
            u_order: 1,
            v_order: 2,
        },
        vec![low, 1., low, 1., 0.5],
        Reals {
            domain: [0, 1, 2, 3],
            constant_parameter: 4,
        },
    )
    .unwrap()
    .with_subspace_count_slot(33)
    .unwrap()
    .with_caller_order_slots([36, 37])
    .unwrap()
    .with_caller_sampling_slots([35, 38])
    .unwrap()
    .with_caller_limit_slots([39, 40])
    .unwrap()
    .with_caller_dimension_slots(vec![48, 49])
    .unwrap()
    .with_caller_output_slots([42, 41])
    .unwrap()
    .with_coefficient_count_origin(origin)
    .unwrap()
}
fn run_count_incidence(iso: i32, role: &str) {
    let (initial, origin) = initial_count_slots(iso, role);
    let partition = [initial[48], initial[49]];
    let warning = role == "status_warning";
    let low = if warning { -1. } else { 0. };
    let mut request = IsoApproxRequest {
        ndimen: initial[32],
        nbsesp: initial[33],
        ndimse: &partition,
        uvfonc: [low, 1., low, 1.],
        tconst: 0.5,
        isofav: iso,
        nbroot: 8,
        rootlg: &ROOTS8,
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: initial[40],
        epsapr: &[1e-6; 4],
    };
    let mut counts = initial[origin..origin + 6].to_vec();
    let mut frame = count_frame(initial.clone(), origin, warning);
    let mut reals = [7.; 1024];
    let (_, tail) = reals.split_at_mut(64);
    let (curve, tail) = tail.split_at_mut(128);
    let (sums, tail) = tail.split_at_mut(64);
    let (diff, tail) = tail.split_at_mut(64);
    let (left, tail) = tail.split_at_mut(64);
    let (right, tail) = tail.split_at_mut(64);
    let (cuts, tail) = tail.split_at_mut(64);
    let (maximum, mean) = tail.split_at_mut(64);
    let mut output = IsoApproxOutput {
        ncoeff: &mut counts,
        courbe: curve,
        nbcrbe: -7,
        somtab: sums,
        diftab: diff,
        contr1: left,
        contr2: right,
        tabdec: cuts,
        errmax: maximum,
        errmoy: mean,
    };
    let calls = std::sync::Mutex::new(0usize);
    let evaluator = |call: &mut Call<'_, '_, '_>| {
        let mut total = calls.lock().unwrap();
        *total += 1;
        assert_eq!(call.integer(Int::Dimension), initial[32]);
        assert_eq!(call.integer(Int::ParameterCount), 8);
        assert_eq!(call.caller_output(Output::Status).unwrap(), 0);
        if *total > 1 {
            call.set_caller_output(Output::Status, 1).unwrap();
            return;
        }
        assert_eq!(
            call.coefficient_count(0).unwrap(),
            if role == "made" || role == "status" || warning {
                0
            } else {
                initial[origin]
            }
        );
        for (index, root) in ROOTS8.iter().enumerate() {
            let parameter = if warning { *root } else { (1. + root) / 2. };
            assert_eq!(call.buffers().parameter(index), parameter);
            for component in 0..initial[32] as usize {
                call.buffers().set_result(
                    index * initial[32] as usize + component,
                    if warning { parameter } else { 1. },
                );
            }
        }
        call.set_caller_output(Output::Status, 0).unwrap();
    };
    let (expected, expected_calls) = expected_count_slots(iso, role);
    let result = frame
        .approximate_iso_owned_outputs(&mut request, &evaluator, &mut output)
        .unwrap();
    // The retained function return is always zero; caller status is an owned output.
    assert_eq!(result, 0, "{iso}/{role}");
    assert_eq!(*calls.lock().unwrap(), expected_calls, "{iso}/{role}");
    assert_eq!(
        &frame.integer_values()[32..],
        &expected[32..],
        "{iso}/{role}"
    );
    assert_eq!(output.nbcrbe, expected[41]);
    assert_eq!(&*output.ncoeff, &expected[origin..origin + 6]);
    assert_eq!(
        &output.tabdec[..12],
        &[low, 1., 7., 7., 7., 7., 7., 7., 7., 7., 7., 7.]
    );
    check_count_curve(&output, role, initial[32], expected[32]);
    if role == "spaces" {
        assert_eq!(&output.errmax[..12], &[7.; 12]);
        assert_eq!(&output.errmoy[..12], &[7.; 12]);
    }
    if role == "degree" || role == "partition_current" {
        assert_eq!(output.errmax[0], 0.);
        assert_eq!(output.errmoy[0], 0.);
    }
}
pub(super) fn check_count_curve(
    output: &IsoApproxOutput<'_>,
    role: &str,
    initial_dimension: i32,
    final_dimension: i32,
) {
    let mut expected = [7.; 12];
    let limit = if role == "status_warning" { 1 } else { 3 };
    expected[..(limit * initial_dimension) as usize].fill(0.);
    if !matches!(role, "spaces" | "limit" | "status_warning") {
        let weights = [
            0.2564561626532326,
            0.221824096603541,
            0.15724713746926035,
            0.07157938446051357,
        ];
        let mut coefficient = 0.;
        for weight in weights {
            coefficient += 2. * weight;
        }
        coefficient *= if role == "order" {
            1.
        } else {
            std::f64::consts::FRAC_1_SQRT_2
        };
        if role == "derivative" {
            coefficient *= 2.;
        }
        for component in 0..final_dimension as usize {
            expected[component * 3] = coefficient;
        }
    }
    assert_eq!(
        output.courbe[..12]
            .iter()
            .map(|x| x.to_bits())
            .collect::<Vec<_>>(),
        expected.map(f64::to_bits),
        "{role}"
    );
}

#[test]
fn initialized_count_none_iso1() {
    run_count_incidence(1, "none");
}

#[test]
fn initialized_count_none_iso2() {
    run_count_incidence(2, "none");
}

#[test]
fn initialized_count_dimension_iso1() {
    run_count_incidence(1, "dimension");
}

#[test]
fn initialized_count_dimension_iso2() {
    run_count_incidence(2, "dimension");
}

#[test]
fn initialized_count_spaces_iso1() {
    run_count_incidence(1, "spaces");
}

#[test]
fn initialized_count_spaces_iso2() {
    run_count_incidence(2, "spaces");
}

#[test]
fn initialized_count_iso_iso1() {
    run_count_incidence(1, "iso");
}

#[test]
fn initialized_count_iso_iso2() {
    run_count_incidence(2, "iso");
}

#[test]
fn initialized_count_roots_iso1() {
    run_count_incidence(1, "roots");
}

#[test]
fn initialized_count_roots_iso2() {
    run_count_incidence(2, "roots");
}

#[test]
fn initialized_count_order_iso1() {
    run_count_incidence(1, "order");
}

#[test]
fn initialized_count_order_iso2() {
    run_count_incidence(2, "order");
}

#[test]
fn initialized_count_derivative_iso1() {
    run_count_incidence(1, "derivative");
}

#[test]
fn initialized_count_derivative_iso2() {
    run_count_incidence(2, "derivative");
}

#[test]
fn initialized_count_degree_iso1() {
    run_count_incidence(1, "degree");
}

#[test]
fn initialized_count_degree_iso2() {
    run_count_incidence(2, "degree");
}

#[test]
fn initialized_count_maximum_iso1() {
    run_count_incidence(1, "maximum");
}

#[test]
fn initialized_count_maximum_iso2() {
    run_count_incidence(2, "maximum");
}

#[test]
fn initialized_count_limit_iso1() {
    run_count_incidence(1, "limit");
}

#[test]
fn initialized_count_limit_iso2() {
    run_count_incidence(2, "limit");
}

#[test]
fn initialized_count_made_iso1() {
    run_count_incidence(1, "made");
}

#[test]
fn initialized_count_made_iso2() {
    run_count_incidence(2, "made");
}

#[test]
fn initialized_count_status_iso1() {
    run_count_incidence(1, "status");
}

#[test]
fn initialized_count_status_iso2() {
    run_count_incidence(2, "status");
}

#[test]
fn initialized_count_partition_current_iso1() {
    run_count_incidence(1, "partition_current");
}

#[test]
fn initialized_count_partition_current_iso2() {
    run_count_incidence(2, "partition_current");
}

#[test]
fn initialized_count_partition_pending_iso1() {
    run_count_incidence(1, "partition_pending");
}

#[test]
fn initialized_count_partition_pending_iso2() {
    run_count_incidence(2, "partition_pending");
}

#[test]
fn initialized_count_status_warning_iso1() {
    run_count_incidence(1, "status_warning");
}

#[test]
fn initialized_count_status_warning_iso2() {
    run_count_incidence(2, "status_warning");
}

#[path = "bivariate_coefficient_count_contracts.rs"]
mod contracts;
