//! Reading one binary `Geom_Curve` record back.
use super::CurveFrameReadPolicy;
use super::curve_write::code;
use super::errors::wrap_failure;
use super::geometry_in::{read_degree, read_direction3, read_knots, read_point3, read_poles};
use super::scalars::{get_bool, get_integer, get_real, stream_get};
use martensite_tkernel::standard::{Failure, Handle, HandleUpcast};
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomCircle, GeomCurve, GeomEllipse, GeomHyperbola, GeomLine,
    GeomOffsetCurve, GeomParabola, GeomTrimmedCurve,
};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};

/// `BinTools_CurveSet::ReadCurve`: the type code byte, then the record of that class.
/// An unknown code raises "UNKNOWN CURVE TYPE"; every failure (a short read, a null
/// direction, a refused construction) is wrapped in "EXCEPTION in
/// BinTools_CurveSet::ReadCurve(..)" with the stream left where it happened.
pub(crate) fn read_curve(stream: &mut IoStream) -> Result<Handle<dyn GeomCurve>, Failure> {
    read_curve_with_frame_policy(stream, CurveFrameReadPolicy::LegacyReplay)
}

pub(crate) fn read_curve_with_frame_policy(
    stream: &mut IoStream,
    policy: CurveFrameReadPolicy,
) -> Result<Handle<dyn GeomCurve>, Failure> {
    wrap_failure("EXCEPTION in BinTools_CurveSet::ReadCurve(..)", || {
        let ctype = stream_get(stream) as u8;
        read_record(ctype, stream, policy)
    })
}

fn read_record(
    ctype: u8,
    stream: &mut IoStream,
    policy: CurveFrameReadPolicy,
) -> Result<Handle<dyn GeomCurve>, Failure> {
    match ctype {
        code::LINE => read_line(stream, policy).map(upcast),
        code::CIRCLE => read_circle(stream, policy).map(upcast),
        code::ELLIPSE => read_ellipse(stream, policy).map(upcast),
        code::PARABOLA => read_parabola(stream, policy).map(upcast),
        code::HYPERBOLA => read_hyperbola(stream, policy).map(upcast),
        code::BEZIER => read_bezier(stream).map(upcast),
        code::BSPLINE => read_bspline(stream).map(upcast),
        code::TRIMMED => read_trimmed(stream, policy).map(upcast),
        code::OFFSET => read_offset(stream, policy).map(upcast),
        _ => Err(Failure::new("UNKNOWN CURVE TYPE")),
    }
}

/// `C = CC;`.
fn upcast<C: HandleUpcast<dyn GeomCurve>>(curve: C) -> Handle<dyn GeomCurve> {
    Handle::new(curve).into_upcast()
}

fn read_line(stream: &mut IoStream, policy: CurveFrameReadPolicy) -> Result<GeomLine, Failure> {
    let point = read_point3(stream)?;
    let direction = read_stored_direction(stream, policy)?;
    Ok(GeomLine::from_point(point, direction))
}

fn read_stored_direction(
    stream: &mut IoStream,
    policy: CurveFrameReadPolicy,
) -> Result<Direction3, Failure> {
    if policy == CurveFrameReadPolicy::LegacyReplay {
        return read_direction3(stream);
    }
    Direction3::from_stored_coefficients(*read_point3(stream)?.coordinates())
}

/// `IS >> P >> A >> AX >> AY`: the conic frame. The Y direction is read (and must not
/// be null) but dropped, since `gp_Ax2(P, A, AX)` recomputes it.
fn read_conic_frame(stream: &mut IoStream) -> Result<(Point3, Direction3, Direction3), Failure> {
    let point = read_point3(stream)?;
    let main = read_direction3(stream)?;
    let x = read_direction3(stream)?;
    read_direction3(stream)?;
    Ok((point, main, x))
}

enum ConicFrame {
    Legacy(Point3, Direction3, Direction3),
    Checked(Axis2),
}
fn read_axis(stream: &mut IoStream, policy: CurveFrameReadPolicy) -> Result<ConicFrame, Failure> {
    if policy == CurveFrameReadPolicy::LegacyReplay {
        let (point, main, x) = read_conic_frame(stream)?;
        return Ok(ConicFrame::Legacy(point, main, x));
    }
    let origin = read_point3(stream)?;
    let main = *read_point3(stream)?.coordinates();
    let x = *read_point3(stream)?.coordinates();
    let y = *read_point3(stream)?.coordinates();
    Axis2::from_stored_coefficients(origin, main, x, y).map(ConicFrame::Checked)
}

/// `gp_Ax2(P, A, AX)`, built after the radii are read, as natively.
fn conic_axis(frame: ConicFrame) -> Result<Axis2, Failure> {
    match frame {
        ConicFrame::Legacy(point, main, x) => Axis2::with_x_direction(point, main, x),
        ConicFrame::Checked(axis) => Ok(axis),
    }
}

fn read_circle(stream: &mut IoStream, policy: CurveFrameReadPolicy) -> Result<GeomCircle, Failure> {
    let frame = read_axis(stream, policy)?;
    let radius = get_real(stream)?;
    GeomCircle::new(conic_axis(frame)?, radius)
}

fn read_ellipse(
    stream: &mut IoStream,
    policy: CurveFrameReadPolicy,
) -> Result<GeomEllipse, Failure> {
    let frame = read_axis(stream, policy)?;
    let (major, minor) = (get_real(stream)?, get_real(stream)?);
    GeomEllipse::new(conic_axis(frame)?, major, minor)
}

fn read_parabola(
    stream: &mut IoStream,
    policy: CurveFrameReadPolicy,
) -> Result<GeomParabola, Failure> {
    let frame = read_axis(stream, policy)?;
    let focal = get_real(stream)?;
    GeomParabola::new(conic_axis(frame)?, focal)
}

fn read_hyperbola(
    stream: &mut IoStream,
    policy: CurveFrameReadPolicy,
) -> Result<GeomHyperbola, Failure> {
    let frame = read_axis(stream, policy)?;
    let (major, minor) = (get_real(stream)?, get_real(stream)?);
    GeomHyperbola::new(conic_axis(frame)?, major, minor)
}

/// The rational flag, the `char16_t` degree, then `degree + 1` poles with their weights.
fn read_bezier(stream: &mut IoStream) -> Result<GeomBezierCurve, Failure> {
    let rational = get_bool(stream);
    let degree = read_degree(stream)?;
    let (poles, weights) = read_poles(stream, degree + 1, rational, read_point3)?;
    if rational {
        GeomBezierCurve::with_weights(&poles, &weights)
    } else {
        GeomBezierCurve::new(&poles)
    }
}

/// The flags, the degree and the counts, the poles with their weights, then each knot
/// with its multiplicity.
fn read_bspline(stream: &mut IoStream) -> Result<GeomBSplineCurve, Failure> {
    let (rational, periodic) = (get_bool(stream), get_bool(stream));
    let degree = read_degree(stream)?;
    let nb_poles = get_integer(stream)?;
    let nb_knots = get_integer(stream)?;
    let (poles, weights) = read_poles(stream, nb_poles, rational, read_point3)?;
    let (knots, mults) = read_knots(stream, nb_knots)?;
    if rational {
        GeomBSplineCurve::with_weights(&poles, &weights, &knots, &mults, degree, periodic)
    } else {
        GeomBSplineCurve::new(&poles, &knots, &mults, degree, periodic)
    }
}

/// The parameters, then the basis record; `Geom_TrimmedCurve(BC, p1, p2)` with its
/// default sense and periodic adjustment.
fn read_trimmed(
    stream: &mut IoStream,
    policy: CurveFrameReadPolicy,
) -> Result<GeomTrimmedCurve, Failure> {
    let (first, last) = (get_real(stream)?, get_real(stream)?);
    let basis = read_curve_with_frame_policy(stream, policy)?;
    GeomTrimmedCurve::new(&basis, first, last, true, true)
}

/// The offset, the reference direction, then the basis record; the basis continuity
/// is checked (`isNotCheckC0` defaults to false).
fn read_offset(
    stream: &mut IoStream,
    policy: CurveFrameReadPolicy,
) -> Result<GeomOffsetCurve, Failure> {
    let offset = get_real(stream)?;
    let direction = read_stored_direction(stream, policy)?;
    let basis = read_curve_with_frame_policy(stream, policy)?;
    GeomOffsetCurve::new(&basis, offset, direction, false)
}
