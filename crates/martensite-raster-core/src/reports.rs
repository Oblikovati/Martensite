//! Caller-owned reporting keeps geometry rendering independent of command runtimes.
/// Injected text reports from geometry rendering; no process or session state is required.
pub trait RasterReports {
    /// Writes to the standard output stream (`std::cout << text`).
    fn write_out(&self, text: &str);
    /// Writes to the standard error stream (`std::cerr << text`).
    fn write_err(&self, text: &str);
    /// Flushes both streams (`flush_standard_streams`).
    fn flush(&self);
}
