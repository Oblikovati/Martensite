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
    #[test]
    fn line_lease_refuses_poison_without_recovering_state() {
        let line = Geom2dLine::new(Axis2d::default());
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _writer = line.state.write();
            panic!("NamedLineWriterPoison: expected typed poisoned read refusal");
        }));
        assert!(panic.is_err());
        assert!(matches!(
            line.try_input_lease(),
            Err(SharedReadFailure::Poisoned)
        ));
    }
    #[test]
    fn copied_axis_remains_owned_after_actual_mutation() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Axis2d>();
        let line = Geom2dLine::new(Axis2d::default());
        let copied = line.try_input_lease().unwrap().axis();
        let mut changed = Axis2d::default();
        changed.set_location(martensite_tkmath::gp::Point2::new(2., 3.));
        *line.state.write() = changed;
        assert_eq!(copied, Axis2d::default());
        assert_eq!(line.try_input_lease().unwrap().axis(), changed);
    }
}
