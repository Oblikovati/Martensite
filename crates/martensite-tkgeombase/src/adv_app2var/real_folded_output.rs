//! Folded output descriptors reuse the same allocation as scalar/curve inputs.
use super::*;
impl AdvApp2VarIsoRealFrame {
    pub(in crate::adv_app2var) fn configure_output_origins(
        &mut self,
        origins: [usize; 2],
    ) -> Result<(), Failure> {
        for origin in origins {
            self.output_slot(origin, 0)?;
        }
        self.values.shared_values();
        Ok(())
    }
    pub(in crate::adv_app2var) fn folded_slots(
        &mut self,
        origins: [usize; 2],
        offset: i32,
    ) -> OwnedFoldedSlots {
        OwnedFoldedSlots::new(self.values.shared_values(), origins, offset)
    }
    pub(in crate::adv_app2var) fn endpoint_slots(
        &mut self,
        origins: [usize; 2],
        offset: i32,
    ) -> OwnedEndpointSlots {
        OwnedEndpointSlots::new(self.values.shared_values(), origins, offset)
    }
    pub(in crate::adv_app2var) fn error_slots(
        &mut self,
        origins: [usize; 2],
        offset: i32,
    ) -> OwnedErrorSlots {
        OwnedErrorSlots::new(self.values.shared_values(), origins, offset)
    }
}
