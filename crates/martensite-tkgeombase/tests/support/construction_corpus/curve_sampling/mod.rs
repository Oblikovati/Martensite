//! The CPnts and GCPnts probe; the counterpart of the implementation
//! (same request layout, member order and token stream).
// The printers run inside `attempt`, whose closure returns `Ok(printer(...))`.
#![allow(clippy::unit_arg)]
use super::{geometry_fixtures as build, print::Output};
mod distributions;

pub use distributions::{
    arc_step_case, dist_case, quabs_case, qudefl_case, tdefl_case, uabs_case, udefl_case,
};

use super::common::{Reader, attempt, end, flag, int, pnt, real, show};
use martensite_tkgeombase::cpnts::{self, PntsCurve};
use martensite_tkgeombase::gcpnts::{AbscissaPoint, TCurveTypes};

/// gb_gcp_curve: build the adaptor (dimension, range flag and bounds, curve spec) and
/// call `$run(&adaptor, &mut reader)`; a construction failure prints its type.
macro_rules! gcp_case {
    ($output:expr, $values:expr, $run:path) => {{
        use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
        use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
        let mut r = Reader::new($values);
        let (dim, ranged) = (r.integer(), r.flag());
        let (u1, u2) = if ranged {
            (r.real(), r.real())
        } else {
            (0.0, 0.0)
        };
        attempt($output, || {
            if dim == 2 {
                let c = build::curve2d(&mut r)?;
                let a = if ranged {
                    Geom2dAdaptorCurve::with_range(&c, u1, u2)?
                } else {
                    Geom2dAdaptorCurve::with_curve(&c)?
                };
                $run($output, &a, &mut r);
                return Ok(());
            }
            let c = build::curve3d(&mut r)?;
            let a = if ranged {
                GeomAdaptorCurve::with_range(&c, u1, u2)?
            } else {
                GeomAdaptorCurve::with_curve(&c)?
            };
            $run($output, &a, &mut r);
            Ok(())
        });
        end($output);
    }};
}
pub(crate) use gcp_case;

fn length_run<A: TCurveTypes>(output: &Output, a: &A, r: &mut Reader) {
    let (u1, u2, tol) = (r.real(), r.real(), r.real());
    let c: PntsCurve = a.pnts_curve();
    show(output, cpnts::AbscissaPoint::length(c), |value| {
        real(output, value)
    });
    show(
        output,
        cpnts::AbscissaPoint::length_with_tolerance(c, tol),
        |value| real(output, value),
    );
    show(
        output,
        cpnts::AbscissaPoint::length_between(c, u1, u2),
        |value| real(output, value),
    );
    show(
        output,
        cpnts::AbscissaPoint::length_between_with_tolerance(c, u1, u2, tol),
        |value| real(output, value),
    );
    show(output, AbscissaPoint::length(a), |value| {
        real(output, value)
    });
    show(
        output,
        AbscissaPoint::length_with_tolerance(a, tol),
        |value| real(output, value),
    );
    show(output, AbscissaPoint::length_between(a, u1, u2), |value| {
        real(output, value)
    });
    show(
        output,
        AbscissaPoint::length_between_with_tolerance(a, u1, u2, tol),
        |value| real(output, value),
    );
}

/// gcp_length.
pub fn length_case(output: &Output, values: &[f64]) {
    gcp_case!(output, values, length_run);
}

fn cpnts_point(output: &Output, p: &cpnts::AbscissaPoint) {
    flag(output, p.is_done());
    show(output, p.parameter(), |value| real(output, value));
}

fn cpnts_abscissa_run<A: TCurveTypes>(output: &Output, a: &A, r: &mut Reader) {
    let (ab, u0, ui, res) = (r.real(), r.real(), r.real(), r.real());
    let (u1, u2, tol) = (r.real(), r.real(), r.real());
    let c: PntsCurve = a.pnts_curve();
    attempt(output, || {
        Ok(cpnts_point(
            output,
            &cpnts::AbscissaPoint::with_resolution(c, ab, u0, res)?,
        ))
    });
    attempt(output, || {
        Ok(cpnts_point(
            output,
            &cpnts::AbscissaPoint::with_start(c, ab, u0, ui, res)?,
        ))
    });
    let mut p = cpnts::AbscissaPoint::new();
    cpnts_point(output, &p);
    attempt(output, || {
        p.init(c)?;
        p.perform(ab, u0, res)?;
        Ok(cpnts_point(output, &p))
    });
    attempt(output, || {
        p.init_with_tolerance(c, tol)?;
        p.adv_perform(ab, u0, ui, res)?;
        Ok(cpnts_point(output, &p))
    });
    attempt(output, || {
        p.init_between(c, u1, u2)?;
        p.perform_from(ab, u0, ui, res)?;
        Ok(cpnts_point(output, &p))
    });
    attempt(output, || {
        p.init_between_with_tolerance(c, u1, u2, tol)?;
        p.adv_perform(ab, u0, ui, res)?;
        Ok(cpnts_point(output, &p))
    });
    p.set_parameter(u1);
    cpnts_point(output, &p);
}

/// cpnts_abscissa.
pub fn cpnts_abscissa_case(output: &Output, values: &[f64]) {
    gcp_case!(output, values, cpnts_abscissa_run);
}

fn cpnts_iterate(output: &Output, it: &mut cpnts::UniformDeflection, cap: i32) {
    flag(output, it.is_all_done());
    let mut n: i32 = 0;
    attempt(output, || {
        while it.more()? && n < cap {
            real(output, it.value()?);
            pnt(output, it.point()?);
            n += 1;
            it.next()?;
        }
        Ok(())
    });
    int(output, n);
}

fn cpnts_udefl_run<A: TCurveTypes>(output: &Output, a: &A, r: &mut Reader) {
    let (d, res, ctl, u1, u2, cap) = (
        r.real(),
        r.real(),
        r.flag(),
        r.real(),
        r.real(),
        r.integer(),
    );
    let c: PntsCurve = a.pnts_curve();
    attempt(output, || {
        Ok(cpnts_iterate(
            output,
            &mut cpnts::UniformDeflection::with_curve(c, d, res, ctl)?,
            cap,
        ))
    });
    attempt(output, || {
        let mut it = cpnts::UniformDeflection::with_range(c, d, u1, u2, res, ctl)?;
        Ok(cpnts_iterate(output, &mut it, cap))
    });
    let mut it = cpnts::UniformDeflection::new();
    flag(output, it.is_all_done());
    show(output, it.more(), |value| flag(output, value));
    show(output, it.value(), |value| real(output, value));
    attempt(output, || it.next());
    attempt(output, || {
        it.initialize_range(c, d, u1, u2, res, ctl)?;
        Ok(cpnts_iterate(output, &mut it, cap))
    });
    attempt(output, || {
        it.initialize(c, d, res, ctl)?;
        Ok(cpnts_iterate(output, &mut it, cap))
    });
}

/// cpnts_udefl.
pub fn cpnts_udefl_case(output: &Output, values: &[f64]) {
    gcp_case!(output, values, cpnts_udefl_run);
}

fn gcp_point(output: &Output, p: &AbscissaPoint) {
    flag(output, p.is_done());
    show(output, p.parameter(), |value| real(output, value));
}

fn gcp_abscissa_run<A: TCurveTypes>(output: &Output, a: &A, r: &mut Reader) {
    let (ab, u0, ui, tol) = (r.real(), r.real(), r.real(), r.real());
    gcp_point(output, &AbscissaPoint::new());
    attempt(output, || {
        Ok(gcp_point(output, &AbscissaPoint::from_abscissa(a, ab, u0)?))
    });
    attempt(output, || {
        Ok(gcp_point(
            output,
            &AbscissaPoint::with_tolerance(tol, a, ab, u0)?,
        ))
    });
    attempt(output, || {
        Ok(gcp_point(
            output,
            &AbscissaPoint::from_start(a, ab, u0, ui)?,
        ))
    });
    attempt(output, || {
        Ok(gcp_point(
            output,
            &AbscissaPoint::from_start_with_tolerance(a, ab, u0, ui, tol)?,
        ))
    });
}

/// gcp_abscissa.
pub fn gcp_abscissa_case(output: &Output, values: &[f64]) {
    gcp_case!(output, values, gcp_abscissa_run);
}
