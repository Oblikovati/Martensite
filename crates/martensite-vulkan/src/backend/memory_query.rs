//! Query retained physical-device properties; budgets are optional and process scoped.
use super::VulkanRenderer;
use crate::memory_report::query::{
    DeviceMemoryQuery, QueriedMemoryHeap, QueriedMemoryProperties, query_report,
};
use crate::{RenderDeviceMemoryReport, RenderMemoryQueryError};
use ash::{Instance, vk};
struct PhysicalMemoryQuery<'a> {
    instance: &'a Instance,
    physical: vk::PhysicalDevice,
}
impl DeviceMemoryQuery for PhysicalMemoryQuery<'_> {
    fn read_memory_properties(&self) -> Result<QueriedMemoryProperties, RenderMemoryQueryError> {
        let extensions = memory_extensions(self.instance, self.physical)?;
        let supported = budget_is_advertised(&extensions);
        let mut budgets = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
        let memory = query_properties(self.instance, self.physical, supported, &mut budgets);
        Ok(owned_properties(memory, budgets, supported))
    }
}
fn memory_extensions(
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> Result<Vec<vk::ExtensionProperties>, RenderMemoryQueryError> {
    // The physical device belongs to the retained instance for this entire borrowed query.
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.enumerate_device_extension_properties(physical) }.map_err(|result| {
        RenderMemoryQueryError::ExtensionEnumeration {
            result: result.as_raw(),
        }
    })
}
fn budget_is_advertised(extensions: &[vk::ExtensionProperties]) -> bool {
    let expected = ash::ext::memory_budget::NAME.to_bytes_with_nul();
    extensions.iter().any(|extension| {
        expected
            .iter()
            .enumerate()
            .all(|(index, byte)| extension.extension_name[index] as u8 == *byte)
    })
}
fn query_properties(
    instance: &Instance,
    physical: vk::PhysicalDevice,
    supported: bool,
    budget: &mut vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'_>,
) -> vk::PhysicalDeviceMemoryProperties {
    let mut properties = vk::PhysicalDeviceMemoryProperties2::default();
    if supported {
        properties = properties.push_next(budget);
    }
    // Both output structs and their pNext link remain live; no pointer escapes this call.
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_memory_properties2(physical, &mut properties) };
    properties.memory_properties
}
fn owned_properties(
    memory: vk::PhysicalDeviceMemoryProperties,
    budget: vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'_>,
    supported: bool,
) -> QueriedMemoryProperties {
    QueriedMemoryProperties {
        heap_count: memory.memory_heap_count,
        budget_supported: supported,
        heaps: std::array::from_fn(|index| QueriedMemoryHeap {
            capacity: memory.memory_heaps[index].size,
            flags: memory.memory_heaps[index].flags.as_raw(),
            budget: budget.heap_budget[index],
            usage: budget.heap_usage[index],
        }),
    }
}
impl VulkanRenderer {
    /// Sample owned capacity and optional process estimates; for example renderer.memory_report()?.heaps().
    /// Actual global free memory remains unknown; the values are advisory and may change immediately.
    pub fn memory_report(&self) -> Result<RenderDeviceMemoryReport, RenderMemoryQueryError> {
        let query = PhysicalMemoryQuery {
            instance: &self.runtime.instance,
            physical: self.runtime.physical,
        };
        query_report(&query, self.driver_identity())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn budget_advertisement_requires_the_complete_terminated_name() {
        let mut extension = vk::ExtensionProperties::default();
        let bytes = ash::ext::memory_budget::NAME.to_bytes_with_nul();
        for (slot, byte) in extension.extension_name.iter_mut().zip(bytes) {
            *slot = *byte as _;
        }
        assert!(budget_is_advertised(&[extension]));
        extension.extension_name[bytes.len() - 1] = b'X' as _;
        assert!(!budget_is_advertised(&[extension]));
        assert!(!budget_is_advertised(&[]));
    }
    #[test]
    fn owned_query_copies_heap_values_without_memory_type_duplication() {
        let memory = shared_heap_properties();
        let raw = owned_properties(memory, Default::default(), false);
        assert_eq!(
            raw.heaps.len(),
            crate::memory_report::query::MEMORY_HEAP_LIMIT
        );
        assert_eq!(raw.heap_count, 1);
        assert_eq!(raw.heaps[0].capacity, 1024);
        assert_eq!(
            raw.heaps[0].flags,
            vk::MemoryHeapFlags::DEVICE_LOCAL.as_raw()
        );
        assert_eq!(raw.heaps[1].capacity, 0);
    }
    #[test]
    fn owned_query_retains_nonzero_process_budget_and_usage() {
        let mut budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
        budget.heap_budget[0] = 768;
        budget.heap_usage[0] = 128;
        let raw = owned_properties(shared_heap_properties(), budget, true);
        assert!(raw.budget_supported);
        assert_eq!(raw.heaps[0].budget, 768);
        assert_eq!(raw.heaps[0].usage, 128);
        assert_eq!(raw.heaps[1].budget, 0);
    }
    fn shared_heap_properties() -> vk::PhysicalDeviceMemoryProperties {
        let mut memory = vk::PhysicalDeviceMemoryProperties {
            memory_heap_count: 1,
            memory_type_count: 2,
            ..Default::default()
        };
        memory.memory_heaps[0] = vk::MemoryHeap {
            size: 1024,
            flags: vk::MemoryHeapFlags::DEVICE_LOCAL,
        };
        memory
    }
}

#[cfg(test)]
#[path = "memory_query_live_tests.rs"]
mod live_tests;
