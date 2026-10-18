//! Truncation precision tests behavior and supporting operations.
use super::mmtrpjj_with_precision_slots_;

#[test]
fn shared_precision_observes_error_clear_before_the_first_threshold_read() {
    let mut slots = vec![7., 1e-6, 7.];
    let (mut bounds, mut count) = ([7.; 2], -7);
    assert_eq!(
        mmtrpjj_with_precision_slots_(
            3,
            1,
            3,
            -1,
            &[1., 0., 1e-8],
            &mut bounds,
            &mut slots,
            1,
            1,
            &mut count
        )
        .unwrap(),
        0
    );
    assert_eq!(count, 3);
    assert_eq!(slots, [7., 0., 7.]);
    assert_eq!(bounds[0].to_bits(), (1e-8_f64 * 2.5_f64.sqrt()).to_bits());
    assert_eq!(bounds[1], 7.);
}

fn owned_truncation(
    coefficients: &[f64],
    order: i32,
    slots: &mut [f64],
    origins: [usize; 2],
) -> (i32, [f64; 2]) {
    let (mut bounds, mut count) = ([7.; 2], -7);
    let length = coefficients.len() as i32;
    assert_eq!(
        mmtrpjj_with_precision_slots_(
            length,
            1,
            length,
            order,
            coefficients,
            &mut bounds,
            slots,
            origins[0],
            origins[1],
            &mut count
        )
        .unwrap(),
        0
    );
    (count, bounds)
}

#[test]
fn equal_values_at_distinct_indices_keep_distinct_precision_identity() {
    let mut slots = vec![7., 1e-6, 1e-6, 7.];
    let (count, bounds) = owned_truncation(&[1., 0., 1e-8], -1, &mut slots, [1, 2]);
    let expected = 1e-8_f64 * 2.5_f64.sqrt();
    assert_eq!(count, 1);
    assert_eq!(slots[1].to_bits(), 1e-6_f64.to_bits());
    assert_eq!(slots[2].to_bits(), expected.to_bits());
    assert_eq!(bounds[0].to_bits(), expected.to_bits());
    assert_eq!((slots[0], slots[3], bounds[1]), (7., 7., 7.));
}

#[test]
fn shared_threshold_reads_apply_to_every_constraint_order_kernel() {
    for order in [-1, 0, 1, 2] {
        let length = if order == -1 { 3 } else { 2 * (order + 1) + 1 };
        let mut coefficients = vec![1.; length as usize];
        *coefficients.last_mut().unwrap() = 1e-8;
        let mut slots = vec![7., 1e-6, 7.];
        let (count, bounds) = owned_truncation(&coefficients, order, &mut slots, [1, 1]);
        assert_eq!(count, length, "continuity order={order}");
        assert_eq!(slots, [7., 0., 7.]);
        assert!(bounds[0] > 0.);
        assert_eq!(bounds[1], 7.);
    }
}

#[test]
fn disjoint_indexed_access_preserves_the_value_interface_for_every_kernel() {
    use super::super::mmtrpjj_;
    for order in [-1, 0, 1, 2] {
        let length = if order == -1 { 3 } else { 2 * (order + 1) + 1 };
        let mut coefficients = vec![1.; length as usize];
        *coefficients.last_mut().unwrap() = 1e-8;
        let mut slots = vec![1e-6, 7.];
        let (count, bounds) = owned_truncation(&coefficients, order, &mut slots, [0, 1]);
        let (mut old_bounds, mut old_error, mut old_count) = ([7.; 2], 7., -7);
        mmtrpjj_(
            length,
            1,
            length,
            1e-6,
            order,
            &coefficients,
            &mut old_bounds,
            &mut old_error,
            &mut old_count,
        );
        assert_eq!(
            (count, bounds.map(f64::to_bits), slots[1].to_bits()),
            (old_count, old_bounds.map(f64::to_bits), old_error.to_bits())
        );
    }
}

#[test]
fn retained_constant_projection_truncation_slices_preserve_counts_and_error_bits() {
    let quadratic = f64::EPSILON / 4.;
    let coefficients = [std::f64::consts::SQRT_2, 0., quadratic];
    let component = quadratic * 2.5_f64.sqrt();
    let bound = (component * component).sqrt();
    for shared in [false, true] {
        let mut slots = vec![7., 1e-6, 7., 7.];
        let error_slot = if shared { 1 } else { 2 };
        let (count, bounds) = owned_truncation(&coefficients, -1, &mut slots, [1, error_slot]);
        assert_eq!(count, if shared { 3 } else { 1 });
        assert_eq!(
            slots[error_slot].to_bits(),
            if shared { 0. } else { bound }.to_bits()
        );
        assert_eq!(bounds[0].to_bits(), component.to_bits());
        assert_eq!((slots[0], slots[3], bounds[1]), (7., 7., 7.));
    }
}

#[test]
fn invalid_slot_indices_are_rejected_before_any_output_mutation() {
    for origins in [[3, 0], [0, 3], [usize::MAX, 0], [0, usize::MAX]] {
        let mut slots = vec![7., 1e-6, 7.];
        let (mut bounds, mut count) = ([7.; 2], -7);
        let failure = mmtrpjj_with_precision_slots_(
            3,
            1,
            3,
            -1,
            &[1., 0., 1e-8],
            &mut bounds,
            &mut slots,
            origins[0],
            origins[1],
            &mut count,
        )
        .unwrap_err();
        assert!(failure.to_string().contains(&format!(
            "precision_slot={}, error_slot={}",
            origins[0], origins[1]
        )));
        assert!(
            failure
                .to_string()
                .contains("expected both indices in 0..3")
        );
        assert_eq!((count, bounds), (-7, [7.; 2]));
        assert_eq!(slots, [7., 1e-6, 7.]);
    }
}

#[test]
fn empty_initialized_owner_is_rejected_without_entering_the_kernel() {
    let (mut bounds, mut count) = ([7.; 2], -7);
    let failure = mmtrpjj_with_precision_slots_(
        3,
        1,
        3,
        -1,
        &[1., 0., 1e-8],
        &mut bounds,
        &mut [],
        0,
        0,
        &mut count,
    )
    .unwrap_err();
    assert!(
        failure
            .to_string()
            .contains("expected both indices in 0..0")
    );
    assert_eq!((count, bounds), (-7, [7.; 2]));
}

#[test]
fn explicit_precision_reset_preserves_independence_between_operations() {
    let mut slots = vec![7., 1e-6, 7.];
    assert_eq!(
        owned_truncation(&[1., 0., 1e-8], -1, &mut slots, [1, 1]).0,
        3
    );
    slots[1] = 1e-6;
    assert_eq!(
        owned_truncation(&[1., 0., 1e-8], -1, &mut slots, [1, 2]).0,
        1
    );
    assert_eq!(slots[1], 1e-6);
}
