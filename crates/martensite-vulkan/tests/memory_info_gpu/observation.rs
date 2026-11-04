//! Structured numeric records preserve dynamic samples without manufacturing goldens.
use martensite_vulkan::{RenderDeviceMemoryReport, RenderMemoryBudget};
use std::fmt::Write;
/// Record owned device/heap identity; for example record_sample(&driver.memory_report().unwrap()).
pub(super) fn record_sample(sample: &RenderDeviceMemoryReport) {
    record_device(sample);
    for heap in sample.heaps() {
        let (budget, usage) = budget_fields(heap.budget());
        println!(
            "{{\"kind\":\"memory-heap\",\"index\":{},\"flags\":{},\"capacity\":{},\"budget\":{},\"usage\":{}}}",
            heap.index(),
            heap.raw_flags(),
            heap.capacity_bytes(),
            budget,
            usage
        );
    }
}
fn record_device(sample: &RenderDeviceMemoryReport) {
    let device = sample.device().unwrap();
    println!(
        "{{\"kind\":\"memory-device\",\"vendor_id\":{},\"device_id\":{},\"driver_id\":{},\"driver_version\":{},\"name_hex\":\"{}\",\"source\":\"{:?}\",\"global_free_known\":false}}",
        device.vendor_id,
        device.device_id,
        device.driver_id,
        device.driver_version,
        bytes_hex(device.device_name.as_bytes()),
        sample.source()
    );
}
fn budget_fields(budget: RenderMemoryBudget) -> (String, String) {
    match budget {
        RenderMemoryBudget::Unavailable => ("null".into(), "null".into()),
        RenderMemoryBudget::Estimated {
            budget_bytes,
            usage_bytes,
        } => (budget_bytes.to_string(), usage_bytes.to_string()),
    }
}
fn bytes_hex(bytes: &[u8]) -> String {
    let mut encoded = String::new();
    for byte in bytes {
        write!(&mut encoded, "{byte:02x}").unwrap();
    }
    encoded
}
#[test]
fn log_encoding_preserves_control_and_non_ascii_bytes() {
    assert_eq!(bytes_hex(b"a\n\""), "610a22");
    assert_eq!(bytes_hex(&[0xff]), "ff");
    assert_eq!(
        budget_fields(RenderMemoryBudget::Unavailable),
        ("null".into(), "null".into())
    );
}
