//! Retained box state is local to each thread-confined drawable.
use crate::{BoxRendering, state::DrawableState};
use martensite_raster_core::{PaletteColor, RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::bnd::OrientedBoundingBox;

/// Owns a copied box and mutable projected state; share only within its drawing thread.
/// Example: `let glyph = BoxGlyph::new(box_value, color);`.
#[derive(Debug)]
pub struct BoxGlyph {
    state: DrawableState,
    rendering: BoxRendering,
}
impl BoxGlyph {
    /// Copies all box values and starts invisible with empty bounds; `BoxGlyph::new(obb,color)`.
    pub fn new(obb: OrientedBoundingBox, color: PaletteColor) -> Self {
        Self {
            state: DrawableState::default(),
            rendering: BoxRendering::new(obb, color),
        }
    }
    /// Borrows the owned box snapshot; `glyph.obb().is_void()`.
    pub fn obb(&self) -> &OrientedBoundingBox {
        self.rendering.obb()
    }
}
impl RasterDrawable for BoxGlyph {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        self.rendering.draw_on(display)
    }
    fn is_3d(&self) -> bool {
        true
    }
    fn bounds(&self) -> [f64; 4] {
        self.state.bounds.get()
    }
    fn set_bounds(&self, bounds: [f64; 4]) {
        self.state.bounds.set(bounds);
    }
    fn visible(&self) -> bool {
        self.state.visible.get()
    }
    fn set_visible(&self, visible: bool) {
        self.state.visible.set(visible);
    }
}
