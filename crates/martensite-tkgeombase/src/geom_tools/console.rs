// Port-only seam for the `std::cout` writes of the implementation,
// GeomTools_*the implementation (Read) and the implementation at .

//! The console GeomTools writes to where the retained code uses `std::cout`: the
//! "UNKNOWN ... TYPE" lines of the compact print, the "Not a ... table" message of
//! the set readers and the debug dump. It is injected so tests can capture it.
use std::io::Write;
use std::sync::Arc;

/// Destination of the text retained GeomTools streams to `std::cout`.
///
/// ```
/// use martensite_tkgeombase::geom_tools::{ConsoleOutput, StdoutConsole};
/// StdoutConsole.write(b"");
/// ```
pub trait ConsoleOutput: Send + Sync {
    /// Writes the bytes exactly as the retained `std::cout <<` would; `console.write(b"x\n")`.
    fn write(&self, bytes: &[u8]);
}

/// The process standard output, the retained destination.
#[derive(Clone, Copy, Debug, Default)]
pub struct StdoutConsole;

impl ConsoleOutput for StdoutConsole {
    fn write(&self, bytes: &[u8]) {
        // A failing `std::cout` never raises natively, so a write error is ignored.
        let _ = std::io::stdout().write_all(bytes);
    }
}

/// The shared standard-output console the default constructors use.
pub(crate) fn stdout_console() -> Arc<dyn ConsoleOutput> {
    Arc::new(StdoutConsole)
}
