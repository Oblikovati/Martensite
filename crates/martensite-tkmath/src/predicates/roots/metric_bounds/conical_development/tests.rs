// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn zero_reference_radius_at_the_seam_has_exact_developed_coordinates() {
    let chart = ConicalDevelopment::new(0.5, 0., 0.25, true).unwrap();
    assert_eq!(chart.bounds([0.25, 3.]).unwrap(), [[3.; 2], [0.; 2]]);
    assert!(matches!(
        chart.bounds([0.25, 0.]),
        Err(ConicalDevelopmentFailure::Slant { .. })
    ));
}

#[test]
fn reversed_meridian_and_indirect_charts_produce_the_same_physical_development() {
    let direct = ConicalDevelopment::new(0.4, 2., 0.5, true).unwrap();
    let narrowing = ConicalDevelopment::new(-0.4, 2., -0.5, true).unwrap();
    let indirect = ConicalDevelopment::new(0.4, 2., -0.5, false).unwrap();
    let point = direct.bounds([0.75, 0.75]).unwrap();
    assert_eq!(point, narrowing.bounds([-0.75, -0.75]).unwrap());
    assert_eq!(point, indirect.bounds([-0.75, 0.75]).unwrap());
    assert!(point[1][0] > 0.);
}

#[test]
fn invalid_carrier_and_unrepresentable_parameter_fail_closed() {
    assert!(matches!(
        ConicalDevelopment::new(f64::NAN, 2., 0., true),
        Err(ConicalDevelopmentFailure::Input { .. })
    ));
    let chart = ConicalDevelopment::new(0.5, 2., -f64::MAX, true).unwrap();
    assert!(chart.bounds([f64::MAX, 1.]).is_err());
}
