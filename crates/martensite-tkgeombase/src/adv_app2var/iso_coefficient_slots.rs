//! Signed logical indices stay within one initialized, typed caller owner.
use super::*;
pub(in crate::adv_app2var) fn coefficient_slot(
    origin: usize,
    index: i32,
    length: usize,
    bindings: AdvApp2VarIntegerBindings,
) -> Result<usize, Failure> {
    let slot = origin
        .checked_add_signed(index as isize)
        .filter(|slot| *slot < length);
    let slot = slot.ok_or_else(|| Failure::with_message("Standard_OutOfRange", &format!(
        "AdvApp2VarIsoOperationFrame: coefficient origin={origin}, index={index}; expected physical slot in 0..{length}"
    )))?;
    validate_caller_slot("CoefficientCount", slot, bindings)?;
    Ok(slot)
}
impl IntegerFrame {
    pub(in crate::adv_app2var) fn coefficient_slot(
        &self,
        origin: usize,
        index: i32,
    ) -> Result<usize, Failure> {
        coefficient_slot(origin, index, self.values.len(), self.bindings)
    }
    pub(in crate::adv_app2var) fn live_caller_dimension(
        &self,
        fallback: &[i32],
        index: usize,
    ) -> i32 {
        self.caller_dimension_slots
            .as_ref()
            .map_or_else(|| fallback[index], |slots| self.values[slots[index]])
    }
}
