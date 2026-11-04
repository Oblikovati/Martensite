//! Owned per-device memory samples distinguish physical capacity, estimates and unknown free memory.
mod budget;
mod error;
mod heap;
pub(crate) mod legacy;
pub(crate) mod query;
mod totals;
use crate::RenderDriverIdentity;
pub use budget::{RenderMemoryBudget, RenderMemorySource};
pub use error::{RenderMemoryQuantity, RenderMemoryQueryError};
pub use heap::RenderDeviceMemoryHeap;
use totals::DeviceLocalTotals;
/// Immutable owned memory sample; for example driver.memory_report()?.heaps().
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderDeviceMemoryReport {
    device: Option<RenderDriverIdentity>,
    source: RenderMemorySource,
    heaps: Box<[RenderDeviceMemoryHeap]>,
    local: DeviceLocalTotals,
}
impl RenderDeviceMemoryReport {
    /// Aggregate validated distinct heaps with checked byte arithmetic; for example from_heaps(device, source, heaps).
    pub(crate) fn from_heaps(
        device: RenderDriverIdentity,
        source: RenderMemorySource,
        heaps: Vec<RenderDeviceMemoryHeap>,
    ) -> Result<Self, RenderMemoryQueryError> {
        let local = DeviceLocalTotals::from_heaps(&heaps)?;
        Ok(Self {
            device: Some(device),
            source,
            heaps: heaps.into_boxed_slice(),
            local,
        })
    }
    /// Preserve the absence of a physical provider; for example RenderDeviceMemoryReport::unavailable().
    pub(crate) fn unavailable() -> Self {
        Self {
            device: None,
            source: RenderMemorySource::InjectedUnavailable,
            heaps: Box::default(),
            local: DeviceLocalTotals::default(),
        }
    }
    /// Selected device provenance, absent for injected providers; for example report.device().
    pub fn device(&self) -> Option<&RenderDriverIdentity> {
        // The report owns this identity after the renderer or instance is released.
        self.device.as_ref()
    }
    /// Identify capacity-only or estimated information; for example report.source().
    pub fn source(&self) -> RenderMemorySource {
        // The source prevents unsupported budgets from masquerading as measured zeros.
        self.source
    }
    /// Read physical-index-ordered owned heap samples; for example report.heaps().len().
    pub fn heaps(&self) -> &[RenderDeviceMemoryHeap] {
        // No Vulkan query buffers, pointers or locks escape through this slice.
        &self.heaps
    }
    /// Checked sum of distinct DEVICE_LOCAL heaps; report.device_local_capacity_bytes().
    pub fn device_local_capacity_bytes(&self) -> u64 {
        // Memory-type aliases do not appear in the heap-index-ordered sample.
        self.local.capacity
    }
    /// Estimated process budget/usage totals, never free VRAM; report.device_local_budget().
    pub fn device_local_budget(&self) -> RenderMemoryBudget {
        self.local.estimate.map_or(
            RenderMemoryBudget::Unavailable,
            |(budget_bytes, usage_bytes)| RenderMemoryBudget::Estimated {
                budget_bytes,
                usage_bytes,
            },
        )
    }
    /// Actual global free bytes remain unknown; for example assert_eq!(report.global_free_bytes(), None).
    pub fn global_free_bytes(&self) -> Option<u64> {
        // VK_EXT_memory_budget estimates process usage, not global free physical memory.
        None
    }
}

#[cfg(test)]
mod tests;
