//! Fixed providers establish byte arithmetic independently of changing live budgets.
mod contracts;
mod rejection;
use super::query::{
    DeviceMemoryQuery, MEMORY_HEAP_LIMIT, QueriedMemoryHeap, QueriedMemoryProperties,
};
use super::*;
use crate::RenderDriverIdentity;
#[derive(Clone)]
struct FixedMemoryQuery {
    result: Result<QueriedMemoryProperties, RenderMemoryQueryError>,
}
impl DeviceMemoryQuery for FixedMemoryQuery {
    fn read_memory_properties(&self) -> Result<QueriedMemoryProperties, RenderMemoryQueryError> {
        // A snapshot clone makes the provider's mutable lifetime independent of the result.
        self.result.clone()
    }
}
fn capacity_provider() -> FixedMemoryQuery {
    let mut heaps = [QueriedMemoryHeap::default(); MEMORY_HEAP_LIMIT];
    heaps[0] = QueriedMemoryHeap {
        flags: 3,
        capacity: 4096,
        ..Default::default()
    };
    FixedMemoryQuery {
        result: Ok(QueriedMemoryProperties {
            heap_count: 1,
            heaps,
            budget_supported: false,
        }),
    }
}
fn mixed_budget_provider() -> FixedMemoryQuery {
    let mut provider = capacity_provider();
    let raw = provider.result.as_mut().unwrap();
    raw.heap_count = 3;
    raw.budget_supported = true;
    raw.heaps[0] = queried_heap(1, 1024, 512, 100);
    raw.heaps[1] = queried_heap(0, 8192, 4096, 4);
    raw.heaps[2] = queried_heap(1, 2048, 1024, 20);
    provider
}
fn identity() -> RenderDriverIdentity {
    RenderDriverIdentity {
        vendor_id: 7,
        device_id: 11,
        driver_id: 13,
        driver_version: 17,
        driver_name: "fixed memory provider".into(),
        driver_info: "owned fixture".into(),
        device_name: "fixed heap device".into(),
    }
}
fn report(provider: &FixedMemoryQuery) -> Result<RenderDeviceMemoryReport, RenderMemoryQueryError> {
    let selected = identity();
    super::query::query_report(provider, selected)
}
fn assert_report_thread_contract() {
    assert_send_sync::<RenderDeviceMemoryReport>();
    assert_send_sync::<RenderDeviceMemoryHeap>();
    assert_send_sync::<RenderMemoryBudget>();
    assert_send_sync::<RenderMemorySource>();
    assert_send_sync::<RenderMemoryQuantity>();
    assert_send_sync::<RenderMemoryQueryError>();
    assert_send_sync::<QueriedMemoryProperties>();
    assert_send_sync::<QueriedMemoryHeap>();
}
fn assert_send_sync<T: Send + Sync>() {
    // The compile-time constraint qualifies owned values, not concurrent driver querying.
    let _: Option<T> = None;
}

fn queried_heap(flags: u32, capacity: u64, budget: u64, usage: u64) -> QueriedMemoryHeap {
    QueriedMemoryHeap {
        flags,
        capacity,
        budget,
        usage,
    }
}
