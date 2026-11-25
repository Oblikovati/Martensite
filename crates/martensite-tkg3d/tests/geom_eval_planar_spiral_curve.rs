// SPDX-License-Identifier: GPL-2.0-only
//! Independent cardinal and derivative fixtures for #611.
use martensite_tkernel::precision;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomGeometry};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkg3d::geom_eval::{GeomEvalPlanarSpiralCurve, SpiralRotation};
use martensite_tkmath::geom_abs::{CurveType, Shape};
use martensite_tkmath::gp::{Axis2, Direction3, Point3, Transform3};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

fn spiral(
    radius: f64,
    pitch: f64,
    turns: f64,
    rotation: SpiralRotation,
) -> Result<GeomEvalPlanarSpiralCurve, Failure> {
    GeomEvalPlanarSpiralCurve::new(Axis2::default(), radius, pitch, turns, rotation)
}

fn outward(rotation: SpiralRotation) -> GeomEvalPlanarSpiralCurve {
    spiral(4., 2., 1., rotation).unwrap()
}

fn near(actual: &[f64; 3], expected: [f64; 3]) {
    for (&actual, expected) in actual.iter().zip(expected) {
        let tolerance = 4e-15 * expected.abs().max(1.);
        assert!(
            (actual - expected).abs() <= tolerance,
            "{actual:.17e} vs {expected:.17e}"
        );
    }
}

#[test]
fn owned_cardinals_keep_radial_pitch_independent_of_rotation() {
    for (rotation, sign) in [
        (SpiralRotation::Counterclockwise, 1.),
        (SpiralRotation::Clockwise, -1.),
    ] {
        let curve = outward(rotation);
        assert_eq!((curve.first_parameter(), curve.last_parameter()), (0., TAU));
        assert_eq!(
            (curve.initial_radius(), curve.radial_pitch(), curve.turns()),
            (4., 2., 1.)
        );
        assert_eq!(curve.rotation(), rotation);
        for (parameter, expected) in [
            (0., [4., 0., 0.]),
            (FRAC_PI_2, [0., sign * 4.5, 0.]),
            (PI, [-5., 0., 0.]),
            (3. * FRAC_PI_2, [0., -sign * 5.5, 0.]),
            (TAU, [6., 0., 0.]),
        ] {
            near(curve.value(parameter).unwrap().coordinates(), expected);
        }
    }
}

#[test]
fn derivatives_match_initial_closed_forms_and_native_dn() {
    for (rotation, sign) in [
        (SpiralRotation::Counterclockwise, 1.),
        (SpiralRotation::Clockwise, -1.),
    ] {
        let curve = outward(rotation);
        let at = curve.eval_d3(0.).unwrap();
        near(at.point.coordinates(), [4., 0., 0.]);
        near(at.d1.xyz().coordinates(), [1. / PI, sign * 4., 0.]);
        near(at.d2.xyz().coordinates(), [-4., sign * 2. / PI, 0.]);
        near(at.d3.xyz().coordinates(), [-3. / PI, -sign * 4., 0.]);
        let u = 0.7;
        let d3 = curve.eval_d3(u).unwrap();
        let d2 = curve.eval_d2(u).unwrap();
        let d1 = curve.eval_d1(u).unwrap();
        assert_eq!(d3.point, curve.eval_d0(u).unwrap());
        assert_eq!((d3.d1, d3.d2), (d2.d1, d2.d2));
        assert_eq!((d1.point, d1.d1), (d3.point, d3.d1));
        for (n, derivative) in [(1, d3.d1), (2, d3.d2), (3, d3.d3)] {
            assert_eq!(curve.dn(u, n).unwrap(), derivative);
        }
    }
}

#[test]
fn high_orders_use_modular_phase_without_loss_of_angle() {
    let u = FRAC_PI_2;
    let n = i32::MAX;
    let radial_rate_term = f64::from(n) / PI;
    for (rotation, sign) in [
        (SpiralRotation::Counterclockwise, 1.),
        (SpiralRotation::Clockwise, -1.),
    ] {
        let curve = outward(rotation);
        let high = curve.dn(0., n).unwrap();
        near(
            high.xyz().coordinates(),
            [-radial_rate_term, -sign * 4., 0.],
        );
        let odd = curve.dn(u, 5).unwrap();
        near(odd.xyz().coordinates(), [-4.5, sign * 5. / PI, 0.]);
    }
}

#[test]
fn native_curve_and_adaptor_evaluate_in_the_placed_right_handed_plane() {
    let axis = Axis2::with_x_direction(
        Point3::new(10., 20., 30.),
        Direction3::new(0., 3., 4.).unwrap(),
        Direction3::X,
    )
    .unwrap();
    let curve =
        GeomEvalPlanarSpiralCurve::new(axis, 4., 2., 1., SpiralRotation::Counterclockwise).unwrap();
    assert_eq!(curve.position().location(), axis.location());
    let native: Handle<dyn GeomCurve> = Handle::new(curve).into_upcast();
    let adaptor = GeomAdaptorCurve::with_curve(&native).unwrap();
    assert_eq!(adaptor.get_type().unwrap(), CurveType::OtherCurve);
    assert_eq!(
        (
            adaptor.first_parameter().unwrap(),
            adaptor.last_parameter().unwrap()
        ),
        (0., TAU)
    );
    near(
        adaptor.value(FRAC_PI_2).unwrap().coordinates(),
        [10., 23.6, 27.3],
    );
    let first = native.eval_d1(0.).unwrap();
    near(first.point.coordinates(), [14., 20., 30.]);
    near(first.d1.xyz().coordinates(), [1. / PI, 3.2, -2.4]);
    assert_eq!(adaptor.dn(0., 1).unwrap(), first.d1);
}

#[test]
fn inward_end_is_admitted_only_while_radius_stays_positive() {
    let rotation = SpiralRotation::Clockwise;
    let curve = spiral(4., -2., 1., rotation).unwrap();
    near(
        curve.value(FRAC_PI_2).unwrap().coordinates(),
        [0., -3.5, 0.],
    );
    near(curve.value(TAU).unwrap().coordinates(), [2., 0., 0.]);
    for pitch in [-4., -5.] {
        let failure = spiral(4., pitch, 1., rotation).unwrap_err();
        assert_eq!(failure.exception_type(), "Standard_ConstructionError");
        assert!(failure.message().contains("endpoint radius"));
    }
}

#[test]
fn authored_and_rounded_radian_limits_both_have_to_be_positive() {
    let rotation = SpiralRotation::Counterclockwise;
    // 3 + (-12)*0.25 is zero. The exact radian endpoint is strictly positive.
    let authored = spiral(3., -12., 0.25, rotation).unwrap_err();
    assert_eq!(authored.exception_type(), "Standard_ConstructionError");
    assert!(authored.message().contains("authored turns"));
    // Exact dyadic arithmetic gives +8.39e-19 by turns and -8.83e-20 by rounded radians.
    let radius = 0.03296703296703297;
    let pitch = -0.14285714285714285;
    let turns = 0.23076923076923078;
    let actual = spiral(radius, pitch, turns, rotation).unwrap_err();
    assert_eq!(actual.exception_type(), "Standard_ConstructionError");
    assert!(actual.message().contains("radian interval"));
}

#[test]
fn nonfinite_inputs_and_native_finite_parameter_limits_are_typed() {
    let rotation = SpiralRotation::Counterclockwise;
    for turns in [
        0.,
        -1.,
        f64::NAN,
        f64::INFINITY,
        precision::infinite() / TAU,
    ] {
        assert_eq!(
            spiral(4., 2., turns, rotation)
                .unwrap_err()
                .exception_type(),
            "Standard_RangeError"
        );
    }
    for radius in [0., -1., f64::NAN, f64::INFINITY] {
        assert_eq!(
            spiral(radius, 2., 1., rotation)
                .unwrap_err()
                .exception_type(),
            "Standard_ConstructionError"
        );
    }
    let invalid_pitch = spiral(4., f64::NAN, 1., rotation).unwrap_err();
    assert_eq!(invalid_pitch.exception_type(), "Standard_ConstructionError");
    let invalid_axis =
        Axis2::default().translated(martensite_tkmath::gp::Vector3::new(f64::NAN, 0., 0.));
    let invalid_frame =
        GeomEvalPlanarSpiralCurve::new(invalid_axis, 4., 2., 1., rotation).unwrap_err();
    assert_eq!(invalid_frame.exception_type(), "Standard_ConstructionError");
}

#[test]
fn evaluation_refusals_distinguish_parameter_order_and_world_overflow() {
    let rotation = SpiralRotation::Counterclockwise;
    let curve = outward(rotation);
    for u in [-1., f64::NAN, f64::INFINITY, TAU.next_up()] {
        assert_eq!(
            curve.eval_d0(u).unwrap_err().exception_type(),
            "Standard_RangeError"
        );
        assert_eq!(
            curve.eval_d3(u).unwrap_err().exception_type(),
            "Standard_RangeError"
        );
    }
    for n in [i32::MIN, 0] {
        let invalid_order = curve.eval_dn(0., n).unwrap_err();
        assert_eq!(invalid_order.exception_type(), "Standard_RangeError");
        assert!(invalid_order.message().contains(&n.to_string()));
    }
    let origin = Point3::new(f64::MAX, 0., 0.);
    let axis = Axis2::with_x_direction(origin, Direction3::Z, Direction3::X).unwrap();
    let displaced = GeomEvalPlanarSpiralCurve::new(axis, f64::MAX / 2., 0., 1., rotation).unwrap();
    assert_eq!(
        displaced.eval_d0(0.).unwrap_err().exception_type(),
        "Standard_Overflow"
    );
    assert_eq!(displaced.dn(0., 1).unwrap().y(), f64::MAX / 2.);
    assert_eq!(displaced.eval_d0(PI).unwrap().x(), f64::MAX / 2.);
}

#[test]
fn radial_overflow_underflow_and_requested_derivative_overflow_refuse() {
    let rotation = SpiralRotation::Counterclockwise;
    let tiny = f64::from_bits(1);
    for (radius, pitch, turns, expected) in [
        (4., 2., f64::MAX, "Standard_Overflow"),
        (f64::MAX, 7. * tiny, 1., "Standard_Overflow"),
        (4., tiny, 1., "Standard_Underflow"),
        (tiny, -7. * tiny, 0.1, "Standard_Underflow"),
    ] {
        let failure = spiral(radius, pitch, turns, rotation).unwrap_err();
        assert_eq!(failure.exception_type(), expected);
    }
    let curve = spiral(1., f64::MAX / 2., 0.125, rotation).unwrap();
    assert!(curve.eval_d3(0.).is_ok());
    let overflow = curve.eval_dn(0., i32::MAX).unwrap_err();
    assert_eq!(overflow.exception_type(), "Standard_Overflow");
}

#[test]
fn high_order_derivatives_handle_subnormal_slope_and_world_projection_limits() {
    let axis = Axis2::with_x_direction(
        Point3::default(),
        Direction3::Z,
        Direction3::new(3., 4., 0.).unwrap(),
    )
    .unwrap();
    let rotation = SpiralRotation::Counterclockwise;
    let tiny = f64::from_bits(1);
    let small = GeomEvalPlanarSpiralCurve::new(axis, tiny, 7. * tiny, 1., rotation).unwrap();
    let value = small.dn(0., 1_000_000).unwrap();
    assert!((value.x() / tiny - 800_000.).abs() <= 2.);
    assert!((value.y() / tiny + 600_000.).abs() <= 2.);

    let pitch = f64::MAX / 2.;
    let large = GeomEvalPlanarSpiralCurve::new(axis, 1., pitch, 0.125, rotation).unwrap();
    let value = large.dn(0., 13).unwrap();
    let scale = 13. / (2. * TAU);
    near(
        &value
            .xyz()
            .coordinates()
            .map(|coordinate| coordinate / f64::MAX),
        [0.6 * scale, 0.8 * scale, 0.],
    );
}

#[test]
fn immutable_eval_curve_traits_copy_and_keep_finite_bounds() {
    let curve = outward(SpiralRotation::Counterclockwise);
    let cloned: Handle<GeomEvalPlanarSpiralCurve> = curve.copy().unwrap().cast();
    assert!(!cloned.is_null());
    assert_eq!(curve.eval_d3(0.4).unwrap(), cloned.eval_d3(0.4).unwrap());
    assert_eq!(cloned.last_parameter(), TAU);
    assert_eq!(curve.continuity(), Shape::CN);
    assert!(curve.is_cn(i32::MAX).unwrap());
    assert!(!curve.is_periodic());
    assert!(!curve.is_closed().unwrap());
    for failure in [curve.reverse(), curve.transform(Transform3::default())] {
        assert_eq!(
            failure.unwrap_err().exception_type(),
            "Standard_NotImplemented"
        );
    }
    assert_eq!(
        curve.reversed_parameter(0.).unwrap_err().exception_type(),
        "Standard_NotImplemented"
    );
    curve.dump_json(&mut DumpStream::default(), -1).unwrap();
}

#[test]
fn zero_pitch_supports_both_circle_arcs_and_closed_full_turn_spans() {
    let rotation = SpiralRotation::Counterclockwise;
    let full = spiral(4., 0., 1., rotation).unwrap();
    assert!(full.is_closed().unwrap());
    assert!(!full.is_periodic());
    let half = spiral(4., 0., 0.5, rotation).unwrap();
    assert!(!half.is_closed().unwrap());
    near(half.value(PI).unwrap().coordinates(), [-4., 0., 0.]);
}

#[test]
fn immutable_spiral_output_replays_across_threads() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<(GeomEvalPlanarSpiralCurve, SpiralRotation)>();
    let curve = outward(SpiralRotation::Clockwise);
    let expected = replay_values(&curve);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..32 {
                    assert_eq!(replay_values(&curve), expected);
                }
            });
        }
    });
    println!("spiral-replay {expected:016x?}");
}

fn replay_values(curve: &GeomEvalPlanarSpiralCurve) -> [[u64; 6]; 5] {
    [0., FRAC_PI_2, PI, 3. * FRAC_PI_2, TAU].map(|u| {
        let result = curve.eval_d3(u).unwrap();
        let [x, y, z] = result.point.coordinates().map(f64::to_bits);
        let [dx, dy, dz] = result.d3.xyz().coordinates().map(f64::to_bits);
        [x, y, z, dx, dy, dz]
    })
}
