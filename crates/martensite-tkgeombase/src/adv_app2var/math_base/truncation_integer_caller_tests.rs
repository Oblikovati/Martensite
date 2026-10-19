//! Truncation integer caller tests behavior and supporting operations.
use super::*;
use std::sync::{Arc, Mutex};
struct IntegerCase {
    integers: Arc<Mutex<Vec<i32>>>,
    reals: Arc<Mutex<Vec<f64>>>,
    integer_origins: AdvApp2VarTruncationIntegerOrigins,
    real_origins: AdvApp2VarTruncationOrigins,
}
fn initialized_integer_case(order: i32, mode: usize) -> IntegerCase {
    let mut integers = vec![-7; 96];
    integers[16..20].copy_from_slice(&[8, 2, 8, order]);
    let mut reals = vec![7.; 256];
    for component in 0..6 {
        reals[16 + component * 8..24 + component * 8].fill(0.);
        reals[16 + component * 8] = component as f64 + 1.;
    }
    reals[25] = 0.01;
    reals[192] = 1e-6;
    IntegerCase {
        integers: Arc::new(Mutex::new(integers)),
        reals: Arc::new(Mutex::new(reals)),
        integer_origins: integer_case_origins(mode),
        real_origins: AdvApp2VarTruncationOrigins {
            coefficients: 16,
            component_bounds: 128,
            precision: 192,
            maximum_error: 224,
        },
    }
}
fn integer_case_origins(mode: usize) -> AdvApp2VarTruncationIntegerOrigins {
    let mut origins = AdvApp2VarTruncationIntegerOrigins {
        coefficient_stride: 16,
        dimensions: 17,
        coefficient_count: 18,
        continuity_order: 19,
        retained_count: 64,
    };
    match mode {
        1 => origins.retained_count = 16,
        2 => origins.retained_count = 17,
        3 => origins.retained_count = 18,
        4 => origins.retained_count = 19,
        5 => {
            origins.coefficient_count = 16;
            origins.retained_count = 16;
        }
        6 => {
            origins.continuity_order = 17;
            origins.retained_count = 17;
        }
        _ => (),
    }
    origins
}
fn expected_linear_owner(case: &IntegerCase, order: i32) -> (Vec<i32>, Vec<u64>) {
    let mut integers = case.integers.lock().unwrap().clone();
    let mut reals = case.reals.lock().unwrap().clone();
    let origins = case.integer_origins;
    integers[origins.retained_count] = (2 * (order + 1)).max(1);
    let dimension = integers[origins.dimensions] as usize;
    reals[128..128 + dimension].fill(0.);
    reals[224] = 0.;
    let retained = if order >= 0 {
        2
    } else if dimension < 2 || integers[origins.coefficient_count] < 2 {
        1
    } else {
        reals[129] = 0.01 * 1.5_f64.sqrt();
        2
    };
    integers[origins.retained_count] = retained;
    (integers, reals.into_iter().map(f64::to_bits).collect())
}
fn verify_linear_case(order: i32, mode: usize) {
    let case = initialized_integer_case(order, mode);
    let expected = expected_linear_owner(&case, order);
    let status = mmtrpjj_with_indexed_slots_(
        Arc::clone(&case.integers),
        case.integer_origins,
        Arc::clone(&case.reals),
        case.real_origins,
    )
    .unwrap();
    assert_eq!(status, 0);
    assert_eq!(*case.integers.lock().unwrap(), expected.0);
    let actual: Vec<_> = case
        .reals
        .lock()
        .unwrap()
        .iter()
        .map(|value| value.to_bits())
        .collect();
    assert_eq!(actual, expected.1);
}
#[test]
fn retained_integer_alias_matrix_preserves_every_initialized_owner_slot() {
    for order in [-1, 0, 1, 2] {
        for mode in 0..6 {
            verify_linear_case(order, mode);
        }
    }
    verify_linear_case(2, 6);
}

fn execute_integer_case(case: &IntegerCase) -> Result<i32, martensite_tkernel::standard::Failure> {
    mmtrpjj_with_indexed_slots_(
        Arc::clone(&case.integers),
        case.integer_origins,
        Arc::clone(&case.reals),
        case.real_origins,
    )
}
fn quadratic_integer_case(mode: usize) -> IntegerCase {
    let mut case = initialized_integer_case(-1, mode);
    case.integer_origins = integer_case_origins(mode);
    case.real_origins = AdvApp2VarTruncationOrigins {
        coefficients: 16,
        component_bounds: 64,
        precision: 96,
        maximum_error: 128,
    };
    case.integers.lock().unwrap()[16..20].copy_from_slice(&[
        3,
        if mode <= 2 { 2 } else { 1 },
        3,
        -1,
    ]);
    let mut reals = case.reals.lock().unwrap();
    reals.fill(7.);
    let tail = if mode == 1 { 1e-8 } else { 0.01 };
    reals[16..22].copy_from_slice(&[1., 0., tail, 2., 0., tail]);
    reals[96] = 1e-6;
    drop(reals);
    case
}
fn verify_quadratic_integer_case(mode: usize) {
    let case = quadratic_integer_case(mode);
    let mut integers = case.integers.lock().unwrap().clone();
    let mut reals = case.reals.lock().unwrap().clone();
    let component = if mode == 1 { 1e-8 } else { 0.01 } * 2.5_f64.sqrt();
    integers[case.integer_origins.retained_count] = if mode == 1 || mode == 3 { 1 } else { 3 };
    reals[64] = if mode == 3 { 0. } else { component };
    reals[128] = if mode == 1 {
        (component * component + component * component).sqrt()
    } else {
        0.
    };
    if mode == 1 {
        reals[65] = component;
    }
    assert_eq!(execute_integer_case(&case).unwrap(), 0);
    assert_eq!(*case.integers.lock().unwrap(), integers);
    let actual: Vec<_> = case
        .reals
        .lock()
        .unwrap()
        .iter()
        .map(|value| value.to_bits())
        .collect();
    assert_eq!(
        actual,
        reals.into_iter().map(f64::to_bits).collect::<Vec<_>>()
    );
}
#[test]
fn four_retained_integer_witnesses_retain_every_owner_bit() {
    for mode in [3, 1, 2, 4] {
        verify_quadratic_integer_case(mode);
    }
}
#[test]
fn combined_integer_stride_and_real_coefficient_error_aliases_share_one_pipeline() {
    let mut case = initialized_integer_case(-1, 1);
    case.real_origins.maximum_error = 25;
    let mut expected = case.reals.lock().unwrap().clone();
    expected[25] = 0.;
    expected[128..130].fill(0.);
    execute_integer_case(&case).unwrap();
    assert_eq!(case.integers.lock().unwrap()[16], 1);
    assert_eq!(*case.reals.lock().unwrap(), expected);
}
fn invalid_integer_origins(role: usize) -> AdvApp2VarTruncationIntegerOrigins {
    let mut origins = integer_case_origins(0);
    match role {
        0 => origins.coefficient_stride = 96,
        1 => origins.dimensions = 96,
        2 => origins.coefficient_count = 96,
        3 => origins.continuity_order = 96,
        _ => origins.retained_count = 96,
    }
    origins
}
#[test]
fn invalid_integer_origin_reports_i32_shape_before_mutating_either_owner() {
    for role in 0..5 {
        let mut case = initialized_integer_case(-1, 0);
        case.integer_origins = invalid_integer_origins(role);
        let before = (
            case.integers.lock().unwrap().clone(),
            case.reals.lock().unwrap().clone(),
        );
        let failure = execute_integer_case(&case).unwrap_err().to_string();
        assert!(failure.contains("origin=96, index=0"));
        assert!(failure.contains("expected initialized i32 slot in 0..96"));
        assert_eq!(*case.integers.lock().unwrap(), before.0);
        assert_eq!(*case.reals.lock().unwrap(), before.1);
    }
}
#[test]
fn invalid_real_origin_also_preserves_the_valid_integer_owner() {
    let mut case = initialized_integer_case(-1, 0);
    case.real_origins.precision = 256;
    let before = case.integers.lock().unwrap().clone();
    assert!(execute_integer_case(&case).is_err());
    assert_eq!(*case.integers.lock().unwrap(), before);
}
#[test]
fn integer_origins_are_plain_send_sync_values() {
    fn require_send_sync<Value: Send + Sync>(_: &Value) {}
    let origins = integer_case_origins(0);
    require_send_sync(&origins);
    assert_eq!(origins, origins);
}
