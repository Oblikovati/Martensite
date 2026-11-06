//! Driver options shared by a driver and its views (the `GlCaps` switches Vulkan keeps).
use martensite_tkernel::standard::SharedState;
use std::sync::Arc;

/// Options a driver applies to all its views; `driver.change_options(|o| o.buffers_no_swap = true)`.
///
/// They follow the native `GlCaps` defaults: vertical sync on (`swapInterval` 1) and frames
/// presented (`buffersNoSwap` off).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderDriverOptions {
    /// Present window frames with vertical sync (the FIFO present mode); native `swapInterval`.
    pub vertical_sync: bool,
    /// Render window views without presenting them; dumps still read the rendered frame.
    pub buffers_no_swap: bool,
    /// Clear the colour alpha to 1 and keep draws from writing it (the default); when off,
    /// frames clear alpha to 0 and every draw writes its alpha. Native `buffersOpaqueAlpha`.
    pub buffers_opaque_alpha: bool,
    /// Keep primitive arrays after their upload; native `keepArrayData`, off by default. The
    /// first ray-traced redraw turns it on and loses the device of the views already redrawn
    /// (driver_device_lost.rs). Vulkan keeps the arrays either way.
    pub keep_array_data: bool,
}

impl Default for RenderDriverOptions {
    fn default() -> Self {
        Self {
            vertical_sync: true,
            buffers_no_swap: false,
            buffers_opaque_alpha: true,
            keep_array_data: false,
        }
    }
}

/// The options instance a driver shares with the views it creates.
pub(crate) type SharedDriverOptions = Arc<SharedState<RenderDriverOptions>>;

impl crate::Graphic3dRenderDriver {
    /// The current options (`GlGraphicDriver::Options`); `driver.options.vertical_sync`.
    pub fn options(&self) -> RenderDriverOptions {
        self.options.read().clone()
    }
    /// Change the options every view of this driver reads on its next redraw
    /// (`GlGraphicDriver::ChangeOptions`).
    /// Example: `driver.change_options(|options| options.vertical_sync = false)`.
    pub fn change_options(&self, change: impl FnOnce(&mut RenderDriverOptions)) {
        change(&mut self.options.write());
    }
    /// Render window views without presenting them (`GlGraphicDriver::SetBuffersNoSwap`).
    /// Example: `driver.set_buffers_no_swap(true)`.
    pub fn set_buffers_no_swap(&self, no_swap: bool) {
        self.options.write().buffers_no_swap = no_swap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_follow_native_caps() {
        let options = RenderDriverOptions::default();
        assert!(options.vertical_sync && !options.buffers_no_swap);
        assert!(options.buffers_opaque_alpha && !options.keep_array_data);
    }
}
