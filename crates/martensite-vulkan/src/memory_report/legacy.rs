//! Commit both legacy outputs only when actual memory information exists.
use super::{RenderDeviceMemoryReport, RenderMemoryBudget, RenderMemoryQueryError};
use martensite_tkernel::tcollection::AsciiString;
/// Preserve unavailable outputs transactionally; example write_result(report, &mut free, &mut text).
pub(crate) fn write_result(
    result: Result<RenderDeviceMemoryReport, RenderMemoryQueryError>,
    free: &mut usize,
    text: &mut AsciiString,
) -> bool {
    let Ok(report) = result else {
        return false;
    };
    if report.heaps().is_empty() {
        return false;
    }
    let description = describe_memory_report(&report);
    *free = 0; // Actual global free memory is unknown, even when process budgets are available.
    *text = description.as_str().into();
    true
}
fn describe_memory_report(report: &RenderDeviceMemoryReport) -> String {
    let mut output = String::from("Vulkan memory information; actual global free memory: unknown");
    for heap in report.heaps() {
        output.push_str(&format!(
            "\n  Heap {} capacity: {} bytes; flags: {:#x}",
            heap.index(),
            heap.capacity_bytes(),
            heap.raw_flags()
        ));
        append_process_budget(&mut output, heap.budget());
    }
    output
}
fn append_process_budget(output: &mut String, budget: RenderMemoryBudget) {
    match budget {
        RenderMemoryBudget::Unavailable => output.push_str("; process budget/usage: unknown"),
        RenderMemoryBudget::Estimated { budget_bytes, usage_bytes } => output.push_str(&format!("; process budget estimate: {budget_bytes} bytes; process usage estimate: {usage_bytes} bytes")),
    }
}
