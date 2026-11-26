// `GetDefType` and `PerformComposite`, repeated with the same body in

// the implementation, at .

//! The distribution steps shared by `UniformDeflection` and
//! `QuasiUniformDeflection`.
use super::intervals::{bound, hunt_interval, interval_bounds};
use super::point_sequence::PointSequence;
use super::t_curve_types::live;
use super::{DeflectionType, PoleCurve, TCurveTypes};
use crate::cpnts::PntsCurve;
use martensite_tkernel::ncollection::ElementCast;
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::native::{std_max, std_min};
use martensite_tkmath::geom_abs::{CurveType, Shape};

/// `GetDefType`: composite above one interval of continuity `shape` (C2 for the
/// uniform, C1 for the quasi-uniform deflection), linear for lines and two-pole
/// curves (rational or not), circular for circles; `def_type(curve, Shape::C2)?`.
pub(super) fn def_type<C: TCurveTypes + ?Sized>(
    curve: &C,
    shape: Shape,
) -> Result<DeflectionType, Failure> {
    let pnts: PntsCurve = curve.pnts_curve();
    if pnts.nb_intervals(shape)? > 1 {
        return Ok(DeflectionType::DefComposite);
    }
    Ok(match pnts.get_type()? {
        CurveType::Line => DeflectionType::Linear,
        CurveType::Circle => DeflectionType::Circular,
        CurveType::BSplineCurve => two_pole_type(live(&curve.bspline_curve()?)?.pole_count()),
        CurveType::BezierCurve => two_pole_type(live(&curve.bezier_curve()?)?.pole_count()),
        _ => DeflectionType::Curved,
    })
}

fn two_pole_type(nb_poles: i32) -> DeflectionType {
    if nb_poles == 2 {
        DeflectionType::Linear
    } else {
        DeflectionType::Curved
    }
}

/// `PerformLinear`: the two ends; always true; `perform_linear(curve, &mut seq, 0.0, 1.0)?`.
pub(super) fn perform_linear(
    curve: PntsCurve<'_>,
    sequence: &mut PointSequence,
    u1: f64,
    u2: f64,
) -> Result<bool, Failure> {
    sequence.append(u1, curve.lifted_d0(u1)?);
    sequence.append(u2, curve.lifted_d0(u2)?);
    Ok(true)
}

/// `PerformCircular`: constant steps of the angle `2 acos(max(1 - Deflection / R, 0))`
/// rounded down to fit `[U1, U2]` plus two points; the last parameter is accumulated,
/// not set to U2; always true; `perform_circular(curve, &mut seq, 0.01, 0.0, 1.0)?`.
///
/// A null deflection makes the retained `(int)` of an infinite count undefined; x86-64
/// gives INT_MIN, so no point is stored, as here.
pub(super) fn perform_circular(
    curve: PntsCurve<'_>,
    sequence: &mut PointSequence,
    deflection: f64,
    (u1, u2): (f64, f64),
) -> Result<bool, Failure> {
    let mut angle: f64 = std_max(1.0 - (deflection / curve.circle_radius()?), 0.0);
    angle = 2.0e0 * martensite_cmath::acos(angle);
    let mut nb_points: i32 = ElementCast::<i32>::cast((u2 - u1) / angle);
    nb_points = nb_points.wrapping_add(2);
    angle = (u2 - u1) / f64::from(nb_points.wrapping_sub(1));
    let (lo, hi) = if u1 <= u2 { (u1, u2) } else { (u2, u1) };
    let mut u: f64 = u1;
    for step in 0..nb_points {
        // #530: anchor exact endpoints and preserve OCCT's interior accumulation
        // clamped to [lo, hi] so rounding never steps outside requested bounds.
        let sample = if step == 0 {
            u1
        } else if step + 1 == nb_points {
            u2
        } else {
            u.clamp(lo, hi)
        };
        sequence.append(sample, curve.lifted_d0(sample)?);
        u += angle;
    }
    Ok(true)
}

/// `PerformComposite`: `perform_curve` on each C2 interval met from U1 to U2 (the
/// retained files both use C2 here), the duplicated junction point removed;
/// `perform_composite(curve, &mut seq, (0.0, 1.0), &mut |seq, a, b| ...)?`.
pub(super) fn perform_composite(
    curve: PntsCurve<'_>,
    sequence: &mut PointSequence,
    (u1, u2): (f64, f64),
    perform_curve: &mut dyn FnMut(&mut PointSequence, f64, f64) -> Result<bool, Failure>,
) -> Result<bool, Failure> {
    let ti = interval_bounds(curve, Shape::C2)?;
    let nb_intervals: i32 = ti.upper() - 1;
    let mut index: i32 = hunt_interval(&ti, u1)?;
    // iterate by continuous segments
    let mut ua: f64 = u1;
    loop {
        let ub: f64 = if index < ti.upper() {
            std_min(u2, bound(&ti, index + 1)?)
        } else {
            u2
        };
        if !perform_curve(sequence, ua, ub)? {
            return Ok(false);
        }
        index += 1;
        if index > nb_intervals || u2 < bound(&ti, index)? {
            return Ok(true);
        }
        // remove last point to avoid duplication
        sequence.remove_last()?;
        ua = ub;
    }
}

#[cfg(test)]
mod circular_endpoint_tests {
    use crate::gcpnts::QuasiUniformDeflection;
    use martensite_tkernel::standard::Handle;
    use martensite_tkg3d::geom::{GeomCircle, GeomCurve};
    use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
    use martensite_tkmath::geom_abs::Shape;
    use martensite_tkmath::gp::{Axis2, Direction3, Point3};

    #[test]
    fn circular_deflection_anchors_exact_endpoints_for_positive_and_negative_ranges() {
        let axis = Axis2::new(
            Point3::new(0.0, 0.0, 0.0),
            Direction3::new(0.0, 0.0, 1.0).unwrap(),
        )
        .unwrap();
        let geom: Handle<dyn GeomCurve> = Handle::new(GeomCircle::new(axis, 2.0).unwrap()).cast();
        let adaptor = GeomAdaptorCurve::with_curve(&geom).unwrap();

        let pi_up = std::f64::consts::PI.next_up();
        let half_pi = std::f64::consts::FRAC_PI_2;
        let ranges = [
            (half_pi, pi_up, 0.1),
            (0.0, std::f64::consts::TAU, 0.03),
            (-pi_up, -half_pi, 0.1),
            (-1.0, 2.3, 0.07),
        ];
        for (u1, u2, deflection) in ranges {
            let sampling =
                QuasiUniformDeflection::with_range(&adaptor, deflection, u1, u2, Shape::C1)
                    .unwrap();
            assert!(sampling.is_done());
            let count = sampling.nb_points().unwrap();
            assert!(count >= 2);
            assert_eq!(sampling.parameter(1).unwrap(), u1);
            assert_eq!(sampling.parameter(count).unwrap(), u2);
            let mut prev = u1;
            for i in 1..=count {
                let p = sampling.parameter(i).unwrap();
                assert!(
                    p >= u1 && p <= u2,
                    "sample {i}/{count} parameter {p} outside [{u1}, {u2}]"
                );
                assert!(p >= prev, "non-monotone sample {p} < {prev}");
                prev = p;
            }
        }
    }
}
