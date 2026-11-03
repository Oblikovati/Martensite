//! An extrusion lease retains the basis binding through closure revalidation.
use super::{ExtrusionState, GeomSurfaceOfLinearExtrusion};
use crate::geom::GeomCurve;
use martensite_tkernel::standard::{Handle, SharedReadFailure};
use std::sync::RwLockReadGuard;

/// A thread-confined extrusion lease; obtain it with `surface.try_input_lease()?`.
pub struct ExtrusionInputLease<'a> {
    fields: RwLockReadGuard<'a, ExtrusionState>,
}
impl GeomSurfaceOfLinearExtrusion {
    /// Borrow the actual extrusion state without waiting; `surface.try_input_lease()?`.
    pub fn try_input_lease(&self) -> Result<ExtrusionInputLease<'_>, SharedReadFailure> {
        Ok(ExtrusionInputLease {
            fields: self.state.try_read()?,
        })
    }
}
impl ExtrusionInputLease<'_> {
    /// Retain the actual basis reference without another getter; `lease.basis()`.
    pub fn basis(&self) -> Handle<dyn GeomCurve> {
        self.fields.basis.clone()
    }
    /// Copy the actual direction components; `lease.direction()`.
    pub fn direction(&self) -> [f64; 3] {
        let d = self.fields.direction;
        [d.x(), d.y(), d.z()]
    }
    /// Refuse delegated evaluators in the caller; `lease.has_evaluation_override()`.
    pub fn has_evaluation_override(&self) -> bool {
        !self.fields.eval_rep.is_null()
    }
}
