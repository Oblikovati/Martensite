//! Iso-parametric curves and the closure tests built on them.
use super::state::SurfaceState;
use crate::geom::GeomBSplineCurve;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::arrays::one_based;
use martensite_tkmath::bsplslib;
use martensite_tkmath::gp::Point3;

impl SurfaceState {
    /// `UIso(U, CheckRational)` (`u`) or `VIso(V, CheckRational)`: the iso curve built
    /// from `BSplSLib::Iso` on the flat knots of the fixed direction, carrying the
    /// knots of the other one. `UIso(U)` is `UIso(U, true)`, as the default argument
    /// of the rational curve constructor makes it natively.
    pub(super) fn iso(
        &self,
        u: bool,
        param: f64,
        check_rational: bool,
    ) -> Result<GeomBSplineCurve, Failure> {
        let (fixed, running) = (self.dir(u), self.dir(!u));
        let count: i32 = self.nb_poles(!u);
        let mut cpoles: Array1<'static, Point3> = one_based(count)?;
        let mut cweights: Array1<'static, f64> = one_based(count)?;
        bsplslib::iso(
            param,
            u,
            &self.poles,
            self.weights_view(),
            &fixed.flat_knots,
            None,
            fixed.deg,
            fixed.periodic,
            &mut cpoles,
            Some(&mut cweights),
        )?;
        let (knots, mults) = (&running.knots, &running.mults);
        if self.rational() {
            return GeomBSplineCurve::with_weights_checked(
                &cpoles,
                &cweights,
                knots,
                mults,
                running.deg,
                running.periodic,
                check_rational,
            );
        }
        GeomBSplineCurve::new(&cpoles, knots, mults, running.deg, running.periodic)
    }
    /// `IsUClosed` (`u`) or `IsVClosed`: periodic, or the iso curves at both bounds
    /// equal within Precision::Confusion.
    pub(super) fn is_closed(&self, u: bool) -> Result<bool, Failure> {
        if self.dir(u).periodic {
            return Ok(true);
        }
        let (first, last) = self.dir(u).bounds()?;
        let first_iso: GeomBSplineCurve = self.iso(u, first, true)?;
        let last_iso: GeomBSplineCurve = self.iso(u, last, true)?;
        first_iso.is_equal(&last_iso, precision::confusion())
    }
}
