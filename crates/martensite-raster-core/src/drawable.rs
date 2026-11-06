//! Drawing-only contracts keep geometry independent of command runtimes.
use crate::RasterDisplay;
use martensite_tkernel::standard::Failure;

/// Thread-confined drawable state shared by views; implement without interpreter hooks.
pub trait RasterDrawable {
    /// Draws geometry using the supplied projection and shared state.
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure>;
    /// Chooses the spatial or planar view category.
    fn is_3d(&self) -> bool;
    /// Returns the cached projected bounds in display order.
    fn bounds(&self) -> [f64; 4];
    /// Records projected bounds after drawing.
    fn set_bounds(&self, bounds: [f64; 4]);
    /// Reports whether the drawable belongs to the display list.
    fn visible(&self) -> bool;
    /// Updates display-list membership state.
    fn set_visible(&self, visible: bool);
}

/// Adapts an existing drawable contract without wrapping or copying its geometry.
/// The adapter is stateless; mutation belongs to the drawable and display.
pub trait DrawableAdapter<D: ?Sized> {
    /// Draws geometry using the supplied projection and shared state.
    fn draw_on(drawable: &D, display: &mut RasterDisplay<'_>) -> Result<(), Failure>;
    /// Chooses the spatial or planar view category.
    fn is_3d(drawable: &D) -> bool;
    /// Returns the cached projected bounds in display order.
    fn bounds(drawable: &D) -> [f64; 4];
    /// Records projected bounds after drawing.
    fn set_bounds(drawable: &D, bounds: [f64; 4]);
    /// Reports whether the drawable belongs to the display list.
    fn visible(drawable: &D) -> bool;
    /// Updates display-list membership state.
    fn set_visible(drawable: &D, visible: bool);
}

/// Direct dispatch for owned drawing-only geometry.
pub struct NativeDrawableAdapter;
impl<D: RasterDrawable + ?Sized> DrawableAdapter<D> for NativeDrawableAdapter {
    fn draw_on(drawable: &D, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        drawable.draw_on(display)
    }
    fn is_3d(drawable: &D) -> bool {
        drawable.is_3d()
    }
    fn bounds(drawable: &D) -> [f64; 4] {
        drawable.bounds()
    }
    fn set_bounds(drawable: &D, bounds: [f64; 4]) {
        drawable.set_bounds(bounds)
    }
    fn visible(drawable: &D) -> bool {
        drawable.visible()
    }
    fn set_visible(drawable: &D, visible: bool) {
        drawable.set_visible(visible)
    }
}
