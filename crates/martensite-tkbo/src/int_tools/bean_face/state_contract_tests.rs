//! Independent storage, ownership and copy contracts, separate from solver coverage.
use super::{BeanFaceIntersector, Context, Handle, Range, Sequence, precision};

fn scalar_values() -> [f64; 7] {
    [
        0.,
        -0.,
        3.,
        -7.,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x7ff8_0000_0000_0042),
    ]
}

#[test]
fn default_constructor_owns_empty_state_and_exact_scalar_defaults() {
    let bean = BeanFaceIntersector::new();
    assert!(bean.context().is_null());
    assert!(bean.curve.get().unwrap().edge().is_null());
    assert!(bean.surface.get().unwrap().face().is_null());
    assert!(!bean.is_done());
    assert_eq!(bean.minimal_square_distance(), f64::MAX);
    assert_eq!(bean.range.range(), (0., 0.));
    assert_eq!(bean.bounds, [0.; 4]);
    assert_eq!(bean.tolerances, [0.; 2]);
    assert_eq!(bean.criteria, precision::confusion());
    assert_eq!(bean.curve_resolution, precision::p_confusion());
    assert!(bean.transformed_surface.is_null());
    assert!(bean.result().is_empty());
    assert_eq!(bean.range_manager.length(), 0);
}

#[test]
fn completion_and_distance_getters_read_exact_stored_values() {
    let mut bean = BeanFaceIntersector::new();
    for done in [false, true] {
        bean.done = done;
        assert_eq!(bean.is_done(), done);
        for value in scalar_values() {
            bean.minimum_square_distance = value;
            assert_eq!(bean.minimal_square_distance().to_bits(), value.to_bits());
        }
    }
}

#[test]
fn curve_parameter_setter_preserves_order_bits_and_other_state() {
    let mut bean = seeded_state();
    for first in scalar_values() {
        for last in scalar_values() {
            bean.set_bean_parameters(first, last);
            assert_eq!(bean.range.first().to_bits(), first.to_bits());
            assert_eq!(bean.range.last().to_bits(), last.to_bits());
            assert_seed_unchanged(&bean);
        }
    }
}

#[test]
fn surface_parameter_setter_preserves_order_bits_and_other_state() {
    let mut bean = seeded_state();
    for value in scalar_values() {
        bean.set_surface_parameters(value, 3., -0., -7.);
        let bits = bean.bounds.map(f64::to_bits);
        assert_eq!(bits, [value, 3., -0., -7.].map(f64::to_bits));
        assert_seed_unchanged(&bean);
    }
}

fn seeded_state() -> BeanFaceIntersector {
    let mut bean = BeanFaceIntersector::new();
    bean.done = true;
    bean.minimum_square_distance = 37.;
    bean.results.append(Range::new(5., 2.));
    bean
}

fn assert_seed_unchanged(bean: &BeanFaceIntersector) {
    assert!(bean.is_done());
    assert_eq!(bean.minimal_square_distance(), 37.);
    assert_eq!(bean.result().length(), 1);
    assert_eq!(bean.result().first().unwrap().range(), (5., 2.));
}

#[test]
fn context_replacement_retains_and_releases_exact_handle_ownership() {
    let mut bean = BeanFaceIntersector::new();
    let context = Handle::new(Context::default());
    bean.set_context(&context);
    assert!(std::ptr::eq(bean.context(), &bean.context));
    assert_eq!(bean.context(), &context);
    assert_eq!(context.ref_count(), 2);
    bean.set_context(&context);
    assert_eq!(context.ref_count(), 2);
    bean.set_context(&Handle::default());
    assert!(bean.context().is_null());
    assert_eq!(context.ref_count(), 1);
}

#[test]
fn result_borrow_is_stable_and_copy_replaces_without_aliasing() {
    let mut bean = seeded_state();
    let mut copied = Sequence::default();
    copied.append(Range::new(-9., -8.));
    assert!(std::ptr::eq(bean.result(), &bean.results));
    bean.result_into(&mut copied);
    assert_eq!(copied.length(), 1);
    assert_eq!(copied.first().unwrap().range(), (5., 2.));
    copied.clear(None);
    bean.results.append(Range::new(8., 9.));
    assert!(copied.is_empty());
    assert_eq!(bean.result().length(), 2);
}
