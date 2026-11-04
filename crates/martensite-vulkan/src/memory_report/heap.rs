//! Heap indices retain device identity; memory types must not duplicate heaps.
use super::RenderMemoryBudget;
/// Owned sample of one physical-device heap; for example report.heaps()[0].capacity_bytes().
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderDeviceMemoryHeap {
    index: u32,
    flags: u32,
    capacity: u64,
    budget: RenderMemoryBudget,
}
impl RenderDeviceMemoryHeap {
    /// Retain physical index and raw flag identity; for example new(0, 1, 4096, budget).
    pub(crate) fn new(index: u32, flags: u32, capacity: u64, budget: RenderMemoryBudget) -> Self {
        Self {
            index,
            flags,
            capacity,
            budget,
        }
    }
    /// Read the physical-device heap index; for example heap.index().
    pub fn index(&self) -> u32 {
        // The index identifies a heap, independently of how many memory types refer to it.
        self.index
    }
    /// Preserve all Vulkan heap flag bits; for example heap.raw_flags().
    pub fn raw_flags(&self) -> u32 {
        // Unknown future flag bits remain visible instead of being silently discarded.
        self.flags
    }
    /// Check DEVICE_LOCAL without conflating host-visible memory types; heap.is_device_local().
    pub fn is_device_local(&self) -> bool {
        // Vulkan assigns the DEVICE_LOCAL heap flag bit zero; this is not a type property.
        self.flags & 1 != 0
    }
    /// Physical heap capacity in bytes, not available budget; heap.capacity_bytes().
    pub fn capacity_bytes(&self) -> u64 {
        // Capacity remains useful even when the optional budget query is unavailable.
        self.capacity
    }
    /// Read the tagged current-process estimate; for example heap.budget().
    pub fn budget(&self) -> RenderMemoryBudget {
        // Returning an owned scalar record cannot borrow a driver query buffer.
        self.budget
    }
}
