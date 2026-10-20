use super::{
    ContapSamplingContext,
    fake_surface::{make_spline_surface, make_surface},
};
use martensite_tkernel::standard::real::{real_first, real_last};

#[test]
fn five_samples_keep_bound_order_reversal_and_finite_sentinel_rules() {
    for bounds in [[1., 2., 9., 18.], [9., 18., 1., 2.]] {
        let (surface, _) = make_surface(bounds, None);
        let mut context = ContapSamplingContext::new();
        assert_eq!(context.nb_sample_points(&surface).unwrap(), 5);
        assert_eq!(context.bounds(), [1., 2., 9., 18.]);
        for (index, expected) in [
            (1, (3., 6.)),
            (2, (3., 14.)),
            (3, (7., 6.)),
            (4, (7., 14.)),
            (99, (5., 10.)),
        ] {
            assert_eq!(context.sample_point(&surface, index).unwrap(), expected);
        }
    }
    check_sentinel_limits();
}

fn check_sentinel_limits() {
    let mut context = ContapSamplingContext::new();
    let (surface, _) = make_surface([real_first(), real_first(), real_last(), real_last()], None);
    context.nb_sample_points(&surface).unwrap();
    assert_eq!(context.bounds(), [-1e5, -1e5, 1e5, 1e5]);
    assert_eq!(context.sample_point(&surface, 1).unwrap(), (-5e4, -5e4));
    let (surface, _) = make_surface([real_first(), 4., 10., real_last()], None);
    context.nb_sample_points(&surface).unwrap();
    assert_eq!(context.bounds(), [-199990., 4., 10., 200004.]);
    assert_eq!(
        context.sample_point(&surface, 1).unwrap(),
        (-149990., 50004.)
    );
}

#[test]
fn spline_grid_keeps_asymmetric_denominators_and_five_cell_threshold() {
    let mut context = ContapSamplingContext::new();
    let (surface, _) = make_spline_surface([1., 2., 9., 18.], None, Some([4, 3, 3, 3]));
    assert_eq!(context.nb_sample_points(&surface).unwrap(), 12);
    assert_eq!(context.sample_point(&surface, 1).unwrap(), (2.6, 5.2));
    assert_eq!(
        context.sample_point(&surface, 12).unwrap(),
        (5.800000000000001, 14.8)
    );
    let (surface, _) = make_spline_surface([1., 2., 9., 18.], None, Some([2, 3, 2, 3]));
    assert_eq!(context.nb_sample_points(&surface).unwrap(), 5);
    assert_eq!(context.sample_point(&surface, 1).unwrap(), (3., 6.));
}

#[test]
fn remaining_sentinel_sides_collapsed_ranges_and_ieee_bounds_are_explicit() {
    let mut context = ContapSamplingContext::new();
    let (surface, _) = make_surface([-10., real_first(), real_last(), 30.], None);
    context.nb_sample_points(&surface).unwrap();
    assert_eq!(context.bounds(), [-10., -199970., 199990., 30.]);
    let (surface, _) = make_surface([2., 3., 2., 3.], None);
    context.nb_sample_points(&surface).unwrap();
    assert_eq!(context.sample_point(&surface, 1).unwrap(), (2., 3.));
    let (surface, _) = make_surface([f64::NEG_INFINITY, -0., f64::INFINITY, 0.], None);
    context.nb_sample_points(&surface).unwrap();
    assert_eq!(context.bounds()[0], f64::NEG_INFINITY);
    assert_eq!(context.bounds()[2], f64::INFINITY);
    assert_eq!(context.bounds()[1].to_bits(), (-0_f64).to_bits());
    assert!(context.sample_point(&surface, 5).unwrap().0.is_nan());
}
