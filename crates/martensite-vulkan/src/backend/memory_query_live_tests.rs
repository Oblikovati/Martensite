//! A real query observer qualifies conversion without comparing separate changing samples.
use super::*;
use crate::{RenderMemoryBudget, RenderMemorySource, VulkanOptions};
use std::sync::Mutex;
struct ObservedPhysicalMemory<'a> {
    actual: PhysicalMemoryQuery<'a>,
    captured: Mutex<Option<QueriedMemoryProperties>>,
}
impl DeviceMemoryQuery for ObservedPhysicalMemory<'_> {
    fn read_memory_properties(&self) -> Result<QueriedMemoryProperties, RenderMemoryQueryError> {
        let raw = self.actual.read_memory_properties()?;
        *self.captured.lock().unwrap() = Some(raw.clone());
        Ok(raw)
    }
}
#[test]
fn actual_physical_properties_match_the_same_owned_sample() {
    let renderer = VulkanRenderer::new(selected_options()).unwrap();
    let query = ObservedPhysicalMemory {
        actual: PhysicalMemoryQuery {
            instance: &renderer.runtime.instance,
            physical: renderer.runtime.physical,
        },
        captured: Mutex::new(None),
    };
    let snapshot = query_report(&query, renderer.driver_identity()).unwrap();
    let raw = query.captured.lock().unwrap().take().unwrap();
    assert_raw_sample(&snapshot, &raw);
    assert!(
        renderer.validation_errors().unwrap().is_empty(),
        "{:?}",
        renderer.validation_errors().unwrap()
    );
}
fn selected_options() -> VulkanOptions {
    let device_index = std::env::var("MARTENSITE_VULKAN_DEVICE").ok().map(|index| {
        index
            .parse()
            .unwrap_or_else(|_| panic!("received {index:?}; expected usize device index"))
    });
    VulkanOptions {
        validation: true,
        device_index,
    }
}
fn assert_raw_sample(snapshot: &RenderDeviceMemoryReport, raw: &QueriedMemoryProperties) {
    assert_eq!(snapshot.heaps().len(), raw.heap_count as usize);
    let source = if raw.budget_supported {
        RenderMemorySource::VulkanBudgetEstimate
    } else {
        RenderMemorySource::VulkanCapacityOnly
    };
    assert_eq!(snapshot.source(), source);
    assert_eq!(snapshot.global_free_bytes(), None);
    for heap in snapshot.heaps() {
        let observed = raw.heaps[heap.index() as usize];
        assert_eq!(heap.capacity_bytes(), observed.capacity);
        assert_eq!(heap.raw_flags(), observed.flags);
        assert_sample_budget(heap, observed, raw.budget_supported);
    }
}
fn assert_sample_budget(
    heap: &crate::RenderDeviceMemoryHeap,
    raw: QueriedMemoryHeap,
    supported: bool,
) {
    let expected = expected_budget(raw, supported);
    assert_eq!(heap.budget(), expected);
    println!(
        "{{\"kind\":\"same-query-heap\",\"index\":{},\"flags\":{},\"capacity\":{},\"supported\":{},\"raw_budget\":{},\"raw_usage\":{}}}",
        heap.index(),
        heap.raw_flags(),
        heap.capacity_bytes(),
        supported,
        raw.budget,
        raw.usage
    );
}

fn expected_budget(raw: QueriedMemoryHeap, supported: bool) -> RenderMemoryBudget {
    if supported {
        RenderMemoryBudget::Estimated {
            budget_bytes: raw.budget,
            usage_bytes: raw.usage,
        }
    } else {
        RenderMemoryBudget::Unavailable
    }
}
