//! The driver limits an image dump tiles by, and the typed failures that replace the endless
//! tile loop a released viewer or a zero tile would otherwise cause.
//!
//! Natively a view cannot outlive its viewer (`V3dView::MyViewer` is a raw pointer the viewer
//! keeps valid), so `ToPixMap` always reads real `MaxViewDumpSizeX/Y` limits. Here the view holds
//! its viewer weakly: once the viewer is released the limits would read zero, the dump would
//! tile by zero pixels and `ToPixMap` would never return.
use super::V3dView;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::graphic3d::Graphic3dGraphicDriver;

/// The failure type of a dump that cannot read its driver limits or would tile by zero pixels.
pub const V3D_DUMP_WITHOUT_LIMITS: &str = "V3dDumpWithoutLimits";

fn dump_without_limits(message: &str) -> Failure {
    Failure::with_message(V3D_DUMP_WITHOUT_LIMITS, message)
        .with_exception_parent("Standard_ProgramError")
}

impl V3dView {
    /// The driver whose limits a dump tiles by; a typed failure when the viewer was released.
    /// Example: `let driver = view.dump_limits_driver()?;`.
    pub(super) fn dump_limits_driver(&self) -> Result<Handle<dyn Graphic3dGraphicDriver>, Failure> {
        let driver = self.viewer().get().map(|v| v.driver()).unwrap_or_default();
        if driver.is_null() {
            return Err(dump_without_limits(
                "V3dView::ToPixMap viewer=released; expected a live viewer whose driver reports MaxViewDumpSizeX/Y",
            ));
        }
        Ok(driver)
    }
}

/// Checks that a tiled dump advances: each tile spans at least one pixel.
/// Example: `checked_dump_tile(Vec2::new(64, 64))?`.
pub(super) fn checked_dump_tile(tile: Vec2<i32>) -> Result<(), Failure> {
    if tile.x >= 1 && tile.y >= 1 {
        return Ok(());
    }
    Err(dump_without_limits(&format!(
        "V3dView::ToPixMap tile={}x{}; expected a tile of at least 1x1 pixels",
        tile.x, tile.y
    )))
}
