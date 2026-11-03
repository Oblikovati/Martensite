//! A nonblocking read view of the stored plane frame.
use super::{GeomPlane, PlaneState};
use martensite_tkernel::standard::SharedReadFailure;
use martensite_tkmath::gp::Axis3;
use std::sync::RwLockReadGuard;

/// A thread-confined plane read lease; obtain it with `plane.try_input_lease()?`.
pub struct PlaneInputLease<'a> {
    fields: RwLockReadGuard<'a, PlaneState>,
}

impl GeomPlane {
    /// Borrow the stored frame without waiting; `plane.try_input_lease()?.position()`.
    pub fn try_input_lease(&self) -> Result<PlaneInputLease<'_>, SharedReadFailure> {
        Ok(PlaneInputLease {
            fields: self.state.try_read()?,
        })
    }
}

impl PlaneInputLease<'_> {
    /// Copy the exact stored local frame while its state remains leased; `lease.position()`.
    pub fn position(&self) -> Axis3 {
        self.fields.pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkmath::gp::Axis3;

    #[test]
    fn plane_lease_blocks_writer_and_copies_frame() {
        let plane = GeomPlane::new(Axis3::default());
        let lease = plane.try_input_lease().unwrap();
        assert_eq!(
            frame_components(lease.position()),
            frame_components(Axis3::default())
        );
        assert!(plane.state.try_write().is_none());
        drop(lease);
        let writer = plane.state.write();
        assert!(matches!(
            plane.try_input_lease(),
            Err(SharedReadFailure::Busy)
        ));
        drop(writer);
        assert!(plane.try_input_lease().is_ok());
    }

    #[test]
    fn plane_lease_refuses_poison_without_recovering_state() {
        let plane = GeomPlane::new(Axis3::default());
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _writer = plane.state.write();
            panic!("NamedPlaneWriterPoison: expected typed poisoned read refusal");
        }));
        assert!(panic.is_err());
        assert!(matches!(
            plane.try_input_lease(),
            Err(SharedReadFailure::Poisoned)
        ));
    }

    fn frame_components(axis: Axis3) -> [[f64; 3]; 4] {
        let origin = axis.location();
        let x = axis.x_direction();
        let y = axis.y_direction();
        let z = axis.direction();
        [
            [origin.x(), origin.y(), origin.z()],
            [x.x(), x.y(), x.z()],
            [y.x(), y.y(), y.z()],
            [z.x(), z.y(), z.z()],
        ]
    }
}
