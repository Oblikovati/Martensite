//! Typed causes survive the legacy boolean adapter through the owned report API.
use std::fmt;
/// The total whose checked sum overflowed; for example RenderMemoryQuantity::Capacity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMemoryQuantity {
    /// Physical capacity of device-local heaps.
    Capacity,
    /// Advisory current-process budget of device-local heaps.
    Budget,
    /// Estimated current-process usage of device-local heaps.
    Usage,
}
/// Memory-query operation, offending heap and cause; for example match an ExtensionEnumeration error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderMemoryQueryError {
    /// Driver failed to enumerate physical-device extensions.
    ExtensionEnumeration {
        /// Exact Vulkan result code returned by the driver.
        result: i32,
    },
    /// Driver returned an invalid number of physical heaps.
    HeapCount {
        /// Actual number supplied by the driver.
        received: u32,
        /// Maximum representable Vulkan heap count.
        maximum: u32,
    },
    /// A supported budget violates the extension's nonzero/capacity constraints.
    HeapBudget {
        /// Actual physical heap index.
        heap: u32,
        /// Driver budget in bytes.
        budget: u64,
        /// Physical heap capacity in bytes.
        capacity: u64,
    },
    /// Unused budget array entries must be zero.
    UnusedHeapBudget {
        /// Offending unused array index.
        heap: u32,
        /// Unexpected budget bytes.
        budget: u64,
        /// Unexpected usage bytes.
        usage: u64,
    },
    /// A checked total cannot be represented in the public byte-count type.
    DeviceLocalTotalOverflow {
        /// Which independent quantity overflowed.
        quantity: RenderMemoryQuantity,
        /// Accumulated bytes before adding this heap.
        left: u64,
        /// Offending additional heap bytes.
        right: u64,
    },
}
impl fmt::Display for RenderMemoryQueryError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExtensionEnumeration { result } => write_query_error(out, *result),
            Self::HeapCount { received, maximum } => write_count_error(out, *received, *maximum),
            Self::HeapBudget { .. } => write_budget_error(out, self),
            Self::UnusedHeapBudget { .. } => write_unused_error(out, self),
            Self::DeviceLocalTotalOverflow {
                quantity,
                left,
                right,
            } => write_total_error(out, *quantity, *left, *right),
        }
    }
}
impl std::error::Error for RenderMemoryQueryError {}
fn write_query_error(out: &mut fmt::Formatter<'_>, result: i32) -> fmt::Result {
    write!(
        out,
        "enumerate memory-budget extensions returned {result}; expected Vulkan success"
    )
}
fn write_count_error(out: &mut fmt::Formatter<'_>, received: u32, maximum: u32) -> fmt::Result {
    // Retain the driver count instead of silently truncating the fixed native array.
    write!(out, "memory heap count {received}; expected 1..={maximum}")
}
fn write_budget_error(out: &mut fmt::Formatter<'_>, error: &RenderMemoryQueryError) -> fmt::Result {
    let RenderMemoryQueryError::HeapBudget {
        heap,
        budget,
        capacity,
    } = error
    else {
        unreachable!("only HeapBudget reaches the budget formatter");
    };
    write!(
        out,
        "heap {heap} budget {budget}; expected 0 < budget <= capacity {capacity}"
    )
}
fn write_unused_error(out: &mut fmt::Formatter<'_>, error: &RenderMemoryQueryError) -> fmt::Result {
    let RenderMemoryQueryError::UnusedHeapBudget {
        heap,
        budget,
        usage,
    } = error
    else {
        unreachable!("only UnusedHeapBudget reaches the unused formatter");
    };
    write!(
        out,
        "unused heap {heap} budget={budget}, usage={usage}; expected both zero"
    )
}
fn write_total_error(
    out: &mut fmt::Formatter<'_>,
    quantity: RenderMemoryQuantity,
    left: u64,
    right: u64,
) -> fmt::Result {
    write!(
        out,
        "device-local {quantity:?} total {left} + {right} bytes overflowed; expected a u64 byte count in 0..={}",
        u64::MAX
    )
}
