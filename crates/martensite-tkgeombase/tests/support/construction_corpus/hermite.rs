//! The Hermit probe; the counterpart of the implementation.
use super::common::{self as p, Reader, attempt, end};
use super::geometry_fixtures as build;
use super::print::Output;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dBSplineCurve;
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkgeombase::hermit;

fn curve(output: &Output, c: &Handle<Geom2dBSplineCurve>) -> Result<(), Failure> {
    p::int(output, c.degree());
    p::int(output, c.nb_poles());
    for i in 1..=c.nb_poles() {
        p::pnt2d(output, c.pole(i)?);
    }
    p::int(output, c.nb_knots());
    for i in 1..=c.nb_knots() {
        p::real(output, c.knot(i)?);
        p::int(output, c.multiplicity(i)?);
    }
    Ok(())
}

pub fn hermit_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (mode, tp, tk, kmin, kmax) = (r.integer(), r.real(), r.real(), r.real(), r.real());
    attempt(output, || {
        if mode == 1 {
            let c: Handle<Geom2dBSplineCurve> = build::curve2d(&mut r)?.cast();
            return curve(output, &hermit::solution_2d(&c, tp, tk)?);
        }
        let c: Handle<GeomBSplineCurve> = build::curve3d(&mut r)?.cast();
        if mode == 0 {
            return curve(output, &hermit::solution_3d(&c, tp, tk)?);
        }
        let (mut a, mut b) = (kmin, kmax);
        hermit::solutionbis(&c, &mut a, &mut b, tp, tk)?;
        p::real(output, a);
        p::real(output, b);
        Ok(())
    });
    end(output);
}
