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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::{GeomLine, GeomSweptSurface};
    use martensite_tkmath::gp::{Axis1, Direction3};

    fn surface() -> GeomSurfaceOfLinearExtrusion {
        let basis = Handle::new(GeomLine::new(Axis1::default())).cast();
        GeomSurfaceOfLinearExtrusion::new(&basis, Direction3::new(0., 0., 1.).unwrap()).unwrap()
    }

    #[test]
    fn extrusion_lease_copies_binding_and_blocks_actual_writer() {
        let surface = surface();
        let expected = surface.basis_curve();
        let lease = surface.try_input_lease().unwrap();
        assert!(lease.basis() == expected);
        assert_eq!(lease.direction(), [0., 0., 1.]);
        assert!(!lease.has_evaluation_override());
        assert!(surface.state.try_write().is_none());
        drop(lease);
        let writer = surface.state.write();
        assert!(matches!(
            surface.try_input_lease(),
            Err(SharedReadFailure::Busy)
        ));
        drop(writer);
        assert!(surface.try_input_lease().is_ok());
    }

    #[test]
    fn extrusion_copied_direction_isolated_after_mutation() {
        let surface = surface();
        let direction = surface.try_input_lease().unwrap().direction();
        surface.state.write().direction = Direction3::new(1., 0., 0.).unwrap();
        assert_eq!(direction, [0., 0., 1.]);
        assert_eq!(surface.try_input_lease().unwrap().direction(), [1., 0., 0.]);
    }

    #[test]
    fn extrusion_lease_reports_actual_poison() {
        let surface = surface();
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _writer = surface.state.write();
            panic!("NamedExtrusionWriterPoison: expected typed poisoned read refusal");
        }));
        assert!(panic.is_err());
        assert!(matches!(
            surface.try_input_lease(),
            Err(SharedReadFailure::Poisoned)
        ));
    }
}
