//! The work column of the Rust case lines: the device work of a case's timed windows per
//! operation, read from `Graphic3dRenderDriver::device_work` (phase 0 of PLAN.md). The C++
//! twins print no work column; `cargo xtask occt-parity` shows it beside the comparison.
use martensite_vulkan::RenderDeviceWork;
use std::time::Duration;

/// One column: its name, with the unit, and its value in one window's work.
type WorkColumn = (&'static str, fn(&RenderDeviceWork) -> f64);

/// The columns in printed order; times in milliseconds, bytes in KiB.
const COLUMNS: [WorkColumn; 16] = [
    ("redraws", |work| work.redraws as f64),
    ("buffers", |work| work.buffers_created as f64),
    ("images", |work| work.images_created as f64),
    ("allocations", |work| work.memory_allocations as f64),
    ("pipelines", |work| work.pipelines_created as f64),
    ("sets", |work| work.descriptor_sets_allocated as f64),
    ("submits", |work| work.submissions as f64),
    ("upload_kib", |work| work.uploaded_bytes as f64 / 1024.),
    ("readback_kib", |work| work.read_back_bytes as f64 / 1024.),
    ("redraw_ms", |work| ms(work.times.redraw)),
    ("scene_ms", |work| ms(work.times.scene)),
    ("upload_ms", |work| ms(work.times.upload)),
    ("record_ms", |work| ms(work.times.recording)),
    ("submit_ms", |work| ms(work.times.submission)),
    ("wait_ms", |work| ms(work.times.waiting)),
    ("readback_ms", |work| ms(work.times.readback)),
];

fn ms(time: Duration) -> f64 {
    time.as_secs_f64() * 1e3
}

/// The work of timed windows and their operations; `windows.add(work, ops)`.
#[derive(Default)]
pub struct TimedWork {
    windows: Vec<(RenderDeviceWork, usize)>,
}

impl TimedWork {
    /// Adds one timed window of `ops` operations.
    pub fn add(&mut self, work: RenderDeviceWork, ops: usize) {
        self.windows.push((work, ops));
    }

    /// The column as `name=value` pairs per operation; `work.column()`.
    pub fn column(&self) -> String {
        let ops: usize = self.windows.iter().map(|(_, ops)| ops).sum();
        let per_op = |value: fn(&RenderDeviceWork) -> f64| {
            let total: f64 = self.windows.iter().map(|(work, _)| value(work)).sum();
            total / ops.max(1) as f64
        };
        let pairs: Vec<String> = (COLUMNS.iter())
            .map(|(name, value)| format!("{name}={:.3}", per_op(*value)))
            .collect();
        pairs.join(" ")
    }
}
