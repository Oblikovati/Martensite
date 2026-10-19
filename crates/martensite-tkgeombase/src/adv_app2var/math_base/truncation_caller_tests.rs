//! Truncation caller tests behavior and supporting operations.
use super::*;
use std::sync::{Arc, Mutex};

fn initialized_case() -> (Arc<Mutex<Vec<f64>>>, AdvApp2VarTruncationOrigins) {
    let mut slots = vec![7.; 256];
    slots[16..22].copy_from_slice(&[1., 0., 0.01, 2., 0., 0.01]);
    slots[96] = 1e-6;
    (
        Arc::new(Mutex::new(slots)),
        AdvApp2VarTruncationOrigins {
            coefficients: 16,
            component_bounds: 64,
            precision: 96,
            maximum_error: 128,
        },
    )
}

fn execute(origins: AdvApp2VarTruncationOrigins, slots: Arc<Mutex<Vec<f64>>>) -> i32 {
    let mut kept = -7;
    assert_eq!(
        mmtrpjj_with_caller_slots_(3, 1, 3, -1, slots, origins, &mut kept).unwrap(),
        0
    );
    kept
}

fn overlapping_origins(case: usize) -> AdvApp2VarTruncationOrigins {
    let (_, mut origins) = initialized_case();
    match case {
        1 => origins.maximum_error = 18,
        2 => origins.component_bounds = 18,
        3 => origins.component_bounds = 96,
        4 => origins.component_bounds = 128,
        5 => origins.precision = 18,
        _ => (),
    }
    origins
}

fn independently_expected(case: usize) -> Vec<u64> {
    let (slots, _) = initialized_case();
    let mut expected = slots.lock().unwrap().clone();
    let origins = overlapping_origins(case);
    let component = 0.01_f64 * 2.5_f64.sqrt();
    expected[origins.component_bounds] = if [1, 2].contains(&case) {
        0.
    } else {
        component
    };
    expected[origins.maximum_error] = if [3, 4].contains(&case) {
        component
    } else {
        0.
    };
    expected.into_iter().map(f64::to_bits).collect()
}

#[test]
fn retained_real_overlap_cases_retain_every_slot_and_source_store_order() {
    for case in 0..6 {
        let (slots, _) = initialized_case();
        let kept = execute(overlapping_origins(case), Arc::clone(&slots));
        assert_eq!(kept, if [1, 2, 3].contains(&case) { 1 } else { 3 });
        let actual: Vec<_> = slots
            .lock()
            .unwrap()
            .iter()
            .map(|value| value.to_bits())
            .collect();
        assert_eq!(actual, independently_expected(case));
    }
}

fn borrowed_results(order: i32, coefficients: &[f64]) -> ([f64; 2], f64, i32) {
    let (mut bounds, mut error, mut count) = ([7.; 2], 7., -7);
    mmtrpjj_(
        10,
        2,
        10,
        1e-6,
        order,
        coefficients,
        &mut bounds,
        &mut error,
        &mut count,
    );
    (bounds, error, count)
}

fn owned_results(order: i32, coefficients: &[f64]) -> (Vec<f64>, i32) {
    let mut owner = coefficients.to_vec();
    owner.extend([7., 7., 1e-6, 7., 7.]);
    let slots = Arc::new(Mutex::new(owner));
    let origins = AdvApp2VarTruncationOrigins {
        coefficients: 0,
        component_bounds: 20,
        precision: 22,
        maximum_error: 23,
    };
    let mut count = -7;
    mmtrpjj_with_caller_slots_(10, 2, 10, order, Arc::clone(&slots), origins, &mut count).unwrap();
    let actual = slots.lock().unwrap().clone();
    (actual, count)
}

fn compare_disjoint(order: i32, scale: f64) {
    let mut coefficients = vec![0.; 20];
    coefficients[0] = 1.;
    coefficients[10] = 2.;
    coefficients[8] = scale;
    coefficients[18] = scale * 0.5;
    let (bounds, error, count) = borrowed_results(order, &coefficients);
    let (actual, actual_count) = owned_results(order, &coefficients);
    assert_eq!(actual_count, count);
    assert_eq!(
        actual[20..22]
            .iter()
            .map(|v| v.to_bits())
            .collect::<Vec<_>>(),
        bounds.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
    assert_eq!(actual[23].to_bits(), error.to_bits());
    assert_eq!(&actual[..20], coefficients.as_slice());
    assert_eq!((actual[22], actual[24]), (1e-6, 7.));
}

#[test]
fn common_owner_matches_borrowed_arithmetic_for_every_dispatched_order() {
    for order in [-1, 0, 1, 2, 7] {
        for scale in [1e-8, 0.01, 3.] {
            compare_disjoint(order, scale);
        }
    }
}

#[test]
fn invalid_initial_origin_keeps_caller_slots_and_count_unchanged() {
    for role in 0..4 {
        let (slots, mut origins) = initialized_case();
        let before = slots.lock().unwrap().clone();
        match role {
            0 => origins.coefficients = 257,
            1 => origins.component_bounds = 257,
            2 => origins.precision = 256,
            _ => origins.maximum_error = 256,
        }
        let mut count = -7;
        let error =
            mmtrpjj_with_caller_slots_(3, 1, 3, -1, Arc::clone(&slots), origins, &mut count)
                .unwrap_err();
        assert!(error.to_string().contains("expected"));
        assert_eq!(count, -7);
        assert_eq!(*slots.lock().unwrap(), before);
    }
}

#[test]
fn coefficient_out_of_bounds_is_checked_after_defined_initial_stores() {
    let (slots, mut origins) = initialized_case();
    origins.coefficients = 255;
    let mut count = -7;
    let error = mmtrpjj_with_caller_slots_(3, 1, 3, -1, Arc::clone(&slots), origins, &mut count)
        .unwrap_err();
    assert!(error.to_string().contains("expected"));
    assert_eq!(count, 1);
    let actual = slots.lock().unwrap();
    assert_eq!((actual[64], actual[128]), (0., 0.));
}

#[test]
fn jacobi_constraint_search_rereads_coefficients_after_output_clears() {
    for minimum in [2, 4, 6] {
        for shared in [0, 1, 2] {
            let (slots, mut origins) = initialized_case();
            {
                let mut values = slots.lock().unwrap();
                values[16..23].fill(0.);
                values[16] = 1.;
                values[16 + minimum as usize - 1] = 0.01;
            }
            if shared == 1 {
                origins.maximum_error = 16 + minimum as usize - 1;
            }
            if shared == 2 {
                origins.component_bounds = 16 + minimum as usize - 1;
            }
            let mut count = -7;
            mmtrpjj_with_caller_slots_(
                7,
                1,
                minimum,
                minimum / 2 - 1,
                Arc::clone(&slots),
                origins,
                &mut count,
            )
            .unwrap();
            assert_eq!(count, if shared == 0 { minimum } else { 1 });
            assert_eq!(
                slots.lock().unwrap()[16 + minimum as usize - 1],
                if shared == 0 { 0.01 } else { 0. }
            );
        }
    }
}

#[test]
fn explicit_origins_are_plain_send_sync_values() {
    fn require_send_sync<Value: Send + Sync>(_: &Value) {}
    let (_, origins) = initialized_case();
    require_send_sync(&origins);
    let copied = origins;
    assert_eq!(copied, origins);
}
