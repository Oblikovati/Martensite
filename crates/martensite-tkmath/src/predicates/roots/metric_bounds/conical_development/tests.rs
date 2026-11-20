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

#[test]
fn authored_rectangle_checks_filled_radial_and_angular_ranges() {
    use crate::predicates::roots::metric_bounds::ConicalSectorWedge::PositiveX;
    let chart = ConicalDevelopment::new(0.5, 0., 0., true).unwrap();
    let band = chart.trim_band([3., 8.], Some([-0.5, 0.5])).unwrap();
    let proof = band.authored([[4., 5.], [-0.25, 0.25]], PositiveX).unwrap();
    assert!(proof.slant[0] <= 4. && proof.slant[1] > 5.);
    assert!(proof.angular.unwrap()[0] < 0.);
    assert!(band.authored([[3., 5.], [0., 1.]], PositiveX).is_err());
    assert!(band.authored([[4., 5.], [1., 2.]], PositiveX).is_err());
}

#[test]
fn cap_projection_uses_stored_slant_origin_and_one_physical_turn() {
    use crate::predicates::roots::metric_bounds::ConicalTrimCarrier;
    let chart = ConicalDevelopment::new(0.5, 0., 0., true).unwrap();
    let band = chart.trim_band([3., 8.], Some([-0.5, 0.5])).unwrap();
    let carrier = ConicalTrimCarrier {
        radius: 1.,
        axial_origin: 4.,
        semi_angle: 0.5,
    };
    let turn = std::f64::consts::TAU;
    let proof = band
        .fitted(carrier, [[turn - 0.125, turn + 0.125], [0., 1.]])
        .unwrap();
    assert!(proof.slant[0] > 3.9 && proof.slant[1] < 5.1);
    let [low, high] = proof.angular.unwrap();
    assert!(low < -0.125 && high > 0.12);
    assert!(band.fitted(carrier, [[0., 0.5], [0., 1.]]).is_err());
    assert!(
        band.fitted(
            ConicalTrimCarrier {
                semi_angle: 0.6,
                ..carrier
            },
            [[0., 0.1], [0., 1.]]
        )
        .is_err()
    );
}

#[test]
fn native_hole_envelope_accounts_for_signed_meridian_and_angular_directions() {
    let direct = ConicalDevelopment::new(0.4, 2., 0.7, true).unwrap();
    let narrowing = ConicalDevelopment::new(-0.4, 2., -0.7, true).unwrap();
    let positive = direct.native_trim_envelope([[0.5, 0.9], [3., 4.]]).unwrap();
    let reversed = narrowing
        .native_trim_envelope([[-0.9, -0.5], [-4., -3.]])
        .unwrap();
    assert_eq!(positive, reversed);
    let full = direct
        .trim_band([1., 8.], None)
        .unwrap()
        .with_angular_enclosures();
    let footprint = full
        .authored(
            [[8., 9.], [-0.5, 0.5]],
            crate::predicates::roots::metric_bounds::ConicalSectorWedge::PositiveX,
        )
        .unwrap();
    assert!(footprint.angular.is_some());
    assert!(full.limits().angular.is_none());
}
