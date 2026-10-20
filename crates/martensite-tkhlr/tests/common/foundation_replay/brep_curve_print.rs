//! One field-order report implementation for owned tokens and legacy stdout adapters.
use super::common::{Sink, flag, pnt2d, real, xy};
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::int_res2d::Domain;
use martensite_tkmath::gp::{Circle2, Ellipse2, Line2, Point2, Xy};

pub(crate) trait CurveFixtureEmitter {
    fn flag(&mut self, value: bool);
    fn real(&mut self, value: f64);
    fn pnt2d(&mut self, value: Point2);
    fn xy(&mut self, value: Xy);
}
impl CurveFixtureEmitter for Sink {
    fn flag(&mut self, value: bool) {
        flag(self, value);
    }
    fn real(&mut self, value: f64) {
        real(self, value);
    }
    fn pnt2d(&mut self, value: Point2) {
        pnt2d(self, value);
    }
    fn xy(&mut self, value: Xy) {
        xy(self, value);
    }
}

/// Report the line frame in its original field order; `lin2d(emit, line)`.
pub fn lin2d(emit: &mut impl CurveFixtureEmitter, l: Line2) {
    emit.pnt2d(l.location());
    emit.xy(l.direction().xy());
}
/// Report the circle frame and radius; `circ2d(emit, circle)`.
pub fn circ2d(emit: &mut impl CurveFixtureEmitter, c: Circle2) {
    emit.pnt2d(c.location());
    emit.xy(c.x_axis().direction().xy());
    emit.xy(c.y_axis().direction().xy());
    emit.real(c.radius());
}
/// Report the ellipse frame and radii; `elips2d(emit, ellipse)`.
pub fn elips2d(emit: &mut impl CurveFixtureEmitter, e: Ellipse2) {
    emit.pnt2d(e.location());
    emit.xy(e.x_axis().direction().xy());
    emit.xy(e.y_axis().direction().xy());
    emit.real(e.major_radius());
    emit.real(e.minor_radius());
}

/// `hb_domain`: the bounds of an IntRes2d domain.
pub fn domain(emit: &mut impl CurveFixtureEmitter, d: &Domain) -> Result<(), Failure> {
    domain_first(emit, d)?;
    domain_last(emit, d)?;
    emit.flag(d.is_closed());
    if d.is_closed() {
        let (a, b) = d.equivalent_parameters();
        emit.real(a);
        emit.real(b);
    }
    Ok(())
}

fn domain_first(emit: &mut impl CurveFixtureEmitter, d: &Domain) -> Result<(), Failure> {
    emit.flag(d.has_first_point());
    if d.has_first_point() {
        emit.real(d.first_parameter()?);
        emit.pnt2d(d.first_point()?);
        emit.real(d.first_tolerance()?);
    }
    Ok(())
}

fn domain_last(emit: &mut impl CurveFixtureEmitter, d: &Domain) -> Result<(), Failure> {
    emit.flag(d.has_last_point());
    if d.has_last_point() {
        emit.real(d.last_parameter()?);
        emit.pnt2d(d.last_point()?);
        emit.real(d.last_tolerance()?);
    }
    Ok(())
}
