//! `GlBackgroundArray::Render`'s chart for the flat backgrounds: the view
//! size the background geometry spans and the world-view matrix under an identity projection.
//! Under a camera tile (a tiled `ToPixMap`) the geometry spans the tile's total size and the
//! matrix scales by the tile size and shifts by the tile offset, so each tile shows its own part
//! of one background.
use crate::{RenderError, error::rejected, identity_clip};
use martensite_tkservice::graphic3d::{Graphic3dCamera, Graphic3dCameraTile};

/// The background geometry a chart places: Render shifts gradients from the lower-left corner
/// and texture quads (built about the origin) from the view centre.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BackgroundGeometry {
    /// Gradient fills, from (0, 0) to the view size (`Graphic3dTOB_GRADIENT`).
    Gradient,
    /// The image quad, centred on the origin (`Graphic3dTOB_TEXTURE`).
    Texture,
}

/// The chart of one background draw. Example:
/// `BackgroundChart::new(&camera, [203, 151], BackgroundGeometry::Gradient)?.world_view()`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BackgroundChart {
    /// `myViewWidth`/`myViewHeight`: the viewport, or the camera tile's total size.
    pub view_size: [f32; 2],
    pub scale: [f32; 2],
    pub shift: [f32; 2],
}
impl BackgroundChart {
    /// The chart of an `extent` viewport under `camera`'s tile; rejects an empty viewport.
    pub(crate) fn new(
        camera: &Graphic3dCamera,
        extent: [u32; 2],
        geometry: BackgroundGeometry,
    ) -> Result<Self, RenderError> {
        if extent.contains(&0) {
            return Err(rejected(extent, "positive background viewport dimensions"));
        }
        let tile = camera.tile();
        if tile.is_valid() {
            return Ok(Self::tiled(&tile, geometry));
        }
        let view_size = extent.map(|size| size as f32);
        let shift = match geometry {
            BackgroundGeometry::Gradient => [-1., -1.],
            BackgroundGeometry::Texture => [0., 0.],
        };
        Ok(Self {
            view_size,
            scale: view_size.map(|size| 2. / size),
            shift,
        })
    }
    /// The camera-tile branch: the geometry spans the total size, shifted by the tile offset.
    fn tiled(tile: &Graphic3dCameraTile, geometry: BackgroundGeometry) -> Self {
        let total = [tile.total_size.x, tile.total_size.y];
        let size = [tile.tile_size.x, tile.tile_size.y];
        let offset = tile.offset_lower_left();
        let offset = [offset.x, offset.y];
        // Each shift is the native float expression, evaluated in the same order.
        let shift = std::array::from_fn(|axis| {
            let tile = size[axis] as f32;
            let offset = 2. * offset[axis] as f32 / tile;
            match geometry {
                BackgroundGeometry::Gradient => -1. - offset,
                BackgroundGeometry::Texture => -1. + total[axis] as f32 / tile - offset,
            }
        });
        Self {
            view_size: total.map(|value| value as f32),
            scale: size.map(|value| 2. / value as f32),
            shift,
        }
    }
    /// The view size in whole pixels, as the gradient geometry takes it.
    pub(crate) fn pixel_size(&self) -> [u32; 2] {
        self.view_size.map(|size| size as u32)
    }
    /// The world-view matrix of the chart (column-major).
    pub(crate) fn world_view(&self) -> [f32; 16] {
        let mut matrix = identity_clip();
        matrix[0] = self.scale[0];
        matrix[5] = self.scale[1];
        matrix[12] = self.shift[0];
        matrix[13] = self.shift[1];
        matrix
    }
}

#[cfg(test)]
#[path = "background_chart_tests.rs"]
mod tests;
