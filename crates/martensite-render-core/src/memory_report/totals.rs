//! Aggregate each physical heap once, preserving estimate and capacity scopes.
use super::{
    RenderDeviceMemoryHeap, RenderMemoryBudget, RenderMemoryQuantity, RenderMemoryQueryError,
};
#[derive(Clone, Debug, Default, PartialEq, Eq)]
/// Own checked totals without caching device state; for example DeviceLocalTotals::from_heaps(&heaps).
pub(super) struct DeviceLocalTotals {
    pub capacity: u64,
    pub estimate: Option<(u64, u64)>,
}
impl DeviceLocalTotals {
    /// Add each device-local heap once; for example DeviceLocalTotals::from_heaps(&heaps).
    pub fn from_heaps(heaps: &[RenderDeviceMemoryHeap]) -> Result<Self, RenderMemoryQueryError> {
        let mut total = Self::default();
        for heap in heaps.iter().filter(|heap| heap.is_device_local()) {
            total.capacity = checked_device_local_total(
                total.capacity,
                heap.capacity_bytes(),
                RenderMemoryQuantity::Capacity,
            )?;
            accumulate_budget(&mut total, heap.budget())?;
        }
        Ok(total)
    }
}
fn accumulate_budget(
    total: &mut DeviceLocalTotals,
    budget: RenderMemoryBudget,
) -> Result<(), RenderMemoryQueryError> {
    if let RenderMemoryBudget::Estimated {
        budget_bytes,
        usage_bytes,
    } = budget
    {
        let (old_budget, old_usage) = total.estimate.unwrap_or_default();
        total.estimate = Some((
            checked_device_local_total(old_budget, budget_bytes, RenderMemoryQuantity::Budget)?,
            checked_device_local_total(old_usage, usage_bytes, RenderMemoryQuantity::Usage)?,
        ));
    }
    Ok(())
}
fn checked_device_local_total(
    left: u64,
    right: u64,
    quantity: RenderMemoryQuantity,
) -> Result<u64, RenderMemoryQueryError> {
    left.checked_add(right)
        .ok_or(RenderMemoryQueryError::DeviceLocalTotalOverflow {
            quantity,
            left,
            right,
        })
}
