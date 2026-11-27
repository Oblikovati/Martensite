//! The shader grid of a Vulkan view (GlView::GridDisplay, GridErase, ShaderGridEcho,
//! ShaderGridSnapPoint and the grid part of ZFitAllBounds).
use crate::Graphic3dRenderView;
use crate::shader_grid::{RenderGridPixel, RenderShaderGrid};
use martensite_tkernel::standard::{Failure, SharedState};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{Axis3, Point3};
use martensite_tkservice::aspect::AspectGridParams;
use martensite_tkservice::graphic3d::{
    Graphic3dCView, Graphic3dDataStructureManager as _, Graphic3dVertex,
};

impl Graphic3dRenderView {
    /// A snapshot of the view's shader grid (`myShaderGrid`); `render_view.shader_grid()`.
    pub fn shader_grid(&self) -> RenderShaderGrid {
        self.shader_grid_state().read().clone()
    }
    /// The displayed grid state; read by frame building.
    pub(crate) fn shader_grid_state(&self) -> &SharedState<RenderShaderGrid> {
        self.retained.shader_grid()
    }
    /// `GridDisplay`: show the grid in `plane` with the current camera; the view redraws later.
    pub(crate) fn display_shader_grid(&self, params: &AspectGridParams, plane: &Axis3) {
        let camera = self.camera();
        if let Some(camera) = camera.get() {
            self.shader_grid_state()
                .write()
                .display(params, plane, camera);
        }
        self.invalidate();
    }
    /// `GridErase`; the view redraws later.
    pub(crate) fn erase_shader_grid(&self) {
        self.shader_grid_state().write().erase();
        self.invalidate();
    }
    /// `ShaderGridEcho`: the snapped grid point and its display point under a window pixel.
    pub(crate) fn echo_shader_grid(&self, x: i32, y: i32) -> Option<(Point3, Point3)> {
        let window = self.window();
        let size = window.get().map(|window| window.size())?;
        let camera = self.camera();
        let query = RenderGridPixel {
            size: [size.0, size.1],
            pixel: [x, y],
        };
        let echo = self.shader_grid_state().read().echo(camera.get()?, query)?;
        Some((echo.point, echo.display))
    }
    /// `ShaderGridSnapPoint`: the grid node nearest a world point.
    pub(crate) fn snap_to_shader_grid(&self, point: &Graphic3dVertex) -> Option<Point3> {
        let camera = self.camera();
        let (x, y, z) = point.coord_f64();
        let xyz = martensite_tkmath::gp::Xyz::new(x, y, z);
        self.shader_grid_state()
            .read()
            .snap_point(camera.get()?, xyz)
    }
    /// `ZFitAllBounds`: the layer boxes plus the visible or bounded grid area.
    pub(crate) fn z_fit_bounds_with_grid(&self) -> Result<(BoundingBox3, BoundingBox3), Failure> {
        let primary = self.min_max_values(false)?;
        let mut graphic = self.min_max_values(true)?;
        let camera = self.camera();
        if let Some(camera) = camera.get() {
            self.shader_grid_state()
                .read()
                .add_z_fit_bounds(&mut graphic, camera);
        }
        Ok((primary, graphic))
    }
}

/// A vertex at a double point, rounded to float as Graphic3dVertex::SetCoord.
pub(crate) fn vertex_of(point: Point3) -> Graphic3dVertex {
    Graphic3dVertex::from_f64(point.x(), point.y(), point.z())
}
