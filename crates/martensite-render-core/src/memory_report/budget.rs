//! Estimated process budgets must never be presented as global free memory.
/// Knowledge available for a heap; for example match RenderMemoryBudget::Unavailable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMemoryBudget {
    /// No supported budget query was available; capacity remains a separate measurement.
    Unavailable,
    /// Implementation-estimated current-process budget and usage, both in bytes.
    Estimated {
        /// Advisory allocation budget for the current process.
        budget_bytes: u64,
        /// Estimated current-process heap usage, which may exceed its budget.
        usage_bytes: u64,
    },
}
impl RenderMemoryBudget {
    /// Advisory headroom, not global free bytes; for example budget.remaining_estimate_bytes().
    pub fn remaining_estimate_bytes(self) -> Option<u64> {
        match self {
            Self::Unavailable => None,
            Self::Estimated {
                budget_bytes,
                usage_bytes,
            } => Some(budget_bytes.saturating_sub(usage_bytes)),
        }
    }
}
/// Origin and scope of the sampled information; for example report.source().
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMemorySource {
    /// Supported Vulkan extension reports advisory current-process budgets and usage.
    VulkanBudgetEstimate,
    /// Vulkan reports physical capacity, but the budget extension is unavailable.
    VulkanCapacityOnly,
    /// An injected rendering backend has no physical-device memory provider.
    InjectedUnavailable,
}
