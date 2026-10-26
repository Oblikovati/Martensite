//! Exact controls pin both adaptor domains before private input borrowing is introduced.
mod curves;
mod expected;
mod planar;
use crate::{fixture, samples};
use expected::{checked_bits, diamond_orders, rational_orders, repeated_orders};
const RATIONAL_PARAMETERS: [f64; 8] = [0., 0.125, 0.25, 0.375, 0.625, 0.75, 0.875, 1.];
const PERIODIC_PARAMETERS: [f64; 9] = [0., 0.25, 1.25, 2.75, 3.5, 4., 4.25, -0.25, 8.25];

#[test]
fn rational_spatial_values_and_derivatives_match_quotient_equations() {
    let adaptor = curves::rational_spatial();
    for parameter in RATIONAL_PARAMETERS {
        let orders = rational_orders(parameter).map(|x| [x, 2. * x, -x]);
        let actual = samples::cached_values(&adaptor, parameter).unwrap();
        checked_bits(
            "rational_spatial",
            parameter,
            &actual,
            &repeated_orders(orders),
        );
    }
}

#[test]
fn rational_planar_values_and_derivatives_match_quotient_equations() {
    let adaptor = curves::rational_planar();
    for parameter in RATIONAL_PARAMETERS {
        let orders = rational_orders(parameter).map(|x| [x, 2. * x]);
        checked_bits(
            "rational_planar",
            parameter,
            &planar::values(&adaptor, parameter),
            &repeated_orders(orders),
        );
    }
}

#[test]
fn periodic_spatial_midpoints_and_end_bounds_match_polygon_equations() {
    let adaptor = curves::periodic_spatial();
    for parameter in PERIODIC_PARAMETERS {
        let orders = diamond_orders(parameter).map(|[x, y]| [x, y, 0.]);
        let actual = samples::cached_values(&adaptor, parameter).unwrap();
        checked_bits(
            "periodic_spatial",
            parameter,
            &actual,
            &repeated_orders(orders),
        );
    }
}

#[test]
fn periodic_planar_midpoints_and_end_bounds_match_polygon_equations() {
    let adaptor = curves::periodic_planar();
    for parameter in PERIODIC_PARAMETERS {
        let actual = planar::values(&adaptor, parameter);
        checked_bits(
            "periodic_planar",
            parameter,
            &actual,
            &repeated_orders(diamond_orders(parameter)),
        );
    }
}

#[test]
fn linear_first_last_and_interior_knots_keep_all_derivatives() {
    let adaptor = fixture::axis_adaptor(4).unwrap();
    for parameter in [0., 0.5, 1., 2., 3.5, 4.] {
        let actual = samples::cached_values(&adaptor, parameter).unwrap();
        checked_bits(
            "linear_boundaries",
            parameter,
            &actual,
            &samples::closed_values(parameter),
        );
    }
}
