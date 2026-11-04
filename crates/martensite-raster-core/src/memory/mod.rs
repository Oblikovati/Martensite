//! Display-free raster windows whose captures match a 24-bit X server pixmap byte for byte.
//!
//! Drawing follows the X sample server's frame-buffer rules: zero-width segments with both
//! endpoints, the default zero-line tie bias and error-preserving clipping, text in the
//! 6x13 fixed font at the XDrawString baseline origin, the sixteen raster operations, and
//! clearing as a fill with pixel 0. Captures go through the same image writer.
mod canvas;
mod centered_line;
mod glyphs;
mod line_rule;
mod palette;
mod raster_op;
mod window;
mod zero_clip;
mod zero_line;

use crate::{MAXCOLOR, PointerEvent, RasterWindow, RasterWindowSystem};
pub use line_rule::ZeroWidthLineRule;
use martensite_tkernel::standard::Failure;
use palette::{PALETTE_PIXELS, named_pixel};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
pub use window::MemoryWindow;

/// A window system without a display: every window is an in-memory pixel buffer.
///
/// ```
/// use martensite_raster_core::{MemoryWindowSystem, PixelSegment, RasterWindowSystem};
/// let system = MemoryWindowSystem::new();
/// let mut window = system.create_window("Preview", (0, 0), (8, 8), 0);
/// window.set_color(1);
/// window.draw_segments(&[PixelSegment::new(0, 0, 7, 0)]);
/// assert!(!window.is_mapped());
/// ```
#[derive(Debug)]
pub struct MemoryWindowSystem {
    palette: Rc<RefCell<[u32; MAXCOLOR]>>,
    next_identifier: Cell<u64>,
    line_rule: ZeroWidthLineRule,
}

impl Default for MemoryWindowSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryWindowSystem {
    /// A system with the standard palette of [`crate::COLOR_NAMES`] already defined.
    ///
    /// ```
    /// use martensite_raster_core::{MemoryWindowSystem, RasterWindowSystem};
    /// assert!(MemoryWindowSystem::new().define_color(3, "Gold"));
    /// ```
    pub fn new() -> Self {
        Self::with_line_rule(ZeroWidthLineRule::default())
    }

    /// A system whose windows draw segments with `line_rule`.
    ///
    /// ```
    /// use martensite_raster_core::{MemoryWindowSystem, ZeroWidthLineRule};
    /// let system = MemoryWindowSystem::with_line_rule(ZeroWidthLineRule::Accelerated);
    /// assert_eq!(system.line_rule(), ZeroWidthLineRule::Accelerated);
    /// ```
    pub fn with_line_rule(line_rule: ZeroWidthLineRule) -> Self {
        Self {
            palette: Rc::new(RefCell::new(PALETTE_PIXELS)),
            next_identifier: Cell::new(1),
            line_rule,
        }
    }

    /// The segment rule of every window this system creates; `system.line_rule()`.
    pub fn line_rule(&self) -> ZeroWidthLineRule {
        self.line_rule
    }

    /// Create a concrete window; sizes must lie in `1..=65535` like native windows.
    ///
    /// ```
    /// use martensite_raster_core::{MemoryWindowSystem, RasterWindow};
    /// let system = MemoryWindowSystem::new();
    /// let window = system.try_create_memory_window("Preview", (5, 8), (400, 300)).unwrap();
    /// assert_eq!((window.width(), window.height()), (400, 300));
    /// assert!(system.try_create_memory_window("Bad", (0, 0), (0, 4)).is_err());
    /// ```
    pub fn try_create_memory_window(
        &self,
        title: &str,
        position: (i32, i32),
        size: (i32, i32),
    ) -> Result<MemoryWindow, Failure> {
        let size = checked_size([size.0, size.1])?;
        let identifier = self.next_identifier.get();
        self.next_identifier.set(identifier + 1);
        let palette = Rc::clone(&self.palette);
        Ok(MemoryWindow::new(
            identifier,
            title,
            position,
            size,
            palette,
            self.line_rule,
        ))
    }
}

impl RasterWindowSystem for MemoryWindowSystem {
    fn create_window(
        &self,
        title: &str,
        position: (i32, i32),
        size: (i32, i32),
        existing: u64,
    ) -> Box<dyn RasterWindow> {
        self.try_create_window(title, position, size, existing)
            .expect("memory window allocation failed; use try_create_window to keep the failure")
    }
    fn try_create_window(
        &self,
        title: &str,
        position: (i32, i32),
        size: (i32, i32),
        existing: u64,
    ) -> Result<Box<dyn RasterWindow>, Failure> {
        if existing != 0 {
            return Err(window_failure(&format!("existing window={existing}")));
        }
        let window = self.try_create_memory_window(title, position, size)?;
        Ok(Box::new(window))
    }
    fn define_color(&self, index: usize, name: &str) -> bool {
        let Some(pixel) = named_pixel(name).filter(|_| index < MAXCOLOR) else {
            return false;
        };
        self.palette.borrow_mut()[index] = pixel;
        true
    }
    fn flush(&self) { /* Memory drawing is immediate. */
    }
    fn next_event(&self) -> PointerEvent {
        // Memory windows are never shown, so no pointer event ever arrives.
        PointerEvent::default()
    }
}

/// Accept raster sizes in `1..=65535`; `checked_size([400, 400])`.
fn checked_size(size: [i32; 2]) -> Result<[i32; 2], Failure> {
    if size.iter().any(|value| !(1..=65535).contains(value)) {
        return Err(window_failure(&format!(
            "dimensions={size:?}, expected sizes in 1..=65535"
        )));
    }
    Ok(size)
}

/// A typed allocation failure naming the offending value.
fn window_failure(value: &str) -> Failure {
    Failure::with_type(
        "RasterWindowDefinitionError",
        &format!("Memory raster {value}; expected a new window of positive size"),
        "MemoryWindowSystem",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redefining_a_color_changes_later_selections_only() {
        let system = MemoryWindowSystem::new();
        let mut window = system
            .try_create_memory_window("W", (0, 0), (2, 1))
            .unwrap();
        assert!(system.define_color(1, "Khaki"));
        assert!(!system.define_color(MAXCOLOR, "Red"));
        assert!(!system.define_color(2, "NoSuchColor"));
        window.set_color(1);
        window.draw_segments(&[crate::PixelSegment::new(0, 0, 0, 0)]);
        assert_eq!(window.pixel(0, 0), Some(0xf0_e68c));
        assert_eq!(crate::COLOR_NAMES.len(), MAXCOLOR);
    }

    #[test]
    fn windows_get_distinct_identifiers_and_reject_existing_handles() {
        let system = MemoryWindowSystem::new();
        let first = system.create_window("A", (0, 0), (4, 4), 0);
        let second = system.create_window("B", (0, 0), (4, 4), 0);
        assert!(first.is_window(1) && second.is_window(2) && !first.is_window(2));
        assert!(system.try_create_window("C", (0, 0), (4, 4), 9).is_err());
        assert_eq!(system.next_event(), PointerEvent::default());
    }

    #[test]
    fn sizes_outside_the_native_range_are_typed_failures() {
        assert!(checked_size([1, 65535]).is_ok());
        let failure = checked_size([0, 4]).unwrap_err();
        assert!(format!("{failure:?}").contains("[0, 4]"));
        assert!(checked_size([4, 65536]).is_err());
    }
}
