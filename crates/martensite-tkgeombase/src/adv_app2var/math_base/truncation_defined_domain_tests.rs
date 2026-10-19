//! Truncation defined domain tests behavior and supporting operations.
use super::*;
use std::sync::{Arc, Mutex};
const INTEGER_ORIGINS: AdvApp2VarTruncationIntegerOrigins = AdvApp2VarTruncationIntegerOrigins {
    coefficient_stride: 0,
    dimensions: 1,
    coefficient_count: 2,
    continuity_order: 3,
    retained_count: 0,
};
struct DomainCase {
    integers: Arc<Mutex<Vec<i32>>>,
    reals: Arc<Mutex<Vec<f64>>>,
    origins: AdvApp2VarTruncationOrigins,
}
fn initialized_domain_case(integers: Vec<i32>, error_origin: usize) -> DomainCase {
    DomainCase {
        integers: Arc::new(Mutex::new(integers)),
        reals: Arc::new(Mutex::new(vec![7.; 512])),
        origins: AdvApp2VarTruncationOrigins {
            coefficients: 64,
            component_bounds: 256,
            precision: 288,
            maximum_error: error_origin,
        },
    }
}
fn execute_domain_case(case: &DomainCase) {
    assert_eq!(
        mmtrpjj_with_indexed_slots_(
            Arc::clone(&case.integers),
            INTEGER_ORIGINS,
            Arc::clone(&case.reals),
            case.origins
        )
        .unwrap(),
        0
    );
}
fn owner_bits(owner: &Arc<Mutex<Vec<f64>>>) -> Vec<u64> {
    owner
        .lock()
        .unwrap()
        .iter()
        .map(|value| value.to_bits())
        .collect()
}
struct MaximaRecord {
    order: i32,
    count: i32,
    bound: f64,
}
fn maxima_record(line: &str) -> MaximaRecord {
    let mut fields = line.split_whitespace();
    let order = fields.next().unwrap().parse().unwrap();
    let count = fields.next().unwrap().parse().unwrap();
    let bits = u64::from_str_radix(fields.next().unwrap(), 16).unwrap();
    assert!(fields.next().is_none());
    MaximaRecord {
        order,
        count,
        bound: f64::from_bits(bits),
    }
}
fn maxima_case(record: &MaximaRecord) -> DomainCase {
    let case = initialized_domain_case(vec![59, 2, record.count, record.order, -7], 257);
    let mut reals = case.reals.lock().unwrap();
    reals[64..182].fill(0.);
    reals[64 + 58 + record.count as usize] = 1.;
    reals[288] = 0.;
    drop(reals);
    case
}
fn verify_maximum(record: &MaximaRecord) {
    let case = maxima_case(record);
    let mut expected = owner_bits(&case.reals);
    expected[256] = 0_f64.to_bits();
    expected[257] = record.bound.to_bits();
    execute_domain_case(&case);
    assert_eq!(
        *case.integers.lock().unwrap(),
        [record.count, 2, record.count, record.order, -7]
    );
    assert_eq!(owner_bits(&case.reals), expected);
}
#[test]
fn every_reviewed_jacobi_maximum_preserves_captured_stride_and_second_bound_identity() {
    let records: Vec<_> = include_str!("truncation_jacobi_maxima.bits")
        .lines()
        .map(maxima_record)
        .collect();
    assert_eq!(records.len(), 165);
    assert_eq!(
        [0, 1, 2].map(|order| records
            .iter()
            .filter(|record| record.order == order)
            .count()),
        [57, 55, 53]
    );
    for record in records {
        verify_maximum(&record);
    }
}
fn stride_case(stride: i32) -> DomainCase {
    let case = initialized_domain_case(vec![stride, 3, 2, -1, -7], 320);
    let mut reals = case.reals.lock().unwrap();
    reals[288] = if stride == 0 { 0. } else { 10. };
    if stride == 0 {
        reals[65] = 1.;
    } else {
        reals[65] = 0.;
        reals[(65 + stride) as usize] = 4.;
        reals[(65 + 2 * stride) as usize] = 3.;
    }
    drop(reals);
    case
}
fn expected_stride_bits(case: &DomainCase, stride: i32) -> Vec<u64> {
    let mut expected = owner_bits(&case.reals);
    let base = 1.5_f64.sqrt();
    let (bounds, error) = if stride == 0 {
        ([base; 3], 0.)
    } else {
        ([0., 4. * base, 3. * base], 5. * base)
    };
    for (index, bound) in bounds.into_iter().enumerate() {
        expected[256 + index] = bound.to_bits();
    }
    expected[320] = error.to_bits();
    expected
}
#[test]
fn signed_prefix_and_zero_stride_follow_independent_linear_tail_closed_forms() {
    for stride in [-1, 0, 2] {
        let case = stride_case(stride);
        let expected = expected_stride_bits(&case, stride);
        execute_domain_case(&case);
        assert_eq!(
            *case.integers.lock().unwrap(),
            [if stride == 0 { 2 } else { 1 }, 3, 2, -1, -7]
        );
        assert_eq!(owner_bits(&case.reals), expected);
    }
}
