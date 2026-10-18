//! Checked signed caller origins use initialized real storage without aliased slices.
use super::*;
impl AdvApp2VarIsoRealFrame {
    pub(in crate::adv_app2var) fn output_slot(
        &self,
        origin: usize,
        index: i32,
    ) -> Result<usize, Failure> {
        checked_real_slot(origin, index, self.values.len())
    }
    pub(in crate::adv_app2var) fn output_value(
        &self,
        origin: usize,
        index: i32,
    ) -> Result<f64, Failure> {
        Ok(self.values.value(self.output_slot(origin, index)?))
    }
    pub(in crate::adv_app2var) fn set_output_value(
        &mut self,
        origin: usize,
        index: i32,
        value: f64,
    ) -> Result<(), Failure> {
        let slot = self.output_slot(origin, index)?;
        self.values.set_value(slot, value);
        Ok(())
    }
    pub(in crate::adv_app2var) fn shift_output_values(
        &mut self,
        origin: usize,
        source: i32,
        target: i32,
        bytes: i32,
    ) -> Result<(), Failure> {
        if bytes < 0 || bytes % 8 != 0 {
            return Err(Failure::with_message(
                "Standard_DomainError",
                &format!(
                    "AdvApp2VarIsoRealFrame: copy bytes={bytes}; expected nonnegative multiple of 8"
                ),
            ));
        }
        let count = (bytes / 8) as usize;
        let source = self.output_copy_range(origin, source, count)?;
        let target = self.output_copy_range(origin, target, count)?;
        self.values
            .with_values_mut(|values| values.copy_within(source, target.start));
        Ok(())
    }
    fn output_copy_range(
        &self,
        origin: usize,
        index: i32,
        count: usize,
    ) -> Result<std::ops::Range<usize>, Failure> {
        checked_real_range(origin, index, count, self.values.len())
    }
}

pub(in crate::adv_app2var) fn checked_real_slot(
    origin: usize,
    index: i32,
    capacity: usize,
) -> Result<usize, Failure> {
    origin.checked_add_signed(index as isize).filter(|slot|*slot<capacity).ok_or_else(||Failure::with_message(
        "Standard_OutOfRange",&format!("AdvApp2VarIsoRealFrame: output origin={origin}, index={index}; expected physical real slot in 0..{capacity}")))
}
pub(in crate::adv_app2var) fn checked_real_range(
    origin: usize,
    index: i32,
    count: usize,
    capacity: usize,
) -> Result<std::ops::Range<usize>, Failure> {
    origin.checked_add_signed(index as isize)
        .and_then(|start|start.checked_add(count).map(|end|start..end))
        .filter(|range|range.end<=capacity).ok_or_else(||Failure::with_message("Standard_OutOfRange",&format!(
            "AdvApp2VarIsoRealFrame: copy origin={origin}, index={index}, count={count}; expected range inside 0..={capacity}")))
}

#[cfg(test)]
#[path = "real_cut_output_tests.rs"]
mod tests;
