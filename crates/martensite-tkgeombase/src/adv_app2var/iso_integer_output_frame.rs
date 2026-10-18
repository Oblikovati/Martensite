//! Stage access to actual integer slots does not keep a guard across user code.
use super::*;
use crate::adv_app2var::approx_f2var::IsoApproxRequest;
impl IntegerFrame {
    pub(in crate::adv_app2var) fn validate_output_slots(
        &self,
        slots: [usize; 2],
    ) -> Result<(), Failure> {
        for (name, slot) in [("Status", slots[0]), ("CompletedPieces", slots[1])] {
            if slot >= self.values.len() {
                return Err(Failure::with_message(
                    "Standard_OutOfRange",
                    &format!(
                        "AdvApp2VarIsoOperationFrame: {name} slot={slot}; expected 0..{}",
                        self.values.len()
                    ),
                ));
            }
            validate_caller_slot(name, slot, self.bindings)?;
        }
        Ok(())
    }
    pub(in crate::adv_app2var) fn output_slot(&self, slot: usize) -> i32 {
        self.values[slot]
    }
    pub(in crate::adv_app2var) fn set_output_slot(&mut self, slot: usize, value: i32) {
        self.values[slot] = value;
    }
    pub(in crate::adv_app2var) fn refresh_iso_request(&self, request: &mut IsoApproxRequest<'_>) {
        request.ndimen = self.values[self.bindings.dimension];
        request.isofav = self.values[self.bindings.favorite_iso];
        if let Some(slot) = self.subspace_slot {
            request.nbsesp = self.values[slot];
        }
        if let Some(slots) = self.caller_order_slots {
            [request.iordre, request.ideriv] = slots.map(|slot| self.values[slot]);
        }
        if let Some(slots) = self.caller_limit_slots {
            [request.nbcrmx, request.ncflim] = slots.map(|slot| self.values[slot]);
        }
        if let Some(slots) = self.caller_sampling_slots {
            [request.nbroot, request.ndgjac] = slots.map(|slot| self.values[slot]);
        }
    }
    pub(in crate::adv_app2var) fn validate_iso_request(
        &self,
        request: &IsoApproxRequest<'_>,
    ) -> Result<(), Failure> {
        if let Some(slots) = &self.caller_dimension_slots
            && (slots.len() != request.ndimse.len()
                || !slots
                    .iter()
                    .zip(request.ndimse)
                    .all(|(slot, value)| self.values[*slot] == *value))
        {
            return Err(Failure::with_message(
                "Standard_DomainError",
                &format!(
                    "AdvApp2VarIsoOperationFrame: caller dimension input={:?}; expected initialized dimension slots={:?}",
                    request.ndimse,
                    self.caller_dimensions()
                ),
            ));
        }
        let mut actual = *request;
        self.refresh_iso_request(&mut actual);
        let expected = [
            request.ndimen,
            request.isofav,
            request.nbsesp,
            request.iordre,
            request.ideriv,
            request.nbcrmx,
            request.ncflim,
            request.nbroot,
            request.ndgjac,
        ];
        let actual = [
            actual.ndimen,
            actual.isofav,
            actual.nbsesp,
            actual.iordre,
            actual.ideriv,
            actual.nbcrmx,
            actual.ncflim,
            actual.nbroot,
            actual.ndgjac,
        ];
        if expected == actual {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_DomainError",
            &format!(
                "AdvApp2VarIsoOperationFrame: request integer fields={expected:?}; expected initialized caller fields={actual:?}"
            ),
        ))
    }
}
