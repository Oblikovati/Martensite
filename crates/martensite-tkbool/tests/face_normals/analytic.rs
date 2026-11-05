//! Closed-form normals supply an independent reference for every retained baseline input.
use super::{FRAC_PI_2, PI, compare_observation, observe};

#[test]
fn authored_planes_spheres_cones_and_degenerate_patches_have_closed_form_normals() {
    for kind in 0..5 {
        for orientation in 0..4 {
            check_analytic_points(kind, orientation);
        }
    }
}
fn check_analytic_points(kind: i32, orientation: usize) {
    for point in 0..4 {
        let expected = expected_observation(kind, orientation, point);
        let label = format!("kind={kind}, orientation={orientation}, point={point}");
        compare_observation(observe(kind, orientation, point), &expected, &label);
    }
}
fn expected_observation(kind: i32, orientation: usize, point: usize) -> [f64; 9] {
    if kind >= 3 {
        let status = if kind == 3 { -1. } else { 0. };
        return [status, status, 1., 7., 8., 9., 1., 0., 0.];
    }
    let (vector, unit) = analytic_normal(kind, point);
    let sign = if orientation == 1 { -1. } else { 1. };
    let span = if kind == 1 { PI } else { 2. };
    [
        1.,
        1.,
        span,
        vector[0],
        vector[1],
        vector[2],
        sign * unit[0],
        sign * unit[1],
        sign * unit[2],
    ]
}
fn analytic_normal(kind: i32, point: usize) -> ([f64; 3], [f64; 3]) {
    if kind == 0 {
        return ([0., 0., 1.], [0., 0., 1.]);
    }
    let (u, v): (f64, f64) = [(0.3, 0.2), (0., 0.), (0.4, FRAC_PI_2), (0.4, -FRAC_PI_2)][point];
    if kind == 1 {
        let radial = [
            martensite_cmath::cos(v) * martensite_cmath::cos(u),
            martensite_cmath::cos(v) * martensite_cmath::sin(u),
            martensite_cmath::sin(v),
        ];
        let length = if point >= 2 { 2. } else { 1. };
        return (radial.map(|x| length * x), radial);
    }
    cone_closed_form(u, v, point)
}
fn cone_closed_form(u: f64, v: f64, point: usize) -> ([f64; 3], [f64; 3]) {
    if point == 1 {
        return ([0., 0., -1.], [0., 0., -1.]);
    }
    let alpha: f64 = 0.35;
    let outward = [
        martensite_cmath::cos(alpha) * martensite_cmath::cos(u),
        martensite_cmath::cos(alpha) * martensite_cmath::sin(u),
        -martensite_cmath::sin(alpha),
    ];
    if point == 2 {
        // At this singular sample the documented lower-bound evaluation is v=-1.2.
        // The cone's analytic cross product has magnitude |v*sin(alpha)|.
        return (
            outward.map(|x| -1.2 * martensite_cmath::sin(alpha) * x),
            outward.map(|x| -x),
        );
    }
    let unit = outward.map(|x| v.signum() * x);
    (unit, unit)
}
