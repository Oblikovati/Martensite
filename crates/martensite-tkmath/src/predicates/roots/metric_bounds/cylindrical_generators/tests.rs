// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::gp::Axis3;

const IDENTITY: [[f64; 4]; 3] = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]];
const TRIM: [f64; 2] = [0., 10.];

#[test]
fn exact_axis_pair_and_lost_bits_at_large_translation() {
    let cylinder = cylinder(1.5);
    let chart = CylindricalGeneratorChart::capture(cylinder, IDENTITY, Direction3::Y).unwrap();
    assert_eq!(chart.kind(), CylindricalPullKind::Radial);
    assert_eq!(
        chart.enclose(parallel_x_lines([-1.5, 1.5]), TRIM),
        Ok([0.; 2])
    );

    let x = 2f64.powi(53);
    let mut placement = IDENTITY;
    placement[0][3] = x;
    let chart = CylindricalGeneratorChart::capture(cylinder, placement, Direction3::Y).unwrap();
    assert_eq!(
        chart.enclose(parallel_x_lines([x - 1.5, x + 1.5]), TRIM),
        Ok([0.5; 2])
    );
}

#[test]
fn transported_center_rounding_and_axial_scale_are_charged() {
    let mut frame = Axis3::default();
    frame.set_location(Point3::new(1., 0., 0.));
    let cylinder = Cylinder3::new(frame, 4.).unwrap();
    let x = 2f64.powi(53);
    let mut placement = IDENTITY;
    placement[0][3] = x;
    placement[2][2] = 1.25;
    let chart = CylindricalGeneratorChart::capture(cylinder, placement, Direction3::Y).unwrap();
    let bound = chart
        .enclose(parallel_x_lines([x - 4., x + 4.]), [0., 12.])
        .unwrap();
    // Both rounded anchors miss the original chart by 1. Axial direction
    // differs by 1/4, so the endpoint displacement norm is sqrt(1 + 3²).
    let expected = 10f64.sqrt();
    assert!(
        bound
            .into_iter()
            .all(|value| expected <= value && value <= expected.next_up())
    );
}

#[test]
fn irrational_roots_have_a_finite_transport_bound() {
    let cylinder = cylinder(5.);
    let pull = Direction3::new(1., 1., 0.).unwrap();
    let component = 5. / 2f64.sqrt();
    let lines = [[-component, component], [component, -component]]
        .map(|[x, y]| Line3::new(Point3::new(x, y, 0.), Direction3::Z));
    let chart = CylindricalGeneratorChart::capture(cylinder, IDENTITY, pull).unwrap();
    let bound = chart.enclose(lines, TRIM).unwrap();
    assert!(bound.into_iter().all(|value| 0. < value && value < 3e-15));
}

#[test]
fn exact_placed_axial_and_skew_pulls_are_distinct() {
    let cylinder = cylinder(5.);
    let rotated = [[0., 0., 1., 0.], [0., 1., 0., 0.], [-1., 0., 0., 0.]];
    let axial = CylindricalGeneratorChart::capture(cylinder, rotated, Direction3::X).unwrap();
    assert_eq!(axial.kind(), CylindricalPullKind::Axial);
    let lines = parallel_x_lines([-5., 5.]);
    assert_eq!(
        axial.enclose(lines, TRIM),
        Err(CylindricalGeneratorError::NonRadial)
    );
    let skew = Direction3::new(1., 1., 0.).unwrap();
    let chart = CylindricalGeneratorChart::capture(cylinder, rotated, skew).unwrap();
    assert_eq!(chart.kind(), CylindricalPullKind::Skew);
}

fn cylinder(radius: f64) -> Cylinder3 {
    Cylinder3::new(Axis3::default(), radius).unwrap()
}

fn parallel_x_lines(x: [f64; 2]) -> [Line3; 2] {
    x.map(|x| Line3::new(Point3::new(x, 0., 0.), Direction3::Z))
}
