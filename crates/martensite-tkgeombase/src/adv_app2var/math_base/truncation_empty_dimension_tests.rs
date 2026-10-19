//! Truncation empty dimension tests behavior and supporting operations.
use super::*;
use std::sync::{Arc, Mutex};
const REAL_ORIGINS: AdvApp2VarTruncationOrigins = AdvApp2VarTruncationOrigins {
    coefficients: 16,
    component_bounds: 64,
    precision: 96,
    maximum_error: 128,
};
const INTEGER_ORIGINS: AdvApp2VarTruncationIntegerOrigins = AdvApp2VarTruncationIntegerOrigins {
    coefficient_stride: 0,
    dimensions: 1,
    coefficient_count: 2,
    continuity_order: 3,
    retained_count: 4,
};
fn initialized_empty_dimension_reals() -> Vec<f64> {
    let mut values = vec![7.; 256];
    values[96] = 1e-6;
    values
}
fn expected_empty_dimension_bits() -> Vec<u64> {
    let mut expected = initialized_empty_dimension_reals();
    expected[128] = 0.;
    expected.into_iter().map(f64::to_bits).collect()
}
fn assert_empty_dimension_reals(owner: &Arc<Mutex<Vec<f64>>>) {
    let observed: Vec<_> = owner
        .lock()
        .unwrap()
        .iter()
        .map(|value| value.to_bits())
        .collect();
    assert_eq!(observed, expected_empty_dimension_bits());
}
fn check_real_empty_dimension(dimension: i32, order: i32) {
    let owner = Arc::new(Mutex::new(initialized_empty_dimension_reals()));
    let mut retained = -7;
    let status = mmtrpjj_with_caller_slots_(
        8,
        dimension,
        8,
        order,
        Arc::clone(&owner),
        REAL_ORIGINS,
        &mut retained,
    )
    .unwrap();
    assert_eq!((status, retained), (0, 1));
    assert_empty_dimension_reals(&owner);
}
fn check_integer_empty_dimension(dimension: i32, order: i32) {
    let integers = Arc::new(Mutex::new(vec![8, dimension, 8, order, -7]));
    let reals = Arc::new(Mutex::new(initialized_empty_dimension_reals()));
    let status = mmtrpjj_with_indexed_slots_(
        Arc::clone(&integers),
        INTEGER_ORIGINS,
        Arc::clone(&reals),
        REAL_ORIGINS,
    )
    .unwrap();
    assert_eq!(status, 0);
    assert_eq!(*integers.lock().unwrap(), [8, dimension, 8, order, 1]);
    assert_empty_dimension_reals(&reals);
}
#[test]
fn empty_dimension_real_adapter_preserves_retained_completion_and_all_owner_bits() {
    for dimension in [i32::MIN, -1, 0] {
        for order in [-1, 0, 1, 2] {
            check_real_empty_dimension(dimension, order);
        }
    }
}
#[test]
fn empty_dimension_integer_adapter_preserves_retained_completion_and_all_owner_bits() {
    for dimension in [i32::MIN, -1, 0] {
        for order in [-1, 0, 1, 2] {
            check_integer_empty_dimension(dimension, order);
        }
    }
}
fn check_value_empty_dimension(dimension: i32, order: i32) {
    let (mut bounds, mut error, mut retained) = ([7.], 7., -7);
    let status = mmtrpjj_(
        8,
        dimension,
        8,
        1e-6,
        order,
        &[7.; 64],
        &mut bounds,
        &mut error,
        &mut retained,
    );
    assert_eq!((status, retained, error, bounds[0]), (0, 1, 0., 7.));
}
#[test]
fn empty_dimension_value_entrypoint_keeps_its_existing_completion() {
    for dimension in [i32::MIN, -1, 0] {
        for order in [-1, 0, 1, 2] {
            check_value_empty_dimension(dimension, order);
        }
    }
}
