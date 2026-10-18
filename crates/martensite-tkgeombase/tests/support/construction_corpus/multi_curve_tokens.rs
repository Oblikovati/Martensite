//! One fallible multi-point/curve formatter preserves count and partial-output order.
use super::{common as p, print::Output};
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::app_par_curves::{MultiCurve, MultiCurveEval, MultiPoint};

pub fn point(output: &Output, m: &MultiPoint) -> Result<(), Failure> {
    p::int(output, m.nb_points());
    p::int(output, m.nb_points2d());
    for i in 1..=m.nb_points() {
        p::pnt(output, m.point(i)?);
    }
    for i in m.nb_points() + 1..=m.nb_points() + m.nb_points2d() {
        p::pnt2d(output, m.point2d(i)?);
    }
    Ok(())
}

pub fn curve(output: &Output, c: &MultiCurve) -> Result<(), Failure> {
    p::int(output, c.nb_poles());
    p::int(output, c.nb_curves()?);
    for j in 1..=c.nb_poles() {
        point(output, &c.value(j)?)?;
    }
    Ok(())
}
