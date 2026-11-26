//! A nonblocking read view of the stored Cylinder3 coefficients.
use super::{CylinderState, GeomCylindricalSurface};
use martensite_tkernel::standard::SharedReadFailure;
use martensite_tkmath::gp::Axis3;
use std::sync::RwLockReadGuard;

/// Thread-confined lease on the original stored geometry cell.
pub struct CylinderInputLease<'a> {
    fields: RwLockReadGuard<'a, CylinderState>,
}

impl GeomCylindricalSurface {
    /// Borrow the original coefficients without waiting. Use `carrier.try_input_lease()?.coefficients()`.
    pub fn try_input_lease(&self) -> Result<CylinderInputLease<'_>, SharedReadFailure> {
        Ok(CylinderInputLease {
            fields: self.state.try_read()?,
        })
    }
}

impl CylinderInputLease<'_> {
    /// Copy the placement and radius while this lease holds the live cell.
    /// `lease.coefficients().1` gives the original stored radius.
    pub fn coefficients(&self) -> (Axis3, f64) {
        (self.fields.pos, self.fields.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_carrier_lease_holds_the_original_value_until_it_drops() {
        let carrier = GeomCylindricalSurface::new(Axis3::default(), 2.).unwrap();
        let lease = carrier.try_input_lease().unwrap();
        assert_eq!(lease.coefficients().1, 2.);
        assert!(carrier.state.try_write().is_none());
        drop(lease);
        carrier.set_radius(4.).unwrap();
        assert_eq!(carrier.try_input_lease().unwrap().coefficients().1, 4.);
    }
}
