//! Process and owned recording sinks for the existing raster reporting interface.
use crate::RasterReports;
use std::cell::RefCell;
use std::io::Write;

/// The process's standard output and error.
///
/// ```
/// use martensite_raster_core::{RasterReports, ProcessReports};
/// ProcessReports.flush();
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct ProcessReports;

impl RasterReports for ProcessReports {
    fn write_out(&self, text: &str) {
        // `std::ostream` reports write failures through state bits command-host never reads.
        let _ = std::io::stdout().write_all(text.as_bytes());
    }

    fn write_err(&self, text: &str) {
        let _ = std::io::stderr().write_all(text.as_bytes());
    }

    fn flush(&self) {
        let _ = std::io::stderr().flush();
        let _ = std::io::stdout().flush();
    }
}

/// Invocation-local text recording (the test double of [`ProcessReports`]).
/// Its RefCell buffers remain thread-confined; returned strings own their bytes.
///
/// ```
/// use martensite_raster_core::{RasterReports, RecordingReports};
/// let console = RecordingReports::default();
/// console.write_out("a");
/// console.write_err("b");
/// assert_eq!(console.output(), "a");
/// assert_eq!(console.errors(), "b");
/// ```
#[derive(Debug, Default)]
pub struct RecordingReports {
    output: RefCell<String>,
    errors: RefCell<String>,
}

impl RecordingReports {
    /// Copy all output without clearing it; `let text = reports.output()`.
    pub fn output(&self) -> String {
        self.output.borrow().clone()
    }

    /// Copy all errors without clearing them; `let text = reports.errors()`.
    pub fn errors(&self) -> String {
        self.errors.borrow().clone()
    }

    /// Take output while retaining errors; `let text = reports.take_output()`.
    pub fn take_output(&self) -> String {
        std::mem::take(&mut *self.output.borrow_mut())
    }
}

impl RasterReports for RecordingReports {
    fn write_out(&self, text: &str) {
        self.output.borrow_mut().push_str(text);
    }

    fn write_err(&self, text: &str) {
        self.errors.borrow_mut().push_str(text);
    }

    fn flush(&self) {}
}

#[cfg(test)]
#[path = "report_streams_tests.rs"]
mod tests;
