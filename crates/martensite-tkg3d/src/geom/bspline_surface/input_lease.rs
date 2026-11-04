//! A held state guard makes all surface coefficient and layout copies coherent.
use super::{GeomBSplineSurface, StoredSplineSurfaceInput, SurfaceState, stored_input};
use martensite_tkernel::standard::{Failure, SharedReadFailure};
use std::sync::RwLockReadGuard;

/// A thread-confined surface input lease; `surface.try_input_lease()?` never waits.
pub struct SplineSurfaceInputLease<'a> {
    fields: RwLockReadGuard<'a, SurfaceState>,
}
impl GeomBSplineSurface {
    /// Borrow the actual stored evaluator state without waiting; `surface.try_input_lease()?`.
    pub fn try_input_lease(&self) -> Result<SplineSurfaceInputLease<'_>, SharedReadFailure> {
        Ok(SplineSurfaceInputLease {
            fields: self.state.try_read()?,
        })
    }
}
impl SplineSurfaceInputLease<'_> {
    /// Inspect poles, weights and U/V knot/multiplicity/flat-knot sizes before allocation; `lease.stored_sizes()`.
    pub fn stored_sizes(&self) -> [usize; 8] {
        stored_input::sizes(&self.fields)
    }
    /// Copy raw numeric fields without geometric admission; `lease.stored_input()?`.
    pub fn stored_input(&self) -> Result<StoredSplineSurfaceInput, Failure> {
        stored_input::copy(&self.fields)
    }
}
#[cfg(test)]
#[path = "input_lease_tests.rs"]
mod tests;
