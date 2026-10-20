//! Independent calculus checks for planar analytic curves; no captured numeric output.
use martensite_tkg2d::geom2d::{
    Geom2dCircle, Geom2dCurve, Geom2dEllipse, Geom2dHyperbola, Geom2dLine, Geom2dParabola,
};
use martensite_tkmath::gp::{Axis22d, Direction2, Point2};

type DerivativeJet = [[f64; 2]; 4];

#[derive(Clone, Copy, Debug)]
enum AnalyticFamily {
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
}

fn local_derivatives(family: AnalyticFamily, u: f64, scale: f64) -> DerivativeJet {
    match family {
        AnalyticFamily::Line => [[u, 0.0], [1.0, 0.0], [0.0; 2], [0.0; 2]],
        AnalyticFamily::Circle => trigonometric_derivatives(u, scale, scale),
        AnalyticFamily::Ellipse => trigonometric_derivatives(u, 2.0 * scale, scale),
        AnalyticFamily::Hyperbola => hyperbolic_derivatives(u, scale),
        // A parabola with focus distance f satisfies x=y²/(4f), y=u.
        AnalyticFamily::Parabola => [
            [u * u / (4.0 * scale), u],
            [u / (2.0 * scale), 1.0],
            [1.0 / (2.0 * scale), 0.0],
            [0.0; 2],
        ],
    }
}

fn hyperbolic_derivatives(u: f64, scale: f64) -> DerivativeJet {
    let even = [
        2.0 * scale * martensite_cmath::cosh(u),
        scale * martensite_cmath::sinh(u),
    ];
    let odd = [
        2.0 * scale * martensite_cmath::sinh(u),
        scale * martensite_cmath::cosh(u),
    ];
    [even, odd, even, odd]
}

fn trigonometric_derivatives(u: f64, a: f64, b: f64) -> DerivativeJet {
    let (sine, cosine) = martensite_cmath::sincos(u);
    [
        [a * cosine, b * sine],
        [-a * sine, b * cosine],
        [-a * cosine, -b * sine],
        [a * sine, -b * cosine],
    ]
}

fn global_derivatives(local: DerivativeJet, sense: bool) -> DerivativeJet {
    // X=(0,1); Y=(-1,0) for a direct frame and (1,0) for an indirect frame.
    let y_sign = if sense { -1.0 } else { 1.0 };
    let mut global = local.map(|[x, y]| [y_sign * y, x]);
    global[0][0] += 3.0;
    global[0][1] -= 5.0;
    global
}

fn analytic_fixture(family: AnalyticFamily, scale: f64, sense: bool) -> Box<dyn Geom2dCurve> {
    let frame = Axis22d::new(Point2::new(3.0, -5.0), Direction2::Y, sense).unwrap();
    match family {
        AnalyticFamily::Line => Box::new(Geom2dLine::from_point(frame.location(), Direction2::Y)),
        AnalyticFamily::Circle => Box::new(Geom2dCircle::new(frame, scale).unwrap()),
        AnalyticFamily::Ellipse => Box::new(Geom2dEllipse::new(frame, 2.0 * scale, scale).unwrap()),
        AnalyticFamily::Hyperbola => {
            Box::new(Geom2dHyperbola::new(frame, 2.0 * scale, scale).unwrap())
        }
        AnalyticFamily::Parabola => Box::new(Geom2dParabola::new(frame, scale).unwrap()),
    }
}

fn actual_derivatives(curve: &dyn Geom2dCurve, u: f64) -> DerivativeJet {
    let result = curve.eval_d3(u).unwrap();
    [
        [result.point.x(), result.point.y()],
        [result.d1.x(), result.d1.y()],
        [result.d2.x(), result.d2.y()],
        [result.d3.x(), result.d3.y()],
    ]
}

fn assert_derivatives(actual: DerivativeJet, expected: DerivativeJet, label: &str) {
    for (order, (actual_pair, expected_pair)) in actual.into_iter().zip(expected).enumerate() {
        assert_coordinate_pair(
            actual_pair,
            expected_pair,
            &format!("{label}: derivative {order}"),
        );
    }
}

#[test]
fn analytic_jets_follow_closed_forms_in_both_frame_orientations() {
    let families = [
        AnalyticFamily::Line,
        AnalyticFamily::Circle,
        AnalyticFamily::Ellipse,
        AnalyticFamily::Hyperbola,
        AnalyticFamily::Parabola,
    ];
    let mut sample_count: usize = 0;
    for family in families {
        sample_count += check_scales_and_orientations(family);
    }
    assert_eq!(sample_count, 1320);
}

fn check_parameter_grid(family: AnalyticFamily, sense: bool, scale: f64) -> usize {
    let curve = analytic_fixture(family, scale, sense);
    for index in -16..=16 {
        let u = f64::from(index) / 8.0;
        let expected = global_derivatives(local_derivatives(family, u, scale), sense);
        let label = format!("{family:?}, direct={sense}, scale={scale}, u={u}");
        assert_derivatives(actual_derivatives(curve.as_ref(), u), expected, &label);
        check_lower_evaluators(curve.as_ref(), u, expected, &label);
    }
    33
}

fn check_scales_and_orientations(family: AnalyticFamily) -> usize {
    let mut sample_count: usize = 0;
    for sense in [false, true] {
        for scale in [0.125, 1.0, 8.0, 128.0] {
            sample_count += check_parameter_grid(family, sense, scale);
        }
    }
    sample_count
}

fn check_lower_evaluators(curve: &dyn Geom2dCurve, u: f64, expected: DerivativeJet, label: &str) {
    let point = curve.eval_d0(u).unwrap();
    assert_coordinate_pair([point.x(), point.y()], expected[0], label);
    let first = curve.eval_d1(u).unwrap();
    assert_coordinate_pair([first.point.x(), first.point.y()], expected[0], label);
    assert_coordinate_pair([first.d1.x(), first.d1.y()], expected[1], label);
    let second = curve.eval_d2(u).unwrap();
    assert_coordinate_pair([second.point.x(), second.point.y()], expected[0], label);
    assert_coordinate_pair([second.d1.x(), second.d1.y()], expected[1], label);
    assert_coordinate_pair([second.d2.x(), second.d2.y()], expected[2], label);
    for (order, pair) in expected.into_iter().enumerate().skip(1) {
        let derivative = curve.eval_dn(u, i32::try_from(order).unwrap()).unwrap();
        assert_coordinate_pair([derivative.x(), derivative.y()], pair, label);
    }
}

fn assert_coordinate_pair(actual: [f64; 2], expected: [f64; 2], label: &str) {
    for (axis, (actual_value, expected_value)) in actual.into_iter().zip(expected).enumerate() {
        let roundoff = 32.0 * f64::EPSILON * expected_value.abs().max(1.0);
        assert!(
            (actual_value - expected_value).abs() <= roundoff,
            "{label}: axis {axis}: {actual_value}, expected {expected_value} ±{roundoff}"
        );
    }
}

#[path = "support/analytic_reference_checks.rs"]
mod independent_reference_checks;
