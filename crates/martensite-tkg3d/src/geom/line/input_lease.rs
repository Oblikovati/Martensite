//! Retain the actual line state while a larger input closure is copied.
use super::GeomLine;
use martensite_tkernel::standard::SharedReadFailure;
use martensite_tkmath::gp::Axis1;
use std::sync::RwLockReadGuard;

/// A thread-confined line read lease; obtain one with `line.try_input_lease()?`.
pub struct LineInputLease<'a> {
    fields: RwLockReadGuard<'a, Axis1>,
}
impl GeomLine {
    /// Borrow without waiting or recovering poison; `line.try_input_lease()?.axis()`.
    pub fn try_input_lease(&self) -> Result<LineInputLease<'_>, SharedReadFailure> {
        Ok(LineInputLease {
            fields: self.state.try_read()?,
        })
    }
}
impl LineInputLease<'_> {
    /// Copy the axis without reacquiring a lock; `lease.axis()`.
    pub fn axis(&self) -> Axis1 {
        *self.fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lease_copies_the_real_axis_and_refuses_the_actual_writer() {
        let line = GeomLine::new(Axis1::default());
        let lease = line.try_input_lease().unwrap();
        assert_eq!(lease.axis(), Axis1::default());
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
