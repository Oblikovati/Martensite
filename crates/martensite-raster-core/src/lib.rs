//! Native raster window contracts shared by geometry rendering and capture support.
//! Window objects are thread-confined; the Linux X11 backend requires a live X server,
//! while the in-memory backend draws the same pixels on every platform without a display.
mod memory;
mod segment;
mod window;
pub use memory::{MemoryWindow, MemoryWindowSystem, ZeroWidthLineRule};
pub use segment::PixelSegment;
pub use window::{BUTTON_PRESS, MOTION_NOTIFY, PointerEvent, RasterWindow, RasterWindowSystem};
#[cfg(target_os = "linux")]
mod x11;
#[cfg(target_os = "linux")]
pub use x11::{RasterFailureContext, VirtualWindowSystem, WindowFailureContext};
/// Number of colors in the standard capture palette.
pub const MAXCOLOR: usize = 15;
/// Stable color names keep indexed geometry captures repeatable.
pub const COLOR_NAMES: [&str; MAXCOLOR] = [
    "White", "Red", "Green", "Blue", "Cyan", "Gold", "Magenta", "Maroon", "Orange", "Pink",
    "Salmon", "Violet", "Yellow", "Khaki", "Coral",
];

pub mod color;
pub mod display;
pub mod drawable;
pub mod ostream;
#[doc(hidden)]
pub mod shared_buffer;
pub mod view;
pub mod viewer;
pub use color::{MarkerShape, PaletteColor, PaletteKind};
pub use display::{RasterBounds, RasterDisplay};
pub use ostream::{FormattedText, format_scientific};
pub use view::RasterView;
pub use viewer::{RasterSelection, RasterViewer};

mod mesh_edges;
mod reports;
pub mod sampling;
pub use mesh_edges::MeshEdgeSets;
pub use reports::RasterReports;

mod report_streams;
pub use report_streams::{ProcessReports, RecordingReports};
