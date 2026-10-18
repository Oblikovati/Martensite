//! The Rust twin of the ComputeApprox, ComputeApproxOnPolarSurface and
//! ProjectedCurve operations of the implementation.
use super::common::{self as p, Reader, attempt, end};
use super::geom_convert::curve2d as print_curve2d;
use super::geom2d_convert::{bezier as bezier2d, bspline as bspline2d};
use super::geometry_adaptor_fixtures::{adapted_curve, adapted_curve2d, adapted_surface};
use super::print::Output;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkgeombase::app_par_curves::Constraint;
use martensite_tkgeombase::proj_lib::{
    ComputeApprox, ComputeApproxOnPolarSurface, ProjectedCurve, make_pcurve_of_type,
};
use martensite_tkmath::geom_abs::{CurveType, Shape};

/// `GbPlOptions`: `tol degmin degmax maxseg bndpnt maxdist`.
pub struct Options {
    tol: f64,
    degmin: i32,
    degmax: i32,
    maxseg: i32,
    bnd: Constraint,
    maxdist: f64,
}

fn constraint(code: i32) -> Constraint {
    match code {
        0 => Constraint::NoConstraint,
        1 => Constraint::PassPoint,
        2 => Constraint::TangencyPoint,
        _ => Constraint::CurvaturePoint,
    }
}

fn options(r: &mut Reader<'_>) -> Options {
    Options {
        tol: r.real(),
        degmin: r.integer(),
        degmax: r.integer(),
        maxseg: r.integer(),
        bnd: constraint(r.integer()),
        maxdist: r.real(),
    }
}

type Inputs = (Handle<dyn Adaptor3dCurve>, Handle<dyn Adaptor3dSurface>);

fn inputs(r: &mut Reader<'_>) -> Result<Inputs, Failure> {
    let c = adapted_curve(r)?;
    Ok((c, adapted_surface(r)?))
}

pub fn approx_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let variant = r.integer();
    let o = options(&mut r);
    attempt(output, || {
        let (c, s) = inputs(&mut r)?;
        let a = if variant == 1 {
            ComputeApprox::with_curve(&c, &s, o.tol)?
        } else {
            let mut a = ComputeApprox::new();
            a.set_tolerance(o.tol);
            a.set_degree(o.degmin, o.degmax);
            a.set_max_segments(o.maxseg);
            a.set_bnd_pnt(o.bnd);
            a.perform(&c, &s)?;
            a
        };
        bspline2d(output, &a.bspline());
        bezier2d(output, &a.bezier());
        p::real(output, a.tolerance());
        Ok(())
    });
    end(output);
}

fn polar_results(output: &Output, x: &ComputeApproxOnPolarSurface) {
    p::flag(output, x.is_done());
    bspline2d(output, &x.bspline());
    p::show(output, x.curve2d(), |c| print_curve2d(output, &c));
    p::real(output, x.tolerance());
}

fn polar_setup(x: &mut ComputeApproxOnPolarSurface, o: &Options) {
    x.set_tolerance(o.tol);
    x.set_degree(o.degmin, o.degmax);
    x.set_max_segments(o.maxseg);
    x.set_bnd_pnt(o.bnd);
    x.set_max_dist(o.maxdist);
}

fn polar_variant(
    output: &Output,
    variant: i32,
    o: &Options,
    r: &mut Reader<'_>,
    (c, s): Inputs,
) -> Result<(), Failure> {
    match variant {
        0 => {
            let mut x = ComputeApproxOnPolarSurface::new();
            polar_setup(&mut x, o);
            x.perform(&c, &s)?;
            polar_results(output, &x);
        }
        1 => polar_results(
            output,
            &ComputeApproxOnPolarSurface::with_curve(&c, &s, o.tol)?,
        ),
        2 => {
            let mut x = ComputeApproxOnPolarSurface::new();
            polar_setup(&mut x, o);
            let init = x.build_initial_curve2d(&c, &s)?;
            p::flag(output, init.is_null());
            polar_results(output, &x);
            if !init.is_null() {
                bspline2d(output, &x.project_using_initial_curve2d(&c, &s, &init)?);
                polar_results(output, &x);
            }
        }
        3 => {
            let init: Handle<dyn Adaptor2dCurve2d> = adapted_curve2d(r)?;
            polar_results(
                output,
                &ComputeApproxOnPolarSurface::with_initial_curve(&init, &c, &s, o.tol)?,
            );
        }
        4 => {
            let init: Handle<dyn Adaptor2dCurve2d> = adapted_curve2d(r)?;
            let bis: Handle<dyn Adaptor2dCurve2d> = adapted_curve2d(r)?;
            polar_results(
                output,
                &ComputeApproxOnPolarSurface::with_seam_curves(&init, &bis, &c, &s, o.tol)?,
            );
        }
        _ => {
            let init: Handle<dyn Adaptor2dCurve2d> = adapted_curve2d(r)?;
            let mut x = ComputeApproxOnPolarSurface::new();
            polar_setup(&mut x, o);
            bspline2d(output, &x.perform_with_initial(&init, &c, &s)?);
            polar_results(output, &x);
        }
    }
    Ok(())
}

pub fn polar_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let variant = r.integer();
    let o = options(&mut r);
    attempt(output, || {
        let ins = inputs(&mut r)?;
        polar_variant(output, variant, &o, &mut r, ins)
    });
    end(output);
}

fn projected_typed(output: &Output, x: &ProjectedCurve) {
    p::show(output, x.line(), |value| p::lin2d(output, value));
    p::show(output, x.circle(), |value| p::circ2d(output, value));
    p::show(output, x.ellipse(), |value| p::elips2d(output, value));
    p::show(output, x.hyperbola(), |value| p::hypr2d(output, value));
    p::show(output, x.parabola(), |value| p::parab2d(output, value));
    let kind = x.get_type().expect("probe type");
    let null_spline = (kind == CurveType::BSplineCurve && x.bspline().expect("probe").is_null())
        || (kind == CurveType::BezierCurve && x.bezier().expect("probe").is_null());
    p::flag(output, null_spline);
    if !null_spline {
        p::show(output, x.degree(), |value| p::int(output, value));
        p::show(output, x.is_rational(), |value| p::flag(output, value));
        p::show(output, x.nb_poles(), |value| p::int(output, value));
        p::show(output, x.nb_knots(), |value| p::int(output, value));
    }
}

/// gb_pl_projected_results.
fn projected_results(output: &Output, x: &ProjectedCurve) -> Result<(), Failure> {
    p::int(output, x.get_type()? as i32);
    p::real(output, x.get_tolerance());
    p::flag(output, x.is_periodic()?);
    p::show(output, x.first_parameter(), |value| p::real(output, value));
    p::show(output, x.last_parameter(), |value| p::real(output, value));
    projected_typed(output, x);
    bspline2d(output, &x.bspline()?);
    bezier2d(output, &x.bezier()?);
    p::show(output, make_pcurve_of_type(x), |c| {
        print_curve2d(output, &c)
    });
    p::show(output, x.continuity(), |s| p::int(output, s as i32));
    p::show(output, x.nb_intervals(Shape::C1), |value| {
        p::int(output, value)
    });
    p::show(output, x.is_closed(), |value| p::flag(output, value));
    p::show(output, x.period(), |value| p::real(output, value));
    p::show(output, x.resolution(1.0e-3), |value| p::real(output, value));
    attempt(output, || x.trim(0., 1., 1.0e-7).map(|_| ()));
    attempt(output, || {
        p::pnt2d(output, x.value(x.first_parameter()?)?);
        Ok(())
    });
    let curve = x.get_curve();
    p::flag(output, curve.is_null());
    if let Some(c) = curve.get() {
        p::real(output, c.first_parameter()?);
        p::real(output, c.last_parameter()?);
    }
    attempt(output, || {
        p::int(output, x.shallow_copy()?.get_type()? as i32);
        Ok(())
    });
    Ok(())
}

pub fn projected_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let variant = r.integer();
    let o = options(&mut r);
    attempt(output, || {
        let (c, s) = inputs(&mut r)?;
        let x = match variant {
            0 => {
                let x = ProjectedCurve::new();
                x.load_surface(&s);
                x.load_tolerance(o.tol);
                x.set_degree(o.degmin, o.degmax);
                x.set_max_segments(o.maxseg);
                x.set_bnd_pnt(o.bnd);
                x.set_max_dist(o.maxdist);
                x.perform(&c)?;
                x
            }
            1 => ProjectedCurve::with_curve(&s, &c)?,
            _ => ProjectedCurve::with_tolerance(&s, &c, o.tol)?,
        };
        projected_results(output, &x)
    });
    end(output);
}
