//! Thin owned input isolates validation from Vulkan FFI and supports named fake queries.
use super::{
    RenderDeviceMemoryHeap, RenderDeviceMemoryReport, RenderMemoryBudget, RenderMemoryQueryError,
    RenderMemorySource,
};
use crate::RenderDriverIdentity;
pub(crate) const MEMORY_HEAP_LIMIT: usize = 16;
#[derive(Clone, Copy, Debug, Default)]
/// Pointer-free raw heap sample; for example QueriedMemoryHeap::default().
pub(crate) struct QueriedMemoryHeap {
    pub flags: u32,
    pub capacity: u64,
    pub budget: u64,
    pub usage: u64,
}
#[derive(Clone, Debug)]
/// Preserve the bounded native array until validation; for example snapshot.heap_count.
pub(crate) struct QueriedMemoryProperties {
    pub heap_count: u32,
    pub heaps: [QueriedMemoryHeap; MEMORY_HEAP_LIMIT],
    pub budget_supported: bool,
}
/// Inject operation-local memory I/O without a global provider; for example query.read_memory_properties().
pub(crate) trait DeviceMemoryQuery {
    /// Return an owned raw sample or typed driver cause; for example provider.read_memory_properties().
    fn read_memory_properties(&self) -> Result<QueriedMemoryProperties, RenderMemoryQueryError>;
}
/// Validate an actual provider snapshot; example query_report(&provider, identity).
pub(crate) fn query_report(
    query: &impl DeviceMemoryQuery,
    identity: RenderDriverIdentity,
) -> Result<RenderDeviceMemoryReport, RenderMemoryQueryError> {
    let raw = query.read_memory_properties()?;
    validate_heap_count(raw.heap_count)?;
    validate_unused(&raw)?;
    let heaps = (0..raw.heap_count)
        .map(|index| validated_heap(&raw, index))
        .collect::<Result<Vec<_>, _>>()?;
    let source = if raw.budget_supported {
        RenderMemorySource::VulkanBudgetEstimate
    } else {
        RenderMemorySource::VulkanCapacityOnly
    };
    RenderDeviceMemoryReport::from_heaps(identity, source, heaps)
}
fn validate_heap_count(received: u32) -> Result<(), RenderMemoryQueryError> {
    if received == 0 || received as usize > MEMORY_HEAP_LIMIT {
        return Err(RenderMemoryQueryError::HeapCount {
            received,
            maximum: MEMORY_HEAP_LIMIT as u32,
        });
    }
    Ok(())
}
fn validate_unused(raw: &QueriedMemoryProperties) -> Result<(), RenderMemoryQueryError> {
    if !raw.budget_supported {
        return Ok(());
    }
    for (index, heap) in raw.heaps.iter().enumerate().skip(raw.heap_count as usize) {
        if heap.budget != 0 || heap.usage != 0 {
            return Err(RenderMemoryQueryError::UnusedHeapBudget {
                heap: index as u32,
                budget: heap.budget,
                usage: heap.usage,
            });
        }
    }
    Ok(())
}
fn validated_heap(
    raw: &QueriedMemoryProperties,
    index: u32,
) -> Result<RenderDeviceMemoryHeap, RenderMemoryQueryError> {
    let heap = raw.heaps[index as usize];
    let budget = validated_budget(heap, index, raw.budget_supported)?;
    Ok(RenderDeviceMemoryHeap::new(
        index,
        heap.flags,
        heap.capacity,
        budget,
    ))
}
fn validated_budget(
    heap: QueriedMemoryHeap,
    index: u32,
    supported: bool,
) -> Result<RenderMemoryBudget, RenderMemoryQueryError> {
    if !supported {
        return Ok(RenderMemoryBudget::Unavailable);
    }
    if heap.budget == 0 || heap.budget > heap.capacity {
        return Err(RenderMemoryQueryError::HeapBudget {
            heap: index,
            budget: heap.budget,
            capacity: heap.capacity,
        });
    }
    Ok(RenderMemoryBudget::Estimated {
        budget_bytes: heap.budget,
        usage_bytes: heap.usage,
    })
}
