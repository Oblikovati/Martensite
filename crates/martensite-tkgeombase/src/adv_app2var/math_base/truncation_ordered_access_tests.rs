//! Truncation ordered access tests behavior and supporting operations.
use super::*;
use std::sync::{Arc, Mutex};
const REAL_ORIGINS: AdvApp2VarTruncationOrigins = AdvApp2VarTruncationOrigins {
    coefficients: 64,
    component_bounds: 256,
    precision: 512,
    maximum_error: 320,
};
const INTEGER_ORIGINS: AdvApp2VarTruncationIntegerOrigins = AdvApp2VarTruncationIntegerOrigins {
    coefficient_stride: 0,
    dimensions: 1,
    coefficient_count: 2,
    continuity_order: 3,
    retained_count: 4,
};
#[test]
fn unused_one_past_precision_follows_retained_constant_tail_path() {
    let reals = Arc::new(Mutex::new(vec![7.; 512]));
    let mut retained = -7;
    assert_eq!(
        mmtrpjj_with_ordered_caller_slots_(
            3,
            1,
            1,
            -1,
            Arc::clone(&reals),
            REAL_ORIGINS,
            &mut retained
        )
        .unwrap(),
        0
    );
    assert_eq!(retained, 1);
    let mut expected = vec![7.; 512];
    expected[256] = 0.;
    expected[320] = 0.;
    assert_eq!(*reals.lock().unwrap(), expected);
}
#[test]
fn unused_coefficient_bound_and_precision_roles_need_no_proxy_scalar_read() {
    for order in [-1, 0, 1, 2] {
        let integers = Arc::new(Mutex::new(vec![3, 0, 1, order, -7]));
        let reals = Arc::new(Mutex::new(vec![7.; 512]));
        let origins = AdvApp2VarTruncationOrigins {
            coefficients: 512,
            component_bounds: 512,
            ..REAL_ORIGINS
        };
        assert_eq!(
            mmtrpjj_with_ordered_indexed_slots_(
                Arc::clone(&integers),
                INTEGER_ORIGINS,
                Arc::clone(&reals),
                origins
            )
            .unwrap(),
            0
        );
        assert_eq!(*integers.lock().unwrap(), [3, 0, 1, order, 1]);
        let mut expected = vec![7.; 512];
        expected[320] = 0.;
        assert_eq!(*reals.lock().unwrap(), expected);
    }
}
#[test]
fn actual_precision_read_failure_retains_earlier_source_ordered_writes() {
    let reals = Arc::new(Mutex::new(vec![7.; 512]));
    reals.lock().unwrap()[65] = 1.;
    let mut retained = -7;
    let failure = mmtrpjj_with_ordered_caller_slots_(
        3,
        1,
        2,
        -1,
        Arc::clone(&reals),
        REAL_ORIGINS,
        &mut retained,
    )
    .unwrap_err();
    assert!(failure.to_string().contains("expected"));
    assert_eq!(retained, 1);
    let mut expected = vec![7.; 512];
    expected[65] = 1.;
    expected[256] = 1.5_f64.sqrt();
    expected[320] = 0.;
    assert_eq!(*reals.lock().unwrap(), expected);
}

#[test]
fn zero_dimension_tail_still_consumes_the_first_norm_bound() {
    let reals = Arc::new(Mutex::new(vec![7.; 512]));
    let origins = AdvApp2VarTruncationOrigins {
        coefficients: 512,
        component_bounds: 512,
        precision: 288,
        ..REAL_ORIGINS
    };
    let mut retained = -7;
    let failure =
        mmtrpjj_with_ordered_caller_slots_(3, 0, 2, -1, Arc::clone(&reals), origins, &mut retained)
            .unwrap_err();
    assert!(failure.to_string().contains("origin=512"));
    assert_eq!(retained, 1);
    let mut expected = vec![7.; 512];
    expected[320] = 0.;
    assert_eq!(*reals.lock().unwrap(), expected);
}
#[test]
fn error_output_is_consumed_even_when_every_other_real_role_is_unused() {
    let reals = Arc::new(Mutex::new(vec![7.; 512]));
    let origins = AdvApp2VarTruncationOrigins {
        coefficients: 512,
        component_bounds: 512,
        precision: 512,
        maximum_error: 512,
    };
    let mut retained = -7;
    assert!(
        mmtrpjj_with_ordered_caller_slots_(3, 0, 1, -1, Arc::clone(&reals), origins, &mut retained)
            .is_err()
    );
    assert_eq!(retained, 1);
    assert_eq!(*reals.lock().unwrap(), vec![7.; 512]);
}
