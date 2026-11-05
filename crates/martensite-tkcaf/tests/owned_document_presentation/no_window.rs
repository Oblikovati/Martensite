//! A named fake refuses implicit native allocations while observing actual nonbatch registry behavior.
use martensite_raster_core::{PointerEvent, RasterReports, RasterWindow, RasterWindowSystem};
use std::cell::Cell;
/// Thread-confined operation counter; tests never share its Cell across threads.
#[derive(Default)]
pub(super) struct NoWindowSystem {
    pub(super) flushes: Cell<usize>,
}
impl RasterWindowSystem for NoWindowSystem {
    fn create_window(
        &self,
        title: &str,
        _: (i32, i32),
        _: (i32, i32),
        _: u64,
    ) -> Box<dyn RasterWindow> {
        // No-window originals must never allocate merely because a drawable was published.
        panic!("unexpected window title={title:?}; expected zero native allocation");
    }
    fn define_color(&self, _: usize, _: &str) -> bool {
        // The unviewed fixture does not consume any palette allocation results.
        true
    }
    fn flush(&self) {
        let count = self.flushes.get() + 1;
        self.flushes.set(count);
    }
    fn next_event(&self) -> PointerEvent {
        // A registry operation never waits for an interactive event.
        PointerEvent::default()
    }
}
/// Stateless report destination; all valid analytic control shapes produce no diagnostic.
pub(super) struct QuietReports;
impl RasterReports for QuietReports {
    fn write_out(&self, text: &str) {
        // An unexpected report is an observable failure, never discarded output.
        assert!(text.is_empty(), "unexpected output report={text:?}");
    }
    fn write_err(&self, text: &str) {
        // Errors remain failures even when a test uses an unviewed renderer.
        assert!(text.is_empty(), "unexpected error report={text:?}");
    }
    fn flush(&self) {
        // This stateless destination is Send+Sync; the geometry fixture itself is not.
        let _: &(dyn Send + Sync) = self;
    }
}
