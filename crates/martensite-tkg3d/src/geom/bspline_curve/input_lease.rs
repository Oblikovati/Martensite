//! A spline lease feeds the existing owned admission without a second read.
use super::{BSplineState, GeomBSplineCurve, closed_extrusion_capture::capture_leased_basis};
use crate::geom::{ClosedSplineExtrusionError, ClosedSplineExtrusionInput};
use martensite_tkernel::standard::SharedReadFailure;
use std::sync::RwLockReadGuard;

/// A thread-confined spline read lease; obtain it with `curve.try_input_lease()?`.
pub struct SplineInputLease<'a> {
    fields: RwLockReadGuard<'a, BSplineState>,
}
impl GeomBSplineCurve {
    /// Borrow the actual evaluator state without waiting; `curve.try_input_lease()?`.
    pub fn try_input_lease(&self) -> Result<SplineInputLease<'_>, SharedReadFailure> {
        Ok(SplineInputLease {
            fields: self.state.try_read()?,
        })
    }
}
impl SplineInputLease<'_> {
    /// Copy the stored closed-extrusion input; `lease.closed_extrusion_input([0.,0.,1.], [-1.,1.])?`.
    pub fn closed_extrusion_input(
        &self,
        direction: [f64; 3],
        interval: [f64; 2],
    ) -> Result<ClosedSplineExtrusionInput, ClosedSplineExtrusionError> {
        capture_leased_basis(&self.fields, direction, interval)
    }
}
