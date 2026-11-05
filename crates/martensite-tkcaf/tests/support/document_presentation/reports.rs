//! Caller-owned stream forwarding keeps drawing reports outside the geometry components.
use martensite_raster_core::RasterReports;
use std::io::Write;
/// Stateless stream destination; Send+Sync because it retains no stream or mutable state.
pub(crate) struct ProcessReports;
impl RasterReports for ProcessReports {
    fn write_out(&self, text: &str) {
        let mut output = std::io::stdout();
        output.write_all(text.as_bytes()).unwrap();
    }
    fn write_err(&self, text: &str) {
        let mut output = std::io::stderr();
        output.write_all(text.as_bytes()).unwrap();
    }
    fn flush(&self) {
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();
    }
}
