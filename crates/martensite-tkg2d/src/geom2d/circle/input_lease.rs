//! A nonblocking read view of the stored Circle2 coefficients.
use super::{CircleState, Geom2dCircle};
use martensite_tkernel::standard::SharedReadFailure;
use martensite_tkmath::gp::Axis22d;
use std::sync::RwLockReadGuard;

/// Thread-confined lease on the original stored geometry cell.
pub struct PlanarCircleInputLease<'a> {
    fields: RwLockReadGuard<'a, CircleState>,
}

impl Geom2dCircle {
    /// Borrow the original coefficients without waiting. Use `carrier.try_input_lease()?.coefficients()`.
    pub fn try_input_lease(&self) -> Result<PlanarCircleInputLease<'_>, SharedReadFailure> {
        Ok(PlanarCircleInputLease {
            fields: self.state.try_read()?,
        })
    }
}

impl PlanarCircleInputLease<'_> {
    /// Copy the placement and radius while this lease holds the live cell.
    /// `lease.coefficients().1` gives the original stored radius.
    pub fn coefficients(&self) -> (Axis22d, f64) {
        (self.fields.pos, self.fields.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_carrier_lease_holds_the_original_value_until_it_drops() {
        let carrier = Geom2dCircle::new(Axis22d::default(), 2.).unwrap();
        let lease = carrier.try_input_lease().unwrap();
        assert_eq!(lease.coefficients().1, 2.);
        assert!(carrier.state.try_write().is_none());
        drop(lease);
        carrier.set_radius(4.).unwrap();
        assert_eq!(carrier.try_input_lease().unwrap().coefficients().1, 4.);
    }
}
