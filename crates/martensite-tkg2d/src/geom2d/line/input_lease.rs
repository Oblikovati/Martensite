//! Retain the actual line state while a larger input closure is copied.
use super::Geom2dLine;
use martensite_tkernel::standard::SharedReadFailure;
use martensite_tkmath::gp::Axis2d;
use std::sync::RwLockReadGuard;

/// A thread-confined line read lease; obtain one with `line.try_input_lease()?`.
pub struct PlanarLineInputLease<'a> {
    fields: RwLockReadGuard<'a, Axis2d>,
}
impl Geom2dLine {
    /// Borrow without waiting or recovering poison; `line.try_input_lease()?.axis()`.
    pub fn try_input_lease(&self) -> Result<PlanarLineInputLease<'_>, SharedReadFailure> {
        Ok(PlanarLineInputLease {
            fields: self.state.try_read()?,
        })
    }
}
impl PlanarLineInputLease<'_> {
    /// Copy the axis without reacquiring a lock; `lease.axis()`.
    pub fn axis(&self) -> Axis2d {
        *self.fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lease_copies_the_real_axis_and_refuses_the_actual_writer() {
        let line = Geom2dLine::new(Axis2d::default());
        let lease = line.try_input_lease().unwrap();
        assert_eq!(lease.axis(), Axis2d::default());
        assert!(line.state.try_write().is_none());
        drop(lease);
        let writer = line.state.write();
        assert!(matches!(
            line.try_input_lease(),
            Err(SharedReadFailure::Busy)
        ));
        drop(writer);
        assert!(line.try_input_lease().is_ok());
    }
}
