//! An owned reference to the `CAMetalLayer` that backs a Cocoa window's view, which the
//! Vulkan driver hands to `VK_EXT_metal_surface` (the `NSView` itself is not a valid
//! `pLayer`).
use super::appkit::CocoaAppKitCalls;
use std::sync::Arc;

/// Keeps a `CAMetalLayer` alive while a Vulkan surface presents to it; dropping the lease
/// releases its reference. Example: `window.metal_layer()?.layer_address()`.
pub struct CocoaMetalLayerLease {
    calls: Arc<dyn CocoaAppKitCalls>,
    layer: usize,
}

impl CocoaMetalLayerLease {
    /// Takes over one reference the AppKit seam handed out for `layer`.
    pub(super) fn adopt(calls: Arc<dyn CocoaAppKitCalls>, layer: usize) -> Self {
        Self { calls, layer }
    }
    /// The `CAMetalLayer*` address, valid while the lease lives; `lease.layer_address()`.
    pub fn layer_address(&self) -> usize {
        self.layer
    }
}

impl std::fmt::Debug for CocoaMetalLayerLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CocoaMetalLayerLease")
            .field("layer", &format_args!("{:#x}", self.layer))
            .finish()
    }
}

impl Drop for CocoaMetalLayerLease {
    fn drop(&mut self) {
        self.calls.release_object(self.layer);
    }
}
