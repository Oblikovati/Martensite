//! Re-exports the backend-agnostic `FrameWorkLedger` from `martensite-render-core` and adds
//! the Vulkan descriptor-set counting helper (#633).
use ash::vk;
#[cfg(test)]
pub(crate) use martensite_render_core::frame_work_ledger::PhaseAccounts;
pub(crate) use martensite_render_core::frame_work_ledger::{
    DeviceCount, FrameWorkLedger, PhaseGuard, WorkPhase,
};

/// Extension trait adding Vulkan descriptor-set counting to [`FrameWorkLedger`].
pub(crate) trait VulkanWorkLedgerExt {
    /// Count the sets of one successful allocation; `runtime.work.add_sets(&info)`.
    fn add_sets(&self, info: &vk::DescriptorSetAllocateInfo<'_>);
}

impl VulkanWorkLedgerExt for FrameWorkLedger {
    fn add_sets(&self, info: &vk::DescriptorSetAllocateInfo<'_>) {
        self.add_descriptor_sets(u64::from(info.descriptor_set_count));
    }
}

impl super::VulkanRenderer {
    /// The work counters of this device; `renderer.work_ledger().totals().submissions`.
    pub(crate) fn work_ledger(&self) -> std::sync::Arc<FrameWorkLedger> {
        self.runtime.work.clone()
    }
}

#[cfg(test)]
#[path = "frame_work_ledger_tests.rs"]
mod tests;
